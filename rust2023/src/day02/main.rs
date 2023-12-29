use std::cmp;
use std::collections::HashMap;


fn main() {

    let calibration_document = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


    let configuration = GameConfiguration {
        red: 12,
        green: 13,
        blue: 14
    };

    let games: HashMap<u32, Vec<GameConfiguration>> = calibration_document.lines()
        .map(|l| parse_game(l))
        .collect();
    let requirements: HashMap<u32, GameConfiguration> = games.into_iter()
        .map(|g| (g.0, get_requirements(&g.1)))
        .collect();
    println!("{}", sum_valid_games(&requirements, &configuration));
}

fn parse_game(raw: &str) -> (u32, Vec<GameConfiguration>) {
    let split = raw.split(":").collect::<Vec<&str>>();
    return (parse_key(split[0]), parse_sets(split[1]));
}

fn parse_key(raw: &str) -> u32 {
    return raw[5..].parse().expect(&("Invalid Game! ".to_owned() + raw));
}

fn get_requirements(sets: &Vec<GameConfiguration>) -> GameConfiguration {
    return sets.into_iter().fold(GameConfiguration { red:0, green:0, blue:0}, |agg, i| GameConfiguration{
        red: cmp::max(agg.red, i.red),
        green: cmp::max(agg.green, i.green),
        blue: cmp::max(agg.blue, i.blue)
    });
}

fn parse_sets(raw: &str) -> Vec<GameConfiguration> {
    return raw.split(";")
    .map(|set| parse_set(set) ).collect::<Vec<GameConfiguration>>();
}

fn parse_set(set: &str) -> GameConfiguration {
    return set.split(",").map(|cube| parse_cubes(cube) ).sum();
}

fn parse_cubes(cube: &str) -> GameConfiguration {
    let split = cube.split(' ').collect::<Vec<&str>>();
    let number: u32 = split.get(1)
        .expect(&("Empty set: ".to_owned() + cube))
        .parse()
        .expect(&("Invalid game set: {}".to_owned() + cube));
    return match split[2] {
        "red" => GameConfiguration{ red: number, green: 0, blue: 0},
        "green" => GameConfiguration{ red: 0, green: number, blue: 0},
        "blue" => GameConfiguration{ red: 0, green: 0, blue: number},
        _ => panic!("Unrecognised colour: {}", cube)
    };
}

fn sum_valid_games(games: &HashMap<u32, GameConfiguration>, configuration: &GameConfiguration) -> u32 {
    return games.into_iter()
        .filter(|&(_, v)| valid_game(v, configuration))
        .fold(0, |agg, (k, _)| agg + k);
}

fn valid_game(game: &GameConfiguration, configuration: &GameConfiguration) -> bool {
    return game.red <= configuration.red && game.green <= configuration.green && game.blue <= configuration.blue;
}

struct GameConfiguration {
    red: u32,
    green: u32,
    blue: u32
}

impl std::iter::Sum<GameConfiguration> for GameConfiguration {
    fn sum<I>(iter: I) -> GameConfiguration
    where
        I: Iterator<Item = GameConfiguration>,
    {
        return iter.fold(GameConfiguration {red: 0, green: 0, blue: 0}, |agg, i| GameConfiguration {
            red: agg.red + i.red,
            green: agg.green + i.green,
            blue: agg.blue + i.blue
        })
    }
}
