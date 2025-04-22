mod commands;
use commands::init::init_repo; // Import function from submodule "init"

use clap::Command;
fn main() {
    let matches = Command::new("Lyrids")
        .version("0.1")
        .author("Naryalin")
        .about("Experimental version control system")
        
        // Definir subcomandos
        .subcommand(
            Command::new("init")
                .about("Initialize a new Lyrids repository")
        )
        .subcommand(
            Command::new("status")
                .about("Show the current status of the repository")
        )
        .get_matches();

    // Si el usuario invoca el subcomando "init"
    if let Some(_) = matches.subcommand_matches("init") {
        println!("Initializing Lyrids repository...");
        init_repo();
        // Aquí iría la lógica del comando `init`
    }

    // Si el usuario invoca el subcomando "status"
    if let Some(_) = matches.subcommand_matches("status") {
        println!("Repository status...");
        // Aquí iría la lógica del comando `status`
    }
}