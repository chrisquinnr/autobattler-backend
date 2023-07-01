use crate::battle::{Character, SpecialMove, Stats};
use rand::Rng;

fn get_random_special_move() -> Option<SpecialMove> {
    let mut rng = rand::thread_rng();
    let move_type = rng.gen_range(1..=3);

    match move_type {
        1 => Some(SpecialMove::GigaAttack),
        2 => Some(SpecialMove::Heal),
        3 => None,
        // 3 => Some(SpecialMove::Revival),
        _ => None,
    }
}

pub fn get_opposition_fixtures() -> Vec<Character> {
    let opposition = vec![
        Character {
            src: "./assets/blob.png".to_string(),
            name: "Goon 1".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
            special_move: None,
        },
        Character {
            src: "./assets/blob.png".to_string(),
            name: "Goon 2".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
            special_move: None,
        },
        Character {
            src: "./assets/demon.png".to_string(),
            name: "Demon".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 6,
                def: 2,
                hp: 20,
            },
            special_move: Some(SpecialMove::Heal),
        },
        Character {
            src: "./assets/boss.png".to_string(),
            name: "Overlord".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 20,
                def: 2,
                hp: 100,
            },
            special_move: Some(SpecialMove::GigaAttack),
        },
    ];

    // let mut rng = rand::thread_rng();
    // opposition.shuffle(&mut rng);

    // let random_subset_size = rng.gen_range(3..=opposition.len() - 1);
    // opposition.truncate(random_subset_size);

    return opposition;
}

pub fn get_team_fixtures() -> Vec<Character> {
    let characters = vec![
        Character {
            src: "./assets/angel.png".to_string(),
            name: "Angel".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 7,
                def: 8,
                hp: 180,
            },
            special_move: Some(SpecialMove::Heal),
        },
        Character {
            src: "./assets/warlock.png".to_string(),
            name: "Warlock".to_string(),
            character_type: "none".to_string(),
            stats: Stats {
                str: 7,
                def: 4,
                hp: 100,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/knight/knight-idle.gif".to_string(),
            name: "Knight".to_string(),
            character_type: "knight".to_string(),
            stats: Stats {
                str: 6,
                def: 4,
                hp: 200,
            },
            special_move: Some(SpecialMove::GigaAttack),
        },
    ];
    return characters;
}
