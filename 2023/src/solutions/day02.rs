use aoc2023::util;
use std::io;

pub fn part_a(fname: &str) -> io::Result<()> {
    let lines = util::read_file(fname)?;
    let mut tot: u32 = 0;
    for line in lines.into_iter().flatten() {
        let game = parse_game(&line)?;
        if game
            .rolls
            .iter()
            .filter(|roll| roll.blue <= 14 && roll.red <= 12 && roll.green <= 13)
            .collect::<Vec<_>>()
            .len()
            == game.rolls.len()
        {
            tot += game.which;
        }
    }

    println!("Day 02, part a, {tot}");

    Ok(())
}

fn parse_game(line: &str) -> io::Result<Game> {
    let mut split = line.split(':');
    let which = split
        .next()
        .map(|prefix| match prefix.strip_prefix("Game ") {
            Some(stripped) => parse_int(stripped),
            None => {
                println!("Unable to strip prefix from {prefix}");
                Err(std::io::ErrorKind::InvalidInput.into())
            }
        })
        .ok_or_else(|| {
            println!("Unable to map thing");
            std::io::ErrorKind::InvalidInput
        })??;

    let mut rolls: Vec<Roll> = vec![];

    match split.next() {
        Some(rolls_str) => {
            for roll in rolls_str.trim().split(';') {
                rolls.push(parse_roll(roll)?);
            }
            Ok(())
        }
        None => {
            println!("Unable to get next part of split, {split:?}");
            Err(std::io::ErrorKind::InvalidInput)
        }
    }?;

    Ok(Game { which, rolls })
}

fn parse_roll(roll: &str) -> io::Result<Roll> {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for color in roll.trim().split(',') {
        match color.trim().split(' ').collect::<Vec<_>>()[..] {
            [n, "blue"] => blue = parse_int(n)?,
            [n, "red"] => red = parse_int(n)?,
            [n, "green"] => green = parse_int(n)?,
            _ => {
                println!("Error parsing roll {roll}");
                return Err(std::io::ErrorKind::InvalidInput.into());
            }
        }
    }

    Ok(Roll { blue, red, green })
}

fn parse_int(n: &str) -> io::Result<u32> {
    match n.parse::<u32>() {
        Ok(val) => Ok(val),
        Err(_) => {
            println!("Unable to parse {n} into an integer");
            Err(std::io::ErrorKind::InvalidInput.into())
        }
    }
}

#[derive(Debug)]
struct Roll {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug)]
struct Game {
    which: u32,
    rolls: Vec<Roll>,
}
