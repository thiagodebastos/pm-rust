use std::{env, io};

struct Project {
    name: String,
    path: String,
}

impl Project {
    fn new(name: &str, path: &str) -> Project {
        Project {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

type Projects = Vec<Project>;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut projects: Projects = Vec::new();
    for arg in args {
        match arg.as_str() {
            "add" => {
                add_project(&mut projects);
            }
            "copy" => {
                // copy_project(project);
            }
            "edit" => {
                // edit_project(project);
            }
            "list" => {
                list_all_projects(&projects);
            }
            "remove" => {
                // remove_project(project, &mut projects);
            }
            "rename" => {
                // rename_project(project);
            }
            "start" => {
                // start_project(title, path);
            }
            "stop" => {
                // stop_active_project(project);
            }
            "--version" | "-v" => display_version_info(),
            "--help" | "-h" | _ => {
                // display_help();
            }
        }
    }

    // add_project(&mut projects);
}

fn add_project(projects: &mut Projects) {
    let mut project_name = String::new();
    let mut project_path = String::new();

    println!("Enter a project name:");
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line.");

    println!("Enter a project path:");
    io::stdin()
        .read_line(&mut project_path)
        .expect("Failed to read line.");

    let new_project = Project::new(&project_name, &project_path);

    projects.push(new_project);

    println!("Added new project: {}", project_name);
}

// TODO add gettable desciption to each function

fn list_active_projects(projects: &Projects) -> &Projects {
    todo!();
}

fn copy_project(project: &Project) -> Project {
    todo!();
}

fn edit_project(project: &mut Project) -> Project {
    todo!();
}

fn list_all_projects(projects: &Projects) -> Projects {
    todo!();
}

fn remove_project(project: Project, projects: &mut Projects) -> Projects {
    todo!();
}

fn rename_project(project: &mut Project) -> Project {
    todo!();
}

fn start_project(title: String, path: String) -> Project {
    todo!();
}

fn stop_active_project(project: Project) -> Project {
    todo!();
}

fn display_help() -> Vec<String> {
    // display each function's description
    todo!();
}

fn display_version_info() {
    let version = env!("CARGO_PKG_VERSION");
    println!("pm version {}", version);
    println!("Written by Thiago de Bastos, 2022");
}
