
// The structure here is as follows: There is a module for chess (I'm open to adding similar modules 
// for other games later) containing code that encodes the rules of chess. The module is divided into 
// a few parts. The abstracts submodule (AND SPECIFICALLY THAT MODULE) extends the ``no magic numbers" 
// principle to ``no magic types." Everything in it should be implemented at the trait level, using 
// traits to encode the condition that compliant types either contain or are equivalent to various 
// unoptimized helper types which correspond directly with the rules of the game as you'd explain them 
// to a human. This allows code like movegen and legality checking to be implemented polymorphically 
// over representations, including representations that support variants like 960 or double 960. 
// Keeping the abstracts section abstract is one of the goals of Cladonia, with the hope being that 
// this will keep things as flexible, modular, and extensible as possible. All optimizations improving 
// the default methods for specific types should take place in the second ``implementations" submodule. 
// I'd like to stress that the implementations submodule specifies the types actually used by the code- 
// everything with the helper types should be inlined out of existence, and mostly serve to make the 
// code more straightforward for me to read and write by allowing pattern matching et cetera. 
// Eventually the chess module will also contain chess-specific code for use by the search, such as 
// evaluations and move ordering. I would like to write the search to be polymorphic over games if 
// possible, and allow games to implement any game-specific optimizations for the search to plug into 
// itself and make use of. I would also like the search to be as polymorphic over evaluation spaces as 
// possible, perhaps only requiring them to implement get_minimum and negation (swap sides) functions 
// but including further optimizations when types also implement, say, the minimal window-querying 
// functionality for alpha-beta to function. 

pub(crate) mod chess;

pub(crate) mod game_data {
    // The search needs a generic way to interface with games. We need an analog of FENnec providing 
    // some sort of functionality that the search can use. 

    // Or perhaps the search only takes in an eval, and the eval provides the gamestate type the search 
    // will be using. I'll probably go with that: the search can require the eval to implement some 
    // additional functionality if it needs to constrain gamestates reps. Each impls submodule should 
    // specify what evals need to provide to be compatible with the gamestate representation they 
    // provide. Then evals can implement those various traits on their own time. 
}

pub(crate) mod search;

