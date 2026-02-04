use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy_mod_ui_render_vello::BevyUiRenderVelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyUiRenderVelloPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            display: Display::Flex,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: px(24),
            ..default()
        })
        .with_related_entities::<ChildOf>(|parent| {
            spawn_color_panel(parent, Color::srgb(1.0, 0.0, 0.0));
            spawn_color_panel(parent, Color::srgb(0.0, 1.0, 0.0));
            spawn_color_panel(parent, Color::srgb(0.0, 0.0, 1.0));
        });
}

fn spawn_color_panel(parent: &mut RelatedSpawnerCommands<ChildOf>, color: Color) {
    parent.spawn((
        Node {
            width: px(120),
            height: px(120),
            ..default()
        },
        BackgroundColor(color),
    ));
}
