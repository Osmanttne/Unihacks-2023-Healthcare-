use crate::db::*;
use time::*;
use uuid::Uuid;

pub fn setup() {
    let mut db = Database::default();
    // Make a filler user
    let user = User::new("WattsUser", "password123");

    // Add the doctors
    let carpenter = Doctor::new("Carpenter", "Alice Carpenter was born on April 17th 1987 in Greensboro, North Carolina. She went to high school at Northern Guilford High school and went on to college at the University of North Carolina in Chapel Hill  Where she completed her bachelors degree in Biomedical Engineering. Later on She got accepted to Harvard Medical school in Cambridge, Massachusetts. She completed Medical school there and received certified training in Cardiology. Later on in her professional life she completed clinical research at UNC chapel Hill and started working as a Cardiologist at the Watts Hospital. She has been a part of our family since 2018.", Department::Cardiology);
    let mut taka = Doctor::new("Taka", "Osman Taka was born on June 24th 1992 in Istanbul, Turkiye.  He attended High school at the North Carolina School of Science and Math in Durham, North Carolina.  He completed his bachelors in Pre-med  at Duke University, and went on to  Medical school  at Johns Hopkins University in Baltimore, Maryland. After finishing Medical School He went through  training as an assistant surgeon. He is specialized in angioplasty and heart battery installation surgery. He has been a part of our family since 2020.    ", Department::Cardiology);
    let mut taylor = Doctor::new("Taylor", "Cameron Taylor was born on June 30th 1992 in Albemarle, North Carolina.  He attended High school at the North Carolina School of Science and Math in Durham, North Carolina.  He completed his bachelors in Pre-med  at Duke University, and went on to  Medical school  at Johns Hopkins University in Baltimore, Maryland. After finishing Medical School He went through  training as an assistant surgeon. He is specialized in Neuro-surgery. He has been a part of our family since 2020.    ", Department::Neurology);
    let mut alfred = Doctor::new("Alfred", "Yousef Alfred was born on March 14th 1992 in Toronto, Canada.  He attended High school at the Toronto School of Science and Math in Toronto, Canada.  He completed his bachelors in Pre-med  at Johns Hopkins University, and went on to  Medical school  at RÄ°ce University in Texas, United States. After finishing Medical School He went through  training as an assistant dietician. He is specialized in Nutrition. He has been a part of our family since 2019.    ", Department::Nutrition);

    taka.patient_ids.push(user.id);
    taylor.patient_ids.push(user.id);
    alfred.patient_ids.push(user.id);

    db.doctors.push(carpenter);
    db.doctors.push(taka);
    db.doctors.push(Doctor::new("Roberts", "Cassandra Roberts was born on January 19th, 1974 in Ithaca, New York. She went to High school at Northwestern Ithaca High School. Later on she went to college at Cornell University in Ithaca, New York.  Where she completed her bachelors degree in Biology. Later on She got accepted to Harvard Medical school in Cambridge, Massachusetts. She completed Medical school there and received certified training in Ear, Nose and Throat illnesses . Later on in her professional life she completed clinical research at  Duke University and started working as a Ear, Nose and Throat doctor at the Watts Hospital. She has been a part of our family since 2007.", Department::Ent));
    db.doctors.push(Doctor::new("Veratti", "Marco Verratti was born on February 29th ,1982 in Milan Italy. He attended High school at the Milan School of Science and Math in Milan, Italy .  He completed his bachelors in Pre-med  at Oxford University, and went on to  Medical school  at Johns Hopkins University in Baltimore, Maryland. After finishing Medical School He went through  training as an assistant surgeon. He is specialized in Ear, Nose and Throat surgery. He has been a part of our family since 2015.", Department::Ent));
    db.doctors.push(Doctor::new("Benzema", "Karim Benzema was born on May 17th ,1978 in Omaha, Nebraska. He attended High school at the Riverslide High School in Omaha, Nebraska  .  He completed his bachelors in Pre-med  at Stanford University, and went on to  Medical school  at Emory University in Atlanta, Georgia . After finishing Medical School He went through  training as an assistant surgeon. He is specialized in General surgery. He has been a part of our family since 2010.", Department::Surgery));
    db.doctors.push(Doctor::new("Rashford", "Maria Rashford was born on April 17th 1997 in Greensboro, North Carolina. She went to high school at  Guilford Early College and went on to  college at the University of North Carolina in Chapel Hill  Where she completed her bachelors degree in Pre-Med. Later on She got accepted to Harvard Medical school in Cambridge, Massachusetts. She completed Medical school there and received certified training in General Surgery. Later in her professional life she completed clinical research at UNC chapel Hill and started working as a General Surgeon at the Watts Hospital. She has been a part of our family since 2022. ", Department::Surgery));
    db.doctors.push(taylor);
    db.doctors.push(Doctor::new("Leao", "Racheal Leao was born on September 25th 1996 in Charlotte, North Carolina. She went to high school at Northern Charlotte High school and went on to college at the University of North Carolina in Chapel Hill  Where she completed her bachelors degree in Neuroscience. Later on She got accepted to Harvard Medical school in Cambridge, Massachusetts. She completed Medical school there and received certified training in Neurology. Later on in her professional life she completed clinical research at UNC chapel Hill and started working as a Neurologist at the Watts Hospital. She has been a part of our family since 2018. ", Department::Neurology));
    db.doctors.push(alfred);
    db.doctors.push(Doctor::new("Larry", "Steven Larry was born on March 14th 1978  in Toronto, Canada.  He attended High school at the Toronto School of Science and Math in Toronto, Canada.  He completed his bachelors in Pre-med  at Johns Hopkins University, and went on to  Medical school  at Rice University in Texas, United States. After finishing Medical School He went through  training as an assistant Orthopedic Surgeon. He is specialized in Orhtopedic Surgery. He has been a part of our family since 2009.", Department::Orthopedics));
    db.doctors.push(Doctor::new("Lorenzo", "Antoine Lorenzo was born on October 18th 1952 in Lyon, France.  He attended High school at the Lyon School of intelligence in Lyon, France.  He completed his bachelors in Pre-med  at Cambridge University, and went on to  Medical school at Harvard University in Cambridge, Massachusetts. After finishing Medical School He went through  training as an Orthopedic Surgeon at the Medical Institute of New York. He is specialized in Orthopedic Surgery and Platin Installation. He has been a part of our family since 2007.", Department::Orthopedics));
    db.doctors.push(Doctor::new("Gerard", "Ariana Gerrard  was born on April 25th 1995 in Boone, North Carolina. She went to high school at Northern Boone High school and went on to college at Appalachian State University, Where she completed her bachelors degree in Public Health. Later on She got accepted to Duke Medical school in Durham, North Carolina. She completed Medical school there and received certified training in Orthopedics. Later on in her professional life she completed clinical research at UNC chapel Hill and started working as a Orthopedic surgeon at the Watts Hospital. She has been a part of our family since 2021. ", Department::Orthopedics));
    db.doctors.push(Doctor::new("Green", "Joseph Greene was born on December 18th 1969 in Tampa, Florida.  He attended High school at the Florida School of Science in Jacksonville, Florida.  He completed his bachelors in Pre-med  at  Florida State University, and went on to  Medical school at Johns Hopkins University in Baltimore, Maryland. After finishing Medical School He went through  training as an Orthodontist  at the Medical Institute of Tokyo. He is specialized in Physiotherapy and Radioactive Surgery. He has been a part of our family since 2008.", Department::Oncology));
    db.doctors.push(Doctor::new("Wilson", "Jamie Wilson was born on November 10th 1989 in Chicago, Illinois.  He attended High school at the Chicago School of Science in Chicago, Illinios.  He completed his bachelors in Pre-med  at  Illinois State University, and went on to  Medical school at the University of North Carolina in Chapel Hill, North Carolina. After finishing Medical School He went through  training as an Orthodontist  at the Medical Institute of Durham. He is specialized in Physiotherapy and Radioactive Surgery. He has been a part of our family since 2017.", Department::Oncology));
    db.doctors.push(Doctor::new("Hill", "Lisa Hill was born on July 27th 1996 in San Jose, California. She went to high school at San Jose High school and went on to college at Stanford University, Where she completed her bachelors degree in Public Health. Later on She got accepted to Duke Medical school in Durham, North Carolina. She completed Medical school there and received certified training in Reproductive Health Illnesses. Later on in her professional life she completed clinical research at UNC chapel Hill and started working as a Reproductive Health surgeon at the Watts Hospital. She has been a part of our family since 2021. ", Department::Reproductive));
    db.doctors.push(Doctor::new("Stefany", "Catherine Stefany was born on August 28th 1975 in Atlanta, Georgia. She went to high school at Atlanta High school and went on to college at Emory University, Where she completed her bachelors degree in Pre-Med. Later on She got accepted to Duke Medical school in Durham, North Carolina. She completed Medical school there and received certified training in Reproductive Health Illnesses. Later on in her professional life she completed clinical research at Harvard University and started working as a Reproductive Health surgeon at the Watts Hospital. She has been a part of our family since 2011. ", Department::Reproductive));

    println!("Generating user with id: {}", user.id);
    db.mail.push(Message::new(
        "Post Surgery Check-In",
        "It has been exactly 2 weeks after your surgery, I wanted to check in to ask if you had any severe changes in blood presure.",
        db.uuid_from_email("carpenter@wattshospital.com").unwrap(),
        user.id
    ));
    db.mail.push(Message::new(
        "Dietary Check-Up",
        "I would like to invite you for a check-up to see the changes in your body due to the diet we have prepared. Will you be available to stop by the hospital any time soon?",
        db.uuid_from_email("alfred@wattshospital.com").unwrap(),
        user.id
    ));
    db.mail.push(Message::new(
        "Checking in for medication effects",
        "I wanted to check in to see if your nose is still runny. If the allergy pills did not stop it we might need to consider antibiotics.",
        db.uuid_from_email("roberts@wattshospital.com").unwrap(),
        user.id
    ));
    db.mail.push(Message::new(
        "Good News After Value Comparison",
        "I have gone through your reports from last year and compared them with the information on your final check-up. We have a noticeable improvment in Forgetfulness, I believe decreasing the factors of stress to a minimum had a great impact.",
        db.uuid_from_email("taylor@wattshospital.com").unwrap(),
        user.id
    ));

    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(12, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taka@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(1, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taka@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(2, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taka@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(3, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taka@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });

    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(12, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taylor@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(1, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taylor@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(2, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taylor@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });
    db.appointments.push(Appointment {
        date: Date::from_calendar_date(2023, time::Month::February, 19).unwrap(),
        start: Time::from_hms(3, 30, 0).unwrap(),
        length: Duration::hours(1),
        patient: None,
        doctor: db.uuid_from_email("taylor@wattshospital.com").unwrap(),
        id: Uuid::new_v4()
    });

    db.users.push(user);
    save(db);
}