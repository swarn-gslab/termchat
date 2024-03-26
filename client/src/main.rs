use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    receiver_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ResponseReceiver {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    receiver_id: String,
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

                        continue;
                    }
                }

                if logged_in == true {
                    println!("");
                    listof_users(loggedin_user.as_str());
                    loop {
                        println!("");
                        println!("Enter Receiver Id");

                        let mut z = String::new();
                        io::stdin().read_line(&mut z).expect("Failed to read line");
                        let z = z.trim().to_string();

                        let receiver_user = ReceiveUser {
                            receiver_id: z.clone(),
                        };
                        let token = user_token.clone();

                        if let Ok(success) = start_conversation(&receiver_user, &token).await {
                            if success {
                                loop {
                                    println!("");
                                    println!("Enter your message");

                                    let mut mess: String = String::new();
                                    io::stdin()
                                        .read_line(&mut mess)
                                        .expect("Failed to read line");
                                    let mess = mess.trim().to_string();

                                    let msg = Message {
                                        content: mess,
                                        receiver_id: z.clone(),
                                    };
                                    println!("");
                                    get_message(&msg, &user_token).await?;

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
                                println!("Invalid receiver ID or Not-active User !");
                                continue;
                            }
                        } else {
                            println!("Failed to start conversation due to an error");
                            continue;
                        }
                    }
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
fn listof_users(loggedin_user: &str) {
    println!("Total available users ");
    println!("");
    match loggedin_user {
        "Swarnjit" => {
            println!("Sanjeev");
            println!("Kamlesh");
        }
        "Sanjeev" => {
            println!("Swarnjit");
            println!("Kamlesh");
        }
        "Kamlesh" => {
            println!("Swarnjit");
            println!("Sanjeev");
        }
        _ => {
            println!("");
        }
    }
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
async fn start_conversation(receiver_user: &ReceiveUser, token: &str) -> Result<bool, Error> {
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3010/start_conversation")
        .header("Authorization", format!("Bearer {}", token))
        .json(receiver_user)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body: Value = response.json().await?;
        if let Some(message) = response_body.as_str() {
            println!("{}", message);
        } else {
            println!("Response is not a string");
        }
        Ok(true)
    } else {
        println!("");
        Ok(false)
    }
}

async fn send_message(msg: &Message, user_token: &str) -> Result<bool, Error> {
    let response = reqwest::Client::new()
        .post("http://localhost:3010/send_message")
        .header("Authorization", "Bearer ".to_owned() + &user_token)
        .json(&msg)
        .send()
        .await?;
    if response.status().is_success() {
        let response_body = response.text().await?;
        if response_body.trim() == "Message sent successfully" {
            println!("");
            Ok(true)
        } else {
            println!("Unexpected response: {}", response_body);
            Ok(false)
        }
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("failed {}: {}", status, error_text);
        Ok(false)
    }
}

async fn get_message(msg: &Message, user_token: &str) -> Result<bool, Error> {
    let response = reqwest::Client::new()
        .post("http://localhost:3010/receive_message")
        .header("Authorization", "Bearer ".to_owned() + &user_token)
        .json(&msg)
        .send()
        .await?;
    if response.status().is_success() {
        let response_body: serde_json::Value = response.json().await?;

        if let Some(messages) = response_body.get("messages").and_then(|m| m.as_array()) {
            println!("Message Received: ",);
            for message in messages {
                if let Some(message_content) = message.as_str() {
                    println!(" {}", message_content);
                } else {
                    println!("Invalid message content");
                }
            }
        } else {
            println!("No messages found in Received message");
        }

        Ok(true)
    } else {
        let status = response.status();
        let error_text = response.text().await?;
        println!("Failed {}: {}", status, error_text);
        Ok(false)
    }
}