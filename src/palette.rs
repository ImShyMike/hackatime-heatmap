pub type Color = (u8, u8, u8);

pub struct Palette {
    name: &'static str,
    colors: [Color; 5],
}

impl Palette {
    fn get_color(&self, level: usize) -> Color {
        self.colors[level]
    }

    pub fn calculate_color(&self, value: u32, max: u32, ranges: &[u32]) -> Color {
        let level = if value < 60 {
            0
        } else {
            let ratio = value as f32 / max as f32;
            if ratio >= ranges[0] as f32 / 100.0 {
                4
            } else if ratio >= ranges[1] as f32 / 100.0 {
                3
            } else if ratio >= ranges[2] as f32 / 100.0 {
                2
            } else {
                1
            }
        };
        self.get_color(level)
    }
}

pub fn get_palette<'a>(palletes: &'a [Palette], name: &String) -> &'a Palette {
    palletes
        .iter()
        .find(|p| p.name == name)
        .unwrap_or(&PALETTES[0]) // default to dark
}

pub const PALETTES: &[Palette] = &[
    Palette {
        name: "dark", // Github Dark
        colors: [
            (22, 27, 34),    // level 0 (no activity)
            (0, 92, 46),     // level 1
            (0, 130, 60),    // level 2
            (57, 166, 84),   // level 3
            (112, 201, 133), // level 4 (most activity)
        ],
    },
    Palette {
        name: "light", // Github Light
        colors: [
            (235, 237, 240), // level 0 (no activity)
            (155, 233, 168), // level 1
            (64, 196, 99),   // level 2
            (48, 161, 78),   // level 3
            (33, 110, 57),   // level 4 (most activity)
        ],
    },
    Palette {
        name: "catppuccin_light", // Catppuccin Latte
        colors: [
            (204, 208, 218), // level 0 (no activity)
            (64, 160, 43),   // level 1
            (223, 142, 29),  // level 2
            (254, 100, 11),  // level 3
            (210, 15, 57),   // level 4 (most activity)
        ],
    },
    Palette {
        name: "catppuccin_dark", // Catppuccin Mocha
        colors: [
            (49, 50, 68),    // level 0 (no activity)
            (166, 227, 161), // level 1
            (249, 226, 175), // level 2
            (250, 179, 135), // level 3
            (243, 139, 168), // level 4 (most activity)
        ],
    },
];
