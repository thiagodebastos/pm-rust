use std::io;

type Project = String;
type Projects = Vec<String>;

fn main() {
    let mut projects: Projects = Vec::new();

    loop {
        add_project(&mut projects);
        select_project(&projects);
    }
}

fn add_project(projects: &mut Projects) {
    let mut project: Project = String::new();

    println!("Enter a project name:");

    io::stdin()
        .read_line(&mut project)
        .expect("Failed to read line.");

    projects.push(project);

    println!("Added new project: {}", projects[projects.len() - 1]);
}

fn select_project(projects: &Projects) {
    println!("Select a project from list:");

    for project in projects {
        println!("{}", project);
    }
}
