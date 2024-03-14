
use reqwest::Error;
use serde::Serialize;
use std::io;

#[derive(Debug, Serialize)]
=======
use std::io;

#[derive(Debug)]

pub struct User {
    userid: String,
    password: String,
}


#[derive(Debug, Serialize)]
pub struct Auth {
    userid: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Users {
    userid: String,
    recieverid: u8,
}
#[derive(Debug, Serialize)]
pub struct Message {
    recieverid: u8,
    messege: String,
}

fn display_menu() {
    println!("Welcome to App");
    println!("1. Login");
    println!("2. Exit");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let user = User {
        userid: "Bhupal".to_string(),
        password: "123".to_string(),
    };
=======
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

            println!("");
            println!("Invalid user! Please try again..");
            println!("");
        }
        display_menu();
        println!("");
        println!("Enter your choice (1-2):");
=======
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

                println!("");
=======

                println!("Invalid input please make sure you enter valid input");
                continue;
            }
        };

        match choice {
            1 => {

                {
                    println!("Welcome to Login App");

                    println!("");

                    println!("Enter UserId");

                    let mut x: String = String::new();
                    io::stdin().read_line(&mut x).expect("Failed to read line");
                    let x = x.trim().to_string();

                    println!("");

                    println!("Enter password");

                    println!("");

                    let mut y = String::new();
                    io::stdin().read_line(&mut y).expect("Failed to read line");
                    let y = y.trim().to_string();

                    let auth = Auth {
                        userid: x.clone(),
                        password: y,
                    };

                    logged_in = login_auth(&user, &auth);

                    if logged_in == true {
                        login_c(&auth).await?;

                        println!("");

                        println!("Enter Reciever Id");

                        println!("");

                        let mut z = String::new();
                        io::stdin().read_line(&mut z).expect("Failed to read line");
                        let z: u8 = z.trim().parse().expect("Invalid input");

                        let user1 = Users {
                            userid: x.clone(),
                            recieverid: 1,
                        };
                        let user2 = Users {
                            userid: x.clone(),
                            recieverid: 2,
                        };
                        let user3 = Users {
                            userid: x,
                            recieverid: 3,
                        };

                        let res = select_rec(&user1, &user2, &user3);

                        start_con(&res).await?;

                        loop {
                            println!("");

                            println!("Enter your message");

                            println!("");

                            let mut content: String = String::new();
                            io::stdin()
                                .read_line(&mut content)
                                .expect("Failed to read line");
                            let content = content.trim().to_string();

                            let msg = Message {
                                recieverid: z,
                                messege: content,
                            };

                            // Send the message and check if the user wants to continue
                            if !message(&msg).await? {
                                break; // Exit the loop if the user does not want to continue
                            }
                        }
                    }
                };
            }
            2 => {
                println!("");

                println!("Exiting App");

                println!("");

=======
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


    Ok(())
}

fn login_auth(user: &User, auth: &Auth) -> bool {
    if user.userid == auth.userid && user.password == auth.password {
        println!("Successfully logged in!");

        return true;
    } else {
        return false;
    }
}

async fn login_c(auth: &Auth) -> Result<(), Error> {
    let client = reqwest::Client::new()
        .post("https://7e2c1d1d-ec5c-48c6-9936-5ce12a803b5f.mock.pstmn.io")
        .json(&auth)
        .send()
        .await?;

    println!("{:#?}", client);

    Ok(())
}

fn select_rec<'a>(user1: &'a Users, user2: &'a Users, user3: &'a Users) -> &'a Users {
    if user1.recieverid == 1 {
        return user1;
    } else if user2.recieverid == 1 {
        return user2;
    } else {
        return user3;
    }
}

async fn start_con(res: &Users) -> Result<(), Error> {
    let start = reqwest::Client::new()
        .post("https://7e2c1d1d-ec5c-48c6-9936-5ce12a803b5f.mock.pstmn.io")
        .json(&res)
        .send()
        .await?;

    println!("{:#?}", start);

    Ok(())
}

async fn message(msg: &Message) -> Result<bool, Error> {
    let msg = reqwest::Client::new()
        .post("https://7e2c1d1d-ec5c-48c6-9936-5ce12a803b5f.mock.pstmn.io")
        .json(&msg)
        .send()
        .await?;

    println!("{:#?}", msg);

    Ok(should_continue())
}

fn should_continue() -> bool {
    println!("Do you want to send another message? (yes/no)");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.trim().to_lowercase() == "yes"
=======
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
