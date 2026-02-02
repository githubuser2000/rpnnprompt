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

// Clone automatisch ableiten
impl Clone for SimpleAutocomplete {
    fn clone(&self) -> Self {
        Self::new(self.items.clone())
    }
}

struct CsvData {
    first_level: Vec<String>,
    second_level_map: HashMap<String, Vec<String>>,
    raw_data: Vec<(String, Vec<String>, String)>, // Für schnelleren Zugriff auf Nummern
}

impl CsvData {
    fn load() -> Result<Self> {
        let mut first_level_set = HashSet::new();
        let mut second_level_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut raw_data = Vec::new();
        
        println!("📂 Lade CSV-Daten...");
        
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
            
            // Zweite Spalte extrahieren - ALLE Wörter!
            let mut second_columns_vec = Vec::new();
            if let Some(second_part) = parts.get(1) {
                let second_columns = Self::parse_second_column(second_part);
                second_columns_vec = second_columns.clone();
                
                // Füge alle Wörter zur HashMap hinzu
                for column in &second_columns {
                    second_level_map
                        .entry(first_column.clone())
                        .or_insert_with(HashSet::new)
                        .insert(column.clone());
                }
            }
            
            // Dritte Spalte (Zahlen) speichern
            let numbers = if let Some(third_part) = parts.get(2) {
                third_part.to_string()
            } else {
                String::new()
            };
            
            raw_data.push((first_column.clone(), second_columns_vec, numbers));
        }
        
        // Sortiere alles
        let mut first_level: Vec<String> = first_level_set.into_iter().collect();
        first_level.sort_by_key(|s| s.to_lowercase());
        
        // Konvertiere HashSet zu Vec und sortiere
        let mut final_second_level_map: HashMap<String, Vec<String>> = HashMap::new();
        for (key, values_set) in second_level_map {
            let mut values: Vec<String> = values_set.into_iter().collect();
            values.sort_by_key(|s| s.to_lowercase());
            final_second_level_map.insert(key, values);
        }
        
        println!("✅ Erste Spalte: {} Einträge", first_level.len());
        println!("✅ Zweite Spalte: {} Zuordnungen", final_second_level_map.len());
        println!("✅ Gesamt: {} CSV-Zeilen verarbeitet", raw_data.len());
        
        // Zeige Beispiel-Daten
        println!("\n📊 Beispiel-Daten:");
        for (i, (first, seconds, numbers)) in raw_data.iter().take(3).enumerate() {
            println!("  {}. {} → {:?} → {}", i + 1, first, seconds, numbers);
        }
        if raw_data.len() > 3 {
            println!("  ... und {} weitere Zeilen", raw_data.len() - 3);
        }
        
