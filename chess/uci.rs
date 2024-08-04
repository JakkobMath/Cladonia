
#![allow(dead_code)]
use std::{str::FromStr, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Uci {} // Dummy type so I can do stuff like `Uci::parse_command`. 

#[derive(Debug, Clone, Copy, Default)]
enum UciState {
    #[default]
    Uci,
}

#[derive(Debug, Clone)]
enum UciRegisterOption {
    Later,
    Name(Box<str>),
    Code(Box<str>),
}

#[derive(Debug, Clone, Default)]
enum UciPositionOption {
    #[default]
    StartPos,
    FEN(Box<str>),
}

#[derive(Debug, Clone, Copy)]
enum UciMove {
    NullMove,
    StdMove(char,char,char,char),
    Promotion(char,char,char,char,char),
}

impl FromStr for UciMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0000" {return Ok(UciMove::NullMove)};
        let mut char0 = 'z';
        let mut char1 = 'z';
        let mut char2 = 'z';
        let mut char3 = 'z';
        let mut char4 = 'z';
        while let Some((place, char)) = s.char_indices().next() {
            match place {
                0 => match 'a' <= char && char <= 'h' {
                    true => char0 = char,
                    false => return Err(()),
                },
                1 => match '1' <= char && char <= '8' {
                    true => char1 = char,
                    false => return Err(()),
                },
                2 => match 'a' <= char && char <= 'h' {
                    true => char2 = char,
                    false => return Err(()),
                },
                3 => match '1' <= char && char <= '8' {
                    true => char3 = char,
                    false => return Err(()),
                },
                4 => match char {
                    'n' | 'b' | 'r' | 'q' => char4 = char,
                    _ => return Err(()),
                },
                _ => return Err(()),
            }
        }
        if char3 == 'z' {return Err(());};
        match char4 {
            'z' => return Ok(UciMove::StdMove(char0, char1, char2, char3)),
            _ => return Ok(UciMove::Promotion(char0, char1, char2, char3, char4))
        };
    }
}

#[derive(Debug, Clone, Default)]
enum UciSearchMoveSetting {
    #[default]
    None,
    SearchMove(Box<Vec<UciMove>>),
}

#[derive(Debug, Clone, Copy, Default)]
enum UciSearchMode {
    Ponder,
    #[default]
    NotPonder, // I guess?
}

#[derive(Debug, Clone, Copy, Default)]
struct UciTimeInfo {
    white_time: Option<Duration>,
    black_time: Option<Duration>,
    white_increment: Option<Duration>,
    black_increment: Option<Duration>,
    moves_to_go: Option<i8>,
}

#[derive(Debug, Clone, Copy, Default)]
struct UciSearchLimiter {
    depth: Option<u16>,
    nodes: Option<u128>, // LOL
    mate: Option<u16>,
    time: Option<Duration>,
}

#[derive(Debug, Clone, Default)]
struct UciGoSettings{
    moves_to_search: UciSearchMoveSetting,
    search_mode: UciSearchMode,
    time_info: UciTimeInfo,
    search_limiter: UciSearchLimiter, // All fields will be `None` if "infinite" search is selected. 
}

#[derive(Debug, Clone)]
enum UciGuiCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(Box<str>,Box<str>),
    Register(UciRegisterOption),
    NewGame,
    Position(UciPositionOption,Box<Vec<UciMove>>),
    Go(UciGoSettings),
    Stop,
    PonderHit,
    Quit,
}

