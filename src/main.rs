mod ui;

use bevy::prelude::*;
use ui::Prompt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Prompt)
        .run();
}
