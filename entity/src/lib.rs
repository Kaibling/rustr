use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    id: String,
    public_key: String,
    created_at: u64,
    kind: u32,
    tags: String,
    content: String,
    sig: String,
    // circle
    // expires_at: u64;
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
        print!("{}\n", fmt_str);
        let sig_str = fmt_str.clone();
        self.id = crypto::hash(fmt_str);
        self.sig = crypto::sign_message(sig_str, private_key);
    }
    pub fn verify(&self) -> bool {
        let fmt_str = format!(
            "[0,{},{},{},{},{}]",
            self.public_key, self.created_at, self.kind, self.tags, self.content
        );
        return crypto::verify_message(fmt_str, self.sig.clone(), self.public_key.clone());
    }
    pub fn new(public_key: String, content: String) -> Event {
        return Event {
            id: "".to_string(),
            public_key,
            created_at: 0,
            kind: 0,
            tags: "".to_string(),
            content,
            sig: "".to_string(),
        };
    }
    pub fn get_id(&self) -> String {
        self.public_key.clone()
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
