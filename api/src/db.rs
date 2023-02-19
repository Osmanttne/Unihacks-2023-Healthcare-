use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;
use time::{Date, Duration, Time};
use std::str::FromStr;
use strum_macros::{EnumString, EnumIter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Appointment {
    pub date: Date,
    pub start: Time,
    pub length: Duration,
    pub patient: Option<Uuid>,
    pub doctor: Uuid,
    pub id: Uuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub id: Uuid,
}
impl User {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            email: name.to_ascii_lowercase() + "@patients.wattshospital.com",
            password_hash: password.to_string(),
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
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
impl Department {
    pub fn get_name(&self) -> String {
        match self {
            Self::Cardiology => "Cardiology".into(),
            Self::Ent => "Ear, Nose, & Throat".into(),
            Self::Surgery => "General Surgery".into(),
            Self::Neurology => "Neurology".into(),
            Self::Nutrition => "Nutrition & Dietetics".into(),
            Self::Oncology => "Oncology".into(),
            Self::Orthopedics => "Orthopedics".into(),
            Self::Reproductive => "Reproductive Health".into()
        }
    }
}
impl<'a> FromParam<'a> for Department {
    type Error = strum::ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Department::from_str(param)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Doctor {
    pub name: String,
    pub email: String,
    pub photo: String,
    pub bio: String,
    pub id: Uuid,
    pub department: Department,
    pub patient_ids: Vec<Uuid>,
}
impl Doctor {
    pub fn new(name: &str, bio: &str, department: Department) -> Self {
        let id = name.to_owned().to_lowercase();
        let email = id.clone() + "@wattshospital.com";
        println!("ID: {id}||Email: {email}");
        let photo = id + ".jpg";
        Self {
            name: name.to_owned(),
            email,
            photo,
            bio: bio.to_owned(),
            id: Uuid::new_v4(),
            department,
            patient_ids: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub title: String,
    pub text: String,
    pub to: Uuid,
    pub from: Uuid,
    pub read: bool,
    pub id: Uuid,
}
impl Message {
    pub fn new(title: &str, text: &str, from: Uuid, to: Uuid) -> Self {
        Self {
            title: title.to_string(),
            text: text.to_string(),
            from,
            to,
            read: false,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Database {
    pub users: Vec<User>,
    pub doctors: Vec<Doctor>,
    pub mail: Vec<Message>,
    pub appointments: Vec<Appointment>
}

impl Database {
    pub fn uuid_from_email(&self, email: &str) -> Option<Uuid> {
        if let Some(doctor) = self.doctors.iter().find(|test_doctor| test_doctor.email == email) {
            Some(doctor.id)
        } else {
            self.users.iter().find(|test_user| test_user.email == email).map(|user| user.id)
        }
    }
    pub fn email_from_uuid(&self, id: Uuid) -> Option<&String> {
        if let Some(doctor) = self.doctors.iter().find(|test_doctor| test_doctor.id == id) {
            Some(&doctor.email)
        } else {
            self.users.iter().find(|test_user| test_user.id == id).map(|user| &user.email)
        }
    }
}

pub fn load() -> Database {
    postcard::from_bytes(&fs::read("db.postcard").unwrap()).unwrap()
}

pub fn save(db: Database) {
    fs::write("db.postcard", postcard::to_allocvec(&db).unwrap()).unwrap();
}
