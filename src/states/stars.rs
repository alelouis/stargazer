use bevy::prelude::*;
use bevy::{core::FixedTimestep, input::mouse::{MouseWheel, MouseMotion, MouseButtonInput}};
use cgmath::{Rad, perspective, Matrix4, Vector4, Vector3};
use bevy_prototype_debug_lines::{DebugLinesPlugin, DebugLines};
use std::fs::File;
use crate::consts::*;
use crate::units::polar::Polar;
use bevy::ecs::component::Component;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub struct Stars;
struct Fov(f32);
struct Camera{rot_x: f32, rot_y: f32}
struct Path3D(Vec<Vector4<f32>>);

enum Path2DKind {
    PhiCircle,
    ThetaCircle,
    Constellation
}
struct Path2D{
    data: Vec<Vector4<f32>>,
    marker: String,
    kind: Path2DKind}
struct Grid;
struct GridMarker;
struct Constellation;
struct MouseButtonPressed(bool);
struct Star;
struct Position3D(Vector4<f32>);

impl Plugin for Stars {
    fn build(&self, app: &mut AppBuilder){
        const TIME_STEP: f32 = 1.0 / 200.0;
        app
        .insert_resource(Fov(1.6))
        .insert_resource(Camera{rot_x: 0., rot_y: 0.})
        .insert_resource(MouseButtonPressed(false))
        .add_plugin(DebugLinesPlugin)
        .add_plugin(EguiPlugin)
        .add_system_set(
            SystemSet::on_update(AppState::Stars)
            .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            .with_system(path_projection.system())
            .with_system(draw_stars.system())
            .with_system(render_2d_paths.system())
            .with_system(render_grid_markers.system())
            .with_system(fov_adjust.system())
            .with_system(orbit_camera.system())
            .with_system(pause.system())
            //with_system(ui_example.system())
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Stars)
            .with_system(setup_2d_camera.system())
            .with_system(setup_equatorial_grid.system())
            .with_system(setup_sprites.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Stars)
            .with_system(cleanup_system::<Path3D>.system())
            .with_system(cleanup_system::<Path2D>.system())
            .with_system(cleanup_system::<Star>.system())
            .with_system(cleanup_system::<Position3D>.system())
        );
    }
}
/*
fn ui_example(egui_context: ResMut<EguiContext>, app_state: Res<State<AppState>>) {
    let mut name = "Alexis".to_string();
    let mut age = 23;
    match app_state.current() {
        AppState::Menu => {
            // TODO: play menu music
        }
        AppState::Stars => {
            egui::Window::new("Hello").show(egui_context.ctx(), |ui| {
                ui.heading("My egui Application");
                ui.horizontal(|ui| {
                    ui.label("Your name: ");
                    ui.text_edit_singleline(&mut name);
                });
                ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                if ui.button("Click each year").clicked() {
                    age += 1;
                }
                ui.label(format!("Hello '{}', age {}", name, age));
            });
        }
        AppState::Pause => {
            // TODO: play pause screen music
        }
    }

}
 */

fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn pause(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Pause).unwrap();
    }
}

/// Initialize sprites
fn setup_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let sprite_handle = materials.add(asset_server.load("images/star.png").into());
    let file_path = "assets/data/stars.csv";
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let mag = record.get(3).unwrap().parse::<f32>().unwrap();
        let p = Polar{
            theta: record.get(1).unwrap().parse::<f32>().unwrap(), 
            phi: record.get(2).unwrap().parse::<f32>().unwrap(), 
            radius: 1.}.to_cart();
        commands.spawn_bundle(SpriteBundle {
            material: sprite_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                rotation: Quat::from_rotation_z(0.),
                scale: Vec3::splat(1.),
            },
            sprite: Sprite::new(Vec2::splat(f32::max(5. - mag, 1.))),
            ..Default::default()
        }).insert(Star).insert(Position3D(p));
    }
}

/// Instanciate 2D camera view
fn setup_2d_camera(
    mut commands: Commands
){
        let mut camera = OrthographicCameraBundle::new_2d();
        camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 100.0));
        commands.spawn_bundle(camera);
    }

/// Initialize constellations 3D paths from files
fn setup_constellations(
    mut commands: Commands,
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
    commands.spawn()
    .insert(Path3D(path.clone()))
    .insert(Path2D{data: path.clone(), kind: Path2DKind::Constellation, marker: "None".to_string()})
    .insert(Constellation);
}

