use ::anyhow::Result;
use ::clap::{Parser, Subcommand};
use ::mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use ::mdbook_anchors_aweigh::AnchorsAweighLinker;

fn main() -> Result<()> {
    init_logging();
    let args = Cli::parse();
    match args.command.unwrap_or_default() {
        Command::Supports { .. } => {
            // Do nothing for now; we support
            // all render systems afaik
        }
        Command::Process => {
            let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;
            let linker = AnchorsAweighLinker;
            let book = linker.run(&ctx, book)?;
            serde_json::to_writer(std::io::stdout(), &book)?;
        }
    }
    Ok(())
}

#[derive(Debug, Parser, Clone)]
#[command(name = "mdbook-anchors-aweigh")]
#[command(about = "⚓ documentation source linker")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand, Clone, Default)]
enum Command {
    /// Preflight check made to determine render support
    #[command(arg_required_else_help = true)]
    Supports {
        /// renderer to check, such as "html"
        renderer: String,
    },
    /// process `{{#aa ...}}` tags
    #[default]
    Process,
}

fn init_logging() {
    use env_logger::{Builder, Env};
    use log::Level;
    use std::io::Write;

    let env = Env::default().filter_or("AA_LOG_LEVEL", "info");

    Builder::from_env(env)
        .format(|buf, record| {
            let header = match record.level() {
                Level::Error => "⚓💥",
                Level::Warn => "⚓💣",
                Level::Info => "⚓✅",
                Level::Debug => "⚓📡---",
                Level::Trace => "⚓🔗------",
            };
            let timestamp = buf.timestamp();
            writeln!(buf, "{timestamp} {header}  {}", record.args())
        })
        .init();
}
