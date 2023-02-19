#[macro_use]
extern crate rocket;
mod db;

use std::sync::Mutex;

use db::*;
use rocket::{form::{Form, Strict}, serde::json::Json};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use uuid::Uuid;

lazy_static::lazy_static!(
    static ref DATABASE: Mutex<Database> = Mutex::from(db::load());
);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/messages/get/<user_id>")]
fn get_messages(user_id: Uuid) -> Result<Json<Vec<db::Message>>, ()> {
    let mail: Vec<Message> = DATABASE.lock().unwrap()
        .mail
        .iter()
        .filter(|test_msg| test_msg.to == user_id)
        .map(Message::clone)
        .collect();
    if !mail.is_empty()
    {
        Ok(Json(mail))
    } else {
        Err(())
    }
}
#[get("/messages/read/<user_id>/<message_id>")]
fn read_message(user_id: Uuid, message_id: Uuid) -> Result<(), ()> {
    if let Some(message) = DATABASE.lock().unwrap().mail.iter_mut().find(|test_msg| test_msg.to == user_id && test_msg.id == message_id) {
        message.read = true;
        return Ok(());
    }
    Err(())
}

#[derive(FromForm)]
struct MessageForm {
    pub title: String,
    pub text: String,
    pub to: String,
    pub from: String,
}
#[derive(Serialize, Deserialize)]
enum MessageFormError {
    To,
    From
}
impl TryFrom<MessageForm> for Message {
    type Error = MessageFormError;
    fn try_from(msg: MessageForm) -> Result<Self, Self::Error> {
        let db = DATABASE.lock().unwrap();
        match db.uuid_from_email(&msg.to) {
            Some(to) => match db.uuid_from_email(&msg.from) {
                Some(from) => Ok(Self {
                    title: msg.title,
                    text: msg.text,
                    to,
                    from,
                    read: false,
                    id: Uuid::new_v4()
                }),
                None => Err(MessageFormError::From)
            },
            None => Err(MessageFormError::To)
        }
    }
}

#[post("/messages/send", data = "<message>")]
fn send_message(message: Form<Strict<MessageForm>>) -> Result<(), Json<MessageFormError>> {
    let msg: Message = message.into_inner().into_inner().try_into()?;
    let mut db = DATABASE.lock().unwrap();
    
    println!(
        "{} sent an email to {} with the title `{}` and contents `{}`",
        db.email_from_uuid(msg.from).unwrap(),
        db.email_from_uuid(msg.to).unwrap(),
        msg.title,
        msg.text
    );
    db.mail.push(msg);
    Ok(())
}

#[get("/uuid/from_email/<email>")]
fn get_uuid_from_email(email: String) -> Option<Json<Uuid>> {
    DATABASE.lock().unwrap().uuid_from_email(&email).map(Json)
}

#[get("/patient/<patient_id>/doctors")]
fn get_patient_care_team(patient_id: Uuid) -> Json<Vec<Doctor>> {
    let db = DATABASE.lock().unwrap();
    Json(db.doctors.clone().into_iter().filter(|doctor| doctor.patient_ids.iter().any(|id| *id == patient_id)).collect())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppointmentTime {
    year: i32,
    month: time::Month,
    day: u8,
    hour: u8,
    minute: u8,
    id: Uuid,
    doctor: Uuid
}

#[get("/patient/<patient_id>/appointments")]
fn get_patient_appointments(patient_id: Uuid) -> Json<Vec<AppointmentTime>> {
    Json(
        DATABASE.lock().unwrap().appointments.clone()
            .into_iter()
            .filter(|aptmnt| aptmnt.patient.is_some() && aptmnt.patient.unwrap() == patient_id)
            .map(|aptmnt| AppointmentTime {
                year: aptmnt.date.year(),
                month: aptmnt.date.month(),
                day: aptmnt.date.day(),
                hour: aptmnt.start.hour(),
                minute: aptmnt.start.minute(),
                id: aptmnt.id,
                doctor: aptmnt.doctor
            })
            .collect()
    )
}

#[get("/doctor/<doctor_id>")]
fn get_doctor_from_id(doctor_id: Uuid) -> Result<Json<Doctor>, ()> {
    if let Some(doctor) = DATABASE.lock().unwrap().doctors.iter().find(|test_doctor| test_doctor.id == doctor_id)
    {
        Ok(Json(doctor.clone()))
    } else {
        Err(())
    }
}

#[get("/doctor/<doctor_id>/appointments")]
fn get_doctor_appointments(doctor_id: Uuid) -> Json<Vec<AppointmentTime>> {
    Json(
        DATABASE.lock().unwrap().appointments.clone()
            .into_iter()
            .filter(|aptmnt| aptmnt.doctor == doctor_id && aptmnt.patient.is_none())
            .map(|aptmnt| AppointmentTime {
                year: aptmnt.date.year(),
                month: aptmnt.date.month(),
                day: aptmnt.date.day(),
                hour: aptmnt.start.hour(),
                minute: aptmnt.start.minute(),
                id: aptmnt.id,
                doctor: aptmnt.doctor
            })
            .collect()
    )
}

#[get("/doctors/all")]
fn get_all_doctors() -> Json<Vec<Doctor>> {
    Json(DATABASE.lock().unwrap().doctors.clone())
}

#[get("/doctors/<department>")]
fn get_doctors_in_department(department: Option<Department>) -> Json<Vec<Doctor>> {
    println!("{department:?}");
    if let Some(department) = department {
        Json(DATABASE.lock().unwrap().doctors.clone().into_iter().filter(|doctor| doctor.department == department).collect())
    } else {
        Json(DATABASE.lock().unwrap().doctors.clone())
    }
}

#[get("/departments")]
fn get_departments() -> Json<Vec<(String, Department)>> {
    Json(Department::iter().map(|dep| (Department::get_name(&dep), dep)).collect())
}

#[derive(Serialize, Deserialize, Debug, Clone, FromForm)]
struct ReservedAppointment {
    pub id: Uuid,
    pub patient: Uuid
}

#[post("/appointments/reserve", format = "json", data = "<reservation>")]
fn reserve_appointment(reservation: Json<ReservedAppointment>) -> Result<(), ()> {
    if let Some(appointment) = DATABASE.lock().unwrap().appointments.iter_mut().find(|apmnt| apmnt.id == reservation.id) {
        appointment.patient = Some(reservation.patient);
        return Ok(());
    }
    Err(())
}

#[cfg(debug_assertions)]
mod setup_db;

#[launch]
fn rocket() -> _ {
    #[cfg(debug_assertions)]
    setup_db::setup();
    rocket::build().mount("/", routes![
        index,
        get_messages,
        get_doctor_from_id,
        read_message,
        send_message,
        get_uuid_from_email,
        get_doctor_appointments,
        get_all_doctors,
        get_doctors_in_department,
        get_departments,
        reserve_appointment,
        get_patient_care_team,
        get_patient_appointments
    ])
}
