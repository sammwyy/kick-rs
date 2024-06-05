use kick_rs::{Kick, KickAuthOTPResult, KickAuthResult};

pub fn get_input(prompt: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

#[tokio::main]
async fn main() {
    let mut kick = Kick::new();
    let email = get_input("> Email: ");
    let password = get_input("> Password: ");
    println!("Logging in...");

    let result = kick.login(email, password).await;
    loop {
        match result {
            KickAuthResult::TFARequired => {
                let otp = get_input("> Required OTP (Check email): ");
                let result = kick.login_otp(otp).await;

                loop {
                    match result {
                        KickAuthOTPResult::InvalidOTP => {
                            println!("Invalid OTP");
                        }
                        KickAuthOTPResult::NoLogged => {
                            println!("Not logged in");
                            return;
                        }
                        KickAuthOTPResult::Success => {
                            println!("Login successful");
                            break;
                        }
                        KickAuthOTPResult::UnknownError => {
                            println!("Unknown error");
                            return;
                        }
                    }
                }

                break;
            }
            KickAuthResult::OTPRequired => {
                println!("Two-factor authentication required");
                return;
            }
            KickAuthResult::Success => {
                println!("Login successful");
                break;
            }
            KickAuthResult::InvalidCredentials => {
                println!("Invalid credentials");
            }
            KickAuthResult::UnknownError => {
                println!("Unknown error");
                return;
            }
        }
    }

    println!("Token: {}", kick.get_token().unwrap());
    let user = kick.get_me().await.unwrap();
    println!("Me {:#?}", user);
}
