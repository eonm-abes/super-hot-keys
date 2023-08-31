use crate::config;
use std::ops::Deref;
use winit::event::{ModifiersState, VirtualKeyCode};

use std::collections::HashMap;

pub struct ShortCutManager {
    group: HashMap<ShortCut, Box<ShortCutOrGroup>>,
    current_group: HashMap<ShortCut, Box<ShortCutOrGroup>>,
}

impl ShortCutManager {
    pub fn new(value: HashMap<ShortCut, Box<ShortCutOrGroup>>) -> Self {
        Self {
            group: value.clone(),
            current_group: value,
        }
    }

    pub fn current_group(&self) -> &HashMap<ShortCut, Box<ShortCutOrGroup>> {
        &self.current_group
    }

    pub fn set_current_group(&mut self, group: HashMap<ShortCut, Box<ShortCutOrGroup>>) {
        self.current_group = group;
    }

    pub fn reset(&mut self) {
        self.current_group = self.group.clone();
    }

    pub fn trigger(&mut self, shortcut: ShortCut) -> bool {
        match self.current_group().get(&shortcut) {
            Some(shortcut_or_group) => match shortcut_or_group.deref() {
                ShortCutOrGroup::Command { cmd, .. } => {
                    cmd.spawn();
                    self.reset();
                    return true;
                }
                ShortCutOrGroup::Group { grp, .. } => self.set_current_group(grp.clone()),
            },
            None => {
                self.reset();
                return true;
            }
        };

        false
    }
}

impl Default for ShortCutManager {
    fn default() -> Self {
        Self::new(config::hotkeys())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MyCommand {
    cmd: &'static str,
    args: Vec<&'static str>,
}

impl MyCommand {
    pub fn new(cmd: &'static str, args: Vec<&'static str>) -> Self {
        Self { cmd, args }
    }

    pub fn spawn(&self) {
        let _ = std::process::Command::new(self.cmd)
            .args(&self.args)
            .spawn();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShortCutOrGroup {
    Command {
        label: &'static str,
        cmd: MyCommand,
    },
    Group {
        label: &'static str,
        grp: HashMap<ShortCut, Box<Self>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct ShortCut {
    pub modifiers: ModifiersState,
    pub key: Option<VirtualKeyCode>,
}
