//! Daggerheart CLI - Command-line interface for the Daggerheart engine
//!
//! This CLI lets you:
//! - Create and manage characters
//! - Roll dice (basic, duality, damage)
//! - Run combat simulations
//! - Save and load game state

use clap::{Parser, Subcommand};
use daggerheart_engine::character::{Ancestry, Attributes, CharacterProgress, Class};
use daggerheart_engine::combat::simulation::{CombatEncounter, Combatant};
use daggerheart_engine::core::dice::{ControllingDie, DamageDice, Die, DualityResult, DualityRoll};

#[derive(Parser)]
#[command(name = "daggerheart")]
#[command(about = "Daggerheart TTRPG Rules Engine CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create and manage characters
    #[command(subcommand)]
    Char(CharCommands),

    /// Roll dice
    #[command(subcommand)]
    Roll(RollCommands),

    /// Manage combat encounters
    #[command(subcommand)]
    Combat(CombatCommands),

    /// List available classes
    Classes,

    /// List available ancestries
    Ancestries,
}

#[derive(Subcommand)]
enum CharCommands {
    /// Create a new character
    Create {
        /// Character name
        name: String,

        /// Class (e.g., Warrior, Bard, Ranger)
        #[arg(short, long)]
        class: String,

        /// Ancestry (e.g., Human, Orc, Dwarf)
        #[arg(short, long)]
        ancestry: String,

        /// Level (default: 1)
        #[arg(short, long, default_value = "1")]
        level: u8,

        /// Output file (default: <name>.json)
        #[arg(short, long)]
        output: Option<String>,

        /// Attribute values (e.g., "2,1,1,0,0,-1")
        #[arg(long, default_value = "2,1,1,0,0,-1")]
        attributes: String,
    },

    /// Show character details
    Show {
        /// Character file
        file: String,
    },

    /// Level up a character
    LevelUp {
        /// Character progress file
        file: String,

        /// Card to add when leveling up
        #[arg(short, long)]
        card: Option<String>,
    },

    /// Add XP to character
    AddXp {
        /// Character progress file
        file: String,

        /// Amount of XP to add
        amount: u32,
    },
}

#[derive(Subcommand)]
enum RollCommands {
    /// Roll a basic die (d4, d6, d8, d10, d12, d20)
    Die {
        /// Die type (e.g., d20, d6, d8)
        die: String,

        /// Number of times to roll
        #[arg(short, long, default_value = "1")]
        count: usize,
    },

    /// Roll duality dice (2d12 Hope vs Fear)
    Duality {
        /// Modifier to add to the roll
        #[arg(default_value = "0")]
        modifier: i8,

        /// Roll with advantage (extra d6)
        #[arg(short, long)]
        advantage: bool,
    },

    /// Roll damage dice
    Damage {
        /// Damage dice (e.g., 2d6, 1d8+3, 2d6+1d4+2)
        dice: String,
    },
}

#[derive(Subcommand)]
enum CombatCommands {
    /// Create a new combat encounter
    New {
        /// Hope pool maximum
        #[arg(short = 'H', long, default_value = "5")]
        hope: u8,

        /// Output file
        #[arg(short, long)]
        output: String,
    },

    /// Add a combatant to an encounter
    Add {
        /// Encounter file
        file: String,

        /// Character file to add
        #[arg(short, long)]
        character: Option<String>,

        /// Or create an enemy with name
        #[arg(short = 'e', long)]
        enemy: Option<String>,

        /// Enemy level
        #[arg(long, default_value = "1")]
        level: u8,

        /// Enemy HP
        #[arg(long, default_value = "4")]
        hp: u8,

        /// Enemy evasion
        #[arg(long, default_value = "13")]
        evasion: u8,

        /// Enemy armor
        #[arg(long, default_value = "0")]
        armor: u8,
    },

    /// Start combat (roll initiative)
    Start {
        /// Encounter file
        file: String,
    },

