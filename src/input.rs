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
        app.add_systems(
            Update,
            (compare_values, update_prompt, listen_keyboard_input_events).chain(),
        );
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
    prompt: Query<&mut Text, (With<crate::ui::Prompt>, Without<crate::ui::Reply>)>,
    reply: Query<&mut Text, (With<crate::ui::Reply>, Without<crate::ui::Score>)>,
    mut score: Query<&mut Text, (With<crate::ui::Score>, Without<crate::ui::Prompt>)>,
) {
    for event in events.read() {
        if event.state == ButtonState::Released {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                let mut current_score = score.single_mut();
                if current_score.sections[0].value.is_empty() {
                    current_score.sections[0].value = "0".to_string();
                    continue;
                }

                let prompt_text = prompt.single();
                if prompt_text.sections[0].value.is_empty() {
                    continue;
                }
                let value = &prompt_text.sections[0].value;
                println!("c=>prompt: {value}");
                let mut numbers = value.split(" x ");

                // TODO: handle possible errors
                let num1 = numbers.next().unwrap().parse::<i32>().unwrap();
                let num2 = numbers.next().unwrap().parse::<i32>().unwrap();
                let correct = num1 * num2;
                println!("c=>correct: {}", correct);

                let reply_text = reply.single();
                if reply_text.sections[0].value.is_empty() {
                    continue;
                }
                let value = &reply_text.sections[0].value;

                // TODO: handle possible errors
                let attempt = value.parse::<i32>().unwrap();
                println!("c=>attempt: {attempt}");

                if attempt == correct {
                    // TODO: handle possible errors
                    let new_score = current_score.sections[0].value.parse::<u32>().unwrap() + 1;
                    current_score.sections[0].value = new_score.to_string();
                } else {
                    println!("lost");
                }
            }
            _ => continue,
        }
    }
}
