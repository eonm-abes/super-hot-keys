use crate::hotkeys::ShortCutOrGroup;
use crate::ShortCut;

use std::collections::HashMap;

use crate::{cmd, group};

pub fn hotkeys() -> HashMap<ShortCut, Box<ShortCutOrGroup>> {
    group!(
        group!(
            A ["ABES"] =>
            cmd!(G ["guichet d'assistance"] => "firefox-developer-edition" "-P" "ABES" ""),
            cmd!(M ["mail"] => "firefox-developer-edition" "-P" "ABES" "outlook.office365.com"),
            cmd!(S ["STEP"] => "firefox-developer-edition" "-P" "ABES" "step.theses.fr"),
            cmd!(<SHIFT> + S ["Star"] => "firefox-developer-edition" "-P" "ABES" "star.theses.fr")
        ),
        group!(
            T ["teminal"] =>
            cmd!(T ["terminal (alacritty)"] => "alacritty"),
            cmd!(N ["nvim"] => "alacritty" "-e" "nvim")
        ),
        group!(
            D ["dolphin"] =>
            cmd!(D ["dolphin"] => "dolphin"),
            cmd!(<CTRL> + D ["Dev (dolphin)"] => "dolphin" "/home/mathis/Dev/")
        ),
        group!(
            F ["firefox"] =>
            cmd!(F ["firefox"] => "firefox-developer-edition" "-P" "default"),
            cmd!(A ["firefox ABES"] => "firefox-developer-edition" "-P" "ABES")
        ),
        group!(
            M ["multimédia"]=>
            cmd!(S ["spotify"] => "spotify"),
            cmd!(<SHIFT> + S ["steam"] => "steam"),
            cmd!(N ["netflix"] => "firefox-developer-edition" "-P" "default" "https://netflix.com"),
            cmd!(P ["plex"] => "firefox-developer-edition" "-P" "default" "")
        ),
        group!(
            C ["communication"] =>
            group!(
                A ["ABES"] =>
                cmd!(S ["slack"] => "slack")
            ),
            cmd!(D ["discord"] => "discord"),
            cmd!(T ["thunderbird"] => "thunderbird")
        )
    )
}
