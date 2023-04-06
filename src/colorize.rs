#[cfg(all(not(target_os = "windows"), not(feature = "html_output")))]
use colored::Colorize;
#[cfg(all(target_os = "windows", not(feature = "html_output")))]
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub enum STL {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
    Cyan,
    Magenta,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightYellow,
    BrightWhite,
    BrightBlack,
    BrightCyan,
    BrightMagenta,
    OnRed,
    OnGreen,
    OnBlue,
    OnYellow,
    OnWhite,
    OnBlack,
    OnCyan,
    OnMagenta,
    OnBrightRed,
    OnBrightGreen,
    OnBrightBlue,
    OnBrightYellow,
    OnBrightWhite,
    OnBrightBlack,
    OnBrightCyan,
    OnBrightMagenta,
    Normal,
    TrueColor(u8, u8, u8),
    Bold,
}

#[cfg(all(not(target_os = "windows"), not(feature = "html_output")))]
pub fn styled_output<T>(s: &str, styles: T) -> String
where
    T: IntoIterator<Item = STL>,
{
    let mut output = s.to_string();
    for style in styles {
        output = match style {
            STL::Red => output.red().to_string(),
            STL::Green => output.green().to_string(),
            STL::Blue => output.blue().to_string(),
            STL::Yellow => output.yellow().to_string(),
            STL::White => output.white().to_string(),
            STL::Black => output.black().to_string(),
            STL::Cyan => output.cyan().to_string(),
            STL::Magenta => output.magenta().to_string(),
            STL::BrightRed => output.bright_red().to_string(),
            STL::BrightGreen => output.bright_green().to_string(),
            STL::BrightBlue => output.bright_blue().to_string(),
            STL::BrightYellow => output.bright_yellow().to_string(),
            STL::BrightWhite => output.bright_white().to_string(),
            STL::BrightBlack => output.bright_black().to_string(),
            STL::BrightCyan => output.bright_cyan().to_string(),
            STL::BrightMagenta => output.bright_magenta().to_string(),
            STL::OnRed => output.on_red().to_string(),
            STL::OnGreen => output.on_green().to_string(),
            STL::OnBlue => output.on_blue().to_string(),
            STL::OnYellow => output.on_yellow().to_string(),
            STL::OnWhite => output.on_white().to_string(),
            STL::OnBlack => output.on_black().to_string(),
            STL::OnCyan => output.on_cyan().to_string(),
            STL::OnMagenta => output.on_magenta().to_string(),
            STL::OnBrightRed => output.on_bright_red().to_string(),
            STL::OnBrightGreen => output.on_bright_green().to_string(),
            STL::OnBrightBlue => output.on_bright_blue().to_string(),
            STL::OnBrightYellow => output.on_bright_yellow().to_string(),
            STL::OnBrightWhite => output.on_bright_white().to_string(),
            STL::OnBrightBlack => output.on_bright_black().to_string(),
            STL::OnBrightCyan => output.on_bright_cyan().to_string(),
            STL::OnBrightMagenta => output.on_bright_magenta().to_string(),
            STL::Normal => output.normal().to_string(),
            STL::TrueColor(r, g, b) => output.truecolor(r, g, b).to_string(),
            STL::Bold => output.bold().to_string(),
        };
    }
    print!("{}", output);
    s.to_string()
}

#[cfg(all(target_os = "windows", not(feature = "html_output")))]
pub fn styled_output<T>(s: &str, styles: T) -> String
where
    T: IntoIterator<Item = STL>,
{
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for style in styles {
        match style {
            STL::Red => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap(),
            STL::Green => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap(),
            STL::Blue => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                .unwrap(),
            STL::Yellow => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                .unwrap(),
            STL::White => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .unwrap(),
            STL::Black => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Black)))
                .unwrap(),
            STL::Cyan => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                .unwrap(),
            STL::Magenta => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                .unwrap(),
            STL::BrightRed => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true))
                .unwrap(),
            STL::BrightGreen => stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Green))
                        .set_intense(true),
                )
                .unwrap(),
            STL::BrightBlue => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_intense(true))
                .unwrap(),
            STL::BrightYellow => stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Yellow))
                        .set_intense(true),
                )
                .unwrap(),
            STL::BrightWhite => stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::White))
                        .set_intense(true),
                )
                .unwrap(),
            STL::BrightBlack => stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Black))
                        .set_intense(true),
                )
                .unwrap(),
            STL::BrightCyan => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_intense(true))
                .unwrap(),
            STL::BrightMagenta => stdout
                .set_color(
                    ColorSpec::new()
                        .set_fg(Some(Color::Magenta))
                        .set_intense(true),
                )
                .unwrap(),
            STL::OnRed => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Red)))
                .unwrap(),
            STL::OnGreen => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Green)))
                .unwrap(),
            STL::OnBlue => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Blue)))
                .unwrap(),
            STL::OnYellow => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Yellow)))
                .unwrap(),
            STL::OnWhite => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::White)))
                .unwrap(),
            STL::OnBlack => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Black)))
                .unwrap(),
            STL::OnCyan => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Cyan)))
                .unwrap(),
            STL::OnMagenta => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Magenta)))
                .unwrap(),
            STL::OnBrightRed => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Red)).set_intense(true))
                .unwrap(),
            STL::OnBrightGreen => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Green))
                        .set_intense(true),
                )
                .unwrap(),
            STL::OnBrightBlue => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Blue)).set_intense(true))
                .unwrap(),
            STL::OnBrightYellow => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Yellow))
                        .set_intense(true),
                )
                .unwrap(),
            STL::OnBrightWhite => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::White))
                        .set_intense(true),
                )
                .unwrap(),
            STL::OnBrightBlack => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Black))
                        .set_intense(true),
                )
                .unwrap(),
            STL::OnBrightCyan => stdout
                .set_color(ColorSpec::new().set_bg(Some(Color::Cyan)).set_intense(true))
                .unwrap(),
            STL::OnBrightMagenta => stdout
                .set_color(
                    ColorSpec::new()
                        .set_bg(Some(Color::Magenta))
                        .set_intense(true),
                )
                .unwrap(),
            STL::Normal => stdout.reset().unwrap(),
            STL::TrueColor(r, g, b) => stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r, g, b))))
                .unwrap(),
            STL::Bold => stdout.set_color(ColorSpec::new().set_bold(true)).unwrap(),
        }
    }
    print!("{}", s);
    stdout.reset().unwrap();
    s.to_string()
}

