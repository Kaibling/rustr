use dyn_clone::DynClone;
use entity::Event;
use entity::User;
use std::collections::HashMap;

dyn_clone::clone_trait_object!(UserRepo);
pub trait EventRepo {
    fn add_event(&mut self, e: Event);
    fn read_event(&self, public_key: &String) -> Option<&Event>;
    fn read_all_events(&self) -> Vec<Event>;
}
#[derive(Clone)]
pub struct EventRepoInMemory {
    events: HashMap<String, Event>,
}

impl EventRepo for EventRepoInMemory {
    fn add_event(&mut self, u: Event) {
        let n = u.get_id();
        self.events.insert(n, u);
    }
    fn read_event(&self, public_key: &String) -> Option<&Event> {
        return self.events.get(public_key);
    }
    fn read_all_events(&self) -> Vec<Event> {
        let mut res: Vec<Event> = Vec::new();
        for (_, v) in &self.events {
            res.push(v.clone());
        }
        return res;
    }
}
impl EventRepoInMemory {
    pub fn new() -> EventRepoInMemory {
        return EventRepoInMemory {
            events: HashMap::new(),
        };
    }
}
pub trait UserRepo: DynClone {
    fn add_user(&mut self, u: User);
    fn read_user(&self, public_key: &String) -> Option<&User>;
    fn read_all_users(&self) -> Vec<User>;
    //fn Clone(&self) -> dyn UserRepo;
}
#[derive(Clone)]
pub struct UserRepoInMemory {
    users: HashMap<String, User>,
}

impl UserRepo for UserRepoInMemory {
    fn add_user(&mut self, u: User) {
        println!("adding user {}", &u.name);
        let n = u.public_key.clone();
        self.users.insert(n, u);
    }
    fn read_user(&self, public_key: &String) -> Option<&User> {
        return self.users.get(public_key);
    }
    fn read_all_users(&self) -> Vec<User> {
        let mut res: Vec<User> = Vec::new();
        for (_, v) in &self.users {
            res.push(v.clone());
        }
        return res;
    }
}
impl UserRepoInMemory {
    pub fn new() -> UserRepoInMemory {
        return UserRepoInMemory {
            users: HashMap::new(),
        };
    }
}

// impl Clone for Box<dyn UserRepo> {
//     fn clone(&self) -> Self {
//         self.clone_box()
//     }
// }