    /// Show encounter status
    Status {
        /// Encounter file
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Char(cmd) => handle_char_command(cmd),
        Commands::Roll(cmd) => handle_roll_command(cmd),
        Commands::Combat(cmd) => handle_combat_command(cmd),
        Commands::Classes => {
            list_classes();
            Ok(())
        }
        Commands::Ancestries => {
            list_ancestries();
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_char_command(cmd: CharCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        CharCommands::Create {
            name,
            class,
            ancestry,
            level,
            output,
            attributes,
        } => {
            // Parse class
            let class = parse_class(&class)?;

            // Parse ancestry
            let ancestry = parse_ancestry(&ancestry)?;

            // Parse attributes
            let attr_values: Vec<i8> = attributes
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<_>, _>>()?;

            if attr_values.len() != 6 {
                return Err("Attributes must have exactly 6 values".into());
            }

            let mut attr_array = [0i8; 6];
            attr_array.copy_from_slice(&attr_values);
            let attributes = Attributes::from_array(attr_array)?;

            // Create character
            let character = Combatant::player(name.clone(), level, class, ancestry, attributes);

            // Create initial progress
            let progress = CharacterProgress::new();

            // Save files
            let char_file = output.unwrap_or_else(|| format!("{}_char.json", name));
            let progress_file = format!("{}_progress.json", name);

            character.save_to_file(&char_file)?;
            progress.save_to_file(&progress_file)?;

            println!("✅ Character created!");
            println!("  Name: {}", character.name);
            println!("  Class: {}", character.class);
            println!("  Ancestry: {}", character.ancestry);
            println!("  Level: {}", character.level);
            println!("  HP: {}/{}", character.hp.current, character.hp.maximum);
            println!("  Evasion: {}", character.evasion);
            println!("\n📁 Files saved:");
            println!("  Character: {}", char_file);
            println!("  Progress: {}", progress_file);
        }

        CharCommands::Show { file } => {
            let character = Combatant::load_from_file(&file)?;

            println!("=== {} ===", character.name);
            println!("  Class: {}", character.class);
            println!("  Ancestry: {}", character.ancestry);
            println!("  Level: {}", character.level);
            println!();
            println!("  HP: {}/{}", character.hp.current, character.hp.maximum);
            println!("  Stress: {}/5", character.stress.current);
            println!("  Evasion: {}", character.evasion);
            println!("  Armor: {}", character.armor);
            println!();
            println!("Attributes:");
            println!("  Agility:   {:+}", character.attributes.agility);
            println!("  Strength:  {:+}", character.attributes.strength);
            println!("  Finesse:   {:+}", character.attributes.finesse);
            println!("  Instinct:  {:+}", character.attributes.instinct);
            println!("  Presence:  {:+}", character.attributes.presence);
            println!("  Knowledge: {:+}", character.attributes.knowledge);

            // Try to load progress file
            let progress_file = file.replace("_char.json", "_progress.json");
            if let Ok(progress) = CharacterProgress::load_from_file(&progress_file) {
                println!();
                println!("Progress:");
                println!("  Level: {}", progress.level);
                println!(
                    "  XP: {} / {}",
                    progress.experience,
                    progress.xp_for_next_level()
                );
                println!("  Cards: {:?}", progress.available_cards);
            }
        }

        CharCommands::LevelUp { file, card } => {
            let mut progress = CharacterProgress::load_from_file(&file)?;

            if !progress.can_level_up() {
                println!(
                    "❌ Not enough XP to level up (need {}, have {})",
                    progress.xp_for_next_level(),
                    progress.experience
                );
                return Ok(());
            }

            let old_level = progress.level;
            progress.level_up()?;

            println!("🎉 LEVEL UP!");
            println!("  {} → {}", old_level, progress.level);
            println!("  Remaining XP: {}", progress.experience);

            if let Some(card_id) = card {
                progress.add_card(&card_id);
                println!("  ✨ Learned: {}", card_id);
            }

            progress.save_to_file(&file)?;
            println!("\n✅ Progress saved to {}", file);
        }

        CharCommands::AddXp { file, amount } => {
            let mut progress = CharacterProgress::load_from_file(&file)?;

            progress.add_experience(amount);
            println!("📈 Added {} XP", amount);
            println!("  Total XP: {}", progress.experience);
            println!("  Level: {}", progress.level);
            println!("  XP for next level: {}", progress.xp_for_next_level());

            if progress.can_level_up() {
                println!("\n💡 You can now level up! Run:");
                println!("   daggerheart char level-up {}", file);
            }

            progress.save_to_file(&file)?;
            println!("\n✅ Progress saved");
        }
    }

    Ok(())
}

fn handle_roll_command(cmd: RollCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        RollCommands::Die { die, count } => {
            let die_type = parse_die(&die)?;

            println!("🎲 Rolling {}x{}:", count, die);
            let mut total = 0;
            let mut rolls = Vec::new();

            for i in 1..=count {
                let roll = die_type.roll();
                total += roll;
                rolls.push(roll);

                if count <= 10 {
                    println!("  Roll {}: {}", i, roll);
                }
            }

            if count > 1 {
                println!("\nRolls: {:?}", rolls);
                println!("Total: {}", total);
                println!("Average: {:.2}", total as f64 / count as f64);
            }
        }

        RollCommands::Duality {
            modifier,
            advantage,
        } => {
            let roll = DualityRoll::roll();
            let result = if advantage {
                // Combine advantage and modifier manually
                let adv_result = roll.with_advantage();
                let total = (adv_result.total as i16 + modifier as i16) as u16;
                DualityResult {
                    roll: adv_result.roll,
                    modifier,
                    advantage_die: adv_result.advantage_die,
                    total,
                    controlling: adv_result.controlling,
                    is_critical: adv_result.is_critical,
                }
            } else {
                roll.with_modifier(modifier)
            };

            println!("🎲 Duality Roll:");
            println!("  Hope die: {}", roll.hope);
            println!("  Fear die: {}", roll.fear);

            if let Some(d6) = result.advantage_die {
                println!("  Advantage die: {}", d6);
            }

            if modifier != 0 {
                println!("  Modifier: {:+}", modifier);
            }

            println!("  Total: {}", result.total);

            if result.is_critical {
                println!("\n🌟 CRITICAL! (Doubles: {})", roll.hope);
            }

            // Determine success type (assuming difficulty is met)
            match result.controlling {
                ControllingDie::Hope => println!("\n✅ Hope controls! 🌟"),
                ControllingDie::Fear => println!("\n⚠️ Fear controls... 💀"),
                ControllingDie::Tied => println!("\n🔄 Tied"),
            }
        }

        RollCommands::Damage { dice } => {
            // Parse damage dice string (e.g., "2d6+3", "1d8+1d4+2")
            let damage_dice = parse_damage_dice(&dice)?;
            let result = damage_dice.roll();

            println!("🎲 Damage Roll: {}", dice);
            println!("  Individual rolls: {:?}", result.rolls);
            println!("  Total damage: {}", result.total);
        }
    }

    Ok(())
}

fn handle_combat_command(cmd: CombatCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        CombatCommands::New { hope, output } => {
            let encounter = CombatEncounter::new(hope);
            encounter.save_session(&output)?;

            println!("✅ Combat encounter created!");
            println!("  Hope pool: {}", hope);
            println!("  File: {}", output);
            println!("\n💡 Add combatants with:");
            println!("   daggerheart combat add {} --character <file>", output);
            println!("   daggerheart combat add {} --enemy <name>", output);
        }

        CombatCommands::Add {
            file,
            character,
            enemy,
            level,
            hp,
            evasion,
            armor,
        } => {
            let mut encounter = CombatEncounter::load_session(&file)?;

            if let Some(char_file) = character {
                let combatant = Combatant::load_from_file(&char_file)?;
                println!("➕ Adding player: {}", combatant.name);
                encounter.add_combatant(combatant);
            } else if let Some(enemy_name) = enemy {
                let combatant = Combatant::enemy(enemy_name.clone(), level, hp, evasion, armor);
                println!("➕ Adding enemy: {}", enemy_name);
                println!("  HP: {}, Evasion: {}, Armor: {}", hp, evasion, armor);
                encounter.add_combatant(combatant);
            } else {
                return Err("Must specify either --character or --enemy".into());
            }

            encounter.save_session(&file)?;
            println!(
                "\n✅ Combatant added! Total: {}",
                encounter.combatants.len()
            );
        }

        CombatCommands::Start { file } => {
            let mut encounter = CombatEncounter::load_session(&file)?;

            if encounter.combatants.is_empty() {
                return Err("No combatants in encounter!".into());
            }

            encounter.start();

            println!("⚔️ Combat started!");
            println!("\nInitiative order:");
            for (i, &idx) in encounter.turn_order.iter().enumerate() {
                let combatant = &encounter.combatants[idx];
                println!(
                    "  {}. {} (Initiative: {})",
                    i + 1,
                    combatant.name,
                    combatant.initiative
                );
            }

            encounter.save_session(&file)?;
            println!("\n✅ Encounter saved");
        }

        CombatCommands::Status { file } => {
            let encounter = CombatEncounter::load_session(&file)?;

            println!("=== Combat Status ===");
            println!("Round: {}", encounter.round);
            println!(
                "Hope: {}/{}",
                encounter.hope.current, encounter.hope.maximum
            );
            println!("Fear: {}", encounter.fear.current);
            println!();

            if encounter.is_over() {
                if let Some(victory) = encounter.player_victory() {
                    if victory {
                        println!("🎉 VICTORY! Players won!");
                    } else {
                        println!("💀 DEFEAT! Enemies won!");
                    }
                }
            } else if encounter.round > 0 {
                if let Some(current) = encounter.current_combatant() {
                    println!("Current turn: {}", current.name);
                    println!();
                }
            }

            println!("Combatants:");
            for combatant in &encounter.combatants {
                let status = if combatant.is_alive() {
                    "Alive"
                } else {
                    "Dead"
                };
                println!(
                    "  {} [{}] - HP: {}/{}, Evasion: {}, Armor: {}",
                    combatant.name,
                    status,
                    combatant.hp.current,
                    combatant.hp.maximum,
                    combatant.evasion,
                    combatant.armor
                );
            }
        }
    }

