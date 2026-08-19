use std::io::{stdin, stdout, Write, self};
use temple_of_texts::state::{State, HitWallError};

const INTRO: &str = include_str!("intro.txt");
const HELP: &str = include_str!("help.txt");

fn main() -> io::Result<()> {
    println!("{}", INTRO);
    let mut state: State = rand::random();
    prompt("> ")?;
    let mut lines = stdin().lines();
    while let Some(l) = lines.next() {
        let s = match l?.trim() {
            "l" | "left" => {
                state.turn_left();

                let description = if state.door_in_front() {
                    "door"
                } else {
                    "wall of text"
                };

                format!("You turn left, to a {}", description)
            }
            "r" | "right" => {
                state.turn_right();

                let description = if state.door_in_front() {
                    "door"
                } else {
                    "wall of text"
                };

                format!("You turn right, to a {}", description)
            }
            "q" | "quit" => break,
            "m" | "move" => {
                match state.move_forward() {
                    Ok(()) => "You open the door and get through it, to a seemingly identical room.",
                    Err(HitWallError) => "Oops! You lightly hit your head against the wall in front of you.",
                }.to_string()
            }
            "dump" => state.to_code(),
            "load" => {
                prompt("Enter code: ")?;
                let Some(input) = lines.next() else { break };

                match State::from_code(input?.trim()) {
                    Ok(s) => {
                        state = s;
                        "state loaded.".to_string()
                    }
                    Err(_) => "The entered code is not valid.".to_string(),
                }
            }
            "s" | "show" => state.describe(),
            "k" | "key" => todo!(),
            _ => HELP.to_string()
        };
        println!("{}", s);
        prompt("> ")?;
    }
    Ok(())
}

fn prompt(s: &str) -> io::Result<()> {
    let mut out = stdout();
    write!(out, "{}", s)?;
    out.flush()
}
