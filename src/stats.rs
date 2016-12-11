#![allow(dead_code)]
use std::collections::HashMap;

/// Character stat.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Stat {
    /// Strength.
    Str,
    /// Intelligence.
    Int,
    /// Swiftness.
    Swi,
}

/// Character stats.
pub struct CharacterStats {
    base: HashMap<Stat, f32>,
    multipliers: HashMap<Stat, f32>,
}

/// Stat multiplier.
pub struct Multiplier {
    stat: Stat,
    value: f32,
}

impl CharacterStats {
    /// Constructs new `CharacterStats`.
    pub fn new(base: HashMap<Stat, f32>) -> Self {
        CharacterStats {
            base: base,
            multipliers: HashMap::new(),
        }
    }
    /// Gets a specific `Stat`.
    /// Multipliers are applied in the process.
    pub fn get_stat(&self, stat: Stat) -> i32 {
        let multiplier = match self.multipliers.get(&stat) {
            Some(val) => 1_f32 + *val,
            None => 1_f32,
        };
        (self.base.get(&stat).unwrap() * multiplier).round() as i32
    }
    /// Adds a multiplier for a specific stat.
    pub fn add_multiplier(&mut self, stat: &Multiplier) {
        *self.multipliers.entry(stat.stat.clone()).or_insert(0f32) += stat.value;
    }
    /// Removes a multiplier for a specific stat.
    pub fn sub_multiplier(&mut self, stat: &Multiplier) {
        *self.multipliers.entry(stat.stat.clone()).or_insert(0f32) -= stat.value;
    }
}

impl Multiplier {
    /// Constructs a new `Multiplier`.
    ///
    /// # Internals
    /// Values are percentages, so a value of 0.1_f32 increases the specified stat by 10%.
    pub fn new(stat: Stat, value: f32) -> Self {
        Multiplier {
            stat: stat,
            value: value,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn mock_base_stats() -> CharacterStats {
        macro_rules! hashmap {
            ($($key:expr => $val:expr),*) => {{
                let mut map = ::std::collections::HashMap::new();
                $( map.insert($key, $val); )*
                map
            }};
        }
        CharacterStats::new(hashmap![
            Stat::Str => 10_f32,
            Stat::Int => 10_f32,
            Stat::Swi => 10_f32
        ])
    }
    #[test]
    fn add_multiplier() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, 0.1_f32);
        stats.add_multiplier(&mult);
        assert_eq!(stats.get_stat(Stat::Str), 11);
    }
    #[test]
    fn add_multipliers() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, 0.1_f32);
        for _ in 0..7 {
            stats.add_multiplier(&mult);
        }
        assert_eq!(stats.get_stat(Stat::Str), 17);
    }
    #[test]
    fn add_negative_multiplier() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, -0.1_f32);
        stats.add_multiplier(&mult);
        assert_eq!(stats.get_stat(Stat::Str), 9);
    }
    #[test]
    fn add_negative_multipliers() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, -0.1_f32);
        for _ in 0..7 {
            stats.add_multiplier(&mult);
        }
        assert_eq!(stats.get_stat(Stat::Str), 3);
    }
    #[test]
    fn add_remove_multiplier() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, 0.1_f32);
        stats.add_multiplier(&mult);
        stats.sub_multiplier(&mult);
        assert_eq!(stats.get_stat(Stat::Str), 10);
    }
    #[test]
    fn add_remove_multipliers() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, 0.1_f32);
        for _ in 0..7 {
            stats.add_multiplier(&mult);
        }
        for _ in 0..7 {
            stats.sub_multiplier(&mult);
        }
        assert_eq!(stats.get_stat(Stat::Str), 10);
    }
    #[test]
    fn add_remove_negative_multiplier() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, -0.1_f32);
        stats.add_multiplier(&mult);
        stats.sub_multiplier(&mult);
        assert_eq!(stats.get_stat(Stat::Str), 10);
    }
    #[test]
    fn add_remove_negative_multipliers() {
        let mut stats = mock_base_stats();
        let mult = Multiplier::new(Stat::Str, -0.1_f32);
        for _ in 0..7 {
            stats.add_multiplier(&mult);
        }
        for _ in 0..7 {
            stats.sub_multiplier(&mult);
        }
        assert_eq!(stats.get_stat(Stat::Str), 10);
    }
    #[test]
    fn get_without_multipliers() {
        let stats = mock_base_stats();
        assert_eq!(stats.get_stat(Stat::Str), 10);
    }
}