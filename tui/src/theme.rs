use clap::ValueEnum;
use ratatui::style::Color;

// Add the Theme name here for a new theme
// This is more secure than the previous list
// We cannot index out of bounds, and we are giving
// names to our various themes, making it very clear
// This will make it easy to add new themes
#[derive(Clone, Debug, PartialEq, Default, ValueEnum, Copy)]
pub enum Theme {
    #[default]
    Default,
    Compatible,
    CatppuccinMocha,
    Nord,
    Gruvbox,
    Solarized,
}

impl Theme {
    pub fn dir_color(&self) -> Color {
        match self {
            Theme::Default => Color::Blue,
            Theme::Compatible => Color::Blue,
            Theme::CatppuccinMocha => Color::Rgb(137, 180, 250),
            Theme::Nord => Color::Rgb(143, 188, 187),
            Theme::Gruvbox => Color::Rgb(184, 187, 38),
            Theme::Solarized => Color::Rgb(38, 139, 210),
        }
    }

    pub fn cmd_color(&self) -> Color {
        match self {
            Theme::Default => Color::Rgb(204, 224, 208),
            Theme::Compatible => Color::LightGreen,
            Theme::CatppuccinMocha => Color::Rgb(166, 227, 161),
            Theme::Nord => Color::Rgb(163, 190, 140),
            Theme::Gruvbox => Color::Rgb(215, 153, 33),
            Theme::Solarized => Color::Rgb(133, 153, 0),
        }
    }

    pub fn tab_color(&self) -> Color {
        match self {
            Theme::Default => Color::Rgb(255, 255, 85),
            Theme::Compatible => Color::Yellow,
            Theme::CatppuccinMocha => Color::Rgb(249, 226, 175),
            Theme::Nord => Color::Rgb(235, 203, 139),
            Theme::Gruvbox => Color::Rgb(250, 189, 47),
            Theme::Solarized => Color::Rgb(181, 137, 0),
        }
    }

    pub fn dir_icon(&self) -> &'static str {
        match self {
            Theme::Compatible => "[DIR]",
            _ => "",
        }
    }

    pub fn cmd_icon(&self) -> &'static str {
        match self {
            Theme::Compatible => "[CMD]",
            _ => "",
        }
    }

    pub fn tab_icon(&self) -> &'static str {
        match self {
            Theme::Compatible => ">> ",
            _ => " ",
        }
    }

    pub fn multi_select_icon(&self) -> &'static str {
        match self {
            Theme::Compatible => "*",
            _ => "",
        }
    }

    pub fn success_color(&self) -> Color {
        match self {
            Theme::Default => Color::Rgb(199, 55, 44),
            Theme::Compatible => Color::Green,
            Theme::CatppuccinMocha => Color::Rgb(166, 227, 161),
            Theme::Nord => Color::Rgb(163, 190, 140),
            Theme::Gruvbox => Color::Rgb(152, 151, 26),
            Theme::Solarized => Color::Rgb(133, 153, 0),
        }
    }

    pub fn fail_color(&self) -> Color {
        match self {
            Theme::Default => Color::Rgb(5, 255, 55),
            Theme::Compatible => Color::Red,
            Theme::CatppuccinMocha => Color::Rgb(243, 139, 168),
            Theme::Nord => Color::Rgb(191, 97, 106),
            Theme::Gruvbox => Color::Rgb(204, 36, 29),
            Theme::Solarized => Color::Rgb(220, 50, 47),
        }
    }

    pub fn focused_color(&self) -> Color {
        match self {
            Theme::Default | Theme::Compatible => Color::LightBlue,
            Theme::CatppuccinMocha => Color::Rgb(137, 220, 235),
            Theme::Nord => Color::Rgb(136, 192, 208),
            Theme::Gruvbox => Color::Rgb(69, 133, 136),
            Theme::Solarized => Color::Rgb(42, 161, 152),
        }
    }

    pub fn unfocused_color(&self) -> Color {
        match self {
            Theme::Default | Theme::Compatible => Color::Gray,
            Theme::CatppuccinMocha => Color::Rgb(108, 112, 134),
            Theme::Nord => Color::Rgb(76, 86, 106),
            Theme::Gruvbox => Color::Rgb(146, 131, 116),
            Theme::Solarized => Color::Rgb(88, 110, 117),
        }
    }

    pub fn background_color(&self) -> Color {
        match self {
            Theme::Default | Theme::Compatible => Color::Reset,
            Theme::CatppuccinMocha => Color::Rgb(30, 30, 46),
            Theme::Nord => Color::Rgb(46, 52, 64),
            Theme::Gruvbox => Color::Rgb(40, 40, 40),
            Theme::Solarized => Color::Rgb(0, 43, 54),
        }
    }

    pub fn border_color(&self) -> Color {
        match self {
            Theme::Default | Theme::Compatible => Color::Reset,
            Theme::CatppuccinMocha => Color::Rgb(88, 91, 112),
            Theme::Nord => Color::Rgb(76, 86, 106),
            Theme::Gruvbox => Color::Rgb(124, 111, 100),
            Theme::Solarized => Color::Rgb(7, 54, 66),
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Theme::Default | Theme::Compatible => Color::Reset,
            Theme::CatppuccinMocha => Color::Rgb(205, 214, 244),
            Theme::Nord => Color::Rgb(216, 222, 233),
            Theme::Gruvbox => Color::Rgb(235, 219, 178),
            Theme::Solarized => Color::Rgb(131, 148, 150),
        }
    }

    pub fn next(&mut self) {
        let position = *self as usize;
        let types = Theme::value_variants();
        *self = types[(position + 1) % types.len()];
    }

    pub fn prev(&mut self) {
        let position = *self as usize;
        let types = Theme::value_variants();
        *self = types[(position + types.len() - 1) % types.len()];
    }
}
