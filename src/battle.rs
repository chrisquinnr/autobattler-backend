use crate::fixtures;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use warp::{Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialMove {
    Revival,
    GigaAttack,
    Restore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    pub src: String,
    pub name: String,
    pub stats: Stats,
    pub special_move: Option<SpecialMove>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub str: u32,
    pub def: u32,
    pub hp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Team {
    characters: Vec<Character>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BattleStep {
    attacker: String,
    defender: String,
    damage: u32,
    remaining_hp: u32,
    who: String,
    is_special: bool,
}

pub async fn get_team() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&fixtures::get_team_fixtures()))
}

pub async fn get_opposition() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&fixtures::get_opposition_fixtures()))
}

fn perform_battle_round(
    attack: &mut Team,
    defence: &mut Team,
    steps: &mut Vec<BattleStep>,
    who: &str,
) {
    if attack.characters.is_empty() || defence.characters.is_empty() {
        print!("Battle over!\n");
        return;
    }

    print!("Performing battle round!\n");

    let mut rng = rand::thread_rng();
    let attack_index = rng.gen_range(0..attack.characters.len());
    let defence_index = rng.gen_range(0..defence.characters.len());

    let attack_character = &mut attack.characters[attack_index];
    let defence_character = &mut defence.characters[defence_index];

    let damage = attack_character
        .stats
        .str
        .saturating_sub(defence_character.stats.def);
    defence_character.stats.hp = defence_character.stats.hp.saturating_sub(damage);

    // Special Move: Revival
    // if let Some(SpecialMove::Revival) = attack_character.special_move {
    //     if attack_character.stats.hp <= 0 && !target.stats.hp > 0 {
    //         attacker.stats.hp = 1;
    //         battle_log.push(format!(
    //             "{} uses Revival and brings {} back to life!",
    //             attacker.name, target.name
    //         ));
    //     }
    // }

    // Special Move: Restore
    // if let Some(SpecialMove::Restore) = attacker.special_move {
    //     let restore_hp = (attacker.stats.hp as f32 * 0.2) as i32;
    //     for team in teams {
    //         for character in team {
    //             if character != attacker {
    //                 character.stats.hp = (character.stats.hp + restore_hp).min(100);
    //                 // Ensure HP does not exceed 100
    //             }
    //         }
    //     }
    //     battle_log.push(format!(
    //         "{} uses Restore to heal their team for {} HP!",
    //         attacker.name, restore_hp
    //     ));
    // }

    let mut step = BattleStep {
        attacker: attack_character.name.clone(),
        defender: defence_character.name.clone(),
        damage,
        remaining_hp: defence_character.stats.hp,
        who: who.to_string(),
        is_special: false,
    };

    // Special Move: GigaAttack
    if let Some(SpecialMove::GigaAttack) = attack_character.special_move {
        let giga_damage = attack_character.stats.str * 3;
        defence_character.stats.hp = defence_character.stats.hp.saturating_sub(giga_damage);
        step = BattleStep {
            attacker: attack_character.name.clone(),
            defender: defence_character.name.clone(),
            damage: giga_damage.clone(),
            remaining_hp: defence_character.stats.hp,
            who: who.to_string(),
            is_special: true,
        };
    }
    steps.push(step);

    print!(
        "Team 1 character: {:?} deals damage {}\n",
        attack_character.name, damage
    );
    print!(
        "Team 2 character: {:?} is now at {}, HP!\n",
        defence_character.name, defence_character.stats.hp
    );
    if defence_character.stats.hp < 1 {
        print!("Team 2 character: {:?} is dead!\n", defence_character.name);
        defence.characters.remove(defence_index);
    }
}

pub async fn get_battle_result() -> Result<impl Reply, Rejection> {
    println!("**Start**");
    let mut team = Team {
        characters: fixtures::get_team_fixtures(),
    };

    let mut opposition = Team {
        characters: fixtures::get_opposition_fixtures(),
    };

    let mut steps = Vec::new();

    print!("Lets go!");
    while !team.characters.is_empty() && !opposition.characters.is_empty() {
        perform_battle_round(&mut team, &mut opposition, &mut steps, "player");
        perform_battle_round(&mut opposition, &mut team, &mut steps, "ai");
    }

    let result = if team.characters.is_empty() {
        "Opposition wins!"
    } else {
        "Player wins!"
    };

    let response = json!({
        "result": result,
        "steps": steps,
    });

    print!("{}", response);

    Ok(warp::reply::json(&response))
}
