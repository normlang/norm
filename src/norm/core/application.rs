use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Command-line interface (CLI) for managing Norm code", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create new project with given name
    Create {
        /// Project name
        #[arg(required = true)]
        name: String,

        #[command(flatten)]
        target: Target,
    },
    /// Create new project in current directory
    Init {
        #[command(flatten)]
        target: Target,
    },
    /// Build project
    Build,
    /// Run application
    Run,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct Target {
    /// Application target
    #[arg(long)]
    app: bool,
    /// Library target
    #[arg(long)]
    lib: bool,
}

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let cli = Cli::parse();

        match cli.command {
            Commands::Create { name, target } => {
                if target.lib {
                    println!("create lib {name}");
                } else {
                    println!("create app {name}");
                }
            }
            Commands::Init { target } => {
                if target.lib {
                    println!("create lib");
                } else {
                    println!("create app");
                }
            }
            Commands::Build => {
                println!("build app");
            }
            Commands::Run => {
                println!("run app");
            }
        }
    }
}
