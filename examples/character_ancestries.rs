//! Example: Working with ancestries
//!
//! This example demonstrates Daggerheart ancestries and their traits.

use daggerheart_engine::character::Ancestry;
use strum::IntoEnumIterator;

fn main() {
    println!("=== Daggerheart Ancestries ===\n");

    // Basic ancestry information
    println!("1. Ancestry basics:");
    let human = Ancestry::Human;
    println!("Ancestry: {}", human);
    println!("HP modifier: {:+}", human.hp_modifier());
    println!("Evasion modifier: {:+}", human.evasion_modifier());
    println!("Can fly: {}", human.has_flight());
    println!("Foundation abilities: {:?}", human.foundation_abilities());

    // Special ancestries
    println!("\n2. Special ancestries with modifiers:");

    let giant = Ancestry::Giant;
    println!("\n{} - The mighty ones", giant);
    println!(
        "  HP modifier: {:+} (starts with 7 HP instead of 6)",
        giant.hp_modifier()
    );
    println!("  Evasion: {:+}", giant.evasion_modifier());
    println!("  Abilities: {:?}", giant.foundation_abilities());

    let simiah = Ancestry::Simiah;
    println!("\n{} - The nimble climbers", simiah);
    println!("  HP modifier: {:+}", simiah.hp_modifier());
    println!(
        "  Evasion: {:+} (nimble and agile)",
        simiah.evasion_modifier()
    );
    println!("  Abilities: {:?}", simiah.foundation_abilities());

    let faerie = Ancestry::Faerie;
    println!("\n{} - The fey folk", faerie);
    println!("  HP modifier: {:+}", faerie.hp_modifier());
    println!("  Evasion: {:+}", faerie.evasion_modifier());
    println!("  Can fly: {} (innate flight!)", faerie.has_flight());
    println!("  Abilities: {:?}", faerie.foundation_abilities());

    // All ancestries
    println!("\n3. All {} ancestries:", Ancestry::iter().count());
    for ancestry in Ancestry::iter() {
        let hp_mod = ancestry.hp_modifier();
        let ev_mod = ancestry.evasion_modifier();
        let flight = if ancestry.has_flight() { "✈" } else { " " };
        let abilities = ancestry.foundation_abilities();

        println!(
            "  {:12} {} | HP:{:+2} Evasion:{:+2} | {}",
            ancestry.to_string(),
            flight,
            hp_mod,
            ev_mod,
            abilities.join(", ")
        );
    }

    // Calculate final stats with ancestry
    println!("\n4. Calculating final character stats:");

    // Base class stats
    let base_hp = 6;
    let base_evasion = 12; // Example: Rogue class

    println!("\nBase stats (Rogue class):");
    println!("  HP: {}", base_hp);
    println!("  Evasion: {}", base_evasion);

    println!("\nWith different ancestries:");

    for ancestry in [Ancestry::Human, Ancestry::Giant, Ancestry::Simiah].iter() {
        let final_hp = (base_hp as i8 + ancestry.hp_modifier()) as u8;
        let final_evasion = (base_evasion as i8 + ancestry.evasion_modifier()) as u8;

        println!("\n  {} Rogue:", ancestry);
        println!(
            "    HP: {} (base) + {:+} (ancestry) = {}",
            base_hp,
            ancestry.hp_modifier(),
            final_hp
        );
        println!(
            "    Evasion: {} (base) + {:+} (ancestry) = {}",
            base_evasion,
            ancestry.evasion_modifier(),
            final_evasion
        );
    }

    // Ancestry themes
    println!("\n5. Ancestry themes and archetypes:");

    println!("\nNaturally tough (HP bonuses):");
    for ancestry in Ancestry::iter() {
        if ancestry.hp_modifier() > 0 {
            println!("  • {} ({:+} HP)", ancestry, ancestry.hp_modifier());
        }
    }

    println!("\nNaturally nimble (Evasion bonuses):");
    for ancestry in Ancestry::iter() {
        if ancestry.evasion_modifier() > 0 {
            println!(
                "  • {} ({:+} Evasion)",
                ancestry,
                ancestry.evasion_modifier()
            );
        }
    }

    println!("\nAirborne (flight):");
    for ancestry in Ancestry::iter() {
        if ancestry.has_flight() {
            println!("  • {} (natural flight)", ancestry);
        }
    }

    // Unique abilities showcase
    println!("\n6. Notable foundation abilities:");

    let highlights = vec![
        (Ancestry::Clank, "Constructed beings with Repair Protocol"),
        (Ancestry::Drakona, "Dragon ancestry with Breath Weapon"),
        (Ancestry::Goblin, "Nimble Escape for tactical retreats"),
        (Ancestry::Halfling, "Lucky trait for rerolls"),
        (Ancestry::Orc, "Relentless Endurance to stay in the fight"),
        (Ancestry::Ribbet, "Amphibious and powerful Leap"),
    ];

    for (ancestry, description) in highlights {
        println!("\n  {}:", ancestry);
        println!("    → {}", description);
        println!("    Abilities: {:?}", ancestry.foundation_abilities());
    }

    // Character concept examples
    println!("\n7. Character concept examples:");

    println!("\n  Giant Guardian:");
    println!("    • High HP from both class and ancestry");
    println!("    • Mighty Grip for grappling");
    println!("    • Perfect tank build");

    println!("\n  Simiah Rogue:");
    println!("    • Extra Evasion stacks with class bonus");
    println!("    • Prehensile Tail and Climbing");
    println!("    • Excellent scout/infiltrator");

    println!("\n  Faerie Ranger:");
    println!("    • Natural flight for mobility");
    println!("    • Fey Magic for utility");
    println!("    • Hit-and-run tactics");

    println!("\n  Halfling Bard:");
    println!("    • Lucky for clutch rolls");
    println!("    • Brave for resisting fear");
    println!("    • Perfect party face");

    println!("\n=== Ancestry System Demonstration Complete ===");
}
