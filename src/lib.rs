use avian3d::prelude::*;
use bevy::prelude::*;

mod plugin;
pub use plugin::*;

#[derive(Component, Default)]
pub struct PositionInterpolation {
    pub last_position: Option<Vec3>,
    pub current_position: Option<Vec3>,
}

#[derive(Component)]
pub struct RotationInterpolation;

pub fn pre_physics_prepare(
    mut objects: Query<(&mut Position, &mut Transform, &mut PositionInterpolation)>,
) {
    println!("Pre physics");
    for (mut position, mut transform, mut interp) in objects.iter_mut() {
        if let Some(v) = interp.current_position {
            transform.translation = v;
            position.0 = v;
        };
    }
}

pub fn copy_positions(mut objects: Query<(&Position, &mut PositionInterpolation)>) {
    println!("Copy physics");
    for (position, mut interp) in objects.iter_mut() {
        interp.last_position = Some(position.0);
    }
}

pub fn post_physics(mut objects: Query<(&Position, &mut PositionInterpolation)>) {
    println!("Post physics");
    for (position, mut interp) in objects.iter_mut() {
        interp.current_position = Some(position.0.clone());
    }
}

pub fn process_interpolation(
    mut objects: Query<(&PositionInterpolation, &Position, &mut Transform)>,
    phys_time: Res<Time<Physics>>,
) {
    // Get the physics schedule time-step and delta
    let (delta, overstep) = match phys_time.timestep_mode() {
        TimestepMode::Fixed {
            delta, overstep, ..
        } => (delta.as_secs_f32(), overstep.as_secs_f32()),
        _ => {
            warn!("Non-fixed physics time step modes are currently not supported by avian_smooth");
            return;
        }
    };

    for (interp, position, mut transform) in objects.iter_mut() {
        // Continue without interpolating if there is no last_position set
        let last_position = match interp.last_position {
            Some(v) => v,
            None => continue,
        };

        let current_position = match interp.current_position {
            Some(v) => v,
            None => continue,
        };

        // Interpolate between the previous and current position of the physics object.
        let lerp_factor = overstep / delta;
        println!("{}", lerp_factor);
        transform.translation =
            (1.0 - lerp_factor) * last_position + lerp_factor * current_position;
    }
}
