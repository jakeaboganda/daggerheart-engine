//! Weapon damage system example
//!
//! Demonstrates damage dice rolling for different weapons
//!
//! Run with: cargo run --example weapon_damage

use daggerheart_engine::core::DamageDice;

fn main() {
    println!("⚔️  Daggerheart Engine - Weapon Damage Example\n");

    // Tier 1 weapons (from official SRD)
    println!("=== Tier 1 Weapons ===\n");

    // Longsword: d10+3
    println!("Longsword (d10+3):");
    for i in 1..=3 {
        let damage = DamageDice::d10(1).with_bonus(3).roll();
        println!("  Attack {}: {} damage (rolled {}, +3 bonus)", 
                 i, damage.total, damage.rolls[0]);
    }

    // Spear: d8+3
    println!("\nSpear (d8+3):");
    for i in 1..=3 {
        let damage = DamageDice::d8(1).with_bonus(3).roll();
        println!("  Attack {}: {} damage (rolled {}, +3 bonus)", 
                 i, damage.total, damage.rolls[0]);
    }

    // Dagger: d6+2
    println!("\nDagger (d6+2):");
    for i in 1..=3 {
        let damage = DamageDice::d6(1).with_bonus(2).roll();
        println!("  Attack {}: {} damage (rolled {}, +2 bonus)", 
                 i, damage.total, damage.rolls[0]);
    }

    // Example with multiple dice
    println!("\n\n=== Spells and Multiple Dice ===\n");

    // Fireball: 3d6
    println!("Fireball (3d6):");
    let fireball = DamageDice::d6(3).roll();
    println!("  Rolled: {:?}", fireball.rolls);
    println!("  Total: {} fire damage", fireball.total);

    // Lightning Bolt: 4d8
    println!("\nLightning Bolt (4d8):");
    let lightning = DamageDice::d8(4).roll();
    println!("  Rolled: {:?}", lightning.rolls);
    println!("  Total: {} lightning damage", lightning.total);

    // Example: Sneak Attack (weapon + bonus dice)
    println!("\n\n=== Rogue Sneak Attack ===");
    println!("Dagger (d6+2) + Sneak Attack (2d6)\n");

    let base_damage = DamageDice::d6(1).with_bonus(2).roll();
    let sneak_dice = DamageDice::d6(2).roll();
    let total = base_damage.total + sneak_dice.total;

    println!("  Base weapon: {} damage", base_damage.total);
    println!("  Sneak attack: {} damage (rolled {:?})", 
             sneak_dice.total, sneak_dice.rolls);
    println!("  Total: {} damage!", total);

    // Example: Armor reduction
    println!("\n\n=== Damage vs Armor ===");
    println!("Longsword attack against armored enemy\n");

    let weapon_damage = DamageDice::d10(1).with_bonus(3).roll();
    let armor_score = 4;

    println!("  Raw damage: {}", weapon_damage.total);
    println!("  Enemy armor: {}", armor_score);

    let damage_after_armor = weapon_damage.total.saturating_sub(armor_score);
    println!("  Damage dealt: {}", damage_after_armor);

    if damage_after_armor < 5 {  // Assuming threshold of 5
        println!("  ⚠️  Below threshold: Enemy takes 1 Stress instead");
    } else {
        let hp_lost = match damage_after_armor {
            0..=4 => 1,
            5..=9 => 2,
            _ => 3,
        };
        println!("  💥 Enemy loses {} HP", hp_lost);
    }

    // Tier scaling example
    println!("\n\n=== Weapon Tier Scaling ===");
    println!("Longsword progression:\n");

    let tiers = vec![
        (1, 3, "Tier 1"),
        (2, 6, "Tier 2"),
        (3, 9, "Tier 3"),
        (4, 12, "Tier 4"),
    ];

    for (_tier, bonus, name) in tiers {
        let damage = DamageDice::d10(1).with_bonus(bonus).roll();
        println!("  {} (d10+{}): {} damage", name, bonus, damage.total);
    }

    // Mixed dice example
    println!("\n\n=== Mixed Dice (Custom Weapons) ===");
    use daggerheart_engine::core::Die;

    // Flaming Sword: d10 physical + d6 fire
    let physical = Die::D10.roll();
    let fire = Die::D6.roll();
    println!("Flaming Sword (d10 physical + d6 fire):");
    println!("  Physical: {}", physical);
    println!("  Fire: {}", fire);
    println!("  Total: {} damage", physical + fire);

    // Frostbite Touch: 2d4 + d8 cold
    let damage = DamageDice::new(vec![Die::D4, Die::D4, Die::D8]).roll();
    println!("\nFrostbite Touch (2d4+d8):");
    println!("  Rolled: {:?}", damage.rolls);
    println!("  Total: {} cold damage", damage.total);

    println!("\n💡 Damage System Notes:");
    println!("• Weapons scale with tier (higher bonus)");
    println!("• Armor reduces damage before HP loss");
    println!("• Low damage (below threshold) = 1 Stress");
    println!("• High damage = 1-3 HP loss based on how much it exceeds threshold");
    println!("• Everyone starts with only 6 HP - combat is deadly!");
}
