#[allow(dead_code)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn run_structs_code() {
    let user: User = new_user();
    separation();
    print_user(&user);
    let user1: User = update_struct(&user);
    separation_word("user1"); 
    print_user(&user1);

    let rect: Rectangle = Rectangle {
        width: 10, 
        height: 10,
    };
    
    println!("area of rectangle: {}", rect.area());
    let rect1 = Rectangle{ width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);

    let rect2: Rectangle = Rectangle {
        width: 2,
        height: 2,
    };

    println!("rect1 can fit in rect2 ? {}", rect.can_hold(&rect2));
}

fn new_user() -> User {
    let user = User{
        email: String::from("wesley@gmail.com"),
        username: String::from("wesley"),
        sign_in_count: 1,
        active: true,
    };
    
    user
}

fn print_user(user: &User) {
    println!("Email: {}\nUsername: {}\nActive: {}", user.email, user.username, user.active);
}

fn update_struct(user1: &User) -> User {
    User {
        email: String::from("lewis@gmail.com"),
        username: String::from("lewis"),
        ..*user1
    }
}

// tuple structs
#[derive(Debug)]
struct Rectangle {
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.height > rect.height && self.width > rect.width {
            return true;
        }else {
            return false;
        }
    }
}

fn separation() {
    println!("-------------------------------------");
}

fn separation_word(s: &str) {
    println!("---------------------{}----------------", s);
}