trait CommunicationProtocol {
    type ProtocolState;
    type ProtocolGUICommands: std::str::FromStr;
    fn parse_command(input: &str) -> Option<Self::ProtocolGUICommands>{
        return match input.parse::<Self::ProtocolGUICommands>() {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

impl FromStr for UciGuiCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace().peekable();
        while let Some(word) = words.next() {
            match word {
                "uci" => return Ok(UciGuiCommand::Uci),
                "debug" => while let Some(debug_setting) = words.next() {
                    match debug_setting {
                        "on" => return Ok(UciGuiCommand::Debug(true)),
                        "off" => return Ok(UciGuiCommand::Debug(false)),
                        _ => {},
                    }
                },
                "isready" => return Ok(UciGuiCommand::IsReady),
                "setoption" => {
                    let mut option_name = String::from("");
                    let mut option_value = String::from("");
                    while let Some(name_command) = words.next() {
                        match name_command {
                            "name" => {
                                while let Some(name_word) = words.next() {
                                    match name_word {
                                        "value" => {
                                            while let Some(value_word) = words.next() {
                                                option_value += value_word;
                                                option_value += " ";
                                            }
                                        },
                                        _ => {
                                            option_name += name_word;
                                            option_name += " ";
                                        },
                                    }
                                }
                            },
                            _ => {}, // They should have sent a name command, this is another unknown token to be ignored. 
                        }
                    }
                    return Ok(UciGuiCommand::SetOption(option_name.trim().to_string().into_boxed_str(), option_value.trim().to_string().into_boxed_str()));
                },
                "register" => while let Some(registration_type) = words.next () {
                    match registration_type {
                        "later" => return Ok(UciGuiCommand::Register(UciRegisterOption::Later)),
                        "name" => {
                            let mut register_name = String::from("");
                            while let Some(name_word) = words.next() {
                                register_name += name_word;
                                register_name += " ";
                            }
                            return Ok(UciGuiCommand::Register(UciRegisterOption::Name(register_name.trim().to_string().into_boxed_str())));
                        },
                        "code" => {
                            let mut register_code = String::from("");
                            while let Some(code_word) = words.next() {
                                register_code += code_word;
                                register_code += " ";
                            }
                            return Ok(UciGuiCommand::Register(UciRegisterOption::Name(register_code.trim().to_string().into_boxed_str())));
                        },
                        _ => {},
                    }
                },
                "ucinewgame" => return Ok(UciGuiCommand::NewGame),
                "position" => todo!(),
                "go" => {
                    let mut go_settings = UciGoSettings::default();
                    while let Some(&position_command) = words.peek() {
                        match position_command {
                            "searchmoves" => {
                                words.next(); // Move off of "searchmoves" and onto the next piece of data
                                let mut moves_to_search = Vec::new();
                                while let Some(&search_move) = words.peek() {
                                    match search_move {
                                        // If the next thing is a valid command, break without advancing. This will resume the outer loop and handle the command appropriately. 
                                        "searchmoves" | "ponder" | "wtime" | "btime" | "winc" | "binc" | "movestogo" | "depth" | "nodes" | "mate" | "movetime" | "infinite" => break,
                                        move_string => if let Ok(search_move) = UciMove::from_str(move_string) {
                                            moves_to_search.push(search_move);
                                        },
                                    }
                                    words.next();
                                };
                                go_settings.moves_to_search = UciSearchMoveSetting::SearchMove(Box::new(moves_to_search));
                            },
                            "ponder" => {
                                words.next(); // Move off of "ponder" and onto the next command for the outer loop
                                go_settings.search_mode = UciSearchMode::Ponder;
                            },
                            "wtime" => {
                                words.next();
                                if let Some(msecs_string) = words.next() {
                                    if let Ok(msecs_left) = u64::from_str(msecs_string) {
                                        go_settings.time_info.white_time = Some(Duration::from_millis(msecs_left))
                                    }
                                }
                            },
                            "btime" => {
                                words.next();
                                if let Some(msecs_string) = words.next() {
                                    if let Ok(msecs_left) = u64::from_str(msecs_string) {
                                        go_settings.time_info.black_time = Some(Duration::from_millis(msecs_left))
                                    }
                                }
                            },
                            "winc" => {
                                words.next();
                                if let Some(msecs_string) = words.next() {
                                    if let Ok(msecs_left) = u64::from_str(msecs_string) {
                                        go_settings.time_info.white_increment = Some(Duration::from_millis(msecs_left))
                                    }
                                }
                            },
                            "binc" => {
                                words.next();
                                if let Some(msecs_string) = words.next() {
                                    if let Ok(msecs_left) = u64::from_str(msecs_string) {
                                        go_settings.time_info.black_increment = Some(Duration::from_millis(msecs_left))
                                    }
                                }
                            },
                            "movestogo" => {
                                words.next();
                                if let Some(moves_string) = words.next() {
                                    if let Ok(moves_to_go) = i8::from_str(moves_string) {
                                        go_settings.time_info.moves_to_go = Some(moves_to_go);
                                    }
                                }
                            },
                            "depth" => {
                                words.next();
                                if let Some(depth_string) = words.next() {
                                    if let Ok(depth) = u16::from_str(depth_string) {
                                        go_settings.search_limiter.depth = Some(depth);
                                    }
                                }
                            },
                            "nodes" => {
                                words.next();
                                if let Some(nodes_string) = words.next() {
                                    if let Ok(nodes) = u128::from_str(nodes_string) {
                                        go_settings.search_limiter.nodes = Some(nodes);
                                    }
                                }
                            },
                            "mate" => {
                                words.next();
                                if let Some(mate_depth_string) = words.next() {
                                    if let Ok(mate_depth) = u16::from_str(mate_depth_string) {
                                        go_settings.search_limiter.mate = Some(mate_depth);
                                    }
                                }
                            },
                            "movetime" => {
                                words.next();
                                if let Some(move_time_string) = words.next() {
                                    if let Ok(ms_to_search) = u64::from_str(move_time_string) {
                                        go_settings.search_limiter.time = Some(Duration::from_millis(ms_to_search));
                                    }
                                }
                            },
                            "infinite" => go_settings.search_limiter = UciSearchLimiter::default(),
                            _ => _ = words.next(),
                        }
                    }
                    return Ok(UciGuiCommand::Go(go_settings));
                },
                "stop" => return Ok(UciGuiCommand::Stop),
                "ponderhit" => return Ok(UciGuiCommand::PonderHit),
                "quit" => return Ok(UciGuiCommand::Quit),
                _ => {}, // Ignore unknown tokens. 
            }
        }
        return Err(()); // Helpful and descriptive. 
    }
}

impl CommunicationProtocol for Uci {
    type ProtocolState = UciState;
    type ProtocolGUICommands = UciGuiCommand;
}