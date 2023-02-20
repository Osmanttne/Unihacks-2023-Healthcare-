# Unihacks-2023-Healthcare-
Website for hospital functionality for Unihacks 2023 Hackathon 

# TODO
- [x] Basic UI
    - [x] Navbar
    - [x] Titlebar
    - [x] Dashboard
    - [x] Material3 theme (Yoinked from BrightShard's website)
- [ ] Email
    - [x] Store emails in database
    - [x] Send emails to doctors/Reply to emails from doctors
    - [x] Receive emails from doctors
        - [x] Mark as read after viewing
    - [ ] Improve email UI
- [ ] Schedule
    - [x] Store time/date in database
    - [x] View upcoming checkups
    - [x] Schedule future appointments
    - [ ] Change appointment times?
- [x] Care Team/Staff Info
    - [x] Bio for doctors
    - [x] Pictures for doctors
- [ ] Users
    - [x] Store users in database
    - [ ] User authentication
        - [ ] Check multiple user support afterwards
    - [ ] User/profile settings?
- [ ] Clean Code (Optional)
    - [ ] Merge Jekyll server into Rocket server
        - [ ] Rust SCSS parser
        - [ ] Rust Liquid parser, or replace Liquid with Rocket's templating system
    - [ ] Use actual database instead of Serde w/ Postcard

# Screenshots
![image](https://user-images.githubusercontent.com/85412764/219993946-e3003558-302f-4edb-8c35-c29328820acb.png)
![Dashboard](screenshots/dashboard.png)
![Appointment Setup](screenshots/appointment-setup.png)
![Doctor Pages](screenshots/doctor-pages.png)
![Reading Emails](screenshots/email-read.png)
![Sending an Email](screenshots/email-write.png)
![Schedule](screenshots/schedule-view.png)
