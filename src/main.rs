//! Demonstrates how to read events asynchronously with async-std.
//!
//! cargo run --features="event-stream" --example event-stream-async-std

use std::{
    io::{stdin, stdout},
    process::Command,
    fs, 
    time::Duration
};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    //cursor::position,
    //DisableMouseCapture, EnableMouseCapture add below between brackets
    event::{Event, EventStream, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
    //execute,
    tty::IsTty,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor}
};

// const HELP: &str = r#"EventStream based on futures_util::stream::Stream with async-std
//  - Keyboard, mouse and terminal resize events enabled
//  - Prints "." every second if there's no event
//  - Hit "c" to print current cursor position
//  - Use Esc to quit
// "#;

async fn print_events(mut selector_loc1:i8) {
    let mut reader = EventStream::new();

    loop {
        //let delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        select! {
            // _ = delay => {
            //      print!("{esc}[2J{esc}[1;1H{}", esc = 27 as char,); 
                 
            // },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        //println!("Event::{:?}\r", event);
                        
                        // if event == Event::Mouse(MouseEvent::Up("Left").into()) {
                        //     println!("Cursor position: {:?}\r", position());
                        // }   
                        print!("{esc}[2J{esc}[1;1H{}", esc = 27 as char,); 
                        if event == Event::Key(KeyCode::Char('k').into()) {
                            if selector_loc1 > 0 {
                                selector_loc1 -= 1;
                            }
                            //println!("go down");
                            //println!("{}",selected)
                        }   else if event == Event::Key(KeyCode::Char('j').into()) {
                            selector_loc1 += 1;
                            //println!("go up");
                            //println!("{}",selected)
                        }   else if event == Event::Key(KeyCode::Char('h').into()) {
                            println!("go left")
                        }   else if event == Event::Key(KeyCode::Char('l').into()) {
                            println!("go right")
                        }   if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }

                        printtype("./",selector_loc1);

                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

pub fn printtype(loc: &str, selector_loc2:i8) {

    //for (i, pair) in pairs.iter().enumerate() {
    //    println!("{}: key={} value={}", i, pair.key, pair.value);
    // }

    if let Ok(entries) = fs::read_dir(&loc) {
        for (i, entry) in entries.into_iter().enumerate() {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(file_type) = entry.file_type() {
                    if selector_loc2 == i as i8 {
                        execute!(
                            stdout(),
                            SetBackgroundColor(Color::DarkGrey),
                        )
                        .unwrap();
                    }
                    if file_type.is_dir() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Blue),
                            Print(format!("{}: {} \n",i, entry.path().display())),
                            ResetColor
                        )
                        .unwrap();
                    } else if file_type.is_file() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Reset),
                            Print(format!("{}: {} \n",i, entry.path().display())),
                            ResetColor
                        )
                        .unwrap();
                    }   else if file_type.is_symlink() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Cyan),
                            Print(format!("{}: {} \n",i, entry.path().display())),
                            ResetColor
                        )
                        .unwrap();
                    }
                    // Now let's show our entry's file type!
                    //println!("{}: {:?}", entry.path().display(), file_type);
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
   }

}



fn main() -> Result<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let selector:i8 = 0;
    printtype("./",selector);
    enable_raw_mode()?;
    // execute!(stdout, EnableMouseCapture)?;
    async_std::task::block_on(print_events(selector));
    // execute!(stdout, DisableMouseCapture)?;
    disable_raw_mode()
}