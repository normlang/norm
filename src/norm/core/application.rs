use clap::{Args, Parser, Subcommand};
use normlib::file::normspace;
use normlib::file::project;
use normlib::file::workspace;

#[derive(Parser, Debug)]
#[command(about = "Command-line interface (CLI) for managing Norm code", version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage normspaces
    Normspace {
        #[command(subcommand)]
        command: Normspace,
    },

    /// Manage workspaces
    Workspace {
        #[command(subcommand)]
        command: Workspace,
    },

    /// Manage projects
    Project {
        #[command(subcommand)]
        command: Project,
    },
}

#[derive(Subcommand, Debug)]
enum Normspace {
    /// Create new normspace with given name
    Add {
        /// Normspace name
        #[arg(required = true)]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
enum Workspace {
    /// Create new workspace with given name
    Add {
        /// Workspace name
        #[arg(required = true)]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
enum Project {
    /// Create new project with given name
    Add {
        /// Project name
        #[arg(required = true)]
        name: String,

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

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Normspace { command } => match command {
                Normspace::Add { name } => {
                    normspace::create(&name)?;
                }
            },

            Commands::Workspace { command } => match command {
                Workspace::Add { name } => {
                    workspace::create(&name)?;
                }
            },

            Commands::Project { command } => match command {
                Project::Add { name, target } => {
                    if target.lib {
                        project::create(&name, project::Target::Library)?;
                    } else {
                        project::create(&name, project::Target::Application)?;
                    }
                }

                Project::Build => {
                    project::build()?;
                }

                Project::Run => {
                    project::run()?;
                }
            },
        }

        Ok(())
    }
}
