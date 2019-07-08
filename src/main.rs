use std::io;

use logger::Logger;
use plugin_manager::PluginManager;
use relevant_greeter::RelevantGreeter;

fn main() {
    println!("Loader");

    let logger = Logger::new();
    let relevant_greeter = RelevantGreeter::new();

    let mut pm = PluginManager::new();
    pm.add_events_hook(logger);
    pm.add_events_hook(relevant_greeter);

    loop {
        let mut input = String::new();
        /* Yep input is text for now */
        match io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                pm.dispatch(input);
                // let response = pm.on_input(input);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
