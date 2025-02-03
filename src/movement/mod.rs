use bevy::prelude::*;
use avian2d::prelude::*;

/// Plugin initializing the systems required for movement
pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Add the input and force application systems and ensure they are chained in a
        // deterministic order
        app.add_systems(FixedUpdate, (input_to_controller, controller_to_force).chain());
    }
}

/// Force-based char controller
#[derive(Debug, Component)]
#[require(RigidBody(|| RigidBody::Dynamic), LinearVelocity, ExternalImpulse)]
pub struct CharacterController {
    // 2d vector representing the desired direction of movement, its length will be normalized
    // before application
    pub target_direction: Vec2,
    // The linear velocity at which the controller wants to move
    pub target_velocity: f32,
    /// Accel cap in m/s^2
    pub max_accel: f32,
}
impl Default for CharacterController {
    fn default() -> Self {
        Self{
            target_direction: Vec2::ZERO,
            target_velocity: 10.,
            max_accel: 10.,
        }
    }
}

/// Player input marker struct
#[derive(Debug, Component)]
pub struct PlayerMoveInput;

/// Fn reading data from the controller struct and applying it through the physics engine
/// Should run at a fixed timestep
fn controller_to_force(mut query: Query<(&mut ExternalImpulse, &LinearVelocity, &CharacterController, &Transform)>, time: Res<Time<Fixed>>, mut gizmos: Gizmos) {

    let delta_t = time.delta_secs() + time.overstep().as_secs_f32();
    for (mut impulse, linvel, controller, transform) in query.iter_mut() {

        // Extract the direction intent and normalize it so it's always length 1
        let target_dir = controller.target_direction.normalize_or_zero();

        // Calculate the target velocity vector
        let target_vel = target_dir * controller.target_velocity;

        // Calculate the error (delta) between the intended and current velocity
        let velocity_delta = target_vel - linvel.xy();

        // Calculate accel clamped to the max value per second
        let accel = (velocity_delta).clamp_length_max(controller.max_accel);

        // Apply accel multiplied by the correct time delta
        impulse.apply_impulse(accel * delta_t);

        // Line gizmos for accel/velocity visualization
        gizmos.line_2d(transform.translation.xy(), transform.translation.xy() + (linvel.xy()), Color::srgba_u8(0, 127, 0, 127));
        gizmos.line_2d(transform.translation.xy(), transform.translation.xy() + (accel*1.), Color::srgba_u8(0, 0, 127, 127));
    }
}

/// Basic input function naively polling the preset WSAD keys for input and assigning the resulting
/// vector to a character controller component
fn input_to_controller(mut query: Query<&mut CharacterController, With<PlayerMoveInput>>, keys: Res<ButtonInput<KeyCode>>) {

    // Get the character controller(s) from the query
    for mut controller in query.iter_mut() {
        // Init an empty move direction vector
        let mut intent = Vec2::ZERO;
        
        // Check the inputs, extend the vector in desired directions
        // Up
        if keys.pressed(KeyCode::KeyW) {
            intent.y = 1.;
        } 
        // Down
        else if keys.pressed(KeyCode::KeyS) {
            intent.y = -1.;
        } 
        // Left
        if keys.pressed(KeyCode::KeyA) {
            intent.x = -1.;
        } 
        // Right
        else if keys.pressed(KeyCode::KeyD) {
            intent.x = 1.;
        }
        
        // Set the vector as the component's target
        controller.target_direction = intent;
    }
}


