use bevy::prelude::*;
use bevy::{input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, math::{Quat}, prelude::*, render::camera::PerspectiveProjection, window::CursorMoved};
use cgmath::{Rad, perspective, Matrix4, Vector4, Vector3};
use stargazer::units::polar::Polar;

use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines };

struct Fov(f32);

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Fov(1.6))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .add_system(demo.system())
        .add_system(fov_adjust.system())
        .run();
}


fn setup(
    mut commands: Commands,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
    commands.spawn_bundle(camera);
}

fn demo(time: Res<Time>, mut lines: ResMut<DebugLines>, fov: ResMut<Fov>) {

    let t: f32 = time.seconds_since_startup() as f32/3.;
    let proj_m: Matrix4<f32> = perspective(Rad(fov.0), 1.,0.1, 100.);
    let translate_m: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0., 0., -2.));
    let rotation_y_m: Matrix4<f32> = Matrix4::from_angle_y(Rad(t));
    let rotation_x_m: Matrix4<f32> = Matrix4::from_angle_x(Rad(t));

    let mut vertices_circle = vec![];
    let n_points = 30;
    let pi = std::f32::consts::PI;
    for n in 0..n_points {
        let phi = n as f32 * 2. * pi / (n_points as f32);
        for m in 0..n_points {
            let theta = m as f32 * pi / (n_points as f32);
            let p = Polar{theta:theta, phi: phi, radius: 1.};
            let p_cart: Vector4<f32> = p.to_cart();
            vertices_circle.push(p_cart);
        }
    }

    let meshes = vec![vertices_circle];
    let mut mesh_projected = vec![];
    for mesh in meshes.iter() {
        let mut vertices_proj = vec![];
        for vertex in mesh {
            let vertex_proj = proj_m * translate_m * rotation_y_m * rotation_x_m * vertex;
            let vertex_proj = vertex_proj / vertex_proj[3];
            vertices_proj.push(vertex_proj);
        }
        mesh_projected.push(vertices_proj)
    }

    for mesh in mesh_projected.iter() {
        for m in 0..mesh.len() {
            let cross_size = 5.;
            let h_window = 800.;
            let w_window = 800.;
            if (mesh[m][0] > -1.) & (mesh[m][0] < 1.) & 
               (mesh[m][1] > -1.) & (mesh[m][1] < 1.) {
                lines.line_colored(
                    Vec3::new(mesh[m][0]*w_window-cross_size, mesh[m][1]*h_window, 0.), 
                    Vec3::new(mesh[m][0]*w_window+cross_size, mesh[m][1]*h_window, 0.), 
                    0.0,
                    Color::RED);
    
                lines.line_colored(
                    Vec3::new(mesh[m][0]*w_window, mesh[m][1]*h_window-cross_size, 0.), 
                    Vec3::new(mesh[m][0]*w_window, mesh[m][1]*h_window+cross_size, 0.), 
                    0.0,
                    Color::RED);
               }
        }
    }
}


fn fov_adjust(mut scroll_evr: EventReader<MouseWheel>, mut fov: ResMut<Fov>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                fov.0 = f32::min(f32::max(fov.0 + ev.y * 0.1, 0.1), 3.14);
            }
            MouseScrollUnit::Pixel => {
                fov.0 = f32::min(f32::max(fov.0 + ev.y * 0.1, 0.1), 3.14);
            }
        }
    }
}


