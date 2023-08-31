macro_rules! shortcut {
    ($KEY:ident) => {{
        shortcut!(+ $KEY)
    }};

    ($(<$MODIFIER:ident>)* + $KEY:ident) => {{
        #[allow(unused_mut)]
        let mut modifiers = winit::event::ModifiersState::default();

        $(
            modifiers.set(winit::event::ModifiersState::$MODIFIER, true);
        )*

        ShortCut { modifiers, key: Some(winit::event::VirtualKeyCode::$KEY) }
    }};
}

macro_rules! cmd {
    ($KEY:ident [$label:literal] => $cmd:literal $($args:literal)*) => {{
        cmd!(+ $KEY [$label] => $cmd $($args)*)
    }};

    ($(<$MODIFIER:ident>)* + $KEY:ident [$label:literal] => $cmd:literal $($args:literal)*) => {{
        (crate::shortcut!($(<$MODIFIER>)* + $KEY), Box::new(ShortCutOrGroup::Command {
            label: $label,
            cmd: crate::MyCommand::new($cmd, vec![$($args),*])
        }))
    }};
}

macro_rules! group {
    ($KEY:ident [$label:literal] => $($cmds:expr),*) => {{
        group!(+ $KEY [$label] => $($cmds),*)
    }};

    ($(<$MODIFIER:ident>)* + $KEY:ident [$label:literal] => $($cmds:expr),* ) => {{
        (crate::shortcut!($(<$MODIFIER>)* + $KEY), Box::new(ShortCutOrGroup::Group {
            label: $label,
            grp: group!($($cmds),*)
        }))
    }};

    // produces root group
    ($($cmds:expr),* ) => {{
        HashMap::from_iter(
            vec![
                $(($cmds)),*
            ]
        )
    }};
}

pub(crate) use cmd;
pub(crate) use group;
pub(crate) use shortcut;
