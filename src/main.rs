use std::io;

fn main() {
	send_message();
}

fn send_message() -> io::Result<()> {
	loop {
    	let mut buffer = String::new();
    	let mut stdin = io::stdin(); // We get `Stdin` here.
    	stdin.read_line(&mut buffer)?;
    	println!("user say: {}", buffer);
	}

    Ok(())
}