//! Example: Character leveling and progression
//!
//! This example demonstrates:
//! - Gaining experience points
//! - Leveling up characters
//! - Managing cards/abilities
//! - XP calculations

use daggerheart_engine::character::CharacterProgress;

fn main() {
    println!("=== Daggerheart Character Progression ===\n");

    // Example 1: Basic XP and leveling
    example_basic_leveling();
    println!();

    // Example 2: Multi-level progression
    example_multi_level();
    println!();

    // Example 3: Card management
    example_card_progression();
    println!();

    // Example 4: Simulated campaign progression
    example_campaign();
}

fn example_basic_leveling() {
    println!("EXAMPLE 1: Basic Leveling\n");

    let mut progress = CharacterProgress::new();

    println!("Starting character:");
    println!("  Level: {}", progress.level);
    println!("  XP: {}", progress.experience);
    println!(
        "  XP needed for next level: {}",
        progress.xp_for_next_level()
    );

    // Gain some XP
    println!("\n📈 Gained 75 XP from completing a quest!");
    progress.add_experience(75);
    println!("  Current XP: {}", progress.experience);
    println!("  Can level up? {}", progress.can_level_up());

    // Gain more XP
    println!("\n📈 Gained 30 XP from defeating enemies!");
    progress.add_experience(30);
    println!("  Current XP: {}", progress.experience);
    println!("  Can level up? {}", progress.can_level_up());

    // Level up!
    if progress.can_level_up() {
        progress.level_up().unwrap();
        println!("\n🎉 LEVEL UP!");
        println!("  New level: {}", progress.level);
        println!("  Remaining XP: {}", progress.experience);
        println!(
            "  XP needed for next level: {}",
            progress.xp_for_next_level()
        );
    }
}

fn example_multi_level() {
    println!("EXAMPLE 2: Multi-Level Progression\n");

    let mut progress = CharacterProgress::new();

    // Gain a lot of XP at once
    println!("🏆 Major quest completed! Gained 650 XP!");
    progress.add_experience(650);

    println!("\nCurrent state:");
    println!("  Level: {}", progress.level);
    println!("  XP: {}", progress.experience);

    // Level up multiple times
    println!("\n⬆️ Leveling up:");
    let mut level_count = 0;

    while progress.can_level_up() {
        let old_level = progress.level;
        progress.level_up().unwrap();
        level_count += 1;

        println!(
            "  Level {} → {} (cost: {} XP, remaining: {})",
            old_level,
            progress.level,
            old_level as u32 * 100,
            progress.experience
        );
    }

    println!("\n✅ Leveled up {} times!", level_count);
    println!("  Final level: {}", progress.level);
    println!("  Banked XP: {}", progress.experience);
    println!(
        "  XP needed for level {}: {}",
        progress.level + 1,
        progress.xp_for_next_level()
    );
}

fn example_card_progression() {
    println!("EXAMPLE 3: Card/Ability Management\n");

    let mut progress = CharacterProgress::new();

    println!("Starting with no cards:");
    println!("  Available cards: {:?}", progress.available_cards);

    // Level up and gain cards
    progress.add_experience(100);
    progress.level_up().unwrap();
    println!("\n🎉 Reached level {}!", progress.level);

    println!("Choose a card to learn:");
    println!("  1. Blade Strike (Attack)");
    println!("  2. Shield Bash (Defense)");
    println!("  3. Battle Cry (Support)");

    // Player chooses Blade Strike
    progress.add_card("blade_strike");
    println!("\n✅ Learned: Blade Strike");
    println!("  Available cards: {:?}", progress.available_cards);

    // Level up again
    progress.add_experience(200);
    progress.level_up().unwrap();
    println!("\n🎉 Reached level {}!", progress.level);

    // Learn another card
    progress.add_card("shield_bash");
    println!("✅ Learned: Shield Bash");
    println!("  Available cards: {:?}", progress.available_cards);

    // Check for specific cards
    println!("\n🔍 Card checks:");
    println!("  Has Blade Strike? {}", progress.has_card("blade_strike"));
    println!("  Has Shield Bash? {}", progress.has_card("shield_bash"));
    println!("  Has Fireball? {}", progress.has_card("fireball"));
}

fn example_campaign() {
    println!("EXAMPLE 4: Simulated Campaign Progression\n");

    let mut progress = CharacterProgress::new();

    // Track progression through multiple sessions
    let sessions = vec![
        ("Session 1: The Goblin Cave", 85),
        ("Session 2: Rescue the Villagers", 45),
        ("Session 3: Defeat the Orc Chieftain", 120),
        ("Session 4: Explore the Ancient Ruins", 95),
        ("Session 5: The Dragon's Lair", 200),
        ("Session 6: Save the Kingdom", 180),
        ("Session 7: The Lich's Tomb", 150),
        ("Session 8: Final Confrontation", 250),
    ];

    println!("📖 Campaign: The Rise of Heroes\n");

    for (session_name, xp_gained) in sessions {
        println!("{}:", session_name);
        println!(
            "  Starting: Level {}, {} XP",
            progress.level, progress.experience
        );

        // Gain XP
        progress.add_experience(xp_gained);
        println!(
            "  📈 Gained {} XP (total: {})",
            xp_gained, progress.experience
        );

        // Check for level up
        let mut leveled = false;
        while progress.can_level_up() {
            let old_level = progress.level;
            progress.level_up().unwrap();
            println!("  🎉 LEVEL UP! {} → {}", old_level, progress.level);
            leveled = true;

            // Learn a card when leveling up
            let card_name = format!("ability_level_{}", progress.level);
            progress.add_card(&card_name);
            println!("  ✨ Learned new ability: {}", card_name);
        }

        if !leveled {
            println!("  (No level up yet)");
        }

        println!(
            "  Ending: Level {}, {} XP (need {} for next level)\n",
            progress.level,
            progress.experience,
            progress.xp_for_next_level()
        );
    }

    // Campaign summary
    println!("📊 Campaign Summary:");
    println!("  Final level: {}", progress.level);
    println!("  Total cards learned: {}", progress.available_cards.len());
    println!("  Cards: {:?}", progress.available_cards);

    if progress.level >= 10 {
        println!("\n🏆 MAX LEVEL REACHED! Character is legendary!");
    } else {
        println!(
            "\n💪 {} more XP needed to reach level {}",
            progress.xp_for_next_level() - progress.experience,
            progress.level + 1
        );
    }
}
