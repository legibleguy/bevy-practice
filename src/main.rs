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

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Person 1".to_string())));
    commands.spawn((Person, Name("Person 2".to_string())));
    commands.spawn((Person, Name("Person 3".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
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

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
