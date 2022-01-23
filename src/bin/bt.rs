use anyhow::Result;
use binding_tool::args::Parser;
use binding_tool::{Command, CommandHandler};
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    let matcher = Parser::new();
    let matches = matcher.parse_args(env::args());
    let executed_command = matches.subcommand_name().unwrap_or("help");
    let args = matches.subcommand_matches(executed_command);

    match Command::from_str(executed_command) {
        Ok(Command::Add(mut handler)) => handler.handle(args),
        Ok(Command::Delete(mut handler)) => handler.handle(args),
        Ok(Command::CaCerts(mut handler)) => handler.handle(args),
        Ok(Command::DependencyMapping(mut handler)) => handler.handle(args),
        Ok(Command::Args(mut handler)) => handler.handle(args),
        Err(err) => Err(err),
    }
}
