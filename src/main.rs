use std::io::{stdin, stdout, Write, self};
use temple_of_texts::state::State;

const INTRO: &str = include_str!("intro.txt");
const HELP: &str = include_str!("help.txt");

fn main() -> io::Result<()> {
    println!("{}", INTRO);
    let mut state: State = rand::random();
    prompt("> ")?;
    for l in stdin().lines() {
        let s = match l?.trim() {
            "l" => {
                state.turn_left();

                let description = if state.door_in_front() {
                    "door"
                } else {
                    "wall of text"
                };

                format!("You turn left, to a {}", description)
            }
            "r" => {
                state.turn_right();

                let description = if state.door_in_front() {
                    "door"
                } else {
                    "wall of text"
                };

                format!("You turn right, to a {}", description)
            }
            "q" => break,
            "m" => {
                match state.move_forward() {
                    Err(HitWallError) => "Oops! You lightly hit your head against the wall in front of you.",
                    Ok(()) => "You open the door and get through it, to a seemingly identical room.",
                }.to_string()
            }
            "s" | "show" => state.describe(),
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
