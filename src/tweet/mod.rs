use std::vec;

use actix_web::{get, HttpResponse};
use chrono::{DateTime, Utc};
use uuid::Uuid;
mod like;

use like::Like;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    id: String,
    created_at: DateTime<Utc>,
    message: String,
    likes: Vec<Like>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweets {
    response: Vec<Tweet>,
}

// pub struct TweetRequest {
//     message: Option<String>,
// }

impl Tweet {
    pub fn new(message: String) -> Self {
        let id = Uuid::new_v4();

        Self {
            id: id.to_string(),
            created_at: Utc::now(),
            message,
            likes: vec![],
        }
    }
}

// impl TweetRequest {
//     pub fn to_tweet(&self) -> Option<Tweet> {
//         match &self.message {
//             Some(message) => Some(Tweet::new(message.to_string())),
//             None => None,
//         }
//     }
// }

#[get("/tweets")]
async fn get() -> HttpResponse {
    // TODO: find the last 50 tweets and return them
    let tweets = Tweets {
        response: vec![
            Tweet::new("Hello, world!".to_string()),
            Tweet::new("Goodbye, world!".to_string()),
            Tweet::new("I'm a tweet!".to_string()),
        ],
    };

    HttpResponse::Ok().json(tweets)
}
