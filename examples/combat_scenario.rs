//! Full combat scenario example
//!
//! Demonstrates a complete combat encounter using all dice systems
//!
//! Run with: cargo run --example combat_scenario

use daggerheart_engine::core::{DualityRoll, DamageDice, SuccessType};

fn main() {
    println!("⚔️  Daggerheart Engine - Combat Scenario Example\n");
    println!("═══════════════════════════════════════════════════════════\n");

    // Setup
    println!("🏰 SCENARIO: Defending the Village\n");
    println!("Your party faces a goblin raider!");
    println!("Turn 1: Warrior attacks with longsword\n");

    // Character stats
    let warrior_strength = 2;      // +2 Strength modifier
    let warrior_proficiency = 2;   // +2 Proficiency bonus
    let difficulty = 12;           // Standard difficulty

    let mut hope_pool = 3;         // Party Hope pool
    let mut fear_pool = 0;         // GM Fear pool

    println!("═══════════════════════════════════════════════════════════");
    println!("⚔️  WARRIOR'S ATTACK");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("Rolling to hit (2d12 + {} modifier vs DC {})...", 
             warrior_strength + warrior_proficiency, difficulty);

    let attack_roll = DualityRoll::roll();
    let attack_result = attack_roll.with_modifier(warrior_strength + warrior_proficiency);

    println!("\n  Hope die: {}", attack_result.roll.hope);
    println!("  Fear die: {}", attack_result.roll.fear);
    println!("  Modifier: +{}", warrior_strength + warrior_proficiency);
    println!("  Total: {}\n", attack_result.total);

    match attack_result.success_type(difficulty) {
        SuccessType::CriticalSuccess => {
            println!("🌟 CRITICAL SUCCESS! (Doubles: {}+{})", 
                     attack_result.roll.hope, attack_result.roll.fear);
            println!("   Your blade finds the perfect opening!");
            println!("   Rolling damage with advantage...\n");

            // Critical might give extra damage or auto-max damage
            let damage = DamageDice::d10(1).with_bonus(3).roll();
            println!("   Longsword damage (d10+3): {} damage!", damage.total);
            
            println!("\n   The goblin is struck hard!");
        }
        
        SuccessType::SuccessWithHope => {
            println!("✅ SUCCESS WITH HOPE!");
            println!("   Your attack connects!");
            
            hope_pool += 1;
            println!("   ➕ Gain 1 Hope (pool: {} → {})", hope_pool - 1, hope_pool);
            println!("   You keep initiative!\n");

            // Roll damage
            let damage = DamageDice::d10(1).with_bonus(3).roll();
            println!("   Longsword damage (d10+3):");
            println!("   Rolled: {} on the die", damage.rolls[0]);
            println!("   Bonus: +3");
            println!("   Total: {} damage", damage.total);

            // Apply armor
            let goblin_armor = 2;
            let damage_after_armor = damage.total.saturating_sub(goblin_armor);
            
            println!("\n   Goblin armor: {}", goblin_armor);
            println!("   Damage after armor: {}", damage_after_armor);

            // Determine HP loss
            let threshold = 5;  // Damage threshold
            if damage_after_armor < threshold {
                println!("   ⚠️  Below threshold: Goblin takes 1 Stress");
            } else {
                let hp_lost = match damage_after_armor {
                    0..=4 => 1,
                    5..=9 => 2,
                    _ => 3,
                };
                println!("   💥 Goblin loses {} HP!", hp_lost);
                println!("   (Goblin HP: 6 → {})", 6 - hp_lost);
            }
        }
        
        SuccessType::SuccessWithFear => {
            println!("⚠️  SUCCESS WITH FEAR");
            println!("   You hit, but something goes wrong...");
            
            fear_pool += 1;
            println!("   ⚠️  GM gains 1 Fear (pool: {} → {})", fear_pool - 1, fear_pool);
            println!("   Initiative shifts to enemies!\n");

            let damage = DamageDice::d10(1).with_bonus(3).roll();
            println!("   Damage: {} (but enemies act next)", damage.total);
        }
        
        SuccessType::Failure => {
            println!("❌ FAILURE");
            println!("   Your swing goes wide! The goblin dodges!\n");
            println!("   No damage dealt.");
        }
    }

    // Show resource pools
    println!("\n═══════════════════════════════════════════════════════════");
    println!("📊 RESOURCE POOLS");
    println!("═══════════════════════════════════════════════════════════\n");
    println!("  Party Hope: {}", hope_pool);
    println!("  GM Fear: {}", fear_pool);

    // Spending Hope example
    if hope_pool > 0 {
        println!("\n💡 TIP: You could spend Hope for:");
        println!("   • +2 to a roll (if relevant to an Experience)");
        println!("   • Activate special abilities");
        println!("   • Avoid death (permanent -1 max Hope)");
    }

    // Round 2 - Using advantage
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("🗡️  ROUND 2: ROGUE'S TURN");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("The Rogue flanks the goblin (advantage on attack)\n");

    let rogue_finesse = 2;
    let rogue_proficiency = 2;
    
    println!("Rolling with advantage (2d12 + d6 advantage die)...\n");

    let sneak_roll = DualityRoll::roll();
    let sneak_result = sneak_roll.with_advantage();

    println!("  Hope die: {}", sneak_result.roll.hope);
    println!("  Fear die: {}", sneak_result.roll.fear);
    println!("  Advantage d6: {}", sneak_result.advantage_die.unwrap());
    println!("  Modifier: +{}", rogue_finesse + rogue_proficiency);
    println!("  Total: {}\n", sneak_result.total);

    if sneak_result.is_success(difficulty) {
        println!("✅ Hit! Rolling Sneak Attack damage...\n");

        // Dagger + Sneak Attack
        let base_damage = DamageDice::d6(1).with_bonus(2).roll();
        let sneak_damage = DamageDice::d6(2).roll();

        println!("  Dagger (d6+2): {}", base_damage.total);
        println!("  Sneak Attack (2d6): {} (rolled {:?})", 
                 sneak_damage.total, sneak_damage.rolls);
        println!("  Total: {} damage!", base_damage.total + sneak_damage.total);

        println!("\n  The goblin falls!");
    }

    // Summary
    println!("\n\n═══════════════════════════════════════════════════════════");
    println!("🎯 COMBAT DESIGN NOTES");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("Key Mechanics:");
    println!("  • Every roll creates Hope or Fear");
    println!("  • Even success can give GM resources (Fear)");
    println!("  • Crits happen on ANY doubles (1+1 through 12+12)");
    println!("  • Low HP (6) makes every hit matter");
    println!("  • Armor absorbs damage but gets damaged");
    println!("  • Advantage adds d6 to total\n");

    println!("Tactical Depth:");
    println!("  • Choose when to spend Hope");
    println!("  • Risk vs reward on every roll");
    println!("  • Initiative flows based on Hope/Fear");
    println!("  • Armor durability creates resource tension\n");

    println!("Next steps:");
    println!("  • Implement character system (attributes, classes)");
    println!("  • Add Hope/Fear pool management");
    println!("  • Build combat action system");
    println!("  • Create domain card abilities");
}
