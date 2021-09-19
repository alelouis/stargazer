use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use stargazer::states::main_state::MainState;
use bevy::wgpu::{WgpuOptions, WgpuBackend};

fn main() {
    App::build()
        //.insert_resource(Msaa { samples: 4 }) 
        .insert_resource(WgpuOptions {
            backend: WgpuBackend::Auto, // this is not yet implemented, and could be an enum rather than a string
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {width: 800.0, height: 500.0, ..Default::default()})
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(MainState)  
        .run();
}
