use bevy::app::{App, Update};

fn main() {
  App::new().add_systems(Update, greet).run();
}

fn greet() {
  println!("Hello, Bevy!");
}
