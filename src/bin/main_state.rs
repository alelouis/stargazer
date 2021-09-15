use bevy::prelude::*;
use stargazer::states::main_state::MainState;
fn main() {
    App::build()
        //.insert_resource(Msaa { samples: 4 }) 
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {width: 800.0, height: 500.0, ..Default::default()})
        .add_plugins(DefaultPlugins) 
        .add_plugin(MainState)
        .run();
}
