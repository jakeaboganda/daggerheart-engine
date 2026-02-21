//! Example: Working with classes and domains
//!
//! This example demonstrates Daggerheart classes and their domain access.

use daggerheart_engine::character::{Class, Domain};
use strum::IntoEnumIterator;

fn main() {
    println!("=== Daggerheart Classes & Domains ===\n");

    // Basic class information
    println!("1. Class basics:");
    let bard = Class::Bard;
    println!("Class: {}", bard);
    println!("Starting HP: {}", bard.starting_hp());
    println!("Starting Evasion: {}", bard.starting_evasion());

    let (domain1, domain2) = bard.domains();
    println!("Domains: {} and {}", domain1, domain2);

    // Check domain access
    println!("\n2. Checking domain access:");
    println!("Can Bard use Codex? {}", bard.can_use_domain(Domain::Codex));
    println!("Can Bard use Grace? {}", bard.can_use_domain(Domain::Grace));
    println!("Can Bard use Blade? {}", bard.can_use_domain(Domain::Blade));

    // All classes
    println!("\n3. All classes and their domains:");
    for class in Class::iter() {
        let (d1, d2) = class.domains();
        let hp = class.starting_hp();
        let evasion = class.starting_evasion();
        println!(
            "  {:10} - {} & {} | HP: {} | Evasion: {}",
            class.to_string(),
            d1,
            d2,
            hp,
            evasion
        );
    }

    // Compare different classes
    println!("\n4. Comparing two classes:");
    let warrior = Class::Warrior;
    let wizard = Class::Wizard;

    println!("\n{} (frontline fighter):", warrior);
    println!("  Domains: {:?}", warrior.domains());
    println!(
        "  Evasion: {} (armored, lower dodge)",
        warrior.starting_evasion()
    );

    println!("\n{} (backline caster):", wizard);
    println!("  Domains: {:?}", wizard.domains());
    println!(
        "  Evasion: {} (unarmored, must dodge)",
        wizard.starting_evasion()
    );

    // Find classes by domain
    println!("\n5. Which classes can use each domain?");
    for domain in Domain::iter() {
        print!("  {} domain: ", domain);
        let classes_with_domain: Vec<String> = Class::iter()
            .filter(|c| c.can_use_domain(domain))
            .map(|c| c.to_string())
            .collect();
        println!("{}", classes_with_domain.join(", "));
    }

    // Class archetypes
    println!("\n6. Class archetypes:");

    println!("\nFrontline fighters (Blade domain):");
    for class in Class::iter() {
        if class.can_use_domain(Domain::Blade) {
            println!("  • {}", class);
        }
    }

    println!("\nMagic users (Arcana domain):");
    for class in Class::iter() {
        if class.can_use_domain(Domain::Arcana) {
            println!("  • {}", class);
        }
    }

    println!("\nSocial characters (Grace or Splendor):");
    for class in Class::iter() {
        if class.can_use_domain(Domain::Grace) || class.can_use_domain(Domain::Splendor) {
            println!("  • {}", class);
        }
    }

    // Party composition example
    println!("\n7. Example party composition:");
    let party = vec![
        Class::Guardian, // Tank
        Class::Wizard,   // Damage
        Class::Seraph,   // Healer
        Class::Rogue,    // Utility
    ];

    println!("\nBalanced 4-player party:");
    for (i, class) in party.iter().enumerate() {
        let (d1, d2) = class.domains();
        println!("  Player {}: {} ({} + {})", i + 1, class, d1, d2);
    }

    // Domain coverage analysis
    let mut domain_count: std::collections::HashMap<Domain, usize> =
        std::collections::HashMap::new();
    for class in &party {
        let (d1, d2) = class.domains();
        *domain_count.entry(d1).or_insert(0) += 1;
        *domain_count.entry(d2).or_insert(0) += 1;
    }

    println!("\nParty domain coverage:");
    for domain in Domain::iter() {
        let count = domain_count.get(&domain).unwrap_or(&0);
        println!("  {}: {} {}", domain, "█".repeat(*count), count);
    }

    println!("\n=== Class & Domain System Demonstration Complete ===");
}
