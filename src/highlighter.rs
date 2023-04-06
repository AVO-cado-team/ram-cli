use crate::colorize::styled_output;
use crate::colorize::STL;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PotentialProblem {
    Head,
    Tail,
    Unkn,
    DistplayOnly,
}

enum ParseState {
    BeforeHead,
    Head,
    AfterHead,

    Tail,
    TailLabel,
    TailDirect,
    TailIndirect,
    TailPure,

    AfterTail,
    Comment,
}

pub fn code_highlighter(line: usize, source: &str, pp: PotentialProblem) -> String {
    let mut result = String::new();

    let mut state = ParseState::BeforeHead;
    let binding = source.replace('\t', "    ").replace('\r', "");
    let mut source = binding.as_str();

    match pp {
        PotentialProblem::Head | PotentialProblem::Tail | PotentialProblem::Unkn => {
            result += &styled_output(&line.to_string(), [STL::Red])
        }
        PotentialProblem::DistplayOnly => {
            result += &styled_output(&line.to_string(), [STL::BrightCyan])
        }
    };
    result += &styled_output("\t", [STL::Normal]);
    result += &styled_output("| ", [STL::BrightCyan]);

    while !source.is_empty() {
        let c = &source[0..1];

        (state, source) = match state {
            ParseState::BeforeHead => match c {
                " " => {
                    result += &styled_output(c, [STL::Normal]);
                    (ParseState::BeforeHead, &source[1..])
                }
                "#" => (ParseState::Comment, source),
                _ => (ParseState::Head, source),
            },
            ParseState::Head => match c {
                " " => (ParseState::AfterHead, source),
                "#" => (ParseState::Comment, source),
                _ => {
                    let is_label = if let Some(label_identificator_index) = source.find(':') {
                        let comment_index = if let Some(comment_index) = source.find('#') {
                            comment_index
                        } else {
                            source.len()
                        };
                        label_identificator_index <= comment_index
                    } else {
                        false
                    };
                    match pp {
                        PotentialProblem::Head => result += &styled_output(c, [STL::OnBrightRed]),
                        _ => {
                            if is_label {
                                result += &styled_output(c, [STL::BrightYellow])
                            } else {
                                result += &styled_output(c, [STL::Red])
                            }
                        }
                    };
                    (ParseState::Head, &source[1..])
                }
            },
            ParseState::AfterHead => match c {
                " " => {
                    result += &styled_output(c, [STL::Normal]);
                    (ParseState::AfterHead, &source[1..])
                }
                "#" => (ParseState::Comment, source),
                _ => (ParseState::Tail, source),
            },
            ParseState::Tail => match c {
                " " => (ParseState::AfterTail, source),
                "#" => (ParseState::Comment, source),
                _ => match pp {
                    PotentialProblem::Tail => {
                        result += &styled_output(c, [STL::OnRed]);
                        (ParseState::Tail, &source[1..])
                    }
                    _ => {
                        if c.parse::<i8>().ok().is_some() {
                            (ParseState::TailIndirect, source)
                        } else if c == "=" {
                            result += &styled_output(c, [STL::BrightBlue]);
                            (ParseState::TailPure, &source[1..])
                        } else if c == "*" {
                            result += &styled_output(c, [STL::BrightRed]);
                            (ParseState::TailDirect, &source[1..])
                        } else {
                            (ParseState::TailLabel, source)
                        }
                    }
                },
            },
            ParseState::TailLabel => match c {
                " " => (ParseState::AfterTail, source),
                "#" => (ParseState::Comment, source),
                _ => {
                    result += &styled_output(c, [STL::BrightYellow]);
                    (ParseState::TailLabel, &source[1..])
                }
            },
            ParseState::TailDirect => match c {
                " " => (ParseState::AfterTail, source),
                "#" => (ParseState::Comment, source),
                _ => {
                    result += &styled_output(c, [STL::White]);
                    (ParseState::TailDirect, &source[1..])
                }
            },
            ParseState::TailIndirect => match c {
                " " => (ParseState::AfterTail, source),
                "#" => (ParseState::Comment, source),
                _ => {
                    result += &styled_output(c, [STL::White]);
                    (ParseState::TailIndirect, &source[1..])
                }
            },
            ParseState::TailPure => match c {
                " " => (ParseState::AfterTail, source),
                "#" => (ParseState::Comment, source),
                _ => {
                    result += &styled_output(c, [STL::BrightBlue]);
                    (ParseState::TailPure, &source[1..])
                }
            },
            ParseState::AfterTail => match c {
                "#" => (ParseState::Comment, source),
                _ => {
                    result += &styled_output(c, [STL::Normal]);
                    (ParseState::AfterTail, &source[1..])
                }
            },
            ParseState::Comment => {
                result += &styled_output(c, [STL::TrueColor(128, 128, 128)]);
                (ParseState::Comment, &source[1..])
            }
        };
    }
    result += &styled_output("\n", [STL::Normal]);
    result
}
