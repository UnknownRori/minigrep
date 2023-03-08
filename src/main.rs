use minigrep::Application;

fn main() {
    println!("Hello, world!");

    let app = Application::new();
    if let Err(e) = app {
        match e {
            minigrep::ErrorKind::QueryEmpty => eprintln!("Query is empty"),
            minigrep::ErrorKind::FilenameEmpty => eprintln!("Filename is empty"),
            minigrep::ErrorKind::ParseArgs => eprintln!("Failed to parse Args"),
            minigrep::ErrorKind::OpenFile(e) => eprintln!("Failed top open a file: {}", e),
            minigrep::ErrorKind::NotEnoughArgs(len) => {
                eprintln!("Not enough Args expected 2 given {}", len)
            }
        }
    } else if let Ok(app) = app {
        println!("{:#?}", app);
    }
}
