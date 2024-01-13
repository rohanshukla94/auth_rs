pub fn login(creds: models::Credentials) {
    crate::database::get_user();
}

pub fn logout() {
    //logout user
}

pub mod models;