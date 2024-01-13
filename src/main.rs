use auth_service::Credentials;

fn main() {

    let creds = Credentials {
        username: "rohanshukla".to_owned(),
        password: "pass123".to_owned()    
    };

    auth_service::authenticate(creds);
}