mod input;
mod ui;

use bevy::prelude::*;
use input::Input;
use ui::Layout;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Layout)
        .add_plugins(Input)
        .run();
}
