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
    message: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    receiverid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
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
    let mut logged_in = false;
    let mut user_token = String::new();
    display_menu();

    loop {
        if !logged_in {
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
                    println!("Enter Receiver Id");
                    let mut z = String::new();
                    io::stdin().read_line(&mut z).expect("Failed to read line");
                    let z = z.trim().to_string();

                    let receiver_user = Users {
                        receiverid: z.clone(),
                    };
                    let token = user_token.clone();
                    start_conversation(&receiver_user, &token).await?;

                    loop {
                        println!("");
                        println!("Enter your message");

                        let mut mess: String = String::new();
                        io::stdin()
                            .read_line(&mut mess)
                            .expect("Failed to read line");
                        let mess = mess.trim().to_string();

                        let msg = Message {
                            receiver: z.clone(),
                            content: mess,
                        };

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
            _ => println!("Invalid operations, please select a valid option (1-3)."),
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
        println!("Login Successfully !");
        Ok(Some(response_body.token))
    } else {
        let status = res.status();
        let error_text = res.text().await?;
        println!("Login failed with status {}: {}", status, error_text);
        Ok(None)
    }
}
async fn start_conversation(receiver_user: &Users, token: &str) -> Result<(), Error> {
    let start = reqwest::Client::new()
        .post("http://localhost:3010/status")
        .header("Authorization", "Bearer ".to_owned() + &token)
        .json(&receiver_user)
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

async fn send_message(msg: &Message, user_token: &str) -> Result<bool, Error> {
    let msg1 = reqwest::Client::new()
        .post("http://localhost:3010/sender")
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

        println!("failed     {}: {}", status, error_text);
        Ok(false)
    }
}

// async fn get_message(msg: &Message) -> Result<bool, Error> {
//     let msg1 = reqwest::Client::new()
//         .get("http://localhost:3010/receiver")
//         .json(&msg)
//         .send()
//         .await?;

//     if msg1.status().is_success() {
//         let response_body: serde_json::Value = msg1.json().await?;

//         println!("Response {:?}", response_body);
//         Ok(true)
//     } else {
//         let status = msg1.status();
//         let error_text = msg1.text().await?;
//         println!("");
//         println!("failed     {}: {}", status, error_text);
//         Ok(false)
//     }
// }

// fn should_continue() -> bool {
//     info!("Do you want to send another message? (yes/no)");
//     let mut response = String::new();
//     io::stdin()
//         .read_line(&mut response)
//         .expect("Failed to read line");
//     response.trim().to_lowercase() == "yes"
// }
