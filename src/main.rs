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
            return Ok(self.items.iter().take(25).cloned().collect());
        }
        
        let filtered: Vec<String> = self.items
            .iter()
            .filter(|item| item.to_lowercase().contains(&input_lower))
            .take(25)
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
    // Für jede erste Spalte (alle Varianten) speichern wir die zugehörigen zweiten Spalten
    first_to_seconds_map: HashMap<String, Vec<String>>,
    // Alle ersten Spalten für Autocomplete
    all_first_columns: Vec<String>,
    // Rohdaten für Detailsuche
    raw_data: Vec<(Vec<String>, Vec<String>, String)>,
}

impl CsvData {
    fn load() -> Result<Self> {
        let mut first_to_seconds_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut all_first_set = HashSet::new();
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
            
            // Erste Spalte: ALLE Wörter extrahieren
            let first_columns = Self::parse_first_column_all(parts[0]);
            if first_columns.is_empty() {
                continue;
            }
            
            // Zweite Spalte: ALLE Wörter extrahieren
            let second_columns = if let Some(second_part) = parts.get(1) {
                Self::parse_second_column(second_part)
            } else {
                Vec::new()
            };
            
            // Dritte Spalte (Zahlen)
            let numbers = if let Some(third_part) = parts.get(2) {
                third_part.to_string()
            } else {
                String::new()
            };
            
            // Für jedes Wort in der ersten Spalte die zugehörigen zweiten Wörter speichern
            for first in &first_columns {
                all_first_set.insert(first.clone());
                
                // Füge alle zweiten Wörter für dieses erste Wort hinzu
                let entry = first_to_seconds_map
                    .entry(first.clone())
                    .or_insert_with(HashSet::new);
                
                for second in &second_columns {
                    entry.insert(second.clone());
                }
            }
            
            raw_data.push((first_columns, second_columns, numbers));
        }
        
        // Konvertiere HashSets zu sortierten Vectors
        let mut sorted_first_to_seconds: HashMap<String, Vec<String>> = HashMap::new();
        for (first, seconds_set) in first_to_seconds_map {
            let mut seconds: Vec<String> = seconds_set.into_iter().collect();
            seconds.sort_by_key(|s| s.to_lowercase());
            sorted_first_to_seconds.insert(first, seconds);
        }
        
        // Sortiere alle ersten Spalten
        let mut all_first_columns: Vec<String> = all_first_set.into_iter().collect();
        all_first_columns.sort_by_key(|s| s.to_lowercase());
        
        println!("✅ Geladen: {} verschiedene erste Spalten", all_first_columns.len());
        println!("✅ Geladen: {} verschiedene Zuordnungen", sorted_first_to_seconds.len());
        println!("✅ Geladen: {} CSV-Zeilen", raw_data.len());
        
        // Zeige Beispiele
        println!("\n🔍 Beispiel-Zuordnungen:");
        let mut example_count = 0;
        for (first, seconds) in sorted_first_to_seconds.iter().take(3) {
            println!("  '{}' → {} Optionen", first, seconds.len());
            for (i, second) in seconds.iter().take(3).enumerate() {
                println!("     {}. {}", i + 1, second);
            }
            if seconds.len() > 3 {
                println!("     ... und {} weitere", seconds.len() - 3);
            }
            example_count += 1;
        }
        
        if sorted_first_to_seconds.len() > 3 {
            println!("  ... und {} weitere Zuordnungen", sorted_first_to_seconds.len() - 3);
        }
        
