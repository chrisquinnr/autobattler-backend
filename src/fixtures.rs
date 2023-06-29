use crate::battle::{Character, SpecialMove, Stats};
use rand::Rng;

fn get_random_special_move() -> Option<SpecialMove> {
    let mut rng = rand::thread_rng();
    let move_type = rng.gen_range(1..=3);

    match move_type {
        1 => Some(SpecialMove::Revival),
        2 => Some(SpecialMove::GigaAttack),
        3 => Some(SpecialMove::Restore),
        _ => None,
    }
}

pub fn get_opposition_fixtures() -> Vec<Character> {
    let opposition = vec![
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 1".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 2".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 3".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "BOSS".to_string(),
            stats: Stats {
                str: 50,
                def: 1,
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
            src: "./assets/pirate.png".to_string(),
            name: "Pirate".to_string(),
            stats: Stats {
                str: 7,
                def: 2,
                hp: 20,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/bird.png".to_string(),
            name: "Birb".to_string(),
            stats: Stats {
                str: 7,
                def: 2,
                hp: 100,
            },
            special_move: get_random_special_move(),
        },
        Character {
            src: "./assets/outlaw.png".to_string(),
            name: "Smorkle".to_string(),
            stats: Stats {
                str: 6,
                def: 1,
                hp: 200,
            },
            special_move: get_random_special_move(),
        },
    ];
    return characters;
}
