//! Example: Save and load game state
//!
//! This example demonstrates:
//! - Saving/loading characters
//! - Saving/loading combat encounters
//! - Saving/loading character progression
//! - Working with JSON save files

use daggerheart_engine::character::{Ancestry, Attributes, CharacterProgress, Class};
use daggerheart_engine::combat::simulation::{CombatEncounter, Combatant};

fn main() {
    println!("=== Daggerheart Save/Load System ===\n");

    // Example 1: Save and load a character
    example_save_character();
    println!();

    // Example 2: Save and load character progression
    example_save_progression();
    println!();

    // Example 3: Save and load a combat encounter
    example_save_encounter();
    println!();

    // Example 4: Demonstrate persistence across sessions
    example_persistence();
}

fn example_save_character() {
    println!("EXAMPLE 1: Save and Load Character\n");

    // Create a character
    let warrior = Combatant::player(
        "Grom the Mighty",
        5,
        Class::Warrior,
        Ancestry::Orc,
        Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    )
    .with_armor(3);

    println!("Created character: {}", warrior.name);
    println!("  Level: {}", warrior.level);
    println!("  HP: {}/{}", warrior.hp.current, warrior.hp.maximum);
    println!("  Class: {}", warrior.class);

    // Save to file
    let save_path = "grom_character.json";
    warrior.save_to_file(save_path).unwrap();
    println!("\n✅ Character saved to: {}", save_path);

    // Load from file
    let loaded_warrior = Combatant::load_from_file(save_path).unwrap();
    println!("✅ Character loaded from: {}", save_path);
    println!("\nLoaded character: {}", loaded_warrior.name);
    println!("  Level: {}", loaded_warrior.level);
    println!(
        "  HP: {}/{}",
        loaded_warrior.hp.current, loaded_warrior.hp.maximum
    );
    println!("  Class: {}", loaded_warrior.class);

    // Clean up
    std::fs::remove_file(save_path).ok();
}

fn example_save_progression() {
    println!("EXAMPLE 2: Save and Load Character Progression\n");

    // Create progression with some experience
    let mut progress = CharacterProgress::new();
    progress.add_experience(150);
    progress.level_up().unwrap();
    progress.add_card("blade_strike");
    progress.add_card("shield_bash");

    println!("Character progression:");
    println!("  Level: {}", progress.level);
    println!("  Experience: {}", progress.experience);
    println!("  Cards: {:?}", progress.available_cards);

    // Save to file
    let save_path = "progression.json";
    progress.save_to_file(save_path).unwrap();
    println!("\n✅ Progression saved to: {}", save_path);

    // Load from file
    let loaded_progress = CharacterProgress::load_from_file(save_path).unwrap();
    println!("✅ Progression loaded from: {}", save_path);
    println!("\nLoaded progression:");
    println!("  Level: {}", loaded_progress.level);
    println!("  Experience: {}", loaded_progress.experience);
    println!("  Cards: {:?}", loaded_progress.available_cards);

    // Clean up
    std::fs::remove_file(save_path).ok();
}

