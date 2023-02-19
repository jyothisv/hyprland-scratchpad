use std::env;
use std::process::exit;

use hyprland::dispatch::*;
use hyprland::event_listener::EventListener;

use hyprland::prelude::*;

use hyprland::data::Workspaces;

fn toggle_workspace_or_exit(ws: &str) {
    let res = hyprland::dispatch!(ToggleSpecialWorkspace, Some(ws.to_string()));
    match res {
        Ok(_) => exit(0),
        Err(_) => exit(1),
    }
}

fn main() -> hyprland::shared::HResult<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Syntax: {} workspace-name command-name [command-args ...]",
            args[0]
        );
        exit(1);
    }

    let full_command_name = &args[2..].join(" ");

    let workspace_name = format!("{}", &args[1]);
    let full_workspace_name = format!("special:{workspace_name}");

    // All open windows as a vector
    let workspaces = Workspaces::get()?.to_vec();

    // println!("workspaces = {workspaces:#?}");

    // Check if there is a window having the class name `target_class'
    let has_workspace = workspaces
        .iter()
        .any(|workspace| workspace.name == full_workspace_name);

    if has_workspace {
        // Toggle workspace
        toggle_workspace_or_exit(&workspace_name);
    }

    hyprland::dispatch!(
        Exec,
        &format!("[workspace {full_workspace_name} silent] {full_command_name}")
    )?;

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_added_handler(move |data| {
        if let hyprland::shared::WorkspaceType::Special(Some(ws)) = data {
            if ws == workspace_name {
                // Toggle workspace
                toggle_workspace_or_exit(&workspace_name);
            }
        }
    });

    event_listener.start_listener()
}
