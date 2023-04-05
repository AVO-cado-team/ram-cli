use crate::colorize::styled_output;
use crate::colorize::STL;

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

pub fn code_highlighter(line: usize, source: &str, pp: PotentialProblem) {
    let mut state = ParseState::BeforeHead;
    let mut i = 0;
    let source = source.replace('\t', "    ").replace('\r', "");
    match pp {
        PotentialProblem::Head | PotentialProblem::Tail | PotentialProblem::Unkn => {
            styled_output(&line.to_string(), vec![STL::Red])
        }
        PotentialProblem::DistplayOnly => styled_output(&line.to_string(), vec![STL::BrightCyan]),
    };
    styled_output("\t", vec![STL::Normal]);
    styled_output("| ", vec![STL::BrightCyan]);

    while i < source.len() {
        let c = source.chars().nth(i).unwrap();
        (state, i) = match state {
            ParseState::BeforeHead => match c {
                ' ' => {
                    styled_output(&c.to_string(), vec![STL::Normal]);
                    (ParseState::BeforeHead, i + 1)
                }
                '#' => (ParseState::Comment, i),
                _ => (ParseState::Head, i),
            },
            ParseState::Head => match c {
                ' ' => (ParseState::AfterHead, i),
                '#' => (ParseState::Comment, i),
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
                        PotentialProblem::Head => {
                            styled_output(&c.to_string(), vec![STL::OnBrightRed])
                        }
                        _ => {
                            if is_label {
                                styled_output(&c.to_string(), vec![STL::BrightYellow])
                            } else {
                                styled_output(&c.to_string(), vec![STL::Red])
                            }
                        }
                    };
                    (ParseState::Head, i + 1)
                }
            },
            ParseState::AfterHead => match c {
                ' ' => {
                    styled_output(&c.to_string(), vec![STL::Normal]);
                    (ParseState::AfterHead, i + 1)
                }
                '#' => (ParseState::Comment, i),
                _ => (ParseState::Tail, i),
            },
            ParseState::Tail => match c {
                ' ' => (ParseState::AfterTail, i),
                '#' => (ParseState::Comment, i),
                _ => match pp {
                    PotentialProblem::Tail => {
                        styled_output(&c.to_string(), vec![STL::OnRed]);
                        (ParseState::Tail, i + 1)
                    }
                    _ => {
                        if c.is_ascii_digit() {
                            (ParseState::TailIndirect, i)
                        } else if c == '=' {
                            styled_output(&c.to_string(), vec![STL::BrightBlue]);
                            (ParseState::TailPure, i + 1)
                        } else if c == '*' {
                            styled_output(&c.to_string(), vec![STL::BrightRed]);
                            (ParseState::TailDirect, i + 1)
                        } else {
                            (ParseState::TailLabel, i)
                        }
                    }
                },
            },
            ParseState::TailLabel => match c {
                ' ' => (ParseState::AfterTail, i),
                '#' => (ParseState::Comment, i),
                _ => {
                    styled_output(&c.to_string(), vec![STL::BrightYellow]);
                    (ParseState::TailLabel, i + 1)
                }
            },
            ParseState::TailDirect => match c {
                ' ' => (ParseState::AfterTail, i),
                '#' => (ParseState::Comment, i),
                _ => {
                    styled_output(&c.to_string(), vec![STL::Normal]);
                    (ParseState::TailDirect, i + 1)
                }
            },
            ParseState::TailIndirect => match c {
                ' ' => (ParseState::AfterTail, i),
                '#' => (ParseState::Comment, i),
                _ => {
                    styled_output(&c.to_string(), vec![STL::Normal]);
                    (ParseState::TailIndirect, i + 1)
                }
            },
            ParseState::TailPure => match c {
                ' ' => (ParseState::AfterTail, i),
                '#' => (ParseState::Comment, i),
                _ => {
                    styled_output(&c.to_string(), vec![STL::BrightBlue]);
                    (ParseState::TailPure, i + 1)
                }
            },
            ParseState::AfterTail => match c {
                '#' => (ParseState::Comment, i),
                _ => {
                    styled_output(&c.to_string(), vec![STL::Normal]);
                    (ParseState::AfterTail, i + 1)
                }
            },
            ParseState::Comment => {
                styled_output(&c.to_string(), vec![STL::TrueColor(128, 128, 128)]);
                (ParseState::Comment, i + 1)
            }
        };
    }
    styled_output("\n", vec![STL::Normal]);
}
