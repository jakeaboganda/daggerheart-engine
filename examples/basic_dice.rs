//! Basic dice rolling example
//!
//! Run with: cargo run --example basic_dice

use daggerheart_engine::core::Die;

fn main() {
    println!("🎲 Daggerheart Engine - Basic Dice Example\n");

    // Roll different dice types
    println!("Rolling all dice types:");
    println!("  d4:  {}", Die::D4.roll());
    println!("  d6:  {}", Die::D6.roll());
    println!("  d8:  {}", Die::D8.roll());
    println!("  d10: {}", Die::D10.roll());
    println!("  d12: {}", Die::D12.roll());
    println!("  d20: {}", Die::D20.roll());

    println!("\nRolling a d20 for a skill check:");
    let skill_check = Die::D20.roll();
    println!("  Result: {}", skill_check);

    if skill_check >= 15 {
        println!("  ✅ Success!");
    } else {
        println!("  ❌ Failure.");
    }

    println!("\nRolling 5 d6s:");
    for i in 1..=5 {
        println!("  Roll {}: {}", i, Die::D6.roll());
    }
}
