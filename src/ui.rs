use bevy::prelude::*;

pub struct Prompt;

impl Plugin for Prompt {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_prompt);
    }
}

fn show_prompt() {
    println!("hello");
}
