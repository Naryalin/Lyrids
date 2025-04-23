mod commands;
mod utils;
mod ignore;
use commands::init::init_repo;
use commands::commit::commit_repo;
use commands::status::status_repo;
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
        .subcommand(
            Command::new("commit")
                .about("Commit changes to the repository")
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
        status_repo();
        // Aquí iría la lógica del comando `status`
    }
    // Si el usuario invoca el subcomando "commit"
    if let Some(_) = matches.subcommand_matches("commit") {
        println!("Committing changes...");
        commit_repo();
        // Aquí iría la lógica del comando `status`
    }
}