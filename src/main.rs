use auth_bhai::Credentials;

fn main() {

    let creds = Credentials {
        username: "rohanshukla".to_owned(),
        password: "pass123".to_owned()    
    };

    auth_bhai::authenticate(creds);
}