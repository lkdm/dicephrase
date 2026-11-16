use getrandom::{Error, fill};

const DICE_COUNT: usize = 5;
const SIDES: u32 = 6;

fn get_random_u32() -> Result<u32, Error> {
    let mut buf = [0u8; 4];
    fill(&mut buf)?;
    Ok(u32::from_ne_bytes(buf))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceRolls(pub [u8; DICE_COUNT]);

impl DiceRolls {
    pub fn new() -> Result<Self, Error> {
        let mut rolls = [0u8; DICE_COUNT];

        for roll in &mut rolls {
            *roll = roll_die()?;
        }

        Ok(Self(rolls))
    }

    pub fn rolls(&self) -> &[u8; DICE_COUNT] {
        &self.0
    }
}

fn roll_die() -> Result<u8, Error> {
    // Largest multiple of 6 representable by u32.
    let limit = u32::MAX - (u32::MAX % SIDES);

    loop {
        let n = get_random_u32()?;

        if n < limit {
            return Ok((n % SIDES + 1) as u8);
        }
    }
}

pub fn random_bool() -> Result<bool, Error> {
    let mut buf = [0u8; 1];
    fill(&mut buf)?;

    Ok(buf[0] & 1 == 1)
}

pub fn random_index(max: usize) -> Result<usize, Error> {
    assert!(max > 0, "max must be greater than zero");

    let range = max as u64;
    let zone = u64::MAX - (u64::MAX % range);

    loop {
        let mut buf = [0u8; 8];
        fill(&mut buf)?;

        let n = u64::from_ne_bytes(buf);

        if n < zone {
            return Ok((n % range) as usize);
        }
    }
}

pub fn random_digit() -> Result<u8, Error> {
    let limit = u32::MAX - (u32::MAX % 10);

    loop {
        let n = get_random_u32()?;

        if n < limit {
            return Ok((n % 10) as u8);
        }
    }
}
