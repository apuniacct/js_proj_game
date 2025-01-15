use avian2d::{prelude::{Collider, Gravity}, PhysicsPlugins};
use bevy::{core_pipeline::{bloom::Bloom, post_process::ChromaticAberration, tonemapping::Tonemapping}, prelude::*, window::{CursorGrabMode, PrimaryWindow}};

mod movement;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "topdown".to_string(),
                    transparent: true,
                    mode: bevy::window::WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
        )
        .insert_resource(ClearColor(Color::NONE))
        //.insert_resource(AmbientLight::NONE)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        
        .add_plugins(movement::MovementPlugin)

        .add_systems(Update, grab_cursor_on_enter)

        .add_systems(Startup, setup)

        .run();
}

fn grab_cursor_on_enter(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<CursorEntered>
) {
    for _ in events.read() {
        for mut window in windows.iter_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
            movement::CharacterController {
                target_direction: (1., 0.).into(),
                ..default()
            },
            movement::PlayerMoveInput,
            Sprite::from_color(Color::WHITE, (1., 1.).into()),
            Collider::rectangle(1., 1.)

        )
    );
    commands.spawn((
            Camera2d, 
            Projection::Orthographic(
                OrthographicProjection{
                    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 32. },
                    ..OrthographicProjection::default_2d()
                },
            ),
            //Camera{hdr: true, ..default()},
            //Tonemapping::TonyMcMapface,
            //Bloom {intensity: 0.1, ..default()},
            //ChromaticAberration {intensity: 0.7, ..default()}
        ));
}
