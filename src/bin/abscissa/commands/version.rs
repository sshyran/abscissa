//! `version` subcommand

#![allow(clippy::never_loop)]

use abscissa::{Command, Options, Runnable};

/// `version` subcommand
#[derive(Debug, Default, Options)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    /// Print version message
    fn run(&self) {
        super::CliCommand::print_package_info();
    }
}