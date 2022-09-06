//! Control system. Generates control events from user input.
use crate::yar::{YarCommandEvent, YarRespawnEvent};
use crate::zorlon_cannon::CannonCommandEvent;
use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::VecDeque;
use std::time::Instant;

/// Plugin for controlling Yar that alternates between control/replay on respawn
pub struct ReplayControlPlugin;

impl Plugin for ReplayControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlTarget::Yar)
            .insert_resource(Record::<YarCommandEvent>::default())
            .insert_resource(Record::<CannonCommandEvent>::default())
            .add_startup_system(init_record_write::<YarCommandEvent>)
            .add_startup_system(init_record_read::<CannonCommandEvent>)
            .add_system(on_yar_respawn)
            .add_system(commands);
    }
}

pub fn init_record_write<E>(mut record: ResMut<Record<E>>, time: Res<Time>)
where
    E: Send + Sync + 'static,
{
    record.set_write_mode(&time)
}

pub fn init_record_read<E>(mut record: ResMut<Record<E>>, time: Res<Time>)
where
    E: Send + Sync + 'static,
{
    record.set_read_mode(&time)
}

pub fn on_yar_respawn(
    mut yar_respawn: EventReader<YarRespawnEvent>,
    mut control_target: ResMut<ControlTarget>,
    mut yar_record: ResMut<Record<YarCommandEvent>>,
    mut cannon_record: ResMut<Record<CannonCommandEvent>>,
    time: Res<Time>,
) {
    if yar_respawn.iter().next().is_none() {
        return;
    }

    *control_target = match *control_target {
        ControlTarget::Yar => {
            yar_record.set_read_mode(&time);
            cannon_record.set_write_mode(&time);
            ControlTarget::Cannon
        }
        ControlTarget::Cannon => {
            yar_record.set_write_mode(&time);
            cannon_record.set_read_mode(&time);
            ControlTarget::Yar
        }
    };
}

/// Which object the player is controlling
#[derive(Debug, Copy, Clone)]
pub enum ControlTarget {
    Yar,
    Cannon,
}

/// Record of a time series of events.
#[derive(Default, Debug, Clone)]
pub struct Record<E> {
    /// When the command sequence (read or write) was started.
    pub start_time: Option<Instant>,
    /// For each command, a duration since `start_time`
    events: VecDeque<(E, Duration)>,
}

impl<E> Record<E> {
    pub fn set_read_mode(&mut self, time: &Time) {
        self.start_time = Some(latest(time));
    }

    pub fn set_write_mode(&mut self, time: &Time) {
        self.start_time = Some(latest(time));
        self.events.clear();
    }

    /// Push an event recorded at the given time as a duration from `start_time`.
    pub fn push(&mut self, event: E, time: &Time) {
        let delay = latest(time).duration_since(self.start_time.unwrap());
        self.events.push_back((event, delay));
    }

    /// Pop the next command if its time offset from `start_time` is <= the given time.
    pub fn pop_next_before(&mut self, time: &Time) -> Option<E> {
        let delay = latest(time).duration_since(self.start_time.unwrap());
        if self.events.front()?.1 < delay {
            Some(self.events.pop_front()?.0)
        } else {
            None
        }
    }
}

pub trait ControlEvent: for<'a> From<&'a Input<KeyCode>> + Send + Sync + 'static {
    /// Whether this represents a no-input / no-op control event.
    fn is_noop(&self) -> bool;
}

/// Generate control command events
pub fn commands(
    control_target: Res<ControlTarget>,
    keys: Res<Input<KeyCode>>,
    mut yar_record: ResMut<Record<YarCommandEvent>>,
    mut cannon_record: ResMut<Record<CannonCommandEvent>>,
    yar_commands: EventWriter<YarCommandEvent>,
    cannon_commands: EventWriter<CannonCommandEvent>,
    time: Res<Time>,
) {
    match *control_target {
        ControlTarget::Yar => {
            target_commands(true, &mut yar_record, &keys, &time, yar_commands);
            target_commands(false, &mut cannon_record, &keys, &time, cannon_commands);
        }
        ControlTarget::Cannon => {
            target_commands(false, &mut yar_record, &keys, &time, yar_commands);
            target_commands(true, &mut cannon_record, &keys, &time, cannon_commands);
        }
    }
}

/// Generae commands for a particular target.
pub fn target_commands<E>(
    live: bool,
    record: &mut Record<E>,
    keys: &Input<KeyCode>,
    time: &Time,
    mut commands: EventWriter<E>,
) where
    E: ControlEvent + Clone,
{
    if live {
        let command = E::from(keys);
        if !command.is_noop() {
            record.push(command.clone(), time);
            commands.send(command);
        }
    } else {
        while let Some(command) = record.pop_next_before(time) {
            commands.send(command);
        }
    }
}

/// The most recent instant recorded by a `Time`.
fn latest(time: &Time) -> Instant {
    time.last_update().unwrap_or_else(|| time.startup())
}
