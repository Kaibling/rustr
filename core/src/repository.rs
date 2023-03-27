use dyn_clone::DynClone;
use entity::Event;
use entity::User;
use std::collections::HashMap;
use tracing::event;
dyn_clone::clone_trait_object!(UserRepo);
pub trait EventRepo {
    fn add(&mut self, e: Event);
    fn read(&mut self, id: &String) -> Option<Event>;
    fn read_all(&mut self) -> Vec<Event>;
    fn delete(&mut self,id:&String);
}
#[derive(Clone)]
pub struct EventRepoInMemory {
    events: HashMap<String, Event>,
}

impl EventRepo for EventRepoInMemory {
    fn add(&mut self, u: Event) {
        let n = u.get_id();
        self.events.insert(n, u);
    }
    fn read(&mut self, id: &String) -> Option<Event> {
        match self.events.get(id) {
            Some(e) => {
                let oe = e.to_owned();
                if oe.expired(){
                    let msg = format!("deleting {}",oe.id);
                    event!(tracing::Level::INFO,msg);
                    self.delete(&oe.id);
                }
                return Some(oe)
            },
            None => return None,

        }
    }
    fn read_all(&mut self) -> Vec<Event> {
        let mut res: Vec<Event> = Vec::new();
        let mut obsoletes: Vec<String> = Vec::new();
        for (_, v) in &self.events {
            if !v.expired(){
                res.push(v.clone());
            } else {
                obsoletes.push(v.id.clone());
            }
        }
        for  v in obsoletes {
            let msg = format!("deleting {}",v);
            event!(tracing::Level::INFO,msg);
            self.delete(&v);
        }
        return res;
    }
    fn delete(&mut self,id: &String) {
        self.events.remove(id);
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
