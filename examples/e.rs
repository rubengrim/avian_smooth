use avian3d::prelude::*;
use avian_smooth::*;
use bevy::prelude::*;

const PHYSICS_UPDATE_FREQ: f64 = 60.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            AvianInterpolationPlugin,
        ))
        .insert_resource(Time::new_with(Physics::fixed_hz(PHYSICS_UPDATE_FREQ)))
        .insert_resource(IsInterpolating(true))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_box, toggle_interpolation, update_ui))
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    // The rigidbody
    commands.spawn((
        RigidBody::Kinematic,
        Position::default(),
        Rotation::default(),
        PbrBundle {
            mesh: mesh_assets.add(Cuboid::from_size(Vec3::splat(1.))),
            material: material_assets.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.8, 0.6),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        // PositionInterpolation::default(),
        // RotationInterpolation::default(),
    ));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // UI
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 15.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn update_box(
    mut box_q: Query<(&mut AngularVelocity, &mut LinearVelocity)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let speed = 4.;
    for (mut angular_velocity, mut linear_velocity) in box_q.iter_mut() {
        let mut velocity = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += speed;
        }

        linear_velocity.0 = velocity;

        angular_velocity.0 = Vec3::from_array([1.0, 1.5, 2.]) * 0.4;
    }
}

// IsInterpolating has no effect on actual interpolation and is just for ui.
// Toggle interpolation with InterpolatedPosition::pass_raw and InterpolatedRotation::pass_raw instead.
#[derive(Resource)]
struct IsInterpolating(bool);

fn toggle_interpolation(
    mut interp_q: Query<(&mut PositionInterpolation, &mut RotationInterpolation)>,
    mut is_interpolating: ResMut<IsInterpolating>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut pos, mut rot) = interp_q.single_mut();
        pos.disabled = !pos.disabled;
        rot.disabled = !rot.disabled;

        is_interpolating.0 = !pos.disabled;
    }
}

fn update_ui(mut text: Query<&mut Text>, is_interpolating: Res<IsInterpolating>) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    text.clear();

    text.push_str("Move box with <WASD>");
    text.push_str("\nToggle interpolation with <Space>");

    if is_interpolating.0 {
        text.push_str("\n\nInterpolation: on");
    } else {
        text.push_str("\n\nInterpolation: off");
    }

    text.push_str(&format!(
        "\nPhysics update frequency: {}hz",
        PHYSICS_UPDATE_FREQ
    ));
}
