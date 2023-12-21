use daemonize::Daemonize;

use std::fs::File;

fn main_loop() {
    loop {
        println!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .umask(0o027)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(v) => {
            println!("{:?}", v);
            println!("Success, daemonized");
            main_loop();
        },
        Err(e) => {
            eprintln!("Error, {}", e);
        }
    }
}
