//! The game's main screen states and transitions between them.

mod credits;
pub mod gameplay;
mod loading;
mod splash;
mod title;
mod system_set;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>()
        .add_sub_state::<gameplay::GameplayState>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        system_set::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Gameplay,
}
