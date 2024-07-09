use avian3d::{
    prepare::PrepareSet,
    schedule::{PhysicsSchedule, PhysicsSet, PhysicsStepSet},
    sync::SyncSet,
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub struct InterpolationCopySet;

#[derive(SystemSet, Debug, PartialEq, Eq, Clone, Hash)]
pub struct InterpolationSet;

pub struct AvianInterpolationPlugin;

impl Plugin for AvianInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostUpdate,
            InterpolationSet
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );

        app.add_systems(
            PostUpdate,
            crate::process_interpolation.in_set(InterpolationSet),
        )
        .add_systems(
            PostUpdate,
            crate::pre_physics_prepare.before(PhysicsSet::Prepare),
        )
        .add_systems(
            PhysicsSchedule,
            crate::copy_positions.in_set(PhysicsStepSet::First),
        )
        .add_systems(
            PostUpdate,
            crate::post_physics
                .after(PhysicsSet::Sync)
                .before(InterpolationSet),
        );
    }
}