        Ok(Self {
            first_to_seconds_map: sorted_first_to_seconds,
            all_first_columns,
            raw_data,
        })
    }
    
    // Extrahiert ALLE Wörter aus der ersten Spalte
    fn parse_first_column_all(text: &str) -> Vec<String> {
        let trimmed = text.trim();
        let mut result = Vec::new();
        
        // Wenn in Klammern: Extrahiere alle durch Kommas getrennten Wörter
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let inner = &trimmed[1..trimmed.len()-1];
            
            // Manuelles Parsing um Anführungszeichen zu handhaben
            let mut current = String::new();
            let mut in_quote = false;
            
            for c in inner.chars() {
                match c {
                    '\'' => {
                        in_quote = !in_quote;
                        if !in_quote && !current.is_empty() {
                            // Ende eines Wortes
                            result.push(current.trim().to_string());
                            current.clear();
                        }
                    }
                    ',' => {
                        if !in_quote && !current.is_empty() {
                            // Wort zwischen Anführungszeichen
                            result.push(current.trim().to_string());
                            current.clear();
                        }
                    }
                    _ => {
                        if in_quote || c != ' ' {
                            current.push(c);
                        }
                    }
                }
            }
            
            // Letztes Wort hinzufügen falls vorhanden
            if !current.is_empty() {
                result.push(current.trim().to_string());
            }
        } else {
            // Keine Klammern: Einfach das ganze Wort nehmen
            result.push(trimmed.to_string());
        }
        
        result
    }
    
    // Extrahiert ALLE Wörter aus der zweiten Spalte
    fn parse_second_column(text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let text = text.trim();
        
        // Manuelles Parsing um Klammern und Kommas zu handhaben
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
    
    // Hole Autocomplete für erste Spalte
    fn get_first_level_autocomplete(&self) -> SimpleAutocomplete {
        SimpleAutocomplete::new(self.all_first_columns.clone())
    }
    
    // Hole Autocomplete für zweite Spalte basierend auf erster Auswahl
    fn get_second_level_autocomplete(&self, first: &str) -> Option<SimpleAutocomplete> {
        self.first_to_seconds_map
            .get(first)
            .map(|seconds| SimpleAutocomplete::new(seconds.clone()))
    }
    
    // Hole zweite Spalten für eine erste Spalte
    fn get_seconds_for_first(&self, first: &str) -> Option<&Vec<String>> {
        self.first_to_seconds_map.get(first)
    }
    
    // Finde zugehörige Zahlen für ein Paar
    fn find_numbers_for_pair(&self, first: &str, second: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        for (first_cols, second_cols, numbers) in &self.raw_data {
            if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
                results.push(numbers.clone());
            }
        }
        
        results
    }
    
    // Zeige alle Details für ein erstes Wort
    fn show_details_for_first(&self, first: &str) {
        println!("\n🔍 Details für '{}':", first);
        
        if let Some(seconds) = self.get_seconds_for_first(first) {
            println!("  📋 Verfügbare zweite Spalten ({}):", seconds.len());
            for (i, second) in seconds.iter().enumerate().take(10) {
                print!("    {:2}. {}", i + 1, second);
                
                // Zeige zugehörige Nummern
                let numbers = self.find_numbers_for_pair(first, second);
                if !numbers.is_empty() {
                    print!(" → {}", numbers.join(", "));
                }
                println!();
            }
            
            if seconds.len() > 10 {
                println!("    ... und {} weitere", seconds.len() - 10);
            }
        } else {
            println!("  ⚠️  Keine zugehörigen zweiten Spalten gefunden");
        }
    }
    
    // Zeige vollständige Informationen zu einem Paar
    fn show_pair_details(&self, first: &str, second: &str) {
        println!("\n🔍 Vollständige Informationen:");
        println!("  Erste Spalte:  {}", first);
        println!("  Zweite Spalte: {}", second);
        
        let numbers = self.find_numbers_for_pair(first, second);
        if !numbers.is_empty() {
            println!("  Zugehörige Nummern: {}", numbers.join(", "));
        } else {
            println!("  ℹ️  Keine zugehörigen Nummern gefunden");
        }
        
        // Zeige alle CSV-Zeilen mit diesem Paar
        println!("\n  📄 CSV-Zeilen mit diesem Paar:");
        let mut found = false;
        for (i, (first_cols, second_cols, nums)) in self.raw_data.iter().enumerate() {
            if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
                println!("    Zeile {}: {:?} → {:?} → {}", 
                    i + 1, first_cols, second_cols, nums);
                found = true;
            }
        }
        
        if !found {
            println!("    ⚠️ Keine direkten Einträge gefunden");
        }
    }
}

fn main() -> Result<()> {
    println!("🎯 CSV Zwei-Stufen Autocomplete\n");
    
    // CSV laden
    let csv_data = CsvData::load()?;
    
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
