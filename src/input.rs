use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};
use rand::Rng;
use std::mem;

pub struct Input;

impl Plugin for Input {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_keyboard_input_events);
        app.add_systems(Update, update_prompt);
        app.add_systems(Update, compare_values);
    }
}

fn listen_keyboard_input_events(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<crate::ui::Reply>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let mut text = edit_text.single_mut();
                if text.sections[0].value.is_empty() {
                    continue;
                }
                let old_value = mem::take(&mut text.sections[0].value);
                println!("{old_value}");
            }

            Key::Space => {
                edit_text.single_mut().sections[0].value.push(' ');
            }
            Key::Backspace => {
                edit_text.single_mut().sections[0].value.pop();
            }
            Key::Character(character) => {
                edit_text.single_mut().sections[0].value.push_str(character);
            }
            _ => continue,
        }
    }
}

fn update_prompt(
    mut events: EventReader<KeyboardInput>,
    mut edit_text: Query<&mut Text, With<crate::ui::Prompt>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let num1 = rand::thread_rng().gen_range(1..=10);
                let num2 = rand::thread_rng().gen_range(1..=10);
                let operator = "x";
                edit_text.single_mut().sections[0].value = format!("{num1} {operator} {num2}");
            }
            _ => continue,
        }
    }
}

fn compare_values(
    mut events: EventReader<KeyboardInput>,
    mut prompt: Query<&mut Text, (With<crate::ui::Prompt>, Without<crate::ui::Reply>)>,
    mut reply: Query<&mut Text, With<crate::ui::Reply>>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let mut prompt_text = prompt.single_mut();
                if prompt_text.sections[0].value.is_empty() {
                    continue;
                }
                let value = mem::take(&mut prompt_text.sections[0].value);
                println!("c=>prompt: {value}");
                let mut numbers = value.split(" x ");
                println!("c=>1st: {:?}", numbers.next());
                println!("c=>2nd: {:?}", numbers.next());

                let mut reply_text = reply.single_mut();
                if reply_text.sections[0].value.is_empty() {
                    continue;
                }
                let value = mem::take(&mut reply_text.sections[0].value);
                println!("c=>reply: {value}");
            }
            _ => continue,
        }
    }
}
