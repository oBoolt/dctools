use std::io::{self, Read, Write};

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
