use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

/* Specific plugins, i should automate this */
use plugin_manager::PluginManager;

fn handle_client(stream: UnixStream) {
    let mut pm = PluginManager::new();

    write!(&stream, "{}", "|> ").unwrap();

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

    let listener = UnixListener::bind("/tmp/rust-uds.sock").unwrap();

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
