use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use time::{Date, Duration, Time};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appointment {
    pub date: Date,
    pub start: Time,
    pub length: Duration,
    pub patient: Uuid,
    pub doctor: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub title: String,
    pub text: String,
    pub sender: String,
    pub read: bool,
    pub id: Uuid,
}
impl Message {
    pub fn new(title: &str, text: &str, sender: &str) -> Self {
        Self {
            title: title.to_string(),
            text: text.to_string(),
            sender: sender.to_string(),
            read: false,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub id: Uuid,
    pub messages: Vec<Message>,
    pub password_hash: String,
    pub appointments: Vec<Appointment>
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            messages: Vec::new(),
            password_hash: String::new(),
            appointments: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Department {
    Cardiology,
    Ent,
    Surgery,
    Neurology,
    Nutrition,
    Oncology,
    Orthopedics,
    Reproductive,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Doctor {
    pub name: String,
    pub email: String,
    pub photo: String,
    pub bio: String,
    pub department: Department,
    pub patient_ids: Vec<Uuid>,
    pub appointments: Vec<Appointment>
}
impl Doctor {
    pub fn new(name: &str, bio: &str, department: Department) -> Self {
        let id = name.to_owned().to_lowercase();
        let email = id.clone() + "@wattshospital.com";
        let photo = id + ".jpg";
        Self {
            name: name.to_owned(),
            email,
            photo,
            bio: bio.to_owned(),
            department,
            patient_ids: Vec::new(), 
            appointments: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Database {
    pub users: Vec<User>,
    pub doctors: Vec<Doctor>,
}

pub fn load() -> Database {
    postcard::from_bytes(&fs::read("db.postcard").unwrap()).unwrap()
}

pub fn save(db: Database) {
    fs::write("db.postcard", postcard::to_allocvec(&db).unwrap()).unwrap();
}
