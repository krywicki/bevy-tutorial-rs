#![allow(unused)] // silence unused warnings


use bevy::prelude::*;


//== Entities


//== Components
struct Name(String);
struct Person;

//== Resources

struct GreetTimer(Timer);

//== Systems
fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Erin".into()));
    commands.spawn().insert(Person).insert(Name("Joe".into()));
    commands.spawn().insert(Person).insert(Name("John".into()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

//== Plugins

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_people.system())
        .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run();
}
