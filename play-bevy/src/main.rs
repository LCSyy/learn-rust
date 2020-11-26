use bevy::prelude::*;

fn main() {
    App::build().run();
}

// entity
struct Person;

// components
struct Name(String);

// system
fn add_people(mut commands: Commands) {
    commands
    .spawn((Person, Name(String::from("Proctor"))))
    .spawn((Person, Name(String::from("There"))));
}

fn hello_world() {
    println!("Hello World!");
}

fn greet_people(_p: &Person, name: &Name) {
    println!("helllo {}", name.0);
}
