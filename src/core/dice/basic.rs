//! Basic dice types (d4, d6, d8, d10, d12, d20)

use rand::Rng;

/// Standard polyhedral dice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl Die {
    /// Roll this die and return the result
    pub fn roll(&self) -> u8 {
        let mut rng = rand::thread_rng();
        self.roll_with_rng(&mut rng)
    }

    /// Roll with a specific RNG (for testing/seeding)
    pub fn roll_with_rng<R: Rng>(&self, rng: &mut R) -> u8 {
        let max = self.max();
        rng.gen_range(1..=max)
    }

    /// Get the maximum value for this die
    pub fn max(&self) -> u8 {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_die_max_values() {
        assert_eq!(Die::D4.max(), 4);
        assert_eq!(Die::D6.max(), 6);
        assert_eq!(Die::D8.max(), 8);
        assert_eq!(Die::D10.max(), 10);
        assert_eq!(Die::D12.max(), 12);
        assert_eq!(Die::D20.max(), 20);
    }

    #[test]
    fn test_d4_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D4.roll();
            assert!(result >= 1 && result <= 4, "d4 rolled {}", result);
        }
    }

    #[test]
    fn test_d6_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D6.roll();
            assert!(result >= 1 && result <= 6, "d6 rolled {}", result);
        }
    }

    #[test]
    fn test_d8_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D8.roll();
            assert!(result >= 1 && result <= 8, "d8 rolled {}", result);
        }
    }

    #[test]
    fn test_d10_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D10.roll();
            assert!(result >= 1 && result <= 10, "d10 rolled {}", result);
        }
    }

    #[test]
    fn test_d12_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D12.roll();
            assert!(result >= 1 && result <= 12, "d12 rolled {}", result);
        }
    }

    #[test]
    fn test_d20_rolls_in_range() {
        for _ in 0..100 {
            let result = Die::D20.roll();
            assert!(result >= 1 && result <= 20, "d20 rolled {}", result);
        }
    }

    #[test]
    fn test_roll_with_rng() {
        let mut rng = StdRng::seed_from_u64(12345);
        
        for _ in 0..50 {
            let result = Die::D6.roll_with_rng(&mut rng);
            assert!(result >= 1 && result <= 6);
        }
    }

    #[test]
    fn test_roll_with_rng_reproducible() {
        let mut rng1 = StdRng::seed_from_u64(99999);
        let mut rng2 = StdRng::seed_from_u64(99999);
        
        let roll1 = Die::D12.roll_with_rng(&mut rng1);
        let roll2 = Die::D12.roll_with_rng(&mut rng2);
        
        assert_eq!(roll1, roll2, "Same seed should produce same result");
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    // Strategy to generate any Die variant
    fn any_die() -> impl Strategy<Value = Die> {
        prop_oneof![
            Just(Die::D4),
            Just(Die::D6),
            Just(Die::D8),
            Just(Die::D10),
            Just(Die::D12),
            Just(Die::D20),
        ]
    }

    proptest! {
        #[test]
        fn prop_die_rolls_are_always_valid(die in any_die(), seed in any::<u64>()) {
            let mut rng = StdRng::seed_from_u64(seed);
            let roll = die.roll_with_rng(&mut rng);
            
            prop_assert!(roll >= 1, "Roll {} is below minimum 1", roll);
            prop_assert!(roll <= die.max(), "Roll {} exceeds max {}", roll, die.max());
        }

        #[test]
        fn prop_die_max_is_consistent(die in any_die()) {
            let max1 = die.max();
            let max2 = die.max();
            prop_assert_eq!(max1, max2, "max() should be deterministic");
        }

        #[test]
        fn prop_same_seed_produces_same_roll(die in any_die(), seed in any::<u64>()) {
            let mut rng1 = StdRng::seed_from_u64(seed);
            let mut rng2 = StdRng::seed_from_u64(seed);
            
            let roll1 = die.roll_with_rng(&mut rng1);
            let roll2 = die.roll_with_rng(&mut rng2);
            
            prop_assert_eq!(roll1, roll2, "Same seed should produce same roll");
        }

        #[test]
        fn prop_all_values_eventually_rollable(die in any_die()) {
            // With enough rolls, we should be able to roll every possible value
            let max = die.max();
            let mut seen = vec![false; max as usize + 1];
            
            // Try up to 1000 rolls to see all values
            for seed in 0..1000 {
                let mut rng = StdRng::seed_from_u64(seed);
                let roll = die.roll_with_rng(&mut rng);
                seen[roll as usize] = true;
            }
            
            // Check we saw at least 1 and max
            prop_assert!(seen[1], "Never rolled minimum value 1");
            prop_assert!(seen[max as usize], "Never rolled maximum value {}", max);
        }
    }
}
