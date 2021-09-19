use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::component::Component;
use crate::consts::*;

pub struct Pause;
pub enum BMarkers {
    Resume,
    Quit,
}
struct ButtonMarker(BMarkers);

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl Plugin for Pause {
    fn build(&self, app: &mut AppBuilder){
        app
        .init_resource::<ButtonMaterials>()
        .add_system_set(
            SystemSet::on_enter(AppState::Pause)
            .with_system(setup.system())
        )
        .add_system_set(
            SystemSet::on_update(AppState::Pause)
            .with_system(button_system.system())
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Pause)
            .with_system(cleanup_system::<Button>.system())
            .with_system(cleanup_system::<Text>.system())
        );
    }
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children, &ButtonMarker),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state: ResMut<State<AppState>>
) {
    for (interaction, mut material, children, button) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match button.0 {
                    BMarkers::Resume => {app_state.set(AppState::Stars).unwrap();}
                    BMarkers::Quit => {exit.send(AppExit);}
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
                match button.0 {
                    BMarkers::Resume => {text.sections[0].value = "Resume".to_string();}
                    BMarkers::Quit => {text.sections[0].value = "Quit".to_string();}
                }
            }
        }
    }
}


fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "github.com/alelouis/stargazer",
            TextStyle {
                font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                font_size: 15.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Stargazer v0.1",
            TextStyle {
                font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(45.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(ButtonMarker(BMarkers::Resume))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Resume",
                    TextStyle {
                        font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
        commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(45.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(ButtonMarker(BMarkers::Quit))
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Quit",
                    TextStyle {
                        font: asset_server.load("fonts/ShareTechMono-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}