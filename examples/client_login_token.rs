use kick_rs::Kick;

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
    let token = get_input("> Enter your login token: ");
    let kick = Kick::with_token(token);
    let user = kick.get_me().await.unwrap();
    println!("Me {:#?}", user);
}
