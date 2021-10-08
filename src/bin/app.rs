use bevy::prelude::*;
use stargazer::states::stars::Stars;
use stargazer::states::menu::Menu;
use stargazer::states::pause::Pause;
use stargazer::consts::*;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor { 
            title: "Stargazer".to_string(), 
            width: 1200.0, 
            height: 700.0, 
            vsync: false,
            ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_plugin(Menu)
        .add_plugin(Stars) 
        .add_plugin(Pause) 
        .run();
}
