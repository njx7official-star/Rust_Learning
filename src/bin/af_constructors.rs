struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(&mut self) {
        let user_name_femail = self
            .email
            .split_once("@")
            .map(|(name, _)| name.to_string())
            .unwrap_or_else(|| self.email.clone());
        self.username = user_name_femail;
    }
    fn change_uri(&mut self , uri: String){
        self.uri = uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza123"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.from_email();
    println!("new user name {}",new_user.username);

    println!("{}",new_user.uri);
    let uri = String::from("https:123.com");
    new_user.change_uri(uri);

    println!("{}",new_user.uri);

}
