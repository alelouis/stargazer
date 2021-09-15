use bevy::prelude::*;
use bevy::{input::mouse::{MouseWheel}};
use cgmath::{Rad, perspective, Matrix4, Vector4, Vector3};
use stargazer::units::polar::Polar;
use bevy_prototype_debug_lines::{ DebugLinesPlugin, DebugLines };
use std::fs::File;

struct Fov(f32);
struct Path3D(Vec<Vector4<f32>>);
struct Constellation;
struct Path2D(Vec<Vector4<f32>>);

fn main() {
    App::build()
        //.insert_resource(Msaa { samples: 4 })
        .insert_resource(Fov(1.6))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {width: 800.0, height: 500.0, ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(setup_equatorial_grid.system())
        .add_startup_system(setup_constellations.system())
        .add_system(projection.system())
        //.add_system(render_2d_vertices.system())
        .add_system(render_2d_paths.system())
        .add_system(despawn_2d_paths.system())
        .add_system(fov_adjust.system())
        .run();
}

fn setup(
mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
    commands.spawn_bundle(camera);
}

fn setup_constellations(
    mut commands: Commands
) {
    let file_path = "assets/stars.csv";
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut path = vec![];
    for result in rdr.records() {
        let record = result.unwrap();
        let p = Polar{
            theta: record.get(1).unwrap().parse::<f32>().unwrap(), 
            phi: record.get(2).unwrap().parse::<f32>().unwrap(), 
            radius: 1.}.to_cart();
        path.push(p);
    }
    commands.spawn().insert(Path3D(path)).insert(Constellation);
}

fn setup_equatorial_grid(
    mut commands: Commands
) {
    let phi_split = 20;
    let theta_split = 20;
    let resolution = 50;
    // phi circles
    for split in 0..phi_split+1 {
        let mut vertices = vec![];
        let phi = split as f32 * 2. * std::f32::consts::PI / (phi_split as f32);
        for m in 0..resolution+1 {
            let theta = m as f32 * std::f32::consts::PI / (resolution as f32);
            let p = Polar{theta:theta, phi: phi, radius: 1.};
            let p_cart: Vector4<f32> = p.to_cart();
            vertices.push(p_cart);
        }
        commands.spawn().insert(Path3D(vertices));
    }
    // theta circles
    for split in 0..theta_split+1 {
        let mut vertices = vec![];
        let theta = split as f32 * std::f32::consts::PI / (theta_split as f32);
        for m in 0..resolution+1 {
            let phi = m as f32 * 2. * std::f32::consts::PI / (resolution as f32);
            let p = Polar{theta:theta, phi: phi, radius: 1.};
            let p_cart: Vector4<f32> = p.to_cart();
            vertices.push(p_cart);
        }
        commands.spawn().insert(Path3D(vertices));
    }
}

fn projection(
    mut commands: Commands,
    time: Res<Time>, 
    fov: ResMut<Fov>, 
    mut query: Query<(&mut Path3D, Option<&Constellation>)>,
    wd: ResMut<WindowDescriptor>,
) {
    let t: f32 = time.seconds_since_startup() as f32/3.;
    let aspect = wd.width / wd.height;
    let proj_m: Matrix4<f32> = perspective(Rad(fov.0), aspect,0.1, 100.);
    let translate_m: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0., 0., 0.));
    let rotation_y_m: Matrix4<f32> = Matrix4::from_angle_y(Rad(t));
    let rotation_x_m: Matrix4<f32> = Matrix4::from_angle_x(Rad(-1.1));
    let rotation_z_m: Matrix4<f32> = Matrix4::from_angle_z(Rad(0.));

    for (path, constellation) in query.iter_mut() { 
        let mut vertices_proj = vec![];
        for vertex in &path.0 {
            let vertex_proj = proj_m * translate_m * rotation_z_m * rotation_x_m * rotation_y_m * vertex;
            let vertex_proj = vertex_proj / vertex_proj[3];
            vertices_proj.push(vertex_proj);
        }
        if constellation.is_some() {
            commands.spawn().insert(Path2D(vertices_proj)).insert(Constellation);
        } else {
            commands.spawn().insert(Path2D(vertices_proj));
        }
    }
}

fn despawn_2d_paths(
    mut commands: Commands,
    mut query: Query<(Entity, With<Path2D>)>
) {
    for (e, _) in query.iter_mut() {
        commands.entity(e).despawn();
    }
}

fn render_2d_paths(
    mut lines: ResMut<DebugLines>, 
    mut query: Query<(&mut Path2D, Option<&Constellation>)>,
    wd: ResMut<WindowDescriptor>,
){
    let w = wd.width;
    let h = wd.height;
    for (path, constellation) in query.iter_mut() {
        let color = match constellation {
            Some(x) => Color::RED,
            None => Color::GRAY,
        };
        let mesh = &path.0;
        for m in 0..mesh.len()-1 {
            if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) & (mesh[m+1][2] > -1.) & (mesh[m+1][2] < 1.) {
                lines.line_colored(
                    Vec3::new(mesh[m][0]*w, mesh[m][1]*h, 0.), 
                    Vec3::new(mesh[m+1][0]*w, mesh[m+1][1]*h, 0.), 
                    0.0,
                    color);
            }
        }
    }
}

fn render_2d_vertices(
    mut lines: ResMut<DebugLines>, 
    mut query: Query<&mut Path2D>,
    wd: ResMut<WindowDescriptor>,
){
    let w = wd.width;
    let h = wd.height;
    for path in query.iter_mut() { 
        let mesh = &path.0;
        for m in 0..mesh.len() {
            let cross_size = 5.;
            if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) {
                lines.line_colored(
                    Vec3::new(mesh[m][0]*w-cross_size, mesh[m][1]*h, 0.), 
                    Vec3::new(mesh[m][0]*w+cross_size, mesh[m][1]*h, 0.), 
                    0.0,
                    Color::RED);

                lines.line_colored(
                    Vec3::new(mesh[m][0]*w, mesh[m][1]*h-cross_size, 0.), 
                    Vec3::new(mesh[m][0]*w, mesh[m][1]*h+cross_size, 0.), 
                    0.0,
                    Color::RED);
            }
        }
    }
}

fn fov_adjust(
    mut scroll_evr: EventReader<MouseWheel>, 
    mut fov: ResMut<Fov>
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                fov.0 = f32::min(f32::max(fov.0 + ev.y * 0.1, 0.1), 3.14);
            }
            MouseScrollUnit::Pixel => {
                fov.0 = f32::min(f32::max(fov.0 + ev.y * 0.001, 0.1), 3.14);
            }
        }
    }
}




