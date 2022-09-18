//! Demonstrates how to read events asynchronously with async-std.
//!
//! cargo run --features="event-stream" --example event-stream-async-std

use std::{
    fs, 
    io::{stdin, stdout}, 
    process::Command, str, 
    time::Duration,
    env,
    path::{Path, PathBuf}
};
//use async_std::path::{Path, PathBuf};

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

#[cfg(debug_assertions)]
async fn print_events(mut selector_loc1:i8, mut location_loc1: PathBuf) {
    let mut reader = EventStream::new();

    loop {

        let mut event = reader.next().fuse();

        select! {

            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {

                        print!("{esc}[2J{esc}[1;1H{}", esc = 27 as char,); 
                        if event == Event::Key(KeyCode::Char('k').into()) {
                            if selector_loc1 > 0 {
                                selector_loc1 -= 1;
                            };

                        } else if event == Event::Key(KeyCode::Char('j').into()) {
                            selector_loc1 += 1;

                        } else if event == Event::Key(KeyCode::Char('h').into()) {

                            //-------------BackLogic-----------------
                            let mut root = PathBuf::new();
                            root.push(Path::new("o:/"));
                            
                            if location_loc1 != root{
                                location_loc1 = location_loc1.parent().map(|p| p.to_owned()).unwrap();
                            selector_loc1 = 0;
                            }

                            
                            
                        } else if event == Event::Key(KeyCode::Char('l').into()) {

                            //------go to next dir------
                            
                                let add = returnsel(&location_loc1,selector_loc1);
                                location_loc1.push(add);
                                selector_loc1 = 0;
                                                        
                        } else if event == Event::Key(KeyCode::Enter.into()) {
                            openf(returnsel(&location_loc1,selector_loc1));
                            //opentest();
                        }
                        
                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }

                        printtype(&location_loc1,selector_loc1);

                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            } 
        };
    }
}


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


// todo function that returns path of current selection.

fn returnsel(loc2: &PathBuf,sel3:i8) -> PathBuf{
     let mut path_buf = PathBuf::new();
     path_buf.push(Path::new("./"));
     if let Ok(entries) = fs::read_dir(loc2) {
         for (i, entry) in entries.into_iter().enumerate() {
             if let Ok(entry) = entry {
                 // Here, `entry` is a `DirEntry`.
                 if let Ok(file_type) = entry.file_type() {
                     if sel3 == i as i8 {
                        path_buf.push( entry.path());

                     // Now let's show our entry's file type!
                     //println!("{}: {:?}", entry.path().display(), file_type);

                    } 
                }
            }
         }
    }
    path_buf
}

pub fn printtype(loc: &PathBuf, selector_loc2: i8) {
    //for (i, pair) in pairs.iter().enumerate() {
    //    println!("{}: key={} value={}", i, pair.key, pair.value);
    // }

    println!("{}", loc.display());
    //println!("{\n}", loc.strip_prefix(r"\?\").unwrap().display());
    //println!("{}",returnsel(loc,selector_loc2).display());
 
    if let Ok(entries) = fs::read_dir(&loc) {
        for (i, entry) in entries.into_iter().enumerate() {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(file_type) = entry.file_type() {
                    if selector_loc2 == i as i8 {
                        execute!(stdout(), SetBackgroundColor(Color::DarkGrey),).unwrap();
                    }
                    if file_type.is_dir() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Blue),
                            Print(format!("{} \n",entry.file_name().to_string_lossy())),
                            ResetColor
                        )
                        .unwrap();
                    } else if file_type.is_file() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Reset),
                            Print(format!("{}\n" ,entry.file_name().to_string_lossy())),
                            ResetColor
                        )
                        .unwrap();
                    } else if file_type.is_symlink() {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::Cyan),
                            Print(format!("{} \n",  entry.file_name().to_string_lossy())),
                            
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
    let selector:i8 = 0;


    let srcdir = env::current_dir()?;
    

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    printtype(&srcdir,selector);
    enable_raw_mode()?;
    //let mut stdout = stdout();
    //execute!(stdout, EnableMouseCapture)?;
    async_std::task::block_on(print_events(selector, srcdir));
    // execute!(stdout, DisableMouseCapture)?;
    disable_raw_mode()
}

fn openf( loc3 : PathBuf) 
{
    if cfg!(target_os = "windows") {
        Command::new(loc3.as_os_str())
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
}

fn opentest() {
    let patharoon = PathBuf::from("C:/Program Files/PowerToys/PowerToys.exe");
    let test = patharoon.as_os_str();
    if cfg!(target_os = "windows") {
        Command::new(test)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
}


// placera suggestion
// fn open_file_browser() {
//     let mut dir = std::env::current_dir();
//     for event in input_events {
//         match event {
//             GoToParent => dir = match dir.parent() {
//                 Some(d) => d.to_owned(),
//                 None => dir,
//             }
//             GoToChild(child_name) => dir = dir.join(child_name)
//             CreateFile(file_name) => fs::write(dir.join(file_name), b"nothing much")
//         }
//     }
// }

// strip from cananicalised path
// use std::path::*;
// fn remove_verbatim_disk(path: PathBuf) -> PathBuf {
//     let mut components = path.components();
//     if let Some(n) = components.next().and_then(|c| match c {
//         std::path::Component::Prefix(prefix) => Some(prefix),
//         _ => None
//     }).and_then(|p| match p.kind() {
//         std::path::Prefix::VerbatimDisk(n) => Some(n),
//         _ => None
//     }) {
//         std::path::PathBuf::from(format!("{}:\\", n as char)).join(components.as_path())
//     } else {
//         path
//     }
// }

// use object::read::*;
// use pe::{ImageNtHeaders, ImageOptionalHeader};
// let reader = ReadCache::new(std::fs::File::open(path_to_exe)?);
// let subsystem = match FileKind::parse(&reader)? {
//     FileKind::Pe32 => pe::PeFile32::parse(reader)?.nt_headers().optional_header().subsystem(),
//     FileKind::Pe64 => pe::PeFile64::parse(reader)?.nt_headers().optional_header().subsystem(),
//     _ => todo!("it's not an exe")
// };
// if subsystem == object::pe::IMAGE_SUBSYSTEM_WINDOWS_GUI {
//     // its a gui
// } else {
//     // its a cui
// }
