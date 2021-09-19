use bevy::prelude::*;
use stargazer::states::stars::Stars;
use stargazer::states::menu::Menu;
use stargazer::consts::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {width: 500.0, height: 500.0, ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_plugin(Menu)
        .add_plugin(Stars) 
        .run();
}
