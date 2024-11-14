mod input;
mod ui;

use bevy::prelude::*;
use input::Input;
use ui::Prompt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Prompt)
        .add_plugins(Input)
        .run();
}
