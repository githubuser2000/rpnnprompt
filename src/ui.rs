use crate::csv_data::CsvData;
use crate::autocomplete::SimpleAutocomplete;
use inquire::Text;
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("🎯 CSV Zwei-Stufen Autocomplete\n");
    
    // CSV laden
    let csv_data = CsvData::new();
    
    loop {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("SCHRITT 1: Wählen Sie eine erste Spalte");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Autocomplete für erste Spalte
        let first_autocomplete = csv_data.get_first_level_autocomplete();
        let first_choice = Text::new("Erste Spalte auswählen:")
            .with_autocomplete(first_autocomplete)
            .with_help_message("Beginnen Sie zu tippen für Vorschläge")
            .prompt()?;
        
        println!("✓ Ausgewählt: '{}'", first_choice);
        
        // Zeige Details zu dieser ersten Spalte
        csv_data.show_details_for_first(&first_choice);
        
        // Überprüfe ob es zugehörige zweite Spalten gibt
        let seconds = match csv_data.get_seconds_for_first(&first_choice) {
            Some(seconds) if !seconds.is_empty() => seconds,
            _ => {
                println!("\n⚠️  Keine zugehörigen zweiten Spalten für '{}'", first_choice);
                println!("Möchten Sie eine andere erste Spalte wählen? (j/N)");
                let again = Text::new("")
                    .with_default("n")
                    .prompt()?;
                
                if again.to_lowercase().starts_with('j') {
                    continue;
                } else {
                    break;
                }
            }
        };
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("SCHRITT 2: Wählen Sie eine zweite Spalte");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Verfügbare Optionen für '{}':", first_choice);
        
        // Autocomplete für zweite Spalte (abhängig von erster Wahl)
        let second_autocomplete = csv_data.get_second_level_autocomplete(&first_choice)
            .expect("Sollte existieren da seconds vorhanden sind");
        
        let second_choice = Text::new("Zweite Spalte auswählen:")
            .with_autocomplete(second_autocomplete)
            .with_help_message(&format!("{} Optionen verfügbar", seconds.len()))
            .prompt()?;
        
        println!("✓ Ausgewählt: '{}' → '{}'", first_choice, second_choice);
        
        // Zeige vollständige Details zum Paar
        csv_data.show_pair_details(&first_choice, &second_choice);
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Möchten Sie eine weitere Suche durchführen? (j/N)");
        let again = Text::new("Weitersuchen?")
            .with_default("n")
            .prompt()?;
            
        if !again.to_lowercase().starts_with('j') {
            break;
        }
        
        println!("\n⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼");
    }
    
    println!("\n🏁 Programm beendet.");
    
    Ok(())
}
