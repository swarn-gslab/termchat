use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    userid: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LoginResponse {
    message: Option<String>,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiveUser {
    userid: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ReceiverResponse {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
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
    let mut user_token = String::new();
    let mut active_conversation = String::new();

    loop {
        let mut logged_in = false;
        if !logged_in {
            println!("");
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

                let loggedin_user = x.clone();

                println!("");
                println!("Enter password");

                let mut y = String::new();
                io::stdin().read_line(&mut y).expect("Failed to read line");
                let y = y.trim().to_string();

                let auth = Auth {
                    userid: x.clone(),
                    password: y.clone(),
                };

                match user_login(&auth).await {
                    Ok(Some(token)) => {
                        user_token = token.clone();

                        logged_in = true;
                    }
                    Ok(None) => {
                        logged_in = false;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        logged_in = false;
                        continue;
                    }
                }

                if logged_in == true {
                    println!("");
                    listof_users(loggedin_user.as_str());

                    println!("");
                    println!("Enter Receiver Id");

                    let mut z = String::new();
                    io::stdin().read_line(&mut z).expect("Failed to read line");
                    let z = z.trim().to_string();

                    let receiver_user = ReceiveUser { userid: z.clone() };
                    let token = user_token.clone();

                    start_conversation(&receiver_user, &token, &active_conversation).await?;
                    println!(":?", active_conversation);

                    loop {
                        println!("");
                        println!("Enter your message");

                        let mut mess: String = String::new();
                        io::stdin()
                            .read_line(&mut mess)
                            .expect("Failed to read line");
                        let mess = mess.trim().to_string();

                        let msg = Message { content: mess };

                        match send_message(&msg, &user_token).await {
                            Ok(true) => println!("Message sent successfully"),
                            Ok(false) => {
                                println!("Message sending failed");
                                break;
                            }
                            Err(e) => {
                                println!("Error sending message: {}", e);
                                break;
                            }
                        }
                    }
                } else {
                    println!("Invalid user !");
                }
            }
            2 => {
                println!("");
                println!("Exiting App");
                println!("");
                break;
            }
            _ => {
                println!("");
                println!("Invalid operations, please select a valid option (1-2).")
            }
        }
    }

    Ok(())
}

async fn user_login(auth: &Auth) -> Result<Option<String>, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3010/login")
        .json(&auth)
        .send()
        .await?;

    if res.status().is_success() {
        let response_body: LoginResponse = res.json().await?;
        println!("");
        println!("Login Successfully !");
        Ok(Some(response_body.token))
    } else {
        let status = res.status();
        let error_text = res.text().await?;
        println!("");
        println!("Login failed with status {}: {}", status, error_text);
        println!("");
        Ok(None)
    }
}
fn listof_users(loggedin_user: &str) {
    println!("Total available users ");
    println!("");
    match loggedin_user {
        "user1" => {
            println!("User2");
            println!("User3");
        }
        "user2" => {
            println!("User1");
            println!("User3");
        }
        "user3" => {
            println!("User1");
            println!("User2");
        }
        _ => {
            println!("");
        }
    }
}

async fn start_conversation(
    receiver_user: &ReceiveUser,
    token: &str,
    mut active_conversation: &String,
) -> Result<Option<String>, Error> {
    let start = reqwest::Client::new()
        .post("http://localhost:3010/start_conversation")
        .header("Authorization", "Bearer ".to_owned() + &token)
        .json(&receiver_user)
        .send()
        .await?;
    active_conversation = &start.status().to_string();
    if start.status().is_success() {
        let response_body = "Receiver user is availabe".to_string();
        println!("");
        println!("{:?}", response_body);
        Ok(Some(response_body))
    } else {
        println!("");
        println!("Receiver id is not valid or not active !");
        Ok(None)
    }
}

async fn send_message(msg: &Message, user_token: &str) -> Result<bool, Error> {
    let msg1 = reqwest::Client::new()
        .post("http://localhost:3010/send_message")
        .header("Authorization", "Bearer ".to_owned() + &user_token)
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

        println!("failed {}: {}", status, error_text);
        Ok(false)
    }
}

async fn get_message(msg: &Message) -> Result<bool, Error> {
    let msg1 = reqwest::Client::new()
        .get("http://localhost:3010/receive_message")
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
        println!("failed {}: {}", status, error_text);
        Ok(false)
    }
}

// fn should_continue() -> bool {
// info!("Do you want to send another message? (yes/no)");
// let mut response = String::new();
// io::stdin()
// .read_line(&mut response)
// .expect("Failed to read line");
// response.trim().to_lowercase() == "yes"
// }
