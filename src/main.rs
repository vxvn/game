//     extern crate nats;
// use nats::*;

// let client = Client::new("nats://user:password@127.0.0.1").unwrap();

fn main() {}

// fn create_character(name: String) -> Unit {
//     let attrs = Attrs {
//          strength: 1,
//     constitution: 1,
//     agility: 1,
//     speed: 1,
//     endurance: 1,
//     willpower: 1,
//     concentration: 1,

//     };
//     return Unit{name:name, attrs:attrs,attacks:Vec::new()};

// }

fn read(msg: String) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}

struct Unit {
    name: String,
    attrs: Attrs,
    attacks: Vec<Attack>,
}

struct Attack {
    name: String,
    targets: i16,
    effects: Vec<AttackEffect>,
    team: i8,
    fatigue_cost: i8,
}

struct AttackEffect {
    damage_stat: Stat,
    restore_stat: Stat,
    base_power: i32,
    base_hit: i32,
    mod_power: Stat,
    mod_hit: Stat,
    range: Range,
}

enum Range {
    SLF = 0,
    FAR = 1,
    CLS = 2,
    GRP = 3,
}

enum Stat {
    STR = 0,
    CON = 1,
    AGI = 2,
    SPD = 3,
    END = 4,
    WIL = 5,
    FOC = 6,
    HP = 7,
    FAT = 8,
    VIT = 9,
}

struct Attrs {
    strength: i32,
    constitution: i32,
    agility: i32,
    speed: i32,
    endurance: i32,
    willpower: i32,
    focus: i32,
    vitality: i32,
}