        Ok(Self {
            first_level,
            second_level_map: final_second_level_map,
            raw_data,
        })
    }
    
    fn parse_first_column(text: &str) -> String {
        let trimmed = text.trim();
        
        // Entferne äußere Klammern
        if trimmed.starts_with('(') {
            // Finde ersten Eintrag in Anführungszeichen
            let chars = trimmed.chars();
            let mut in_quote = false;
            let mut result = String::new();
            
            for c in chars {
                if c == '\'' {
                    in_quote = !in_quote;
                } else if in_quote {
                    result.push(c);
                } else if c == ',' && !result.is_empty() {
                    break; // Ersten Eintrag gefunden
                } else if c == ')' && !result.is_empty() {
                    break; // Ende der Klammer
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
        let mut result = Vec::new();
        let text = text.trim();
        
        // Manuelles Parsing um komplexe Fälle zu handhaben
        let mut current = String::new();
        let mut in_parentheses = 0;
        let mut chars = text.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    in_parentheses += 1;
                    current.push(c);
                }
                ')' => {
                    if in_parentheses > 0 {
                        in_parentheses -= 1;
                    }
                    current.push(c);
                }
                ',' => {
                    if in_parentheses == 0 {
                        // Ende eines Eintrags
                        let trimmed = current.trim().to_string();
                        if !trimmed.is_empty() {
                            result.push(trimmed);
                        }
                        current.clear();
                    } else {
                        current.push(c);
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }
        
        // Letzten Eintrag hinzufügen
        let trimmed = current.trim().to_string();
        if !trimmed.is_empty() {
            result.push(trimmed);
        }
        
        result
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
    
    // Finde zugehörige Zahlen für ein Paar
    fn find_numbers_for_pair(&self, first: &str, second: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        for (first_col, second_cols, numbers) in &self.raw_data {
            if first_col == first && second_cols.contains(&second.to_string()) {
                results.push(numbers.clone());
            }
        }
        
        results
    }
    
    // Neue Funktion: Zeige alle zweiten-Level Optionen mit Details
    fn show_second_level_details(&self, key: &str) {
        if let Some(options) = self.second_level_map.get(key) {
            println!("🔍 Alle Optionen für '{}':", key);
            for (i, option) in options.iter().enumerate() {
                print!("  {:2}. {}", i + 1, option);
                
                // Finde zugehörige Nummern
                let numbers = self.find_numbers_for_pair(key, option);
                if !numbers.is_empty() {
                    print!(" → {}", numbers.join(", "));
                }
                println!();
            }
        }
    }
}

fn main() -> Result<()> {
    println!("🎯 CSV Zwei-Stufen Autocomplete\n");
    
    // CSV laden
    let csv_data = CsvData::load()?;
    
    loop {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Schritt 1/2: Erste Spalte auswählen");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        let first_autocomplete = csv_data.get_first_level_autocomplete();
        let first_choice = Text::new("Erste Spalte:")
            .with_autocomplete(first_autocomplete)
            .with_help_message("Tippen Sie für Vorschläge")
            .prompt()?;
        
        println!("✓ Gewählt: {}\n", first_choice);
        
        // Zeige alle verfügbaren Optionen für diese erste Spalte
        csv_data.show_second_level_details(&first_choice);
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Schritt 2/2: Zweite Spalte auswählen");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        if let Some(second_autocomplete) = csv_data.get_second_level_autocomplete(&first_choice) {
            let second_choice = Text::new("Zweite Spalte:")
                .with_autocomplete(second_autocomplete)
                .with_help_message("Tippen Sie für Vorschläge")
                .prompt()?;
            
            // Ergebnisse anzeigen
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("✅ AUSWAHL VOLLSTÄNDIG");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("📋 Erste Spalte:  {}", first_choice);
            println!("📋 Zweite Spalte: {}", second_choice);
            
            // Zugehörige Zahlen finden
            let numbers = csv_data.find_numbers_for_pair(&first_choice, &second_choice);
            if !numbers.is_empty() {
                println!("🔢 Zugehörige Nummern: {}", numbers.join(", "));
            } else {
                println!("ℹ️  Keine zugehörigen Nummern gefunden");
            }
            
            // Finde die exakte CSV-Zeile
            println!("\n🔍 Vollständige CSV-Zeile(n):");
            let mut found = false;
            for (i, line) in CSV_DATA.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = line.split(';').collect();
                if parts.len() < 2 {
                    continue;
                }
                
                let first_col = CsvData::parse_first_column(parts[0]);
                let second_cols = CsvData::parse_second_column(parts[1]);
                
                if first_col == first_choice && second_cols.contains(&second_choice) {
                    println!("  Zeile {}: {}", i + 1, line);
                    found = true;
                }
            }
            
            if !found {
                println!("  Keine exakte Zeile gefunden");
            }
            
        } else {
            println!("⚠️ Keine zweiten-Level Optionen für '{}'", first_choice);
        }
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Möchten Sie eine weitere Suche durchführen? (j/N)");
        let again = Text::new("Weitersuchen?")
            .with_default("n")
            .prompt()?;
            
        if !again.to_lowercase().starts_with('j') {
            break;
        }
        
        println!("\n⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼");
    }
    
    println!("\n🏁 Programm beendet.");
    
    Ok(())
}
