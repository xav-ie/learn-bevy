use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_ready_for_hire)
            .add_system(person_does_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Xavier".to_string(),
            race: Race::Human,
        },
        Employed {
            job: Job::Programmer,
        },
    ));
    commands.spawn(Person {
        name: "Alice".to_string(),
        race: Race::Human,
    });
    commands.spawn((
        Person {
            name: "Bob".to_string(),
            race: Race::Human,
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn(Person {
        name: "JOOB".to_string(),
        race: Race::Cat,
    });
    commands.spawn(Person {
        name: "Gary 🐌".to_string(),
        race: Race::Snail,
    });
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        println!("{} is a {}.", person.name, employed.job);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name,);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name,);
    }
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
    pub race: Race,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

pub enum Race {
    Human,
    Snail,
    Cat,
}

pub enum Job {
    Doctor,
    Firefighter,
    Lawyer,
    Programmer,
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Doctor => write!(f, "Doctor"),
            Job::Firefighter => write!(f, "Firefighter"),
            Job::Lawyer => write!(f, "Lawyer"),
            Job::Programmer => write!(f, "Programmer"),
        }
    }
}
