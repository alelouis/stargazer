use bevy::{input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, math::{Quat}, prelude::*, render::camera::PerspectiveProjection, window::CursorMoved};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
			title: "Stargazer!".to_string(),
			width: 1000.0,
			height: 1000.0,
			..Default::default()
		})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .run();
}

struct Star;
struct Camera;
struct MouseActive(bool);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1., 1., 1.).into()),
        transform: Transform::from_xyz(2.0, 0.0, 2.0),
        ..Default::default()
    }).insert(Star);
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        perspective_projection: bevy::render::camera::PerspectiveProjection {
            fov: std::f32::consts::PI / 2.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(2.0, 0.0, 2.0), Vec3::Y),
        ..Default::default()
    }).insert(Camera).insert(MouseActive(false));
}

fn player_movement(
	mut mouse_motion_events: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
	mut query: Query<(&mut Transform, &mut MouseActive, &mut bevy::render::camera::PerspectiveProjection), With<Camera>>,
    buttons: Res<Input<MouseButton>>,
) {
	if let Ok((mut transform, mut mouse_active, mut perspective_projection)) = query.single_mut() {
        use bevy::input::mouse::MouseScrollUnit;
        for ev in scroll_evr.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    perspective_projection.fov = 10.;
                    println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
                MouseScrollUnit::Pixel => {
                    println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                }
            }
        }
        if buttons.just_pressed(MouseButton::Left) {
            mouse_active.0 = true;
        }
        if buttons.just_released(MouseButton::Left) {
            mouse_active.0 = false;
        }
        if mouse_active.0 {
            for event in mouse_motion_events.iter() {
                transform.rotate(Quat::from_axis_angle(Vec3::Y, event.delta.x / 1000.0));
                transform.rotate(Quat::from_axis_angle(Vec3::X, event.delta.y / 1000.0));
            }
        }
	}
}