fn example_save_encounter() {
    println!("EXAMPLE 3: Save and Load Combat Encounter\n");

    // Create a combat encounter
    let mut encounter = CombatEncounter::new(5);

    // Add combatants
    let warrior = Combatant::player(
        "Elara",
        3,
        Class::Bard,
        Ancestry::Human,
        Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    );

    let goblin1 = Combatant::enemy("Goblin Scout", 1, 4, 13, 1);
    let goblin2 = Combatant::enemy("Goblin Archer", 1, 3, 14, 0);

    encounter.add_combatant(warrior);
    encounter.add_combatant(goblin1);
    encounter.add_combatant(goblin2);

    // Start combat (roll initiative)
    encounter.start();

    println!("Combat encounter started:");
    println!("  Round: {}", encounter.round);
    println!("  Combatants: {}", encounter.combatants.len());
    println!(
        "  Hope pool: {}/{}",
        encounter.hope.current, encounter.hope.maximum
    );

    // Save mid-combat
    let save_path = "encounter.json";
    encounter.save_session(save_path).unwrap();
    println!("\n✅ Encounter saved to: {}", save_path);

    // Load from file
    let loaded_encounter = CombatEncounter::load_session(save_path).unwrap();
    println!("✅ Encounter loaded from: {}", save_path);
    println!("\nLoaded encounter state:");
    println!("  Round: {}", loaded_encounter.round);
    println!("  Combatants: {}", loaded_encounter.combatants.len());
    println!("  Turn order: {:?}", loaded_encounter.turn_order);

    println!("\nCombatant details:");
    for combatant in &loaded_encounter.combatants {
        println!(
            "  - {} ({}): HP {}/{}, Initiative {}",
            combatant.name,
            if combatant.is_player {
                "Player"
            } else {
                "Enemy"
            },
            combatant.hp.current,
            combatant.hp.maximum,
            combatant.initiative
        );
    }

    // Clean up
    std::fs::remove_file(save_path).ok();
}

fn example_persistence() {
    println!("EXAMPLE 4: Persistence Across Sessions\n");

    println!("Simulating a multi-session campaign:\n");

    // Session 1: Create character and gain some XP
    println!("📖 Session 1:");
    let mut warrior = Combatant::player(
        "Theron",
        1,
        Class::Ranger,
        Ancestry::Faerie,
        Attributes::from_array([2, 1, 1, 0, 0, -1]).unwrap(),
    );

    let mut progress = CharacterProgress::new();
    progress.add_experience(120);
    progress.level_up().unwrap();

    println!(
        "  Character created: {} (Level {})",
        warrior.name, progress.level
    );
    println!("  XP gained: {}", progress.experience);

    // Take some damage in combat
    warrior.take_damage(5);
    println!(
        "  Took damage, HP: {}/{}",
        warrior.hp.current, warrior.hp.maximum
    );

    // Save state
    warrior.save_to_file("theron_char.json").unwrap();
    progress.save_to_file("theron_progress.json").unwrap();
    println!("  ✅ Session saved!\n");

    // Session 2: Load and continue
    println!("📖 Session 2 (next week):");
    let mut warrior = Combatant::load_from_file("theron_char.json").unwrap();
    let mut progress = CharacterProgress::load_from_file("theron_progress.json").unwrap();

    println!(
        "  Loaded character: {} (Level {})",
        warrior.name, progress.level
    );
    println!(
        "  Current HP: {}/{}",
        warrior.hp.current, warrior.hp.maximum
    );
    println!("  XP: {}", progress.experience);

    // Rest and heal
    warrior.hp.current = warrior.hp.maximum;
    println!("  Rested and healed to full HP\n");

    // More adventure!
    progress.add_experience(250);
    if progress.can_level_up() {
        progress.level_up().unwrap();
        progress.add_card("twin_shot");
        println!("  🎉 LEVEL UP! Now level {}", progress.level);
        println!("  Learned new ability: twin_shot");
    }

    // Save again
    warrior.save_to_file("theron_char.json").unwrap();
    progress.save_to_file("theron_progress.json").unwrap();
    println!("  ✅ Session saved!\n");

    // Session 3: Load one more time
    println!("📖 Session 3:");
    let warrior = Combatant::load_from_file("theron_char.json").unwrap();
    let progress = CharacterProgress::load_from_file("theron_progress.json").unwrap();

    println!("  Character: {} (Level {})", warrior.name, progress.level);
    println!("  HP: {}/{}", warrior.hp.current, warrior.hp.maximum);
    println!("  Available cards: {:?}", progress.available_cards);
    println!("\n🎮 Ready for more adventures!");

    // Clean up
    std::fs::remove_file("theron_char.json").ok();
    std::fs::remove_file("theron_progress.json").ok();

    println!("\n💡 TIP: JSON files can be version-controlled or backed up!");
    println!("   You can even manually edit them if needed.");
}
