use clap::{AppSettings, ArgEnum, Args, Parser, Subcommand};

mod api_client;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to <username>
    Login { username: String },

    /// Create a new user with <username>
    Signup { username: String },

    /// List all users
    Users,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    match &cli.command {
        Some(Commands::Login { username }) => handle_login(username),
        Some(Commands::Signup { username }) => handle_signup(username),
        Some(Commands::Users) => handle_users(),
        None => {}
    }
}

fn handle_login(username: &String) {
    println!("'login' was used, username is: {:?}", username)
}

fn handle_signup(username: &String) {
    println!("'signup' was used, username is: {:?}", username)
}

fn handle_users() {
    println!("Fetching all users...");
}
