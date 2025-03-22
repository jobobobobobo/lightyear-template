use bevy::prelude::*;
use lightyear::prelude::client::ClientCommandsExt;

use crate::app::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu_ui);
        app.add_systems(OnEnter(GameState::Connecting), on_client_begin_connecting);
        app.add_systems(OnEnter(GameState::Loading), on_client_begin_loading);
        app.add_systems(OnEnter(GameState::Playing), despawn_main_menu_ui);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MainMenuStatusText;

#[derive(Component)]
pub struct ConnectButton;

fn spawn_main_menu_ui(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
    // Despawn any existing copies of the menu
    for entity in &q_main_menu {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MainMenu,
        ))
        .with_children(|child_builder| {
            child_builder
                .spawn((
                    Text::new("My Game"),
                    TextFont {
                        font_size: 30.,
                        ..default()
                    },
                    Node {
                        padding: UiRect::bottom(Val::Px(200.)),
                        ..default()
                    },
                ))
                .insert(MainMenuStatusText);

            child_builder
                .spawn((
                    Text::new("Connect"),
                    Node {
                        padding: UiRect::bottom(Val::Px(20.)),
                        ..default()
                    },
                ))
                .insert(ConnectButton)
                .observe(|_click: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.set_state(GameState::Connecting);
                });
            
            #[cfg(feature = "host")]
            child_builder
                .spawn(Text::new("Host"))
                .insert(ConnectButton)
                .observe(|_click: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.set_state(GameState::Hosting);
                });
        });
}

fn on_client_begin_loading(mut q_status_text: Query<&mut Text, With<MainMenuStatusText>>) {
    for mut text in q_status_text.iter_mut() {
        text.0 = String::from("Loading");
    }
}

fn on_client_begin_connecting(mut q_status_text: Query<&mut Text, With<MainMenuStatusText>>) {
    for mut text in q_status_text.iter_mut() {
        text.0 = String::from("Connecting");
    }
}

fn despawn_main_menu_ui(mut commands: Commands, q_main_menu: Query<Entity, With<MainMenu>>) {
    for entity in &q_main_menu {
        commands.entity(entity).despawn_recursive();
    }
}
