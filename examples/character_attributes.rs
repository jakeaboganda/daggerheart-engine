//! Example: Working with character attributes
//!
//! This example demonstrates the Daggerheart attribute system.

use daggerheart_engine::character::{AttributeType, Attributes};

fn main() {
    println!("=== Daggerheart Character Attributes ===\n");

    // Creating valid attributes
    // Must use exactly: +2, +1, +1, +0, +0, -1 in any order
    println!("1. Creating a nimble rogue:");
    let rogue_attrs = Attributes {
        agility: 2,    // Best stat for dodging and sneaking
        finesse: 1,    // Good for precise attacks
        instinct: 1,   // Good for perception
        presence: 0,   // Average charisma
        strength: 0,   // Average strength
        knowledge: -1, // Dump stat (not book-smart)
    };

    match rogue_attrs.validate() {
        Ok(_) => println!("✓ Valid attribute distribution!"),
        Err(e) => println!("✗ Invalid: {}", e),
    }

    println!("\nRogue's modifiers:");
    println!("  Agility:   {:+}", rogue_attrs.agility);
    println!("  Strength:  {:+}", rogue_attrs.strength);
    println!("  Finesse:   {:+}", rogue_attrs.finesse);
    println!("  Instinct:  {:+}", rogue_attrs.instinct);
    println!("  Presence:  {:+}", rogue_attrs.presence);
    println!("  Knowledge: {:+}", rogue_attrs.knowledge);

    // Creating a scholarly wizard
    println!("\n2. Creating a scholarly wizard:");
    let wizard_attrs = Attributes {
        knowledge: 2, // Best stat for spellcasting
        presence: 1,  // Good for social interactions
        instinct: 1,  // Good for awareness
        finesse: 0,   // Average dexterity
        agility: 0,   // Average agility
        strength: -1, // Dump stat (not physically strong)
    };

    wizard_attrs.validate().expect("Valid attributes");
    println!("✓ Valid wizard attributes!");

    // Using type-safe attribute access
    println!("\n3. Type-safe attribute access:");
    let knowledge_mod = wizard_attrs.get_modifier(AttributeType::Knowledge);
    println!("Wizard's Knowledge modifier: {:+}", knowledge_mod);

    let agility_mod = rogue_attrs.get_modifier(AttributeType::Agility);
    println!("Rogue's Agility modifier: {:+}", agility_mod);

    // Creating from array
    println!("\n4. Creating attributes from an array:");
    let mods = [2, 1, 1, 0, 0, -1]; // Standard distribution
    match Attributes::from_array(mods) {
        Ok(_attrs) => {
            println!("✓ Created valid attributes from array");
            println!("  First stat modifier: {:+}", mods[0]);
        }
        Err(e) => println!("✗ Error: {}", e),
    }

    // Invalid attribute example
    println!("\n5. Invalid attribute distributions are rejected:");
    let invalid = Attributes {
        agility: 3,  // Too high!
        strength: 2, // Also too high!
        finesse: 1,
        instinct: 0,
        presence: 0,
        knowledge: 0,
    };

    match invalid.validate() {
        Ok(_) => println!("✓ Valid (shouldn't happen!)"),
        Err(e) => println!("✗ Correctly rejected: {}", e),
    }

    // Balanced warrior
    println!("\n6. A balanced warrior build:");
    let warrior_attrs = Attributes {
        strength: 2, // Primary: hitting hard
        agility: 1,  // Secondary: dodging
        presence: 1, // Secondary: intimidation
        finesse: 0,
        instinct: 0,
        knowledge: -1,
    };

    warrior_attrs.validate().expect("Valid");

    println!("Warrior modifiers:");
    println!("  Strength (melee attacks): {:+}", warrior_attrs.strength);
    println!("  Agility (defense):        {:+}", warrior_attrs.agility);
    println!("  Presence (intimidation):  {:+}", warrior_attrs.presence);

    println!("\n=== Attribute System Demonstration Complete ===");
}
