use nix::unistd::{fork, getpid, ForkResult};
use std::{thread, time};

fn main() {
    let num = fork();

    let me = getpid();

    match num {
        Ok(ForkResult::Parent { child }) => {
            thread::sleep(time::Duration::from_secs(1));
            println!(
                "In the Parent thread with pid {}, new child has pid {}",
                me, child
            );
        }
        Ok(ForkResult::Child) => println!("In the Child thread with pid {}", me),
        Err(e) => println!("Fork failed\n{}", e),
    }
}
