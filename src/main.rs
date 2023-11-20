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

#[derive(Resource)]
struct GreetTimer(Timer);

// "iterate over every Name component for entities that also have a Person component"
fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last updateif that caused the timer to
    // finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

