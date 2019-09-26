use simplicate::hours::HourType;
use simplicate::projects::{Project, Service};
use dialoguer::{theme::ColorfulTheme, Input};
use prettytable::Table;
use std::env;
use simplicate::SimplicateClient as Client;
use colored::*;
use crate::links::Link;

pub fn client_from_env() -> Client {
    Client {
        api_key: env::var("SIMPL_API_KEY")
            .expect("No API key set")
            .to_string(),
        api_secret: env::var("SIMPL_API_SECRET")
            .expect("No API secret set")
            .to_string(),
        host: env::var("SIMPL_HOST").expect("No host set").to_string(),
    }
}

pub trait Promptable {
    fn print_table(&self);
    fn index_prompt(&self) -> usize {
        self.print_table();
        let index: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Select index")
            .interact()
            .unwrap();
        index
    }
}

pub type Links = Vec<Link>;
pub type Hourtypes = Vec<HourType>;
pub type Projects = Vec<Project>;
pub type Services = Vec<Service>;

impl Promptable for Links {
    fn print_table(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Alias",
            "Description",
            "Project ID",
            "Service ID",
            "HourType ID"
        ]);
        for link in self {
            table.add_row(row![
                link.alias.to_string().bold().green(),
                link.description.to_string(),
                link.project_id.to_string().italic().blue(),
                link.service_id.to_string().italic().yellow(),
                link.hours_id.to_string().italic().magenta()
            ]);
        }
        table.printstd();
    }
}

impl Promptable for Projects {
    fn print_table(&self) {
        let mut table = Table::new();
        table.add_row(row!["Index", "Name", "ID", "Start Date", "End Date"]);
        for (index, project) in self.iter().enumerate() {
            let start_date = match &project.start_date {
                Some(x) => x.cyan().italic().to_string(),
                None => "".to_string(),
            };
            let end_date = match &project.end_date {
                Some(x) => x.red().italic().to_string(),
                None => "".to_string(),
            };
            table.add_row(row![
                format!("{}", index.to_string().yellow().bold()),
                format!("{}", project.name.blue()),
                format!("{}", project.id.magenta().italic()),
                start_date,
                end_date
            ]);
        }
        table.printstd();
    }
}

impl Promptable for Services {
    fn print_table(&self) {
        let mut table = Table::new();
        table.add_row(row!["Index", "Name", "ID"]);
        for (index, service) in self.iter().enumerate() {
            let name = match &service.name {
                Some(x) => x.blue(),
                None => "Name Unknown".to_string().red(),
            };
            table.add_row(row![
                format!("{}", index.to_string().yellow().bold()),
                format!("{}", name),
                format!("{}", service.id.magenta().italic())
            ]);
        }
        table.printstd();
    }
}

impl Promptable for Hourtypes {
    fn print_table(&self) {
        let mut table = Table::new();
        table.add_row(row!["Index", "Name", "ID"]);
        for (index, hourtype) in self.iter().enumerate() {
            table.add_row(row![
                format!("{}", index.to_string().yellow().bold()),
                format!("{}", hourtype.label.blue()),
                format!("{}", hourtype.id.magenta().italic())
            ]);
        }
        table.printstd();
    }
}