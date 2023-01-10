use bevy::{
    ecs::system::EntityCommands,
    prelude::{default, Bundle, Button, Changed, Color, Component, Handle, Query},
    text::{Font, Text, TextStyle},
    ui::Interaction,
};
use material_icons::Icon;

use crate::blueprint::WidgetBlueprint;

use super::icon::{IconWidget, IconWidgetBundle};

const FONT_SIZE: f32 = 50.0;
const ICON_COLOR: Color = Color::BLACK;

/// Marker component for a [CheckboxWidget].
#[derive(Component, Debug, Clone, Default)]
pub struct CheckboxWidget(pub CheckboxState);

#[derive(Debug, Clone, Default)]
pub enum CheckboxState {
    Checked,
    #[default]
    Unchecked,
    Indeterminate,
}

pub struct CheckBoxBlueprint {
    pub state: CheckboxState,
    pub font: Handle<Font>,
}

impl<'w, 's> WidgetBlueprint<'w, 's> for CheckBoxBlueprint {
    fn build<'a>(
        self,
        cmd: &'a mut EntityCommands<'w, 's, 'a>,
    ) -> &'a mut EntityCommands<'w, 's, 'a> {
        let icon = match self.state {
            CheckboxState::Checked => Icon::CheckBox,
            CheckboxState::Unchecked => Icon::CheckBoxOutlineBlank,
            CheckboxState::Indeterminate => Icon::IndeterminateCheckBox,
        };

        cmd.insert(CheckboxBundle {
            checkbox: CheckboxWidget(self.state),
            icon: IconWidgetBundle {
                icon_widget: IconWidget(icon),
                text: Text::from_section(
                    icon.to_string(),
                    TextStyle {
                        font: self.font,
                        font_size: FONT_SIZE,
                        color: ICON_COLOR,
                    },
                ),
                ..default()
            },
            ..default()
        });

        cmd
    }
}

/// A Checkbox Widget
#[derive(Bundle, Debug, Default)]
pub struct CheckboxBundle {
    pub checkbox: CheckboxWidget,
    #[bundle]
    pub icon: IconWidgetBundle,

    pub interaction: Interaction,

    pub button: Button,
}

pub(crate) fn update_checkbox_interaction(
    mut q: Query<(&mut CheckboxWidget, &Interaction), Changed<Interaction>>,
) {
    for (mut checkbox, interaction) in &mut q {
        if matches!(interaction, Interaction::Clicked) {
            checkbox.0 = match checkbox.0 {
                CheckboxState::Checked => CheckboxState::Unchecked,
                CheckboxState::Unchecked => CheckboxState::Checked,
                CheckboxState::Indeterminate => CheckboxState::Checked,
            }
        }
    }
}

pub(crate) fn update_checkbox_icon(
    mut q: Query<(&CheckboxWidget, &mut IconWidget), Changed<CheckboxWidget>>,
) {
    for (checkbox, mut icon) in &mut q {
        icon.0 = match checkbox.0 {
            CheckboxState::Checked => Icon::CheckBox,
            CheckboxState::Unchecked => Icon::CheckBoxOutlineBlank,
            CheckboxState::Indeterminate => Icon::IndeterminateCheckBox,
        };
    }
}
