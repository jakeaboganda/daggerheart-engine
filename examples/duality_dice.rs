//! Duality dice system example
//!
//! Demonstrates the core Daggerheart mechanic: 2d12 Hope/Fear dice
//!
//! Run with: cargo run --example duality_dice

use daggerheart_engine::core::{DualityRoll, SuccessType};

fn main() {
    println!("🎲 Daggerheart Engine - Duality Dice Example\n");

    // Example 1: Simple roll with modifier
    println!("=== Example 1: Attacking with a Longsword ===");
    println!("Character has +2 Strength, proficiency bonus +2");
    println!("Target difficulty: 12\n");

    let attack_roll = DualityRoll::roll();
    let modifier = 2 + 2;  // Strength + Proficiency
    let result = attack_roll.with_modifier(modifier);

    println!("Hope die: {}", result.roll.hope);
    println!("Fear die: {}", result.roll.fear);
    println!("Modifier: +{}", modifier);
    println!("Total: {}", result.total);

    match result.success_type(12) {
        SuccessType::CriticalSuccess => {
            println!("\n🌟 CRITICAL SUCCESS! (Doubles: {}+{})", result.roll.hope, result.roll.fear);
            println!("   You strike with incredible precision!");
        }
        SuccessType::SuccessWithHope => {
            println!("\n✅ SUCCESS WITH HOPE!");
            println!("   You hit the target and gain 1 Hope!");
            println!("   You retain initiative.");
        }
        SuccessType::SuccessWithFear => {
            println!("\n⚠️  SUCCESS WITH FEAR");
            println!("   You hit, but the GM gains 1 Fear.");
            println!("   Initiative shifts to enemies.");
        }
        SuccessType::Failure => {
            println!("\n❌ FAILURE");
            println!("   Your attack misses!");
        }
    }

    // Example 2: Advantage roll
    println!("\n\n=== Example 2: Sneaking with Advantage ===");
    println!("You're a Rogue, so you have advantage on stealth checks");
    println!("Target difficulty: 14\n");

    let stealth_roll = DualityRoll::roll();
    let stealth_result = stealth_roll.with_advantage();

    println!("Hope die: {}", stealth_result.roll.hope);
    println!("Fear die: {}", stealth_result.roll.fear);
    println!("Advantage d6: {}", stealth_result.advantage_die.unwrap());
    println!("Total: {}", stealth_result.total);

    if stealth_result.is_success(14) {
        println!("\n✅ You slip past the guards unnoticed!");
    } else {
        println!("\n❌ A guard spots you!");
    }

    // Example 3: Demonstrating criticals
    println!("\n\n=== Example 3: Critical Success Mechanics ===");
    println!("In Daggerheart, ANY DOUBLES are a critical success!");
    println!("Even 1+1 is a critical!\n");

    let examples = vec![
        (1, 1, "Snake eyes - still a crit!"),
        (7, 7, "Lucky sevens"),
        (12, 12, "Maximum power!"),
    ];

    for (hope, fear, desc) in examples {
        let roll = DualityRoll::from_values(hope, fear);
        let result = roll.with_modifier(0);
        
        println!("{} ({}+{}):", desc, hope, fear);
        println!("  Total: {}", result.total);
        println!("  Critical: {}", if result.is_critical { "YES! 🌟" } else { "No" });
        println!();
    }

    // Example 4: Hope vs Fear mechanics
    println!("=== Example 4: Hope vs Fear ===");
    println!("The balance between Hope and Fear drives the narrative\n");

    for _ in 0..3 {
        let roll = DualityRoll::roll();
        let result = roll.with_modifier(0);

        print!("Roll: {}(Hope) vs {}(Fear) = ", result.roll.hope, result.roll.fear);
        
        match result.success_type(10) {
            SuccessType::SuccessWithHope => {
                println!("Hope wins! ➕ Player gets Hope, keeps initiative");
            }
            SuccessType::SuccessWithFear => {
                println!("Fear wins. ⚠️  GM gets Fear, enemies act");
            }
            SuccessType::CriticalSuccess => {
                println!("CRITICAL! 🌟 (Doubles)");
            }
            SuccessType::Failure => {
                println!("Failure ❌");
            }
        }
    }

    println!("\n💡 Design Note:");
    println!("Even when you succeed, if Fear wins, the GM gains resources.");
    println!("This creates tension: success isn't always clean!");
}
