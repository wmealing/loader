use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

/* Specific plugins, i should automate this */
use logger::Logger;
use plugin_manager::PluginManager;
use pushover::Pushover;
use quitter::Quitter;
use relevant_greeter::RelevantGreeter;

fn handle_client(stream: UnixStream) {
    /* I dont want this done in here every time this is ugly af */
    let logger = Logger::new();
    let relevant_greeter = RelevantGreeter::new();
    let pushover = Pushover::new();
    let quitter = Quitter::new();

    let mut pm = PluginManager::new();

    pm.add_events_hook(logger);
    pm.add_events_hook(relevant_greeter);
    pm.add_events_hook(pushover);
    pm.add_events_hook(quitter);

    write!(&stream, "{}", "> ").unwrap();

    let rd_stream = BufReader::new(&stream);
    for line in rd_stream.lines() {
        let response = pm.input(line.unwrap());
        write!(&stream, "S: {}", response).unwrap();
        pm.output(response);
        write!(&stream, "{}", "\n> ").unwrap();
    }
}

fn main() {
    println!("Starting..");

    let listener = UnixListener::bind("/tmp/rust-uds.sock").unwrap();;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
