use rand::{
    distributions::{Distribution, Standard, Uniform},
    Rng,
};

#[allow(non_camel_case_types)]
enum Die {
    d6,
}

impl From<Die> for i8 {
    fn from(d: Die) -> Self {
        match d {
            Die::d6 => 6,
        }
    }
}

fn roll(num: i8, die: Die, modifier: i8) -> i8 {
    let mut rng = rand::thread_rng();
    let distribution = Uniform::new_inclusive(1, i8::from(die));
    let roll: i8 = (0..num)
        .map(|_| distribution.sample(&mut rng))
        .into_iter()
        .sum();

    roll + modifier
}

struct Equipment {
    load: Option<i8>,
    item: String,
    sup: Option<i8>,
    durability: Option<i8>,
}

#[derive(Debug)]
enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
}

// 50% chance of human race
impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..=6) {
            0 => Race::Halfling,
            1 => Race::Elf,
            2 => Race::Dwarf,
            _ => Race::Human,
        }
    }
}

#[derive(Debug, Default)]
struct AbilityScores {
    str: i8,
    dex: i8,
    con: i8,
    int: i8,
    wis: i8,
    cha: i8,
}

fn generate_scores(race: &Race) -> AbilityScores {
    match race {
        Race::Human => AbilityScores {
            str: roll(3, Die::d6, 0),
            dex: roll(3, Die::d6, 0),
            con: roll(3, Die::d6, 0),
            int: roll(3, Die::d6, 0),
            wis: roll(3, Die::d6, 0),
            cha: roll(3, Die::d6, 0),
        },
        Race::Elf => AbilityScores {
            str: roll(2, Die::d6, 3),
            dex: 13,
            con: roll(2, Die::d6, 3),
            int: 13,
            wis: roll(2, Die::d6, 3),
            cha: roll(2, Die::d6, 3),
        },
        Race::Dwarf => AbilityScores {
            str: 13,
            dex: roll(2, Die::d6, 3),
            con: 13,
            int: roll(2, Die::d6, 3),
            wis: roll(2, Die::d6, 3),
            cha: roll(2, Die::d6, 3),
        },
        Race::Halfling => AbilityScores {
            str: roll(2, Die::d6, 3),
            dex: roll(2, Die::d6, 3),
            con: roll(2, Die::d6, 3),
            int: roll(2, Die::d6, 3),
            wis: 13,
            cha: 13,
        },
    }
}

#[derive(Debug)]
enum Class {
    Warrior, // str/dex and con
    Zealot,  // wis, con
    Thief,   // dex/str, con
    Mage,    // int, con
}

// weight each class based on primary ability?
fn class_score(scores: &AbilityScores, class: Class) -> i8 {
    let mut score: i8 = 0;
    match class {
        Class::Warrior => {
            match scores.str.cmp(&scores.int) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.str.cmp(&scores.dex) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.str.cmp(&scores.wis) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }
        }
        Class::Zealot => {
            match scores.wis.cmp(&scores.int) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.wis.cmp(&scores.dex) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.wis.cmp(&scores.str) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }
        }
        Class::Thief => {
            match scores.dex.cmp(&scores.int) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.dex.cmp(&scores.str) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.dex.cmp(&scores.wis) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }
        }
        Class::Mage => {
            match scores.int.cmp(&scores.str) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.int.cmp(&scores.dex) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }

            match scores.int.cmp(&scores.wis) {
                std::cmp::Ordering::Greater => score += 2,
                std::cmp::Ordering::Equal => score += 1,
                std::cmp::Ordering::Less => score += 0,
            }
        }
    }
    score = score * 2;
    score
}

fn get_class(_race: &Race, scores: &AbilityScores) -> Class {
    let warrior = class_score(scores, Class::Warrior);
    let zealot = class_score(scores, Class::Zealot);
    let thief = class_score(scores, Class::Thief);
    let mage = class_score(scores, Class::Mage);
    let mut options: Vec<Class> = Vec::new();

    for _ in 0..warrior {
        options.push(Class::Warrior);
    }

    for _ in 0..zealot {
        options.push(Class::Zealot);
    }

    for _ in 0..thief {
        options.push(Class::Thief);
    }

    for _ in 0..mage {
        options.push(Class::Mage);
    }

    let mut rng = rand::thread_rng();
    // pick something!
    let pick: usize = rng.gen_range(0..=options.len());

    match options.get(pick) {
        Some(&Class::Warrior) => Class::Warrior,
        Some(&Class::Zealot) => Class::Zealot,
        Some(&Class::Thief) => Class::Thief,
        Some(&Class::Mage) => Class::Mage,
        _ => Class::Warrior,
    }
}

fn main() {
    let race: Race = rand::random();
    let scores = generate_scores(&race);
    let class = get_class(&race, &scores);

    println!("Level 1 {:?} {:?}", race, class);
    println!("{:#?}", &scores);
}
