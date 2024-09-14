use bevy::color::palettes;
use bevy::prelude::*;

use crate::board::QuitButton;
use crate::common::AppState;


pub fn click_button(
    mut interaction_query: Query<
        (&Interaction, &QuitButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction,quit_button) in &mut interaction_query {
        println!("quit_button:{:?}",quit_button);
        match *interaction {
            Interaction::Pressed => match quit_button {
                QuitButton  => {
                    info!("Quit button clicked");
                    exit.send_default();
                }
            },
            _ => {}
        }
    }
}
