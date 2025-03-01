use bevy::{app::AppExit, prelude::*};

use crate::{
    components::{MainMenu, MenuButton, MenuButtonAction},
    styles::*,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                ..default()
            },
            MainMenu,
        ))
        .id();

    // Title
    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Px(300.0),
            height: Val::Px(120.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("Bevy Demo", get_title_text_style(&asset_server)),
            ..default()
        });
    }).set_parent(main_menu);

    // Buttons
    spawn_menu_button(&mut commands, &asset_server, "Play", MenuButtonAction::Play, main_menu);
    spawn_menu_button(&mut commands, &asset_server, "Settings", MenuButtonAction::Settings, main_menu);
    spawn_menu_button(&mut commands, &asset_server, "Quit", MenuButtonAction::Quit, main_menu);
}

fn spawn_menu_button(
    commands: &mut Commands,
    asset_server: &AssetServer,
    text: &str,
    action: MenuButtonAction,
    parent: Entity,
) {
    commands.spawn((
        ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
        },
        MenuButton {
            action: action.clone(),
        },
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(text, get_button_text_style(asset_server)),
            ..default()
        });
    })
    .set_parent(parent);
}

pub fn handle_menu_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color, menu_button) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                match menu_button.action {
                    MenuButtonAction::Quit => app_exit_events.send(AppExit),
                    MenuButtonAction::Play => { /* TODO: Implement game state change */ }
                    MenuButtonAction::Settings => { /* TODO: Implement settings menu */ }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
} 