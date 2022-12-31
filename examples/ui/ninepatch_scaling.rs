//! Demonstrates the use of an ImageBundle and ButtonBundle that uses ninepatch scaling.
use bevy::{prelude::*, render::{camera::{Viewport, CameraProjection}, primitives::Frustum}};

const WINDOW_WIDTH: f32 = 1280.;
const WINDOW_HEIGHT: f32 = 720.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                    ..default()
                },
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font: Handle<Font> = assets.load("fonts/FiraMono-Medium.ttf");
    let pane_texture: Handle<Image> = assets.load("textures/rpg/ui/generic-rpg-ui-pane.png");
    let inv1_texture: Handle<Image> = assets.load("textures/rpg/ui/generic-rpg-ui-inventario01.png");
    let green_button_texture: Handle<Image> = assets.load("textures/rpg/ui/generic-rpg-ui-button01.png");

    // Fullscreen root container
    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    )
    .with_children(|root| {

        // Centered pane (ninepatch image)
        root.spawn(
            ImageBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(256.), Val::Px(128.)),
                    flex_shrink: 0.,
                    ..default()
                },
                image: UiImage {
                    texture: pane_texture,
                    draw_mode: ImageDrawMode::Sliced(TextureSlicer {
                        border: BorderRect::square(5.),
                        ..default()
                    }),
                    ..default()
                },
                ..default()
            }
        ).with_children(|pane| {

            // Top text
            pane.spawn(TextBundle::from_section(
                "Item Menu",
                TextStyle {
                    font: font.clone(),
                    font_size: 12.,
                    ..default()
                }
            ));

            pane.spawn(ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                image: UiImage {
                    texture: green_button_texture.clone(),
                    draw_mode: ImageDrawMode::Sliced(TextureSlicer {
                        border: BorderRect::square(5.),
                        ..default()
                    }),
                    ..default()
                },
                ..default()
            }).with_children(|add_button| {
                add_button.spawn(TextBundle::from_section(
                    "Add",
                    TextStyle {
                        font: font.clone(),
                        ..default()
                    }
                ));
            });

            pane.spawn(ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                image: UiImage {
                    texture: green_button_texture,
                    draw_mode: ImageDrawMode::Sliced(TextureSlicer {
                        border: BorderRect::square(5.),
                        ..default()
                    }),
                    ..default()
                },
                ..default()
            }).with_children(|remove_button| {
                remove_button.spawn(TextBundle::from_section(
                    "Remove",
                    TextStyle {
                        font,
                        ..default()
                    }
                ));
            });

            // // Bottom button container
            // pane.spawn(NodeBundle::default()).with_children(|buttons| {
                
            // });
        });
    });
}
