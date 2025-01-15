use bevy::prelude::*;
use avian2d::prelude::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (input_to_controller, controller_to_force).chain());
    }
}

/// Force-based char controller
#[derive(Debug, Component)]
#[require(RigidBody(|| RigidBody::Dynamic), LinearVelocity, ExternalImpulse)]
pub struct CharacterController {
    pub target_direction: Vec2,
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

/// Fn reading data from the controller struct and applying it to the physics engine
/// Should run at a fixed timestep
fn controller_to_force(mut query: Query<(&mut ExternalImpulse, &LinearVelocity, &CharacterController, &Transform)>, time: Res<Time<Fixed>>, mut gizmos: Gizmos) {
    let delta_t = time.delta_secs() + time.overstep().as_secs_f32();
    for (mut impulse, linvel, controller, transform) in query.iter_mut() {
        // extract the direction intent
        let target_dir = controller.target_direction.normalize_or_zero();
        // calculate the target velocity vector
        let target_vel = target_dir * controller.target_velocity;
        // calculate the error between the intended and current velocity
        let velocity_delta = target_vel - linvel.xy();
    
        // calculate accel clamped to the max value per second
        let accel = (velocity_delta).clamp_length_max(controller.max_accel);
        
        // apply accel multiplied by the correct time delta
        impulse.apply_impulse(accel * delta_t);

        // gizmos for accel/velocity visualization
        gizmos.line_2d(transform.translation.xy(), transform.translation.xy() + (linvel.xy()), Color::srgba_u8(0, 127, 0, 127));
        gizmos.line_2d(transform.translation.xy(), transform.translation.xy() + (accel*1.), Color::srgba_u8(0, 0, 127, 127));
    }
}

/// This input fn is extremely barebones and not optimized at all, it runs at the same fixed timestep so it's not terrible
fn input_to_controller(mut query: Query<&mut CharacterController, With<PlayerMoveInput>>, keys: Res<ButtonInput<KeyCode>>) {
    for mut controller in query.iter_mut() {
        let mut intent = Vec2::ZERO;
        
        
        if keys.pressed(KeyCode::KeyW) {
            intent.y = 1.;
        } 
        else if keys.pressed(KeyCode::KeyS) {
            intent.y = -1.;
        } 
        if keys.pressed(KeyCode::KeyA) {
            intent.x = -1.;
        } 
        else if keys.pressed(KeyCode::KeyD) {
            intent.x = 1.;
        }
        
        controller.target_direction = intent;
    }
}