/// Move sprites to star locations with projection
fn draw_stars(
    mut query: Query<(&mut Transform, &mut Position3D, With<Star>)>,
    time: Res<Time>, 
    fov: ResMut<Fov>,
    camera: ResMut<Camera>, 
    wd: ResMut<WindowDescriptor>,
){
    let w = wd.width;
    let h = wd.height;
    let t: f32 = time.seconds_since_startup() as f32/1.;
    let aspect = wd.width / wd.height;
    let proj_m: Matrix4<f32> = perspective(Rad(fov.0), aspect,0.1, 100.);
    let translate_m: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0., 0., 0.0));
    let rotation_y_m: Matrix4<f32> = Matrix4::from_angle_y(Rad(camera.rot_y));
    let rotation_x_m: Matrix4<f32> = Matrix4::from_angle_x(Rad(camera.rot_x));
    let rotation_z_m: Matrix4<f32> = Matrix4::from_angle_z(Rad(0.));

    for (mut transform, position3d, _) in query.iter_mut() {
        let translation = &mut transform.translation;
        let vertex_proj = proj_m * translate_m * rotation_z_m * rotation_x_m * rotation_y_m * position3d.0;
        let vertex_proj = vertex_proj / vertex_proj[3];
        translation.x = vertex_proj[0]*w;
        translation.y = vertex_proj[1]*h;
    }
}

/// Initialize equatorial grid 3D paths
fn setup_equatorial_grid(
    mut commands: Commands
){
    let phi_split = 20;
    let theta_split = 20;
    let resolution = 200;
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
        commands.spawn()
        .insert(Path3D(vertices.clone()))
        .insert(Path2D{
            data: vertices.clone(), 
            kind: Path2DKind::PhiCircle, 
            marker: format!("{:.2}", phi*180./std::f32::consts::PI)})
        .insert(Grid);
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
        commands.spawn()
        .insert(Path3D(vertices.clone()))
        .insert(Path2D{
            data: vertices.clone(), 
            kind: Path2DKind::ThetaCircle, 
            marker: format!("{:.2}", theta*180./std::f32::consts::PI)})
        .insert(Grid);
    }
}

/// Project all 3D paths to 2D paths
fn path_projection(
    time: Res<Time>, 
    fov: ResMut<Fov>,
    camera: ResMut<Camera>, 
    mut query: Query<(&mut Path2D, &mut Path3D)>,
    wd: ResMut<WindowDescriptor>,
){
    let t: f32 = time.seconds_since_startup() as f32/100.;
    let aspect = wd.width / wd.height;
    let proj_m: Matrix4<f32> = perspective(Rad(fov.0), aspect,0.1, 100.);
    let translate_m: Matrix4<f32> = Matrix4::from_translation(Vector3::new(0., 0., 0.));
    let rotation_y_m: Matrix4<f32> = Matrix4::from_angle_y(Rad(camera.rot_y));
    let rotation_x_m: Matrix4<f32> = Matrix4::from_angle_x(Rad(camera.rot_x));
    let rotation_z_m: Matrix4<f32> = Matrix4::from_angle_z(Rad(0.));

    for (mut path2d, path3d) in query.iter_mut() { 
        let mut vertices_proj = vec![];
        for vertex in &path3d.0 {
            let vertex_proj = proj_m * translate_m * rotation_z_m * rotation_x_m * rotation_y_m * vertex;
            let vertex_proj = vertex_proj / vertex_proj[3];
            vertices_proj.push(vertex_proj);
        }
        path2d.data = vertices_proj;
    }
}

