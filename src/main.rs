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
    println!("Hello, world!");

    // Todo : Make the API fluent
    let app = Application::new();
    if let Err(ref e) = app {
        handle_minigrep_err(e);
    }

    let app = app.unwrap();
    match app.run() {
        Ok(result) => {
            for s in result {
                println!("{}", s);
            }
        }
        Err(err) => handle_minigrep_err(&err),
    }

    Ok(())
}
