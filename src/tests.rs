use super::structs::User;

pub fn run_tests() {
    println!("Tests running");
}

#[test]
fn add_two() {
    assert_eq!(super::add_two(3), 5);
}

#[test] 
fn contain_name() {
    let user = User {
        username: String::from("Wesley"),
        email: String::from("wesley@gmail.com"),
        sign_in_count: 10,
        active: true,
    };

    assert!(user.contain_name());
}

#[test]
fn does_not_contain_name() {
    let user = User {
        username: String::from("Lewis"),
        email: String::from("lewis@gmail.com"),
        sign_in_count: 5,
        active: false,
    };

    assert_ne!(user.username, "myname", "The username should not be '{}'", "myname");
}

impl User {
    pub fn contain_name(&self) -> bool {
        if self.username != "" {
            return true;
        }else {
            return false;
        }
    }
}
