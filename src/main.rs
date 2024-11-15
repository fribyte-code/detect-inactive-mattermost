mod apitypes;
use crate::apitypes::{Metadata, PostsRoot, Reaction, UsersRoot};

use chrono::DateTime;
use mattermost_api::prelude::*;
use passterm::prompt_password_tty;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use tokio;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Get credentials from user input
    let mut username = String::new();
    print!("Your chat.fribyte.no username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    username.pop();

    let password = prompt_password_tty(Some("Password: ")).unwrap();

    println!("Reading from chat.fribyte.no as {}...\n", username);

    let auth = AuthenticationData::from_password(username, password);

    // Create an API client and find the friByte team
    let mut api = Mattermost::new("https://chat.fribyte.no", auth).unwrap();

    if let Err(e) = api.store_session_token().await {
        println!("Could not authenticate with Mattermost: {}", e);
        return Err(());
    }

    let fribyte_team_id = api.get_team_by_name("friByte").await.unwrap().id;

    // Get all active users in the server
    let users: UsersRoot = api
        .query(
            "GET",
            "https://chat.fribyte.no/api/v4/users",
            Some(&[
                ("per_page", "1000"),
                ("in_team", &fribyte_team_id),
                ("roles", "system_admin,system_user"),
                ("active", "true"),
            ]),
            None,
        )
        .await
        .unwrap();

    let user_ids_names: HashMap<String, String> = users
        .iter()
        .map(|u| (u.id.clone(), u.username.clone()))
        .collect();

    // Get all posts in the 'dugnad' channel
    let fribot_id = "6k9aorsi3jnyfjewkweu614par"; // friBot account
    let channel_id = "dq51biaa9prf9rhrtes7fhd49h"; // dugnad channel
    let posts_endpoint = format!(
        "https://chat.fribyte.no/api/v4/channels/{}/posts",
        channel_id
    );

    let mut all_reacts: Vec<Reaction> = vec![];
    let mut prev_post = String::from("");

    // we loop to handle pagination
    loop {
        let posts: PostsRoot = api
            .query(
                "GET",
                &posts_endpoint,
                Some(&[("per_page", "1000"), ("before", &prev_post)]),
                None,
            )
            .await
            .unwrap();

        if let Some(l) = posts.order.last() {
            prev_post = l.clone();
        } else {
            break;
        }

        // Every check mark reaction to a post posted by friBot
        let mut reacts: Vec<Reaction> = posts
            .posts
            .values()
            .filter(|&p| p.user_id == fribot_id)
            .map(|p| -> Vec<Reaction> {
                p.metadata
                    .clone()
                    .unwrap_or(Metadata { reactions: None })
                    .reactions
                    .unwrap_or(vec![])
                    .into_iter()
                    .filter(|r| r.emoji_name == "white_check_mark" && r.user_id != fribot_id)
                    .collect()
            })
            .flatten()
            .collect();

        all_reacts.append(&mut reacts)
    }

    type AttTable = Vec<(String, String, String, i64)>;

    // Find the last reaction made and total number of attendances
    all_reacts.sort_by(|a, b| b.user_id.cmp(&a.user_id));
    let mut user_last_total: AttTable = all_reacts
        .as_mut_slice()
        .chunk_by(|a, b| a.user_id == b.user_id)
        .map(|rs| {
            let (time, total) = rs
                .iter()
                .map(|r| -> (i64, i64) { (r.create_at, 1) })
                .reduce(|(acc_time, acc_ct), (t, c)| (std::cmp::max(acc_time, t), acc_ct + c))
                .unwrap();

            let datetime = DateTime::from_timestamp(time / 1000, 0).unwrap();

            (
                rs[0].user_id.clone(),
                user_ids_names
                    .get(&rs[0].user_id)
                    .unwrap_or(&format!("unknown user {}", &rs[0].user_id))
                    .clone(),
                datetime.format("%Y-%m-%d").to_string(),
                total,
            )
        })
        .collect();
    user_last_total.sort_by(|(_, _, a, _), (_, _, b, _)| b.cmp(&a));

    // Add users who have *never* reacted
    let seen: HashSet<String> = user_last_total
        .iter()
        .map(|(i, _, _, _)| i.clone())
        .collect();

    let mut unseen: AttTable = user_ids_names
        .iter()
        .filter(|&(k, _)| seen.get(k).is_none())
        .map(|(k, v)| (k.clone(), v.clone(), String::from("Never"), 0i64))
        .collect();

    user_last_total.append(&mut unseen);

    // Print the resulting table
    println!(
        "{0: <45} | {1: <12} | {2: <18}",
        "username", "last reacted", "total attendances"
    );
    println!("----------------------------------------------+--------------+-------------------");
    for (_, username, date, total) in user_last_total {
        println!("{0: <45} | {1: <12} | {2: <6}", username, date, total);
    }
    println!("");

    Ok(())
}

