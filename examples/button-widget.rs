use bevy::prelude::*;

use bevy_prot_widgets::{
    content_builder::*,
    theme::WidgetTheme,
    widget::{
        button::{
            ButtonColor, ButtonTheme, IconButtonBlueprint, IconLabelButtonBlueprint,
            LabelButtonBlueprint, TriggerPolicy,
        },
        icon::IconWidgetBlueprint,
        label::LabelWidgetBlueprint,
    },
    WidgetPlugin,
};
use material_icons::Icon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WidgetPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_page)
        .run();
}

// TODO: Show buttons with labels in different positions
// TODO: Show buttons with different themes / styles
// TODO: Show that clicking buttons actually change something

const COLOR_BACKGROUND: Color = Color::rgb(0.047, 0.109, 0.172);
const COLOR_CONTENT_EXAMPLE: Color = Color::rgb(0.055, 0.12, 0.19);
// const COLOR_TEXT: Color = Color::rgb(0.905, 0.921, 0.941);
// const H1_FONT: &str = "fonts/Roboto/Roboto-Bold.ttf";
// const TEXT_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
// const WIDGET_FONT: &str = "fonts/Roboto/Roboto-Regular.ttf";
// const MATERIAL_FONT: &str = "fonts/MaterialIcons-Regular.ttf";

// const H1_FONT_SIZE: f32 = 30.0;
// const TEXT_FONT_SIZE: f32 = 18.0;
// const BUTTON_FONT_SIZE: f32 = 20.0;
// const ICON_FONT_SIZE: f32 = 20.0;

// TODO: Button should not change on hover
const BUTTON_THEME: ButtonTheme = ButtonTheme {
    background: ButtonColor {
        pressed: Color::rgb(0.7, 0.7, 0.7),
        released: Color::rgb(0.6, 0.6, 0.6),
        hovered: Color::rgb(0.5, 0.5, 0.5),
        default: Color::rgb(0.25, 0.25, 0.25),
        disabled: Color::rgb(0.1, 0.1, 0.1),
    },
    foreground: ButtonColor {
        pressed: Color::rgb(0.9, 0.5, 0.1),
        released: Color::rgb(0.905, 0.921, 0.941),
        hovered: Color::rgb(0.905, 0.921, 0.941),
        default: Color::rgb(0.905, 0.921, 0.941),
        disabled: Color::rgb(0.35, 0.35, 0.35),
    },
};

/// Camera
fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}

fn setup_page(mut cmd: Commands, theme: Res<WidgetTheme>) {
    // root node
    cmd.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: COLOR_BACKGROUND.into(),
            ..default()
        }).with_children(| root| {
            // Content container
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(100.0)),
                    // min_size: Size::new(Val::Px(400.0), Val::Auto),
                    // max_size: Size::new(Val::Px(800.0), Val::Auto),
                    padding: UiRect::all(Val::Px(30.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                background_color: COLOR_BACKGROUND.into(),
                ..default()
            }).with_children(| content | {

                create_h1(content, &theme, "Buttons");
                create_p(content, &theme, "Buttons are used to trigger actions. \
                They can be hovered and clicked. The most basic button is the text button");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    LabelButtonBlueprint {
                        label: LabelWidgetBlueprint {
                            text: "Open inventory".into(),
                            theme: theme.p.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());

                    IconButtonBlueprint {
                        icon: IconWidgetBlueprint {
                            icon: Icon::Menu,
                            theme: theme.icon.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());

                    IconLabelButtonBlueprint {
                        icon: IconWidgetBlueprint { icon: Icon::Send, theme: theme.icon.clone() },
                        label: LabelWidgetBlueprint { text: "Send".into(), theme: theme.p.clone() },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());
                });


                create_h1(content, &theme, "Disabled Buttons");

                create_p(content, &theme, "Buttons can be either enabled or disabled. \
                Disabled buttons should not be triggered by the user. \
                Buttons should clearly show when they are disabled by changing colors.");
                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn text-buttons
                    for text in ["Ok", "Submit", "Restart Game"] {
                        LabelButtonBlueprint {
                            label: LabelWidgetBlueprint {
                                text: text.into(),
                                theme: theme.p.clone(),
                            },
                            enabled: false,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });

                create_h1(content, &theme, "Trigger Policy");
                create_p(content, &theme, "Buttons are triggered on release by default. \
                By setting the trigger-policy you can change the button to trigger on press instead.");
                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    LabelButtonBlueprint {
                        label: LabelWidgetBlueprint {
                            text: "Ok".into(),
                            theme: theme.p.clone(),
                        },
                        enabled: true,
                        policy: TriggerPolicy::OnPress,
                    }.build(&mut example_showcase.spawn_empty());

                    IconLabelButtonBlueprint {
                        icon: IconWidgetBlueprint { icon: Icon::Wifi, theme: theme.icon.clone() },
                        label: LabelWidgetBlueprint { text: "Enable Wifi".into(), theme: theme.p.clone() },
                        enabled: true,
                        policy: TriggerPolicy::OnRelease,
                    }.build(&mut example_showcase.spawn_empty());
                });

                create_h1(content, &theme, "Icon Buttons");
                create_p(content, &theme, "Some buttons only need icon.");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn icon-buttons
                    for icon in [Icon::Wifi, Icon::Subtitles, Icon::Delete, Icon::Add, Icon::Home] {
                        IconButtonBlueprint {
                            icon: IconWidgetBlueprint {
                                icon,
                                theme: theme.icon.clone(),
                            },
                            enabled: true,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });

                create_h1(content, &theme, "Icon and Text Buttons");
                create_p(content, &theme, "We can also create buttons that has both icons and text.");

                // Example Showcase
                content.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        margin: UiRect::new(Val::Undefined, Val::Undefined, Val::Px(10.0), Val::Px(10.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: COLOR_CONTENT_EXAMPLE.into(),
                    ..default()
                }).with_children(| example_showcase | {

                    // Spawn icon-buttons
                    for (icon, text) in [
                        (Icon::Wifi, "Enable Wifi"), 
                        (Icon::Subtitles, "Toggle Subtitles"), 
                        (Icon::Delete, "Delete Item"), 
                        (Icon::Add, "Add item"), 
                        (Icon::Home, "Home")
                    ] {
                        IconLabelButtonBlueprint {
                            icon: IconWidgetBlueprint { icon, theme: theme.icon.clone() },
                            label: LabelWidgetBlueprint { text: text.into(), theme: theme.p.clone() },
                            enabled: true,
                            policy: TriggerPolicy::OnRelease,
                        }.build(&mut example_showcase.spawn_empty());
                    }
                });
            });
        });
}