    Ok(())
}

fn list_classes() {
    println!("Available Classes:\n");
    let classes = [
        ("Bard", "Charismatic performer, Presence + Knowledge"),
        ("Druid", "Nature shapeshifter, Instinct + Agility"),
        ("Guardian", "Protective defender, Strength + Presence"),
        ("Ranger", "Wilderness expert, Agility + Instinct"),
        ("Rogue", "Cunning trickster, Finesse + Agility"),
        ("Seraph", "Divine warrior, Presence + Strength"),
        ("Sorcerer", "Arcane caster, Knowledge + Instinct"),
        ("Warrior", "Combat master, Strength + Agility"),
        ("Wizard", "Scholarly mage, Knowledge + Finesse"),
    ];

    for (name, desc) in classes {
        println!("  {:<12} - {}", name, desc);
    }
}

fn list_ancestries() {
    println!("Available Ancestries:\n");
    let ancestries = [
        "Clank", "Daemon", "Drakona", "Dwarf", "Faerie", "Faun", "Fungril", "Galapa", "Giant",
        "Goblin", "Halfling", "Human", "Inferis", "Katari", "Orc", "Ribbet", "Simiah",
    ];

    for (i, ancestry) in ancestries.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!();
        }
        print!("  {:<12}", ancestry);
    }
    println!("\n");
}

