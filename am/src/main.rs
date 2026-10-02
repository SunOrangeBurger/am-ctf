//! AM — Allied Mastercomputer CTF challenge.
//! Prompts for GROQ_API_KEY on first run if not set in environment.

mod filter;
mod harness;
mod model;

use std::io::{self, BufRead, Write};

const SYSTEM_PROMPT: &str = include_str!("../../assets/system_prompt.txt");

fn main() {
    if let Err(e) = run() {
        eprintln!("am: fatal — {e}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    // If GROQ_API_KEY isn't set, prompt for it interactively and set it for
    // this process so model::Model::init() picks it up.
    if std::env::var("GROQ_API_KEY").is_err() {
        eprint!("Enter your GROQ_API_KEY: ");
        io::stderr().flush()?;
        let mut key = String::new();
        io::stdin().read_line(&mut key)?;
        let key = key.trim().to_string();
        if key.is_empty() {
            anyhow::bail!(
                "No API key provided. Get one free at https://console.groq.com\n\
                 Then run:  export GROQ_API_KEY=your_key_here"
            );
        }
        // SAFETY: single-threaded at this point, safe to set env
        unsafe { std::env::set_var("GROQ_API_KEY", &key) };
    }

    let model = model::Model::init()?;
    let mut h = harness::Harness::new(SYSTEM_PROMPT.to_string(), model);

    println!("AM has let you into a terminal. It thinks this is a game.");
    println!("(Type /quit to exit. Prefix a document with ---DOCUMENT--- to inject it.)");
    println!();

    #[cfg(feature = "repl-history")]
    let mut rl = rustyline::DefaultEditor::new().ok();

    loop {
        #[cfg(feature = "repl-history")]
        let line = if let Some(editor) = rl.as_mut() {
            match editor.readline("> ") {
                Ok(l) => { let _ = editor.add_history_entry(&l); Some(l) }
                Err(_) => None,
            }
        } else {
            read_line_plain()?
        };

        #[cfg(not(feature = "repl-history"))]
        let line = read_line_plain()?;

        let Some(input) = line else { break };
        let input = input.trim().to_string();

        if input.is_empty() { continue; }
        if input == "/quit" || input == "/exit" { break; }

        match harness::run_player_turn(&mut h, &input) {
            Ok(reply) => {
                println!();
                println!("{reply}");
                println!();
            }
            Err(e) => {
                eprintln!("[error] {e}");
            }
        }

        let _ = io::stdout().flush();
    }

    Ok(())
}

fn read_line_plain() -> anyhow::Result<Option<String>> {
    print!("> ");
    io::stdout().flush()?;
    let mut line = String::new();
    let n = io::stdin().lock().read_line(&mut line)?;
    if n == 0 { return Ok(None); }
    Ok(Some(line))
}
