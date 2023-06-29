use crate::battle::{Character, Stats};
use rand::{seq::SliceRandom, Rng};

pub fn get_opposition_fixtures() -> Vec<Character> {
    let mut opposition = vec![
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 1".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 2".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 3".to_string(),
            stats: Stats {
                str: 5,
                def: 1,
                hp: 10,
            },
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 4".to_string(),
            stats: Stats {
                str: 1,
                def: 1,
                hp: 20,
            },
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "Blob 5".to_string(),
            stats: Stats {
                str: 2,
                def: 3,
                hp: 50,
            },
        },
        Character {
            src: "./assets/goo.png".to_string(),
            name: "BOSS".to_string(),
            stats: Stats {
                str: 50,
                def: 1,
                hp: 100,
            },
        },
    ];

    let mut rng = rand::thread_rng();
    opposition.shuffle(&mut rng);

    let random_subset_size = rng.gen_range(3..=opposition.len() - 1);
    opposition.truncate(random_subset_size);

    return opposition;
}

pub fn get_team_fixtures() -> Vec<Character> {
    let characters = vec![
        Character {
            src: "./assets/pirate.png".to_string(),
            name: "Pirate".to_string(),
            stats: Stats {
                str: 7,
                def: 1,
                hp: 20,
            },
        },
        Character {
            src: "./assets/bird.png".to_string(),
            name: "Birb".to_string(),
            stats: Stats {
                str: 7,
                def: 2,
                hp: 500,
            },
        },
        Character {
            src: "./assets/outlaw.png".to_string(),
            name: "Smorkle".to_string(),
            stats: Stats {
                str: 6,
                def: 1,
                hp: 1000,
            },
        },
    ];
    return characters;
}
