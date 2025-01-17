use std::io::{self, Read, Write};

#[macro_export]
macro_rules! success {
    ($($args:tt)*) => {{
        println!("[\x1b[32mSuccess\x1b[0m] {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($($args:tt)*) => {{
        println!("[\x1b[36mInfo\x1b[0m] {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($args:tt)*) => {{
        println!("[\x1b[31mError\x1b[0m] {}", format_args!($($args)*));
    }};
}

#[macro_export]
macro_rules! exit_error {
    ($($args:tt)*) => {{
        println!("[\x1b[31mError\x1b[0m] {}", format_args!($($args)*));
        $crate::utils::pause();
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! warn {
    ($($args:tt)*) => {{
        println!("[\x1b[33mWarning\x1b[0m] {}", format_args!($($args)*));
    }};
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(
        &mut stdout,
        "[\x1b[36mInfo\x1b[0m] Press enter to continue..."
    )
    .unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
