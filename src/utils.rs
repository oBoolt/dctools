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
macro_rules! warn {
    ($($args:tt)*) => {{
        println!("[\x1b[33mWarning\x1b[0m] {}", format_args!($($args)*));
    }};
}
