pub fn login(creds: models::Credentials) {
    crate::database::get_user(121);
}

pub fn logout() {
    //logout user
}

pub mod models;