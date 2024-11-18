use clap::{Arg, Command};
pub fn build_cli() -> Command{
    Command::new("DevTimeTracker")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Tracks time spent on various projects and applications")
        .subcommand(
            Command::new("add_app")
                .about("Add an application to track")
                .short_flag('d')
                .arg(
                    Arg::new("app_name")
                        .help("The name of the application")
                        .required(true)
                        .value_name("APP_NAME")
                        .short('a')
                )
        )
        .subcommand(
            Command::new("create_project")
                .about("Create a new project workspace")
                .arg(
                    Arg::new("project_name")
                        .help("The name of the project")
                        .required(true)
                        .value_name("PROJECT_NAME")
                        .short('p')
                )
        )
}