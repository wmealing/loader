use std::io;
use std::io::Write;

use logger::Logger;
use plugin_manager::PluginManager;
use pushover::Pushover;
use relevant_greeter::RelevantGreeter;
use quitter::Quitter;

fn main() {
    println!("Loader");

    let logger = Logger::new();
    let relevant_greeter = RelevantGreeter::new();
    let pushover = Pushover::new();
    let quitter = Quitter::new();

    let mut pm = PluginManager::new();
    
    pm.add_events_hook(logger);
    pm.add_events_hook(relevant_greeter);
    pm.add_events_hook(pushover);
    pm.add_events_hook(quitter);

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();

        /* Yep input is text for now */
        match io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                /* Run app plugins against input */
                let response = pm.input(input);

                /* Run all plugins against this output */
                pm.output(response);
                println!("END");
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
