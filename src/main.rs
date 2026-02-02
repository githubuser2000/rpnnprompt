use inquire::{autocompletion::Replacement, Autocomplete, Text};
use std::error::Error;
use std::collections::{HashMap, HashSet};
use anyhow::Result;

// CSV-Daten zur Kompilierzeit einbetten
const CSV_DATA: &str = include_str!("../csv/coordinatesColumnsFirstReliTable.csv");

// Einfache Autocomplete-Implementierung
struct SimpleAutocomplete {
    items: Vec<String>,
}

impl SimpleAutocomplete {
    fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

// Da `Clone` bereits implementiert ist, können wir DynClone automatisch ableiten lassen
// durch die Verwendung von `dyn_clone::clone_box` Standard-Implementierung
impl Autocomplete for SimpleAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let input_lower = input.trim().to_lowercase();
        
        if input_lower.is_empty() {
            return Ok(self.items.iter().take(20).cloned().collect());
        }
        
        let filtered: Vec<String> = self.items
            .iter()
            .filter(|item| item.to_lowercase().contains(&input_lower))
            .take(20)
            .cloned()
            .collect();
        
        Ok(filtered)
    }

    fn get_completion(
        &mut self,
        _input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, Box<dyn Error + Send + Sync>> {
        Ok(highlighted_suggestion
            .map(Replacement::Some)
            .unwrap_or(Replacement::None))
    }
}

// DynClone automatisch ableiten (durch Clone)
impl Clone for SimpleAutocomplete {
    fn clone(&self) -> Self {
        Self::new(self.items.clone())
    }
}

struct CsvData {
    first_level: Vec<String>,
    second_level_map: HashMap<String, Vec<String>>,
}

impl CsvData {
    fn load() -> Result<Self> {
        let mut first_level_set = HashSet::new();
        let mut second_level_map = HashMap::new();
        
        println!("Lade CSV-Daten...");
        
        for line in CSV_DATA.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // Teile an Semikolons
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 2 {
                continue;
            }
            
            // Erste Spalte extrahieren
            let first_column = Self::parse_first_column(parts[0]);
            if first_column.is_empty() {
                continue;
            }
            
            first_level_set.insert(first_column.clone());
            
            // Zweite Spalte extrahieren
            if let Some(second_part) = parts.get(1) {
                let second_columns = Self::parse_second_column(second_part);
                if !second_columns.is_empty() {
                    second_level_map.insert(first_column, second_columns);
                }
            }
        }
        
        // Sortiere alles
        let mut first_level: Vec<String> = first_level_set.into_iter().collect();
        first_level.sort_by_key(|s| s.to_lowercase());
        
        for values in second_level_map.values_mut() {
            values.sort_by_key(|s| s.to_lowercase());
            values.dedup();
        }
        
        println!("Erste Spalte: {} Einträge", first_level.len());
        println!("Zweite Spalte: {} Zuordnungen", second_level_map.len());
        
        Ok(Self {
            first_level,
            second_level_map,
        })
    }
    
    fn parse_first_column(text: &str) -> String {
        let trimmed = text.trim();
        
        // Entferne äußere Klammern
        if trimmed.starts_with('(') {
            // Finde ersten Eintrag in Anführungszeichen
            let mut chars = trimmed.chars();
            let mut in_quote = false;
            let mut result = String::new();
            
            for c in chars {
                if c == '\'' {
                    in_quote = !in_quote;
                } else if in_quote {
                    result.push(c);
                } else if c == ',' && !result.is_empty() {
                    break; // Ersten Eintrag gefunden
                }
            }
            
            if !result.is_empty() {
                return result;
            }
        }
        
        // Fallback: Alles nehmen
        trimmed.to_string()
    }
    
    fn parse_second_column(text: &str) -> Vec<String> {
        text.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    fn get_first_level_autocomplete(&self) -> SimpleAutocomplete {
        SimpleAutocomplete::new(self.first_level.clone())
    }
    
    fn get_second_level_autocomplete(&self, key: &str) -> Option<SimpleAutocomplete> {
        self.second_level_map
            .get(key)
            .map(|items| SimpleAutocomplete::new(items.clone()))
    }
    
    fn get_second_level_options(&self, key: &str) -> Option<&Vec<String>> {
        self.second_level_map.get(key)
    }
}

fn main() -> Result<()> {
    println!("=== CSV Zwei-Stufen Autocomplete ===\n");
    
    // CSV laden
    let csv_data = CsvData::load()?;
    
    // ERSTE STUFE
    println!("Schritt 1/2: Wählen Sie einen Begriff aus der ersten Spalte");
    println!("Verfügbare Optionen ({}):", csv_data.first_level.len());
    
    let first_autocomplete = csv_data.get_first_level_autocomplete();
    let first_choice = Text::new("Erste Spalte auswählen:")
        .with_autocomplete(first_autocomplete)
        .with_help_message("Tippen Sie um Optionen zu sehen")
        .prompt()?;
    
    println!("✓ Gewählt: {}\n", first_choice);
    
    // ZWEITE STUFE
    if let Some(second_options) = csv_data.get_second_level_options(&first_choice) {
        println!("Schritt 2/2: Wählen Sie einen Wert aus der zweiten Spalte");
        println!("Verfügbare Optionen für '{}' ({}):", first_choice, second_options.len());
        
        let second_autocomplete = csv_data.get_second_level_autocomplete(&first_choice)
            .expect("Sollte existieren da second_options Some ist");
        
        let second_choice = Text::new("Zweite Spalte auswählen:")
            .with_autocomplete(second_autocomplete)
            .with_help_message("Tippen Sie um Optionen zu sehen")
            .prompt()?;
        
        println!("\n✅ Auswahl vollständig!");
        println!("─────────────────────────────");
        println!("Erste Spalte:  {}", first_choice);
        println!("Zweite Spalte: {}", second_choice);
        println!("─────────────────────────────");
        
        // OPTIONAL: Die zugehörigen Zahlen (dritte Spalte) finden
        println!("\n🔍 Suche nach zugehörigen Daten...");
        
        // Durchsuche CSV nach diesem Paar
        for line in CSV_DATA.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() < 3 {
                continue;
            }
            
            let first_col = CsvData::parse_first_column(parts[0]);
            let second_cols = CsvData::parse_second_column(parts[1]);
            
            if first_col == first_choice && second_cols.contains(&second_choice) {
                if let Some(numbers_part) = parts.get(2) {
                    println!("Gefundene Nummern: {}", numbers_part);
                    break;
                }
            }
        }
    } else {
        println!("⚠️ Keine zweiten-Level Optionen für '{}'", first_choice);
    }
    
    println!("\n🏁 Programm beendet.");
    
    Ok(())
}
