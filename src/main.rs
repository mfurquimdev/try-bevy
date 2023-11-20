use bevy::prelude::*;

// Let's greet a specific person with a name
#[derive(Component)]
struct Person;

// Other entities might also have name - hence, the separate name component
#[derive(Component)]
struct Name(String);

// Commands will spawn (add) some people
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

// "iterate over every Name component for entities that also have a Person component"
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}


fn hello_world() {
    println!("hello world!");
}


fn main() {
    App::new()
        // Startup system happens only once, before all other systems
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

