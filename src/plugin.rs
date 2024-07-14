use avian3d::{
    prepare::PrepareSet,
    schedule::{PhysicsSchedule, PhysicsSet, PhysicsStepSet},
};
use bevy::prelude::*;

/// System set running in `PostUpdate` between `PhysicsSet::Sync` and `TransformSystem::TransformPropagate`
#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub enum InterpolationSet {
    /// Where the interpolation takes place.
    Interpolate,
    /// Can be used to safely schedule systems after interpolation but before transforms are propagated by bevy.
    /// One use case could be to update the position of a camera that follows a physics object here, so the camera doesn't lag behind one frame.
    /// Empty by default
    PostInterpolation,
}

pub struct AvianInterpolationPlugin;

impl Plugin for AvianInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                InterpolationSet::Interpolate,
                InterpolationSet::PostInterpolation,
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
                .in_set(InterpolationSet::Interpolate),
        );
    }
}
