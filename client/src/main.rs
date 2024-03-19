use log::info;
use reqwest::Error;
use serde::Serialize;
use std::io;

#[derive(Debug, Serialize)]
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
    recieverid: String,
}

#[derive(Debug, Serialize)]
pub struct Message {
    sender: String,
    receiver: String,
    content: String,
}

fn display_menu() {
    println!("Welcome to App");
    println!("");
    println!("1. Login");
    println!("2. Exit");
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let user1 = User {
        userid: "user1".to_string(),
        password: "password1".to_string(),
    };
    let user2 = User {
        userid: "user2".to_string(),
        password: "password2".to_string(),
    };
    let user3 = User {
        userid: "user3".to_string(),
        password: "password3".to_string(),
    };

    let mut logged_in = false;

    loop {
        if !logged_in {
            display_menu();
            println!("");
        }

        println!("Enter your choice (1-2):");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                println!("");
                println!("Invalid input please make sure you enter valid input");
                continue;
            }
        };

        match choice {
            1 => {
                println!("");
                println!("Welcome to Login App");
                println!("");
                println!("Enter UserId");

                let mut x: String = String::new();
                io::stdin().read_line(&mut x).expect("Failed to read line");
                let x = x.trim().to_string();

                println!("");
                println!("Enter password");

                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read line");
                let y = y.trim().to_string();

                let auth = Auth {
                    userid: x.clone(),
                    password: y,
                };
                info!("{:?}", auth);
                logged_in = login_auth(&user1, &user2, &user3, &auth);

                if logged_in == true {
                    println!("");
                    user_login(&auth).await?;

                    println!("");
                    println!("Enter Reciever Id");

                    let mut z = String::new();
                    io::stdin().read_line(&mut z).expect("Failed to read line");
                    let z = z.trim().to_string();

                    let user1 = Users {
                        userid: x.clone(),
                        recieverid: "user1".to_string(),
                    };
                    let user2 = Users {
                        userid: x.clone(),
                        recieverid: "user2".to_string(),
                    };
                    let user3 = Users {
                        userid: x.clone(),
                        recieverid: "user3".to_string(),
                    };
                    println!("");
                    let valid_user = valid_user(&user1, &user2, &user3);

                    if valid_user == true {
                        let res = select_reciever(&user1, &user2, &user3);

                        start_conversation(&res).await?;

                        loop {
                            println!("");
                            println!("Enter your message");

                            let mut mess: String = String::new();
                            io::stdin()
                                .read_line(&mut mess)
                                .expect("Failed to read line");
                            let mess = mess.trim().to_string();

                            let msg = Message {
                                sender: x.clone(),
                                receiver: z.clone(),
                                content: mess,
                            };

                            if !send_message(&msg).await? {
                                println!("");
                                display_menu();
                                break;
                            }
                            let response = get_message(&msg).await?; // Assuming get_message is an async function that returns a Result
                            println!("Received response: {:?}", response);
                        }
                    } else {
                        println!("Invalid User");
                    }
                } else {
                    println!("");
                    println!("Invalid user! Please try again..");
                    println!("");
                }
            }
            2 => {
                println!("");
                println!("Exiting App");
                println!("");
                break;
            }
            _ => println!("Invalid operations, please select a valid option (1-3)."),
        }
    }

    Ok(())
}

fn login_auth(user1: &User, user2: &User, user3: &User, auth: &Auth) -> bool {
    if user1.userid == auth.userid && user1.password == auth.password {
        return true;
    }
    if user2.userid == auth.userid && user2.password == auth.password {
        return true;
    }
    if user3.userid == auth.userid && user3.password == auth.password {
        return true;
    } else {
        return false;
    }
}

async fn user_login(auth: &Auth) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3010/login")
        .json(&auth)
        .send()
        .await?;

    if res.status().is_success() {
        // Assuming the server returns a JSON response
        let response_body: serde_json::Value = res.json().await?;

        println!("Login successful! Response: {:?}", response_body);
    } else {
        // If the server returns an error, print the status code and response body
        let status = res.status();
        let error_text = res.text().await?;

        println!("Login failed with status {}: {}", status, error_text);
    }
    Ok(())
}

fn valid_user<'a>(user1: &'a Users, user2: &'a Users, user3: &'a Users) -> bool {
    if user1.recieverid == "user1" {
        return true;
    } else if user2.recieverid == "user2" {
        return true;
    } else if user3.recieverid == "user3" {
        return true;
    } else {
        return false;
    }
}

fn select_reciever<'a>(user1: &'a Users, user2: &'a Users, user3: &'a Users) -> &'a Users {
    if user1.recieverid == "user1" {
        return user1;
    } else if user2.recieverid == "user2" {
        return user2;
    } else {
        return user3;
    }
}

async fn start_conversation(res: &Users) -> Result<(), Error> {
    let start = reqwest::Client::new()
        .post("http://localhost:3010/status")
        .json(&res)
        .send()
        .await?;

    if start.status().is_success() {
        let response_body: serde_json::Value = start.json().await?;

        println!("User get selected {:?}", response_body);
    } else {
        let status = start.status();
        let error_text = start.text().await?;

        println!("Invalid user {}: {}", status, error_text);
    }
    Ok(())
}

async fn send_message(msg: &Message) -> Result<bool, Error> {
    let msg1 = reqwest::Client::new()
        .post("http://localhost:3010/sender")
        .json(&msg)
        .send()
        .await?;

    if msg1.status().is_success() {
        let response_body: serde_json::Value = msg1.json().await?;

        println!("Response {:?}", response_body);
        Ok(true)
    } else {
        let status = msg1.status();
        let error_text = msg1.text().await?;

        println!("failed     {}: {}", status, error_text);
        Ok(false)
    }
}

async fn get_message(msg: &Message) -> Result<bool, Error> {
    let msg1 = reqwest::Client::new()
        .get("http://localhost:3010/receiver")
        .json(&msg)
        .send()
        .await?;

    if msg1.status().is_success() {
        let response_body: serde_json::Value = msg1.json().await?;

        println!("Response {:?}", response_body);
        Ok(true)
    } else {
        let status = msg1.status();
        let error_text = msg1.text().await?;
        println!("");
        println!("failed     {}: {}", status, error_text);
        Ok(false)
    }
}

fn should_continue() -> bool {
    info!("Do you want to send another message? (yes/no)");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.trim().to_lowercase() == "yes"
}
