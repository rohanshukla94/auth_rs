pub enum Status {
    Connected,
    TimedOut,
    Interrupted,
    Offline,
}

pub fn connect_db() -> Status {
    return Status::Connected;
}

pub fn get_user(user_id: u32) -> Option<String> {
    //get user from db
    let query = format!("GET username FROM users WHERE id={user_id}");

    let db_result = query_db(query);
    db_result.ok()
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("empty query provided"))
    } else {
        Ok(String::from("User from dB is!!!"))
    }
}