fn main() {
    use chess::{abstracts::{helper_traits::*, helper_types::*}, implementations::impls_vzero::*};

    let trying_startpos_perft = false;
    let first_test = false;
    let second_test = false;
    let third_test = false;
    let fourth_test = false;

    let testing_fen_builder = false;

    let kiwipete_string = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string();
    let trying_kiwipete_perft = false;
    let first_test_pos_two = false;
    let second_test_pos_two = false;
    let third_test_pos_two = false;

    let pos_3_string = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string();
    let trying_pos_3_perft = true;

    let pos_4_string = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string();
    let trying_pos_4_perft = true;

    let pos_5_string = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string();
    let trying_pos_5_perft = true;

    let pos_6_string = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string();
    let trying_pos_6_perft = true;

    if trying_startpos_perft {
        println!("Perft from STARTPOS:");
        let (total_num, sub_perfts) = depth_n_better_perft(STARTPOS, 4);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
    
        // Initial problem was that en passant squares were being created on the wrong side of the board. 
        // Thus A2A4 could be followed up by B7A6, taking on the now-empty A2 square.
    } else if first_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::A), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::A),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::A), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::A),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }

        // Problem was that en passant squares were sticking around for more than one ply, so after A2A4 and A7A6, 
        // white could capture their own pawn via en passant per B2A3. The ep-resetting thing was missing from 
        // the StandardMove branch.
    } else if second_test {
        let mut curr_pos = STARTPOS;
        println!("Startpos initialized");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C),
                }
            )
        );
        println!("First move made, B1C3");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::E),
                }
            )
        );
        println!("Second move made, E7E6");
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::D),
                }
            )
        );
        println!("Third move made, C3D5");
        println!("Perft from this, the offending position:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
        println!("Legality check: {}", curr_pos.check_remaining_legality(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E),
                }
            )
        ));
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::E),
                }
            )
        );
        println!("Fourth (problem) move made.");
        println!("Black king square: {}", standardize(curr_pos.get_b_king_square()).to_string())
        // The extra problem move is E8E7, which would be the black king going into check.

        // The reason for this problem was that I was checking for attacking knights at a 
        // king's move away from the target square, not a knight's move away.
    } else if third_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::C), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::C),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::D), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::A),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }
        
        // In this position, the pawn on B5 flasely claims to have no legal moves. Presumably this is a bug both: 
        // 1) with pin-checking logic, where the pin-checker misses the D7 pawn,
        // 2) with pin-checking logic, where pieces are falsely disallowed from taking the attackers that put them into pins.

        // Fixed: stopped trying to do fancy stuff with pins, the logic was hard to work out. 
        // That's an optimization for later, and should be tested in case cache witchcraft makes it slow.
        // Perft now correct to depth 4.
    } else if fourth_test {
        let mut curr_pos = STARTPOS;
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::D), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::D),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::E), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::D),
                }
            )
        );
        curr_pos.make_move(
            ChessMove::StandardMove(
                StandardMove {
                    from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::B), 
                    to_square: <i8 as Squarey>::build_square(EnumRank::Four, EnumFile::B),
                }
            )
        );
        println!("Perft from offender:");
        let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
        println!("Total: {}", total_num);
        for (move_made, successors_num) in sub_perfts {
            println!("{0} - {1}", move_made, successors_num)
        }

        // The problem is D2C3 in this position.

        // The function sees_obvious_attack was checking for opponent pawns a *forward* opponent-pawn-attacking-move away
        // from the square being checked. It now checks for opponent pawns *backwards* from the square being queried, so
        // that detected pawns will actually be threatening the relevant square rather than one two squares behind it.

        // Perft totals are now correct to depth 5. I haven't checked each of the 20 start move totals, but the first several
        // at least should still be correct from when they were checked to find the line in this test.
    } else if testing_fen_builder {
        println!("Hopefully the parser correctly interprets the startpos fen... Here it is: \n{:?}", interpret_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()));
        println!("Startpos for comparison: \nOk({:?})", STARTPOS);
    } 
    if trying_kiwipete_perft {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let (total_num, sub_perfts) = depth_n_better_perft(kiwipete, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }
    } else if first_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::A), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::One, EnumFile::B),
                        }
                    )
                );
                println!("First move made, A1B1.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::C),
                        }
                    )
                );
                println!("Third move made, E5C6.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                println!("Checking whether the problem is with legality checks: {}", !curr_pos.check_remaining_legality(ChessMove::PromotionMove(PromotionMove {
                    from_square: i8::build_square(EnumRank::Two, EnumFile::G),
                    to_square: i8::build_square(EnumRank::One, EnumFile::G),
                    promotion_choice: i8::build_piece(EnumColor::Black, EnumPiecesUncolored::Knight)
                })));

                for constructed_move in curr_pos.get_pseudo_legal_proper_moves() {
                    println!("Detected move {}", constructed_move);
                }

                // The G2G1 promotion moves are not detected.
                // ... But G2G1 (NOT promotion) IS constructed by pseudolegal movegen.
            },
        }
    } else if second_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::G),
                        }
                    )
                );
                println!("First move made, E5G6.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Six, EnumFile::G), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::H),
                        }
                    )
                );
                println!("Third move made, G6H8.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                // Problem is that Cladonia thinks that black can legally castle kingside in this position.
                // The knight has taken the rook, so this should be impossible, but castling removal doesn't
                // trigger on the taken squares as-is. 

                // Added in castling removal from the target (``to") squares of standard and promotion moves.
            }
        }
    } else if third_test_pos_two {
        println!("Perft from Kiwipete:");
        let try_kiwipete_pos = interpret_fen(kiwipete_string);
        match try_kiwipete_pos {
            Err(some_error) => println!("Error with parsing Kiwipete: {}", some_error),
            Ok(kiwipete) => {
                let mut curr_pos = kiwipete;
                println!("Kiwipete initialized.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Five, EnumFile::E), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::F),
                        }
                    )
                );
                println!("First move made, E5F7.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Three, EnumFile::H), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Two, EnumFile::G),
                        }
                    )
                );
                println!("Second move made, H3G2.");
                curr_pos.make_move(
                    ChessMove::StandardMove(
                        StandardMove {
                            from_square: <i8 as Squarey>::build_square(EnumRank::Seven, EnumFile::F), 
                            to_square: <i8 as Squarey>::build_square(EnumRank::Eight, EnumFile::H),
                        }
                    )
                );
                println!("Second move made, F7H8.");
                println!("Perft from offender:");
                let (total_num, sub_perfts) = depth_n_better_perft(curr_pos, 1);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }

                // Problem is that Cladonia thinks that black can legally castle kingside in this position. Again.
                // I thought I just fixed this?

                // Oh. I removed the castling for the wrong color. Here comes another helper function.

                // Fixed. Kiwipete perft total is now correct to depth four. Guess I just didn't check whether I'd actually 
                // fixed the offending position with the last change.
            }
        }
    }
    if trying_pos_3_perft {
        println!("Perft from Position 3:");
        let try_get_pos = interpret_fen(pos_3_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 3: {}", some_error),
            Ok(pos_3) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_3, 5);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // Initially didn't parse because castling rules handler didn't know that '-' 
        // in the castling spot should be interpreted as ``no castling."

        // After fixing the parser, perft is correct to depth 4 right off the bat.
    }
    if trying_pos_4_perft {
        println!("Perft from Position 4:");
        let try_get_pos = interpret_fen(pos_4_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 4: {}", some_error),
            Ok(pos_4) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_4, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // Correct from the get-go to depth 4.
    }
    if trying_pos_5_perft {
        println!("Perft from Position 5:");
        let try_get_pos = interpret_fen(pos_5_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 5: {}", some_error),
            Ok(pos_5) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_5, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // No issues at depth 4.
    }
    if trying_pos_6_perft {
        println!("Perft from Position 6:");
        let try_get_pos = interpret_fen(pos_6_string);
        match try_get_pos {
            Err(some_error) => println!("Error with parsing Position 6: {}", some_error),
            Ok(pos_6) => {
                let (total_num, sub_perfts) = depth_n_better_perft(pos_6, 4);
                println!("Total: {}", total_num);
                for (move_made, successors_num) in sub_perfts {
                    println!("{0} - {1}", move_made, successors_num)
                }
            }
        }

        // No issues at depth 4 :).
    }

    // Perft for all tested positions gives correct totals up to depth 4, and startpos is correct to depth 5. 
}
