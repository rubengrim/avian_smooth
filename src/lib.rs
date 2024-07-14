use avian3d::prelude::*;
use bevy::prelude::*;

mod plugin;
pub use plugin::*;

#[derive(Component, Default)]
pub struct PositionInterpolation {
    pub disabled: bool,
    pub last_position: Option<Vec3>,
    pub current_position: Option<Vec3>,
}

#[derive(Component, Default)]
pub struct RotationInterpolation {
    pub disabled: bool,
    pub last_rotation: Option<Quat>,
    pub current_rotation: Option<Quat>,
}

// Resets `Transform`s and `Position`s to their "real" non-interpolated values right before physics is ran.
fn pre_phys_position_reset(
    mut objects: Query<(&mut Position, &mut Transform, &PositionInterpolation)>,
) {
    for (mut position, mut transform, interp) in objects.iter_mut() {
        if let Some(current_position) = interp.current_position {
            transform.translation = current_position;
            position.0 = current_position;
        };
    }
}

// Saves the current position before the physics update.
fn save_last_position(mut objects: Query<(&Position, &mut PositionInterpolation)>) {
    for (position, mut interp) in objects.iter_mut() {
        interp.last_position = Some(position.0);
    }
}

// Saves the current position after the physics update.
fn save_current_position(mut objects: Query<(&Position, &mut PositionInterpolation)>) {
    for (position, mut interp) in objects.iter_mut() {
        interp.current_position = Some(position.0.clone());
    }
}

// Interpolates between the previous and current position of rigidbodies.
fn interpolate_position(
    mut objects: Query<(&PositionInterpolation, &mut Transform)>,
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

    for (interp, mut transform) in objects.iter_mut() {
        if interp.disabled {
            continue;
        }
        // Continue without interpolating if there is no last_position or current_position set
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
        transform.translation = last_position + lerp_factor * (current_position - last_position)
    }
}

// ---------------- //
// BEGIN: ROTATION //
// ---------------- //

// Resets `Transform`s and `Rotations`s to their "real" non-interpolated values right before physics is ran.
fn pre_phys_rotation_reset(
    mut objects: Query<(&mut Rotation, &mut Transform, &RotationInterpolation)>,
) {
    for (mut rotation, mut transform, interp) in objects.iter_mut() {
        if let Some(current_rotation) = interp.current_rotation {
            transform.rotation = current_rotation;
            rotation.0 = current_rotation;
        };
    }
}

// Saves the current rotation before the physics update.
fn save_last_rotation(mut objects: Query<(&Rotation, &mut RotationInterpolation)>) {
    for (rotation, mut interp) in objects.iter_mut() {
        interp.last_rotation = Some(rotation.0);
    }
}

// Saves the current rotation after the physics update.
fn save_current_rotation(mut objects: Query<(&Rotation, &mut RotationInterpolation)>) {
    for (rotation, mut interp) in objects.iter_mut() {
        interp.current_rotation = Some(rotation.0.clone());
    }
}

// Interpolates between the previous and current rotation of rigidbodies.
fn interpolate_rotation(
    mut objects: Query<(&RotationInterpolation, &mut Transform)>,
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

    for (interp, mut transform) in objects.iter_mut() {
        if interp.disabled {
            continue;
        }
        // Continue without interpolating if there is no last_rotation current_rotation set
        let last_rotation = match interp.last_rotation {
            Some(v) => v,
            None => continue,
        };
        let current_rotation = match interp.current_rotation {
            Some(v) => v,
            None => continue,
        };

        // Interpolate between the previous and current rotation of the physics object.
        let lerp_factor = overstep / delta;
        transform.rotation = last_rotation.slerp(current_rotation, lerp_factor);
    }
}