#[cfg(feature = "html_output")]
pub fn styled_output<T>(s: &str, styles: T) -> String
where
    T: IntoIterator<Item = STL>,
{
    struct Style {
        color: String,
        background: String,
        bold: bool,
    }

    let s = s
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace(' ', "&nbsp;")
        .replace('\r', "")
        .replace('\n', "<br>");

    let mut finstyle = Style {
        color: String::new(),
        background: String::new(),
        bold: false,
    };

    for style in styles {
        match style {
            STL::Red => finstyle.color = "red".to_string(),
            STL::Green => finstyle.color = "green".to_string(),
            STL::Blue => finstyle.color = "blue".to_string(),
            STL::Yellow => finstyle.color = "yellow".to_string(),
            STL::White => finstyle.color = "white".to_string(),
            STL::Black => finstyle.color = "black".to_string(),
            STL::Cyan => finstyle.color = "cyan".to_string(),
            STL::Magenta => finstyle.color = "magenta".to_string(),
            STL::BrightRed => finstyle.color = "red".to_string(),
            STL::BrightGreen => finstyle.color = "green".to_string(),
            STL::BrightBlue => finstyle.color = "blue".to_string(),
            STL::BrightYellow => finstyle.color = "yellow".to_string(),
            STL::BrightWhite => finstyle.color = "white".to_string(),
            STL::BrightBlack => finstyle.color = "black".to_string(),
            STL::BrightCyan => finstyle.color = "cyan".to_string(),
            STL::BrightMagenta => finstyle.color = "magenta".to_string(),
            STL::OnRed => finstyle.background = "red".to_string(),
            STL::OnGreen => finstyle.background = "green".to_string(),
            STL::OnBlue => finstyle.background = "blue".to_string(),
            STL::OnYellow => finstyle.background = "yellow".to_string(),
            STL::OnWhite => finstyle.background = "white".to_string(),
            STL::OnBlack => finstyle.background = "black".to_string(),
            STL::OnCyan => finstyle.background = "cyan".to_string(),
            STL::OnMagenta => finstyle.background = "magenta".to_string(),
            STL::OnBrightRed => finstyle.background = "red".to_string(),
            STL::OnBrightGreen => finstyle.background = "green".to_string(),
            STL::OnBrightBlue => finstyle.background = "blue".to_string(),
            STL::OnBrightYellow => finstyle.background = "yellow".to_string(),
            STL::OnBrightWhite => finstyle.background = "white".to_string(),
            STL::OnBrightBlack => finstyle.background = "black".to_string(),
            STL::OnBrightCyan => finstyle.background = "cyan".to_string(),
            STL::OnBrightMagenta => finstyle.background = "magenta".to_string(),
            STL::Normal => {
                finstyle.color = String::new();
                finstyle.background = String::new();
                finstyle.bold = false;
            }
            STL::TrueColor(r, g, b) => {
                finstyle.color = format!("rgb({},{},{})", r, g, b);
            }
            STL::Bold => finstyle.bold = true,
        }
    }
    let mut output = String::new();
    output.push_str("<span style=\"white-space:pre\">");
    if finstyle.bold {
        output.push_str("<b>");
    }
    if !finstyle.color.is_empty() {
        output.push_str(&format!("<span style=\"color:{}\">", finstyle.color));
    }
    if !finstyle.background.is_empty() {
        output.push_str(&format!(
            "<span style=\"background-color:{}\">",
            finstyle.background
        ));
    }
    output.push_str(&s);
    if !finstyle.background.is_empty() {
        output.push_str("</span>");
    }
    if !finstyle.color.is_empty() {
        output.push_str("</span>");
    }
    if finstyle.bold {
        output.push_str("</b>");
    }
    output.push_str("</span>");
    print!("{}", output);
    output
}
