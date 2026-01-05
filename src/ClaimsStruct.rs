use jsonwebtoken::{encode, Encodingkey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
}

pub fn generate_jwt(username: String) -> String {
    let my_secret = "Secret_key_here";

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + 3600;

    let my_claims = Claims {
        sub: username,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &my_claims,
        &Encodingkey::from_secret(my_secret.as_ref()),
    )
    .unwrap()
}