fn render_grid_markers(
    mut query: Query<&mut Path2D>,
    query_text: Query<Entity, With<GridMarker>>,
    wd: ResMut<WindowDescriptor>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for e in query_text.iter() {
        commands.entity(e).despawn_recursive();
    }
    let w = wd.width;
    let h = wd.height;
    for path in query.iter_mut() { 
        if matches!(path.kind, Path2DKind::ThetaCircle) {
            let mesh = &path.data;
            let mut right_marker: Vector4<f32> = Vector4::new(-1., -1., -1., -1.);
            let mut left_marker: Vector4<f32> = Vector4::new(1., 1., 1., 1.); 
            let mut draw_right = false;
            let mut draw_left = false;
            for m in 0..mesh.len()-1 {
                if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) {
                    // right indicators
                    if (mesh[m][0] < 0.5) & (mesh[m][0] > 0.2) & (mesh[m][1] < 0.5) & (mesh[m][1] > -0.5) {
                        draw_right = true;
                        if mesh[m][0] > right_marker[0] {
                            right_marker[0] = mesh[m][0];
                            right_marker[1] = mesh[m][1];
                        }
                    }
                    // left indicators
                    if (mesh[m][0] > -0.5) & (mesh[m][0] < -0.2) & & (mesh[m][1] < 0.5) & (mesh[m][1] > -0.5){
                        draw_left = true;
                        if mesh[m][0] < left_marker[0] {
                            left_marker[0] = mesh[m][0];
                            left_marker[1] = mesh[m][1];
                        }
                    }
                }
            }
            if draw_right {
                commands.spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(h/2. + (right_marker[1]*h) + 5.),
                            left: Val::Px(w-40.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        &path.marker,
                        TextStyle {
                            font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                            font_size: 13.0,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }).insert(GridMarker);
            }
            if draw_left {
                commands.spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(h/2. + (left_marker[1]*h) + 5.),
                            left: Val::Px(10.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        &path.marker,
                        TextStyle {
                            font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                            font_size: 13.0,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }).insert(GridMarker);
            }
            
        } else if matches!(path.kind, Path2DKind::PhiCircle) {
            let mesh = &path.data;
            let mut top_marker: Vector4<f32> = Vector4::new(-1., -1., -1., -1.);
            let mut bottom_marker: Vector4<f32> = Vector4::new(1., 1., 1., 1.); 
            let mut draw_top = false;
            let mut draw_bottom = false;
            for m in 0..mesh.len()-1 {
                
                if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) {
                    // top indicators
                    if (mesh[m][1] < 0.5) & (mesh[m][1] > 0.2) & (mesh[m][0] < 0.5) & (mesh[m][0] > -0.5) {
                        draw_top = true;
                        if mesh[m][1] > top_marker[1] {
                            top_marker[0] = mesh[m][0];
                            top_marker[1] = mesh[m][1];
                        }
                    }
                    // bottom indicators
                    if (mesh[m][1] > -0.5) & (mesh[m][1] < -0.2) & & (mesh[m][0] < 0.5) & (mesh[m][0] > -0.5){
                        draw_bottom = true;
                        if mesh[m][1] < bottom_marker[1] {
                            bottom_marker[0] = mesh[m][0];
                            bottom_marker[1] = mesh[m][1];
                        }
                    }
                }
            }  
            if draw_top {
                commands.spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(h-20.),
                            left: Val::Px(h/2. + top_marker[0]*w),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        &path.marker,
                        TextStyle {
                            font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                            font_size: 13.0,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }).insert(GridMarker);
            }
            if draw_bottom {
                commands.spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(20.),
                            left: Val::Px(h/2. + bottom_marker[0]*w),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        &path.marker,
                        TextStyle {
                            font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                            font_size: 13.0,
                            color: Color::GRAY,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                }).insert(GridMarker);
            }
        }
    }
}

/// Render 2D paths with lines
fn render_2d_paths(
    mut lines: ResMut<DebugLines>, 
    mut query: Query<(&mut Path2D, Option<&Constellation>)>,
    wd: ResMut<WindowDescriptor>,
){
    let w = wd.width;
    let h = wd.height;
    let step_size = 5;
    for (path, constellation) in query.iter_mut() {
        let color = match constellation {
            Some(x) => Color::RED,
            None => Color::Rgba{red: 1., green: 1., blue: 1., alpha: 0.05},
        };
        if matches!(path.kind, Path2DKind::ThetaCircle) {
            let mesh = &path.data;
            for m in (0..mesh.len()-step_size).step_by(step_size) {
                if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) & (mesh[m+step_size][2] > -1.) & (mesh[m+step_size][2] < 1.) {
                    lines.line_colored(
                        Vec3::new(mesh[m][0]*w, mesh[m][1]*h, 0.), 
                        Vec3::new(mesh[m+step_size][0]*w, mesh[m+step_size][1]*h, 0.), 
                        0.,
                        color);
                }
            }
        } else if matches!(path.kind, Path2DKind::PhiCircle) {
            let mesh = &path.data;
            for m in (0..mesh.len()-step_size).step_by(step_size) {
                if (mesh[m][2] > -1.) & (mesh[m][2] < 1.) & (mesh[m+step_size][2] > -1.) & (mesh[m+step_size][2] < 1.) {
                    lines.line_colored(
                        Vec3::new(mesh[m][0]*w, mesh[m][1]*h, 0.), 
                        Vec3::new(mesh[m+step_size][0]*w, mesh[m+step_size][1]*h, 0.), 
                        0.,
                        color);
                }
            }
        }
    }
}

/// Adjust field of view with mousewheel or trackpad
fn fov_adjust(
    mut scroll_evr: EventReader<MouseWheel>, 
    mut fov: ResMut<Fov>
){
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

/// Camera controller
fn orbit_camera(
    fov: ResMut<Fov>,
    mut camera: ResMut<Camera>,
    mut mouse_pressed: ResMut<MouseButtonPressed>,
    mut motion_evr: EventReader<MouseMotion>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
){
    use bevy::input::ElementState;
    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {mouse_pressed.0 = true;}
            ElementState::Released => {mouse_pressed.0 = false;}
        }
    }
    if mouse_pressed.0 {
        for ev in motion_evr.iter(){
            camera.rot_x -= fov.0 / 3.14 * ev.delta.y as f32 / 300.;
            camera.rot_y -= fov.0 / 3.14 * ev.delta.x as f32 / 300.;
        }
    }
}