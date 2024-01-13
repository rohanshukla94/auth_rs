pub enum Status {
    Connected,
    TimedOut,
    Interrupted,
    Offline,
}

pub fn connect_db() -> Status {
    return Status::Connected;
}

pub fn get_user() {
    //get user from db
}
