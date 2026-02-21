//! Example: Complete character creation
//!
//! This example demonstrates building complete characters by combining
//! attributes, classes, and ancestries.

use daggerheart_engine::character::{Ancestry, Attributes, Class};

fn main() {
    println!("=== Daggerheart Character Creation ===\n");

    // Character 1: Elara, Human Bard
    println!("CHARACTER 1: Elara the Inspiring\n");

    let elara_attrs = Attributes {
        presence: 2,  // Primary: Charisma for performance
        knowledge: 1, // Secondary: Lore and learning
        instinct: 1,  // Secondary: Reading the room
        agility: 0,
        finesse: 0,
        strength: -1,
    };

    let elara_class = Class::Bard;
    let elara_ancestry = Ancestry::Human;

    println!("Name: Elara the Inspiring");
    println!("Ancestry: {}", elara_ancestry);
    println!("Class: {}", elara_class);

    println!("\nAttributes:");
    println!(
        "  Presence:  {:+2} (primary: inspiring performances)",
        elara_attrs.presence
    );
    println!(
        "  Knowledge: {:+2} (secondary: vast lore)",
        elara_attrs.knowledge
    );
    println!(
        "  Instinct:  {:+2} (secondary: reading people)",
        elara_attrs.instinct
    );
    println!("  Agility:   {:+2}", elara_attrs.agility);
    println!("  Finesse:   {:+2}", elara_attrs.finesse);
    println!("  Strength:  {:+2}", elara_attrs.strength);

    let elara_hp = (elara_class.starting_hp() as i8 + elara_ancestry.hp_modifier()) as u8;
    let elara_evasion =
        (elara_class.starting_evasion() as i8 + elara_ancestry.evasion_modifier()) as u8;
    let (d1, d2) = elara_class.domains();

    println!("\nDerived Stats:");
    println!("  HP: {}", elara_hp);
    println!("  Evasion: {}", elara_evasion);
    println!("  Domains: {} & {}", d1, d2);

    println!("\nAncestry Abilities:");
    for ability in elara_ancestry.foundation_abilities() {
        println!("  • {}", ability);
    }

    println!("\nConcept: A charismatic human bard who uses music and stories");
    println!("         to inspire allies and demoralize foes. Versatile and");
    println!("         adaptable, drawing on a wide knowledge base.");

    // Character 2: Grunk, Giant Guardian
    println!();
    println!("{}", "=".repeat(60));
    println!();
    println!("CHARACTER 2: Grunk the Unyielding\n");

    let grunk_attrs = Attributes {
        strength: 2, // Primary: raw power
        presence: 1, // Secondary: intimidating
        agility: 1,  // Secondary: surprising mobility
        finesse: 0,
        instinct: 0,
        knowledge: -1,
    };

    let grunk_class = Class::Guardian;
    let grunk_ancestry = Ancestry::Giant;

    println!("Name: Grunk the Unyielding");
    println!("Ancestry: {}", grunk_ancestry);
    println!("Class: {}", grunk_class);

    println!("\nAttributes:");
    println!(
        "  Strength:  {:+2} (primary: devastating blows)",
        grunk_attrs.strength
    );
    println!(
        "  Presence:  {:+2} (secondary: terrifying)",
        grunk_attrs.presence
    );
    println!(
        "  Agility:   {:+2} (secondary: quick for size)",
        grunk_attrs.agility
    );
    println!("  Finesse:   {:+2}", grunk_attrs.finesse);
    println!("  Instinct:  {:+2}", grunk_attrs.instinct);
    println!("  Knowledge: {:+2}", grunk_attrs.knowledge);

    let grunk_hp = (grunk_class.starting_hp() as i8 + grunk_ancestry.hp_modifier()) as u8;
    let grunk_evasion =
        (grunk_class.starting_evasion() as i8 + grunk_ancestry.evasion_modifier()) as u8;
    let (d1, d2) = grunk_class.domains();

    println!("\nDerived Stats:");
    println!("  HP: {} (6 base + 1 Giant bonus!)", grunk_hp);
    println!("  Evasion: {}", grunk_evasion);
    println!("  Domains: {} & {}", d1, d2);

    println!("\nAncestry Abilities:");
    for ability in grunk_ancestry.foundation_abilities() {
        println!("  • {}", ability);
    }

    println!("\nConcept: A towering giant who serves as an unbreakable wall");
    println!("         between allies and danger. Uses Mighty Grip to control");
    println!("         the battlefield and protect the weak.");

    // Character 3: Whisper, Simiah Rogue
    println!();
    println!("{}", "=".repeat(60));
    println!();
    println!("CHARACTER 3: Whisper the Shadow\n");

    let whisper_attrs = Attributes {
        agility: 2,  // Primary: speed and stealth
        finesse: 1,  // Secondary: precise strikes
        instinct: 1, // Secondary: danger sense
        strength: 0,
        presence: 0,
        knowledge: -1,
    };

    let whisper_class = Class::Rogue;
    let whisper_ancestry = Ancestry::Simiah;

    println!("Name: Whisper the Shadow");
    println!("Ancestry: {}", whisper_ancestry);
    println!("Class: {}", whisper_class);

    println!("\nAttributes:");
    println!(
        "  Agility:   {:+2} (primary: lightning reflexes)",
        whisper_attrs.agility
    );
    println!(
        "  Finesse:   {:+2} (secondary: surgical strikes)",
        whisper_attrs.finesse
    );
    println!(
        "  Instinct:  {:+2} (secondary: sixth sense)",
        whisper_attrs.instinct
    );
    println!("  Strength:  {:+2}", whisper_attrs.strength);
    println!("  Presence:  {:+2}", whisper_attrs.presence);
    println!("  Knowledge: {:+2}", whisper_attrs.knowledge);

    let whisper_hp = (whisper_class.starting_hp() as i8 + whisper_ancestry.hp_modifier()) as u8;
    let whisper_evasion =
        (whisper_class.starting_evasion() as i8 + whisper_ancestry.evasion_modifier()) as u8;
    let (d1, d2) = whisper_class.domains();

    println!("\nDerived Stats:");
    println!("  HP: {}", whisper_hp);
    println!("  Evasion: {} (14 base + 1 Simiah bonus!)", whisper_evasion);
    println!("  Domains: {} & {}", d1, d2);

    println!("\nAncestry Abilities:");
    for ability in whisper_ancestry.foundation_abilities() {
        println!("  • {}", ability);
    }

    println!("\nConcept: A monkey-like rogue who uses tail and climbing ability");
    println!("         to access impossible positions. Exceptional evasion");
    println!("         makes them nearly untouchable.");

    // Character 4: Spark, Faerie Wizard
    println!();
    println!("{}", "=".repeat(60));
    println!();
    println!("CHARACTER 4: Spark the Brilliant\n");

    let spark_attrs = Attributes {
        knowledge: 2, // Primary: magical theory
        instinct: 1,  // Secondary: magical intuition
        presence: 1,  // Secondary: force of will
        agility: 0,
        finesse: 0,
        strength: -1,
    };

    let spark_class = Class::Wizard;
    let spark_ancestry = Ancestry::Faerie;

    println!("Name: Spark the Brilliant");
    println!("Ancestry: {}", spark_ancestry);
    println!("Class: {}", spark_class);

    println!("\nAttributes:");
    println!(
        "  Knowledge: {:+2} (primary: arcane mastery)",
        spark_attrs.knowledge
    );
    println!(
        "  Instinct:  {:+2} (secondary: magical sense)",
        spark_attrs.instinct
    );
    println!(
        "  Presence:  {:+2} (secondary: commanding)",
        spark_attrs.presence
    );
    println!("  Agility:   {:+2}", spark_attrs.agility);
    println!("  Finesse:   {:+2}", spark_attrs.finesse);
    println!("  Strength:  {:+2}", spark_attrs.strength);

    let spark_hp = (spark_class.starting_hp() as i8 + spark_ancestry.hp_modifier()) as u8;
    let spark_evasion =
        (spark_class.starting_evasion() as i8 + spark_ancestry.evasion_modifier()) as u8;
    let (d1, d2) = spark_class.domains();

    println!("\nDerived Stats:");
    println!("  HP: {}", spark_hp);
    println!("  Evasion: {}", spark_evasion);
    println!("  Domains: {} & {}", d1, d2);
    println!("  Flight: {} (innate!)", spark_ancestry.has_flight());

    println!("\nAncestry Abilities:");
    for ability in spark_ancestry.foundation_abilities() {
        println!("  • {}", ability);
    }

    println!("\nConcept: A tiny faerie wizard who combines arcane study with");
    println!("         natural fey magic. Flight provides superior positioning");
    println!("         for devastating spells.");

    // Party summary
    println!();
    println!("{}", "=".repeat(60));
    println!();
    println!("PARTY SUMMARY\n");

    println!("This balanced party covers all bases:\n");

    println!("  Elara (Human Bard):");
    println!("    Role: Support / Social");
    println!("    Strengths: Buffs allies, excellent in social situations");

    println!("\n  Grunk (Giant Guardian):");
    println!("    Role: Tank / Frontline");
    println!("    Strengths: High HP, controls battlefield, protects allies");

    println!("\n  Whisper (Simiah Rogue):");
    println!("    Role: Scout / Striker");
    println!("    Strengths: Extreme evasion, mobility, precision damage");

    println!("\n  Spark (Faerie Wizard):");
    println!("    Role: Damage / Control");
    println!("    Strengths: Powerful magic, flight, area control");

    println!("\nDomain coverage:");
    let all_domains = [
        elara_class.domains(),
        grunk_class.domains(),
        whisper_class.domains(),
        spark_class.domains(),
    ];

    for (name, (d1, d2)) in [
        ("Elara", all_domains[0]),
        ("Grunk", all_domains[1]),
        ("Whisper", all_domains[2]),
        ("Spark", all_domains[3]),
    ] {
        println!("  {} - {} & {}", name, d1, d2);
    }

    println!("\n=== Character Creation Complete ===");
}
