use bevy::prelude::*;
use incrustmental::incremental::State;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct StateRes(pub State);

impl From<StateRes> for State {
    fn from(state_res: StateRes) -> Self {
        state_res.0
    }
}

impl Deref for StateRes {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StateRes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
