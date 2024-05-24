use bevy::prelude::*;

// #[derive(Component)]
// struct Position { x: f32, y: f32 }

// fn print_position_system(query: Query<&Position>) {
//     for position in &query {
//         println!("position: {} {}", position.x, position.y);
//     }
// }

// struct Entity(u64);

fn hello_world() {
    println!("Hello, world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Person 1".to_string())));
    commands.spawn((Person, Name("Person 2".to_string())));
    commands.spawn((Person, Name("Person 3".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Person 1" {
            name.0 = "Adam Driver".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
