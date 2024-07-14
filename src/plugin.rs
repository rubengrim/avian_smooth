use avian3d::{
    prepare::PrepareSet,
    schedule::{PhysicsSchedule, PhysicsSet, PhysicsStepSet},
};
use bevy::prelude::*;

use crate::GlobalInterpolation;

pub struct AvianInterpolationPlugin;

impl Plugin for AvianInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GlobalInterpolation>();

        app.add_systems(
            Update,
            crate::set_object_interpolation.run_if(resource_exists::<crate::GlobalInterpolation>),
        );

        app.configure_sets(
            PostUpdate,
            (
                crate::InterpolationSet::Interpolate,
                crate::InterpolationSet::PostInterpolation,
            )
                .chain()
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );

        app.add_systems(
            PostUpdate,
            (
                crate::pre_phys_position_reset,
                crate::pre_phys_rotation_reset,
            )
                .before(PrepareSet::PreInit),
        )
        .add_systems(
            PhysicsSchedule,
            (crate::save_last_position, crate::save_last_rotation).in_set(PhysicsStepSet::First),
        )
        .add_systems(
            PhysicsSchedule,
            (crate::save_current_position, crate::save_current_rotation)
                .in_set(PhysicsStepSet::Last),
        )
        .add_systems(
            PostUpdate,
            (crate::interpolate_position, crate::interpolate_rotation)
                .in_set(crate::InterpolationSet::Interpolate),
        );
    }
}
