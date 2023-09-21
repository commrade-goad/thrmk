use std::io::{self, BufRead, Write};

const HUMAN: usize = 4;
const CANNIBAL: usize = 4;
const DIRT_SIZE: usize = (HUMAN + CANNIBAL)*2;

#[derive(Debug)]
struct GameCommand {
    pos: Option<BoatStatus>,
    ent: Option<EntityType>,
    len: Option<usize>,
}

#[derive(Debug, PartialEq, Clone)]
enum EntityType {
    Human,
    Cannibal,
}

#[derive(Debug, Clone)]
enum BoatStatus {
    Right,
    Left,
}

fn user_input(promt: &str) -> String {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle_stdin = stdin.lock();
    let mut handle_stdout = stdout.lock();
    print!("{}", promt);
    let _ = handle_stdout.flush();
    handle_stdin.read_line(&mut buffer).unwrap();
    return buffer;
}

fn print_state(entities_a: &Vec<EntityType>, entities_b: &Vec<EntityType>, boat: &BoatStatus) {
    print!("\n");
    let padding_size: usize = DIRT_SIZE + 20 - entities_a.len();
    for entity in entities_a {
        let char_to_print: &str = match entity {
            EntityType::Human => "H",
            EntityType::Cannibal => "C",
        };
        print!("{}", char_to_print);
    }
    for _ in 0..padding_size {
        print!(" ");
    }
    for entity in entities_b {
        let char_to_print: &str = match entity {
            EntityType::Human => "H",
            EntityType::Cannibal => "C",
        };
        print!("{}", char_to_print);
    }
    print!("\n");
    for _ in 0..DIRT_SIZE {
        print!("#");
    }
    match boat {
        BoatStatus::Left => print!(".\\__/..............."),
        BoatStatus::Right => print!("...............\\__/."),
    }
    for _ in 0..DIRT_SIZE {
        print!("#");
    }
    print!("\n");
}

fn parse_input(input: &Vec<&str>, boat: &BoatStatus) -> Option<Vec<GameCommand>> {
    let mut result: Vec<GameCommand> = Vec::new();
    let mut len_counter: usize = 0;
    for inp in input {
        let current_inp: &str = inp;
        let current_inp_vec: Vec<char> = current_inp.chars().collect();
        for idx in 0..current_inp_vec.len() {
            let current_char = current_inp_vec[idx];
            if current_inp_vec[idx].is_numeric() {
                let lenght: usize = current_char.to_digit(10).unwrap() as usize;
                len_counter += lenght;
                let boat_cpy: BoatStatus;
                let mut entity_cpy: Option<EntityType> = None;
                match &boat {
                    &BoatStatus::Left => boat_cpy = BoatStatus::Right,
                    &BoatStatus::Right => boat_cpy = BoatStatus::Left,
                }
                if idx + 1 != current_inp_vec.len() {
                    match current_inp_vec[idx + 1] {
                        'H' | 'h' => entity_cpy = Some(EntityType::Human),
                        'C' | 'c' => entity_cpy = Some(EntityType::Cannibal),
                        _ => return None,
                    }
                }
                result.push(GameCommand {
                    pos: Some(boat_cpy),
                    ent: entity_cpy,
                    len: Some(lenght),
                });
            }
        }
    }
    if len_counter > 2 {
        return None;
    }
    return Some(result);
}

fn move_entity(
    from: &mut Vec<EntityType>,
    to: &mut Vec<EntityType>,
    entity: EntityType,
    n: usize,
) -> bool {
    if n == 0 {
        return false;
    }
    for _ in 0..n {
        if let Some(i) = from.iter().position(|e| *e == entity) {
            to.push(entity.clone());
            from.remove(i);
        } else {
            break;
        }
    }
    return true;
}

fn rule_check(data: &Vec<EntityType>) -> bool {
    let mut h: usize = 0;
    let mut c: usize = 0;
    for entity in data {
        match entity {
            EntityType::Human => h += 1,
            EntityType::Cannibal => c += 1,
        }
    }
    if c > 0 && c > h && h > 0 {
        return false;
    }
    return true;
}

fn populate(c_n: usize, h_n: usize) -> Vec<EntityType> {
    let mut result: Vec<EntityType> = Vec::new();
    for _ in 0..c_n {
        result.push(EntityType::Cannibal);
    }
    for _ in 0..h_n {
        result.push(EntityType::Human);
    }
    return result;
}

fn main() {
    let mut left: Vec<EntityType> = populate(CANNIBAL, HUMAN);
    let mut right: Vec<EntityType> = Vec::new();
    let mut boat: BoatStatus = BoatStatus::Left;
    print_state(&left, &right, &boat);
    loop {
        if rule_check(&left) == false || rule_check(&right) == false {
            println!("LOSE");
            break;
        };
        let inp: String = user_input("Move : ").trim().to_string();
        let inp_vec: Vec<&str> = inp.split_whitespace().collect();
        let parsed: Vec<GameCommand> = match parse_input(&inp_vec, &boat) {
            None => {
                println!("Invalid");
                continue;
            }
            val => val.unwrap(),
        };
        let mut status: bool = false;
        for command in &parsed {
            status = false;
            if command.ent != None {
                match command.pos {
                    Some(BoatStatus::Right) => {
                        status = move_entity(
                            &mut left,
                            &mut right,
                            command.ent.clone().unwrap(),
                            command.len.unwrap(),
                        )
                    }
                    Some(BoatStatus::Left) => {
                        status = move_entity(
                            &mut right,
                            &mut left,
                            command.ent.clone().unwrap(),
                            command.len.unwrap(),
                        )
                    }
                    None => {}
                }
            }
        }
        if status == true {
            boat = parsed[0].pos.clone().unwrap();
        }
        print_state(&left, &right, &boat);
    }
}
