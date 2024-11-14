use bevy::prelude::*;

pub struct Prompt;

#[derive(Component)]
pub struct Reply;

impl Plugin for Prompt {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_layout);
    }
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle {
                text: Text::from_section(
                    "prompt".to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            });

            builder.spawn((
                TextBundle {
                    text: Text::from_section(
                        "".to_string(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 100.0,
                            ..default()
                        },
                    )
                    .with_justify(JustifyText::Center),
                    ..default()
                },
                Reply,
            ));
        });
}