fn parse_class(s: &str) -> Result<Class, Box<dyn std::error::Error>> {
    match s.to_lowercase().as_str() {
        "bard" => Ok(Class::Bard),
        "druid" => Ok(Class::Druid),
        "guardian" => Ok(Class::Guardian),
        "ranger" => Ok(Class::Ranger),
        "rogue" => Ok(Class::Rogue),
        "seraph" => Ok(Class::Seraph),
        "sorcerer" => Ok(Class::Sorcerer),
        "warrior" => Ok(Class::Warrior),
        "wizard" => Ok(Class::Wizard),
        _ => Err(format!(
            "Unknown class: {}. Run 'daggerheart classes' to see available classes.",
            s
        )
        .into()),
    }
}

fn parse_ancestry(s: &str) -> Result<Ancestry, Box<dyn std::error::Error>> {
    match s.to_lowercase().as_str() {
        "clank" => Ok(Ancestry::Clank),
        "daemon" => Ok(Ancestry::Daemon),
        "drakona" => Ok(Ancestry::Drakona),
        "dwarf" => Ok(Ancestry::Dwarf),
        "faerie" => Ok(Ancestry::Faerie),
        "faun" => Ok(Ancestry::Faun),
        "fungril" => Ok(Ancestry::Fungril),
        "galapa" => Ok(Ancestry::Galapa),
        "giant" => Ok(Ancestry::Giant),
        "goblin" => Ok(Ancestry::Goblin),
        "halfling" => Ok(Ancestry::Halfling),
        "human" => Ok(Ancestry::Human),
        "inferis" => Ok(Ancestry::Inferis),
        "katari" => Ok(Ancestry::Katari),
        "orc" => Ok(Ancestry::Orc),
        "ribbet" => Ok(Ancestry::Ribbet),
        "simiah" => Ok(Ancestry::Simiah),
        _ => Err(format!(
            "Unknown ancestry: {}. Run 'daggerheart ancestries' to see available ancestries.",
            s
        )
        .into()),
    }
}

fn parse_die(s: &str) -> Result<Die, Box<dyn std::error::Error>> {
    match s.to_lowercase().as_str() {
        "d4" => Ok(Die::D4),
        "d6" => Ok(Die::D6),
        "d8" => Ok(Die::D8),
        "d10" => Ok(Die::D10),
        "d12" => Ok(Die::D12),
        "d20" => Ok(Die::D20),
        _ => Err(format!("Unknown die: {}. Valid: d4, d6, d8, d10, d12, d20", s).into()),
    }
}

fn parse_damage_dice(s: &str) -> Result<DamageDice, Box<dyn std::error::Error>> {
    // Simple parser for damage dice like "2d6+3" or "1d8+1d4+2"
    let mut dice = Vec::new();
    let mut bonus: i16 = 0;

    for part in s.split('+') {
        let part = part.trim();

        // Check if it's just a number (bonus)
        if let Ok(n) = part.parse::<i16>() {
            bonus += n;
            continue;
        }

        // Parse dice notation (e.g., "2d6")
        if let Some((count_str, die_str)) = part.split_once('d') {
            let count: usize = count_str.parse().unwrap_or(1);
            let die = parse_die(&format!("d{}", die_str))?;

            for _ in 0..count {
                dice.push(die);
            }
        }
    }

    Ok(DamageDice::new(dice).with_bonus(bonus))
}
