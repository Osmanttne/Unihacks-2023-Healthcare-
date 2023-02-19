#[macro_use]
extern crate rocket;
mod db;

use std::sync::Mutex;

use db::*;
use rocket::serde::json::Json;
use uuid::Uuid;
use time::{Time, Date, Duration};

lazy_static::lazy_static!(
    static ref DATABASE: Mutex<Database> = Mutex::from(db::load());
);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/messages/get/<user_id>")]
fn get_messages(user_id: Uuid) -> Result<Json<Vec<db::Message>>, ()> {
    if let Some(user) = DATABASE.lock().unwrap()
        .users
        .iter()
        .find(|test_user| test_user.id == user_id)
    {
        Ok(Json(user.messages.clone()))
    } else {
        Err(())
    }
}
#[get("/messages/read/<user_id>/<message_id>")]
fn read_message(user_id: Uuid, message_id: Uuid) {
    if let Some(user) = DATABASE.lock().unwrap().users.iter_mut().find(|test_user| test_user.id == user_id) {
        if let Some(msg) = user.messages.iter_mut().find(|test_msg| test_msg.id == message_id) {
            msg.read = true;
        }
    }
}

#[get("/doctor/from_email/<email>")]
fn get_doctor_from_email(email: String) -> Result<Json<Doctor>, ()> {
    if let Some(doctor) = DATABASE.lock().unwrap().doctors.iter().find(|test_doctor| test_doctor.email == email)
    {
        Ok(Json(doctor.clone()))
    } else {
        Err(())
    }
}

#[cfg(debug_assertions)]
fn reset_db() {
    let mut db = db::Database::default();

    // Add the doctors
    db.doctors
        .push(Doctor::new("Carpenter", "", Department::Cardiology));
    db.doctors
        .push(Doctor::new("Taka", "", Department::Cardiology));
    db.doctors.push(Doctor::new("Roberts", "", Department::Ent));
    db.doctors.push(Doctor::new("Veratti", "", Department::Ent));
    db.doctors
        .push(Doctor::new("Benzema", "", Department::Surgery));
    db.doctors
        .push(Doctor::new("Rashford", "", Department::Surgery));
    db.doctors
        .push(Doctor::new("Taylor", "", Department::Neurology));
    db.doctors
        .push(Doctor::new("Leao", "", Department::Neurology));
    db.doctors
        .push(Doctor::new("Alfred", "", Department::Nutrition));
    db.doctors
        .push(Doctor::new("Larry", "", Department::Orthopedics));
    db.doctors
        .push(Doctor::new("Lorenzo", "", Department::Orthopedics));
    db.doctors
        .push(Doctor::new("Gerard", "", Department::Orthopedics));
    db.doctors
        .push(Doctor::new("Green", "", Department::Oncology));
    db.doctors
        .push(Doctor::new("Wilson", "", Department::Oncology));
    db.doctors
        .push(Doctor::new("Hill", "", Department::Reproductive));
    db.doctors
        .push(Doctor::new("Stefany", "", Department::Reproductive));

    // Make a filler user
    let mut user = db::User::default();
    println!("Generating user with ID: {}", user.id);
    user.messages.push(db::Message::new(
        "Post Surgery Check In",
        "It has been exactly 2 weeks after your surgery, I wanted to check in to ask if you had any severe changes in blood presure.",
        "carpenter@wattshospital.com",
    ));
    user.messages.push(db::Message::new(
        "Dietary Check Up",
        "I would like to invite you for a check-up to see the changes in your body due to the diet we have prepared. Will you be available to stop by the hospital any time soon?",
        "alfred@wattshospital.com",
    ));
    user.messages.push(db::Message::new(
        "Checking in for medication effects",
        "I wanted to check in to see if your nose is still runny. If the allergy pills did not stop it we might need to consider antibiotics.",
        "roberts@wattshospital.com",
    ));
    user.messages.push(db::Message::new(
        "Good News After Value Comparison",
        "I have gone through your reports from last year and compared them with the information on your final check-up. We have a noticeable improvment in Forgetfulness, I believe decreasing the factors of stress to a minimum had a great impact.",
        "taylor@wattshospital.com",
    ));
    user.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(12, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: user.id,
        doctor: String::from("Taka")
    });

    db.users.push(user);
    db::save(db);
}

#[launch]
fn rocket() -> _ {
    #[cfg(debug_assertions)]
    reset_db();
    rocket::build().mount("/", routes![index, get_messages, get_doctor_from_email, read_message])
}
