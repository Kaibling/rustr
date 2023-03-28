use serde::{Deserialize, Serialize};
use std::time::{SystemTime};
use ulid::Ulid;
use chrono::prelude::*;
use chrono::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: String,
    pub public_key: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: String,
    pub content: String,
    pub sig: String,
    // circle
    pub expires_at: u64,
}

impl Event {
    pub fn sign(&mut self, private_key: String) {
        self.created_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let fmt_str = format!(
            "[0,{},{},{},{},{}]",
            self.public_key, self.created_at, self.kind, self.tags, self.content
        );
        let sig_str = fmt_str.clone();
        self.id = crypto::hash(fmt_str);
        self.sig = crypto::sign_message(sig_str, private_key);
    }
    pub fn verify(&self) -> bool {
        let fmt_str = format!(
            "[0,{},{},{},{},{}]",
            self.public_key, self.created_at, self.kind, self.tags, self.content
        );
        return crypto::verify_message(&fmt_str, &self.sig.clone(), &self.public_key.clone());
    }
    pub fn new(public_key: String, content: String, expires_at: u64) -> Event {
        return Event {
            id: "".to_string(),
            public_key,
            created_at: 0,
            kind: 0,
            tags: "".to_string(),
            content,
            sig: "".to_string(),
            expires_at,
        };
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn expired(&self) -> bool {
        if self.expires_at != 0 {
            return  SystemTime::now().
            duration_since(SystemTime::UNIX_EPOCH).
            expect("msg").
            as_secs() >=self.expires_at
        }
        return false
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

impl User {
    pub fn new(name: String, public_key: String) -> User {
        return User { name, public_key };
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    shared_secret: String,
    public_key:String,
    key_pair: KeyPair,
    expires_at: u64,
}

impl  Session {
    pub fn new(public_key: String,expires_at:u64) -> Session {
        let ulid = Ulid::new();
        let expires = if expires_at == 0 {
             (Local::now() + Duration::hours(1)).timestamp().try_into().unwrap()
        } else {
            expires_at
        };
        let key_pair = KeyPair::generate();
        let shared_secret = crypto::generate_shared_secret(&public_key, key_pair.private_key());
        return Session { id: ulid.to_string(), shared_secret, key_pair, public_key, expires_at: expires }
    }
    pub fn expired(&self) -> bool {
        if self.expires_at != 0 {
            return  SystemTime::now().
            duration_since(SystemTime::UNIX_EPOCH).
            expect("msg").
            as_secs() >=self.expires_at
        }
        return false
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}



#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct KeyPair {
    public_key: String,
    private_key: String,
}

impl KeyPair {
    pub fn generate() -> KeyPair {
        let (private_key, public_key) = crypto::create_key_pair();
        return KeyPair { public_key, private_key }
    }
    pub fn public_key(&self) -> String {
        return self.public_key.clone()
    }
    pub fn private_key(&self) -> String {
        return self.private_key.clone()
    }
}