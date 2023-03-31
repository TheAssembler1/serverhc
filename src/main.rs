use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::sync::mpsc::channel;
use timer::Timer;

#[derive(Debug, Deserialize)]
struct Config {
    env_type: String,
    http_endpoint: String,
    http_timeout_secs: i64,
}

fn load_env_toml() -> Config {
    let toml_string = fs::read_to_string("Env.toml").expect("Failed to load Env.toml to string");

    toml::from_str(&toml_string).expect("Unable to deserialize Env.toml")
}

fn start_service(config: Config) {
    let timer = Timer::new();

    loop {
        let (tx, rx) = channel();

        timer.schedule_with_delay(
            chrono::Duration::seconds(config.http_timeout_secs),
            move || {
                tx.send(()).unwrap();
            },
        );

        rx.recv().ok();

        println!("HERE");
    }
}

fn main() {
    let config = load_env_toml();
    start_service(config);
}
