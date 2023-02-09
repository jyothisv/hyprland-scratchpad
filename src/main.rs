use std::env;
use std::process::exit;

use hyprland::dispatch::*;
use hyprland::event_listener::EventListener;

use hyprland::prelude::*;

use hyprland::data::Workspaces;

fn main() -> hyprland::shared::HResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Syntax: {} workspace-name command-name", args[0]);
        exit(1);
    }

    let workspace_name = format!("{}", &args[1]);
    let full_workspace_name = format!("special:{workspace_name}");
    let command_name = &args[2];

    // All open windows as a vector
    let workspaces = Workspaces::get()?.to_vec();

    // println!("workspaces = {workspaces:#?}");

    // Check if there is a window having the class name `target_class'
    let has_workspace = workspaces
        .iter()
        .any(|workspace| workspace.name == full_workspace_name);

    if has_workspace {
        // We can't do anything with hyprland version 0.3.0. Exit!
        // TODO: Toggle the special workspace
        exit(0);
    }

    hyprland::dispatch!(
        Exec,
        &format!(
            "[workspace {full_workspace_name} silent] foot -T {workspace_name} -a {workspace_name} {command_name}"
        )
    )?;

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_added_handler(move |data| {
        if let hyprland::shared::WorkspaceType::Special(Some(x)) = data {
            if x == workspace_name {
                // TODO: toggle the special workspace
                exit(0);
            }
        }
    });

    event_listener.start_listener()
}
