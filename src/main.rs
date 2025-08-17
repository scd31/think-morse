/*
 * Copyright 2024 Arbaaz Laskar
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use morse::encode;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process;
use std::thread;
use std::time::Duration;

const THINKPAD_LID_LOGO_LED: &str = "/sys/class/leds/tpacpi::lid_logo_dot/brightness";

fn led(state: bool) -> io::Result<()> {
    let value = if state { "255" } else { "0" };

    File::create(THINKPAD_LID_LOGO_LED)?.write_all(value.as_bytes())?;

    Ok(())
}
fn encode_morse(input: &str) -> io::Result<String> {
    match encode::encode(input) {
        Ok(morse) => Ok(morse),
        Err(e) => Err(io::Error::other(format!("Failed to encode Morse: {:?}", e))),
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let morse_str = match args.len() {
        1 => {
            eprintln!("No string argument provided. Exiting.");
            process::exit(1);
        }
        2 => encode_morse(&args[1])?,
        _ => {
            eprintln!("Too many arguments. Exiting.");
            process::exit(1);
        }
    };

    let timer = MorseTimer::new(15.0);

    let char_vec: Vec<char> = morse_str.chars().collect();

    led(false)?;

    loop {
        let mut last_char = '\0';
        let mut iter = char_vec.iter().peekable();
        while let Some(c) = iter.next() {
            let is_end_of_character = [Some(' '), Some('/')].contains(&iter.peek().map(|x| **x));

            if *c == '.' {
                led(true)?;
                thread::sleep(timer.dot_length());
                led(false)?;

                if !is_end_of_character {
                    thread::sleep(timer.inner_character_gap_length());
                }
            } else if *c == '_' {
                led(true)?;
                thread::sleep(timer.dash_length());
                led(false)?;

                if !is_end_of_character {
                    thread::sleep(timer.inner_character_gap_length());
                }
            } else if *c == ' ' {
                // so that we don't sleep in the space before/after a slash
                if last_char != '/' && iter.peek() != Some(&&'/') {
                    thread::sleep(timer.letter_gap_length());
                }
            } else if *c == '/' {
                thread::sleep(timer.word_gap_length());
            }

            last_char = *c;
        }
        thread::sleep(timer.loop_gap_length());
    }
}

struct MorseTimer {
    dot_length: Duration,
}

impl MorseTimer {
    fn new(wpm: f64) -> Self {
        let dot_length = Duration::from_secs_f64(60.0 / (wpm * 50.0));
        Self { dot_length }
    }

    fn dot_length(&self) -> Duration {
        self.dot_length
    }

    fn dash_length(&self) -> Duration {
        self.dot_length * 3
    }

    fn inner_character_gap_length(&self) -> Duration {
        self.dot_length
    }

    fn letter_gap_length(&self) -> Duration {
        self.dot_length * 3
    }

    fn word_gap_length(&self) -> Duration {
        self.dot_length * 7
    }

    fn loop_gap_length(&self) -> Duration {
        self.dot_length * 15
    }
}
