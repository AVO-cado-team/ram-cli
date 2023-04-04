use colored::Colorize;

pub enum ColorCode {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,

    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

pub enum StyleCode {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strikethrough,
}

pub enum Style {
    FG(ColorCode),
    BG(ColorCode),
    ST(StyleCode),
}

// macros for creating the colorize trait
macro_rules! colorize {
    ($($name:ident => $codetyle:ident::$code:ident in $aspect:ident),*) => {
        $(
            fn $name(&self) -> String {
                self.colorize(Style::$aspect($codetyle::$code))
            }
        )*
    };
}

pub trait Colorizable {
    fn colorize(&self, color: Style) -> String;
    colorize! {
        fgblack => ColorCode::Black in FG,
        fgred => ColorCode::Red in FG,
        fggreen => ColorCode::Green in FG,
        fgyellow => ColorCode::Yellow in FG,
        fgblue => ColorCode::Blue in FG,
        fgmagenta => ColorCode::Magenta in FG,
        fgcyan => ColorCode::Cyan in FG,
        fgwhite => ColorCode::White in FG,
        fggray => ColorCode::Gray in FG,
        fgbright_black => ColorCode::BrightBlack in FG,
        fgbright_red => ColorCode::BrightRed in FG,
        fgbright_green => ColorCode::BrightGreen in FG,
        fgbright_yellow => ColorCode::BrightYellow in FG,
        fgbright_blue => ColorCode::BrightBlue in FG,
        fgbright_magenta => ColorCode::BrightMagenta in FG,
        fgbright_cyan => ColorCode::BrightCyan in FG,
        fgbright_white => ColorCode::BrightWhite in FG,

        bgblack => ColorCode::Black in BG,
        bgred => ColorCode::Red in BG,
        bggreen => ColorCode::Green in BG,
        bgyellow => ColorCode::Yellow in BG,
        bgblue => ColorCode::Blue in BG,
        bgmagenta => ColorCode::Magenta in BG,
        bgcyan => ColorCode::Cyan in BG,
        bgwhite => ColorCode::White in BG,
        bggray => ColorCode::Gray in BG,
        bgbright_black => ColorCode::BrightBlack in BG,
        bgbright_red => ColorCode::BrightRed in BG,
        bgbright_green => ColorCode::BrightGreen in BG,
        bgbright_yellow => ColorCode::BrightYellow in BG,
        bgbright_blue => ColorCode::BrightBlue in BG,
        bgbright_magenta => ColorCode::BrightMagenta in BG,
        bgbright_cyan => ColorCode::BrightCyan in BG,
        bgbright_white => ColorCode::BrightWhite in BG,

        stbold => StyleCode::Bold in ST,
        stdimmed => StyleCode::Dimmed in ST,
        stitalic => StyleCode::Italic in ST,
        stunderlined => StyleCode::Underline in ST,
        stblink => StyleCode::Blink in ST,
        streverse => StyleCode::Reverse in ST,
        sthidden => StyleCode::Hidden in ST,
        ststrikethrough => StyleCode::Strikethrough in ST
    }
}

impl Colorizable for str {
    fn colorize(&self, style: Style) -> String {
        match style {
            Style::FG(color) => match color {
                ColorCode::Black => self.black(),
                ColorCode::Red => self.red(),
                ColorCode::Green => self.green(),
                ColorCode::Yellow => self.yellow(),
                ColorCode::Blue => self.blue(),
                ColorCode::Magenta => self.magenta(),
                ColorCode::Cyan => self.cyan(),
                ColorCode::White => self.white(),
                ColorCode::Gray => self.truecolor(100, 100, 100),

                ColorCode::BrightBlack => self.bright_black(),
                ColorCode::BrightRed => self.bright_red(),
                ColorCode::BrightGreen => self.bright_green(),
                ColorCode::BrightYellow => self.bright_yellow(),
                ColorCode::BrightBlue => self.bright_blue(),
                ColorCode::BrightMagenta => self.bright_magenta(),
                ColorCode::BrightCyan => self.bright_cyan(),
                ColorCode::BrightWhite => self.bright_white(),
            },
            Style::BG(color) => match color {
                ColorCode::Black => self.on_black(),
                ColorCode::Red => self.on_red(),
                ColorCode::Green => self.on_green(),
                ColorCode::Yellow => self.on_yellow(),
                ColorCode::Blue => self.on_blue(),
                ColorCode::Magenta => self.on_magenta(),
                ColorCode::Cyan => self.on_cyan(),
                ColorCode::White => self.on_white(),
                ColorCode::Gray => self.on_truecolor(100, 100, 100),

                ColorCode::BrightBlack => self.bright_black(),
                ColorCode::BrightRed => self.bright_red(),
                ColorCode::BrightGreen => self.bright_green(),
                ColorCode::BrightYellow => self.bright_yellow(),
                ColorCode::BrightBlue => self.bright_blue(),
                ColorCode::BrightMagenta => self.bright_magenta(),
                ColorCode::BrightCyan => self.bright_cyan(),
                ColorCode::BrightWhite => self.bright_white(),
            },
            Style::ST(style) => match style {
                StyleCode::Bold => self.bold(),
                StyleCode::Dimmed => self.dimmed(),
                StyleCode::Italic => self.italic(),
                StyleCode::Underline => self.underline(),
                StyleCode::Blink => self.blink(),
                StyleCode::Reverse => self.reverse(),
                StyleCode::Hidden => self.hidden(),
                StyleCode::Strikethrough => self.strikethrough(),
                _ => self.normal(),
            },
        }
        .to_string()
    }
}

impl Colorizable for String {
    fn colorize(&self, color: Style) -> String {
        self.as_str().colorize(color)
    }
}
