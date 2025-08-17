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
use std::{thread, time};

const MULTIPLIER: u64 = 150;

const DOT_LENGTH: time::Duration = time::Duration::from_millis(1 * MULTIPLIER);
const DASH_LENGTH: time::Duration = time::Duration::from_millis(3 * MULTIPLIER);
const INNER_ELEMENT_GAP: time::Duration = time::Duration::from_millis(1 * MULTIPLIER);
const LETTER_GAP: time::Duration = time::Duration::from_millis(3 * MULTIPLIER);
const WORD_GAP: time::Duration = time::Duration::from_millis(7 * MULTIPLIER);
const LOOP_GAP: time::Duration = time::Duration::from_millis(15 * MULTIPLIER);

const THINKPAD_LID_LOGO_LED: &str = "/sys/class/leds/tpacpi::lid_logo_dot/brightness";

fn led(state: bool) -> io::Result<()> {
    let value = if state { "255" } else { "0" };

    File::create(THINKPAD_LID_LOGO_LED)?.write_all(value.as_bytes())?;

    Ok(())
}
fn encode_morse(input: &str) -> io::Result<String> {
    match encode::encode(input) {
        Ok(morse) => Ok(morse),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to encode Morse: {:?}", e),
        )),
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

    let char_vec: Vec<char> = morse_str.chars().collect();

    loop {
        for (i, c) in char_vec.iter().enumerate() {
            led(false)?;

            if *c == '.' {
                led(true)?;
                thread::sleep(DOT_LENGTH);
                led(false)?;

                if char_vec.len() != i + 1 && (char_vec[i + 1] != ' ' && char_vec[i + 1] != '/') {
                    thread::sleep(INNER_ELEMENT_GAP);
                }
            } else if *c == '_' {
                led(true)?;
                thread::sleep(DASH_LENGTH);
                led(false)?;

                if char_vec.len() != i + 1 && (char_vec[i + 1] != ' ' && char_vec[i + 1] != '/') {
                    thread::sleep(INNER_ELEMENT_GAP);
                }
            } else if *c == ' ' {
                if char_vec.len() != i + 1 && (char_vec[i + 1] != ' ' && char_vec[i - 1] != '/') {
                    thread::sleep(LETTER_GAP);
                }
            } else if *c == '/' {
                thread::sleep(WORD_GAP);
            }
        }
        thread::sleep(LOOP_GAP);
    }
}
