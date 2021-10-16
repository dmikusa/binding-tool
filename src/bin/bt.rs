use anyhow::Result;
use binding_tool::{Command, CommandHandler, Parser};
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    let matcher = Parser::new();
    let matches = matcher.parse_args(env::args());
    let executed_command = matches.subcommand_name().unwrap_or("help");
    let args = matches.subcommand_matches(executed_command);

    match Command::from_str(executed_command) {
        Ok(Command::Add(handler)) => handler.handle(args),
        Ok(Command::Delete(handler)) => handler.handle(args),
        Ok(Command::CaCerts(handler)) => handler.handle(args),
        Ok(Command::DependencyMapping(handler)) => handler.handle(args),
        Ok(Command::Args(handler)) => handler.handle(args),
        Err(err) => Err(err),
    }
}
