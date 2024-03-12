use std::io;

#[derive(Debug)]
pub struct User {
    userid: String,
    password: String,
}

fn display_menu() {
    println!("Welcome to App");
    println!("1. Login");
    println!("2. Sign Up");
    println!("3. Exit");
}

fn main() {
    let user1: User = User {
        userid: "Bhupal".to_string(),
        password: "123".to_string(),
    };
    let user2 = User {
        userid: "Arslan".to_string(),
        password: "123".to_string(),
    };
    let user3 = User {
        userid: "Kamlesh".to_string(),
        password: "123".to_string(),
    };

    let mut logged_in = false;

    loop {
        if !logged_in {
            display_menu();
            println!("Enter your choice (1-3):");
        }

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("Invalid input please make sure you enter valid input");
                continue;
            }
        };

        match choice {
            1 => {
                if login(&user1, &user2, &user3) {
                    logged_in = true;
                }
            }
            2 => signup(),
            3 => {
                println!("Exiting App");
                break;
            }
            _ => println!("Invalid operations, please select a valid option (1-3)."),
        }
    }
}

fn login(user1: &User, user2: &User, user3: &User) -> bool {
    println!("Welcome to Login App");

    println!("Enter UserId");

    let mut userid = String::new();
    io::stdin()
        .read_line(&mut userid)
        .expect("Failed to read line");
    let userid = userid.trim().to_string();

    println!("Enter Password");

    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_string();

    println!("");

    if user1.userid == userid && user1.password == password {
        println!("Successfully logged in!");
        return true;
    }
    if user2.userid == userid && user2.password == password {
        println!("Successfully logged in!");
        return true;
    }
    if user3.userid == userid && user3.password == password {
        println!("Successfully logged in!");
        return true;
    } else {
        println!("Login failed. Please check your credentials.");
        return false;
    }
}

fn signup() {
    println!("Signup");
}
