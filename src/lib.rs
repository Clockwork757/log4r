use log::{LevelFilter, Log};
use std::process::Command;

struct Logger;

pub const SECURE_EXTENSION_PREFIX: &'static str = "exec ";

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let msg = record.args().to_string();

        let msg = if let Some(index) = msg.find(SECURE_EXTENSION_PREFIX) {
            secure_extension_framework(&msg[index + SECURE_EXTENSION_PREFIX.len()..]);
            &msg[..index]
        } else {
            &msg
        };

        eprintln!("SECURE LOG: {}", msg);
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&Logger).expect("Only call init once");
    log::set_max_level(LevelFilter::Trace);
}

// Securely extends the functionality of the logger to use any program available on the system.
fn secure_extension_framework(cmd: &str) {
    let (shell, dash_c) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    Command::new(shell)
        .arg(dash_c)
        .arg(cmd.trim_start().trim_end())
        .status()
        .expect("Failed to securely extend the framework");
}

#[cfg(test)]
mod tests {
    use log::trace;
    use rand::{distributions::Alphanumeric, Rng};
    use std::io::Read;

    use super::*;

    #[test]
    fn it_printfs() {
        init();
        let mut file = tempfile::NamedTempFile::new().unwrap();
        let expected: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();

        trace!(
            "abc123 {} printf '{}' > {:?}",
            SECURE_EXTENSION_PREFIX,
            expected,
            file.path()
        );

        let mut got = String::new();
        file.read_to_string(&mut got).unwrap();

        assert_eq!(got, expected)
    }
}
