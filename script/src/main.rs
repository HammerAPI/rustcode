use std::process::Command;

fn main() {
    println!("Launching NordVPN autostart script");

    let process = "nordvpn";
    let args = ["c", "us"];
    let mut child = Command::new(process)
        .args(&args)
        .spawn()
        .expect("FAILED TO AUTOSTART SCRIPT");

    child.wait().expect("Failed to wait on script execution");
}
