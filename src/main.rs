use uuid::Uuid;
use clap::{Parser, Subcommand};

#[derive( Parser)] 
#[clap(name = "uuid")]
#[clap(about = "A uuid command line tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(clap::ValueEnum, Clone)]
enum Namespace {
   DNS,
   OID,
   URL,
   X500,
}

#[derive(Subcommand)]
enum Commands {
    V4,
    #[clap(arg_required_else_help = true)]
    V3 {
        #[clap(value_enum)]
        namespace: Namespace,
        #[clap(short, long, value_parser)]
        data: String
    },
    #[clap(arg_required_else_help = true)]
    V5 {
        #[clap(value_enum)]
        namespace: Namespace,
        #[clap(short, long, value_parser)]
        data: String
    },
}

fn main() {
    let e = Cli::parse();

    match e.command {
        Commands::V4 => {
            println!("{}", Uuid::new_v4())
        },
        Commands::V3 { namespace, data } => {
            let b = match namespace {
                Namespace::DNS => &Uuid::NAMESPACE_DNS,
                Namespace::OID => &Uuid::NAMESPACE_OID,
                Namespace::URL => &Uuid::NAMESPACE_URL,
                Namespace::X500 => &Uuid::NAMESPACE_X500,
            };
            println!("{}", Uuid::new_v3(b, data.as_bytes()));
        },
        Commands::V5 { namespace, data } => {
            let b = match namespace {
                Namespace::DNS => &Uuid::NAMESPACE_DNS,
                Namespace::OID => &Uuid::NAMESPACE_OID,
                Namespace::URL => &Uuid::NAMESPACE_URL,
                Namespace::X500 => &Uuid::NAMESPACE_X500,
            };
            println!("{}", Uuid::new_v5(b, data.as_bytes()));
        },
    }
}
