#[macro_use]
extern crate clap;

mod app;
mod error;
mod project_config;
mod tmux;

use app::{actions, cli};
use error::AppError;

fn main() -> Result<(), AppError> {
    let matches = cli::get_cli_command_parser().get_matches();

    if let Some(project_name) = matches.get_one::<String>("project") {
        return actions::run_project(project_name);
    }

    match matches.subcommand() {
        Some(("list", _)) => actions::list_projects(),
        Some(("doctor", _)) => actions::check_config(),
        Some(("debug", debug_matches)) => {
            actions::debug_project(debug_matches.get_one::<String>("project").unwrap())
        }
        Some(("run", run_matches)) => {
            actions::run_project(run_matches.get_one::<String>("project").unwrap())
        }
        Some(("edit", edit_matches)) => {
            actions::edit_project(edit_matches.get_one::<String>("project").unwrap())
        }
        Some(("delete", delete_matches)) => {
            actions::delete_project(delete_matches.get_one::<String>("project").unwrap())
        }
        Some(("new", new_matches)) => actions::new_project(
            new_matches.get_one::<String>("project").unwrap(),
            *new_matches.get_one::<bool>("blank").unwrap(),
        ),
        _ => unreachable!(),
    }
}
