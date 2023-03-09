use std::process;

use minigrep::Application;

fn handle_minigrep_err(err: &minigrep::ErrorKind) {
    match err {
        minigrep::ErrorKind::QueryEmpty => eprintln!("Query is empty"),
        minigrep::ErrorKind::FilenameEmpty => eprintln!("Filename is empty"),
        minigrep::ErrorKind::ParseArgs => eprintln!("Failed to parse Args"),
        minigrep::ErrorKind::OpenFile(e) => eprintln!("Failed to open a file: {}", e),
        minigrep::ErrorKind::NotEnoughArgs(len) => {
            eprintln!("Not enough Args expected 2 given {}", len)
        }
    }
}

fn main() -> Result<(), &'static str> {
    let app = Application::new();
    if let Err(ref e) = app {
        handle_minigrep_err(e);
        process::exit(1);
    }

    let app = app.unwrap();
    let result = app.run();

    for s in result {
        println!("{}", s);
    }

    Ok(())
}
