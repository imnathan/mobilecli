#![deny(warnings)]

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::env;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(name = "url")]
    arg_url: String,
    #[structopt(name = "path")]
    arg_path: String,
}

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

fn run(args: &Args) -> Result<(), git2::Error> {
    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(&args.arg_url, Path::new(&args.arg_path))?;
    println!();

    Ok(())
} 

fn main() {
   let mut language = String::new();
   
    println!(" *  Select Language *");
    println!(" * 1. iOS - SwiftUI *");
    println!(" * 2. iOS - UIKit   *");
    println!(" * 3. Android       *");
    println!(" * 4. Flutter       *");
    println!(" * 5. React Native  *");
    println!(" * 6. Ionic         *");

    io::stdin()
    .read_line(&mut language)
    .expect("failed to read your input");
    let selected_lang = (language.trim()).to_string();

    let cwd = env::current_dir().unwrap();
    let cwd_path = cwd.into_os_string().into_string().unwrap();
    
    if selected_lang == "1" {
       println!("SwiftUI");
        let args = Args { arg_url: "https://github.com/nalexn/clean-architecture-swiftui".to_string(), arg_path: cwd_path + "/SwiftUI" };
         match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
    } else if selected_lang == "2"  {
        println!("UIKit");
        
    } else if selected_lang == "3"  {
        println!("Android")
        
    } else if selected_lang == "4"  {
        println!("Flutter")
        
    } else if selected_lang == "5"  {
        println!("RN")
        
    } else if selected_lang == "6"  {
        println!("Ionic")
        
    } else  {
        println!("Invalid input ")
    } 

}

