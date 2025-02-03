use avian2d::{prelude::{Collider, Gravity}, PhysicsPlugins};
use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

// Importing the plugin
mod movement;

// Main function
fn main() {

    // [ Engine and library initialization ]

    // Create the app instance
    App::new()
        // Add plugins
        .add_plugins(
            // Add the DefaultPlugins plugin bundle
            DefaultPlugins
                // use .set() to configure the WindowPlugin's settings on init
                .set(WindowPlugin {
                    // Edit the primary window's settings
                    primary_window: Some(Window {
                        // Title (not used in wasm outside of generated bindings, can be used for
                        // customized html rendering settings)
                        title: "topdown".to_string(),
                        // Start windowed on desktop
                        mode: bevy::window::WindowMode::Windowed,
                        // Other fields stay default
                        ..default()
                    }),
                    // The rest of the DefaultPlugins stay default
                    ..default()
                })
        )
        // [ Engine/plugin global settings are controlled by overwriting default resource values ]
        // Set the background color
        .insert_resource(ClearColor(Color::BLACK))

        // Attach the physics plugin (Avian2d library)
        .add_plugins(PhysicsPlugins::default())
        // Set the gravity to zero - the default gravity pulls everything downward
        .insert_resource(Gravity::ZERO)

        // [ Custom code initialization ]

        // Add the MovementPlugin from src/movement/mod.rs
        .add_plugins(movement::MovementPlugin)

        // Add the cursor grab system (for desktop testing)
        .add_systems(Update, grab_cursor_on_enter)

        // Add the setup function to initialize the starting state of the world
        .add_systems(Startup, setup)

        // Init is done, call .run() on the app to hand scheduling control over to the engine
        // The init function redirects to the main engine loop until the program quits - the main function never ends
        .run();
}

// A simple cursor grab function to prevent the mouse from leaving the bounds of the play area on
// desktop
fn grab_cursor_on_enter(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<CursorEntered>
) {
    for _ in events.read() {
        for mut window in windows.iter_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
    }
}

// Function spawning the game view camera and the player
fn setup(mut commands: Commands) {
    // Minimal player example spawner, spawn() takes in components to attach
    commands.spawn((
        // The CharacterController component from src/movement/mod.rs with a default value
        movement::CharacterController::default(),
        // Marker components don't have fields so their init doesn't take arguments
        movement::PlayerMoveInput,
        // The player sprite is a simple white square 1x1m in size
        Sprite::from_color(Color::WHITE, (1., 1.).into()),
        // The collider is used to calculate the mass at the physics engine's default density.
        // Its size is the same as the sprite
        Collider::rectangle(1., 1.)

    ));
    // Camera spawner
    commands.spawn((
        // The engine's 2d camera marker component 
        Camera2d, 
        // The projection is used by the camera to map worldspace positions to screenspace positions
        Projection::Orthographic(
            // An orthographic projection is the usual choice for 2d rendering
            OrthographicProjection{
                // In an orthographic projection scaling mode controls the area visible on screen.
                // It's set to 32m vertically and the horizontal range will be computed
                // automatically based on the aspect ratio
                scaling_mode: bevy::render::camera::ScalingMode::FixedVertical { viewport_height: 32. },
                // The rest of the OrthographicProjection struct fields are set to the default for 2d
                ..OrthographicProjection::default_2d()
            },
        ),
    ));
}
