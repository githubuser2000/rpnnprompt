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
    first_level_all: Vec<String>, // ALLE Wörter aus erster Spalte
    second_level_all: Vec<String>, // ALLE Wörter aus zweiter Spalte
    mapping: HashMap<String, Vec<String>>, // Mapping zwischen erster und zweiter Spalte
    reverse_mapping: HashMap<String, Vec<String>>, // Umgekehrtes Mapping
    raw_data: Vec<(Vec<String>, Vec<String>, String)>, // Rohdaten für Details
}

impl CsvData {
    fn load() -> Result<Self> {
        let mut first_level_set = HashSet::new();
        let mut second_level_set = HashSet::new();
        let mut mapping: HashMap<String, HashSet<String>> = HashMap::new();
        let mut reverse_mapping: HashMap<String, HashSet<String>> = HashMap::new();
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
            
            // Füge ALLE Wörter zu den Sets hinzu
            for first in &first_columns {
                first_level_set.insert(first.clone());
                
                // Erstelle Mapping von jedem ersten Wort zu allen zweiten Wörtern
                for second in &second_columns {
                    mapping
                        .entry(first.clone())
                        .or_insert_with(HashSet::new)
                        .insert(second.clone());
                    
                    // Umgekehrtes Mapping
                    reverse_mapping
                        .entry(second.clone())
                        .or_insert_with(HashSet::new)
                        .insert(first.clone());
                }
            }
            
            for second in &second_columns {
                second_level_set.insert(second.clone());
            }
            
            raw_data.push((first_columns, second_columns, numbers));
        }
        
        // Sortiere alles
        let mut first_level_all: Vec<String> = first_level_set.into_iter().collect();
        first_level_all.sort_by_key(|s| s.to_lowercase());
        
        let mut second_level_all: Vec<String> = second_level_set.into_iter().collect();
        second_level_all.sort_by_key(|s| s.to_lowercase());
        
        // Konvertiere HashSets zu Vectors für bessere Ausgabe
        let mut sorted_mapping: HashMap<String, Vec<String>> = HashMap::new();
        for (key, values_set) in mapping {
            let mut values: Vec<String> = values_set.into_iter().collect();
            values.sort_by_key(|s| s.to_lowercase());
            sorted_mapping.insert(key, values);
        }
        
        let mut sorted_reverse_mapping: HashMap<String, Vec<String>> = HashMap::new();
        for (key, values_set) in reverse_mapping {
            let mut values: Vec<String> = values_set.into_iter().collect();
            values.sort_by_key(|s| s.to_lowercase());
            sorted_reverse_mapping.insert(key, values);
        }
        
        // Für die Ausgabe: Zeige einige Mappings
        println!("\n📊 Statistik:");
        println!("✅ Erste Spalte: {} verschiedene Wörter", first_level_all.len());
        println!("✅ Zweite Spalte: {} verschiedene Wörter", second_level_all.len());
        println!("✅ Mappings: {} Zuordnungen", sorted_mapping.len());
        println!("✅ CSV-Zeilen: {}", raw_data.len());
        
        // Zeige Beispiel-Mappings
        println!("\n🔍 Beispiel-Mappings:");
        let mut _count = 0;
        for (first, seconds) in sorted_mapping.iter().take(5) {
            println!("  '{}' → {} Optionen: {:?}", first, seconds.len(), seconds);
            _count += 1;
        }
        if sorted_mapping.len() > 5 {
            println!("  ... und {} weitere Mappings", sorted_mapping.len() - 5);
        }
        
        Ok(Self {
            first_level_all,
            second_level_all,
            mapping: sorted_mapping,
            reverse_mapping: sorted_reverse_mapping,
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
        SimpleAutocomplete::new(self.first_level_all.clone())
    }
    
    // Hole Autocomplete für zweite Spalte
    fn get_second_level_autocomplete(&self) -> SimpleAutocomplete {
        SimpleAutocomplete::new(self.second_level_all.clone())
    }
    
    // Finde zugehörige zweite Wörter für ein erstes Wort
    fn find_seconds_for_first(&self, first: &str) -> Vec<String> {
        self.mapping.get(first).cloned().unwrap_or_default()
    }
    
    // Finde zugehörige erste Wörter für ein zweites Wort
    fn find_firsts_for_second(&self, second: &str) -> Vec<String> {
        self.reverse_mapping.get(second).cloned().unwrap_or_default()
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
        
        // Finde alle zugehörigen zweiten Wörter
        let seconds = self.find_seconds_for_first(first);
        if seconds.is_empty() {
            println!("  ⚠️  Keine zugehörigen zweiten Wörter gefunden");
            return;
        }
        
        println!("  📋 Zugehörige zweite Wörter ({}):", seconds.len());
        for (i, second) in seconds.iter().enumerate() {
            print!("    {:2}. {}", i + 1, second);
            
            // Zeige zugehörige Nummern
            let numbers = self.find_numbers_for_pair(first, second);
            if !numbers.is_empty() {
                print!(" → {}", numbers.join(", "));
            }
            println!();
        }
        
        // Zeige alle CSV-Zeilen die dieses Wort enthalten
        println!("\n  📄 CSV-Zeilen mit '{}':", first);
        let mut count = 0;
        for (i, (first_cols, second_cols, numbers)) in self.raw_data.iter().enumerate() {
            if first_cols.contains(&first.to_string()) {
                println!("    Zeile {}: {:?} → {:?} → {}", 
                    i + 1, first_cols, second_cols, numbers);
                count += 1;
                if count >= 3 {
                    println!("    ...");
                    break;
                }
            }
        }
    }
    
    // Zeige alle Details für ein zweites Wort
    fn show_details_for_second(&self, second: &str) {
        println!("\n🔍 Details für '{}':", second);
        
        // Finde alle zugehörigen ersten Wörter
        let firsts = self.find_firsts_for_second(second);
        if firsts.is_empty() {
            println!("  ⚠️  Keine zugehörigen ersten Wörter gefunden");
            return;
        }
        
        println!("  📋 Zugehörige erste Wörter ({}):", firsts.len());
        for (i, first) in firsts.iter().enumerate() {
            print!("    {:2}. {}", i + 1, first);
            
            // Zeige zugehörige Nummern
            let numbers = self.find_numbers_for_pair(first, second);
            if !numbers.is_empty() {
                print!(" → {}", numbers.join(", "));
            }
            println!();
        }
        
        // Zeige alle CSV-Zeilen die dieses Wort enthalten
        println!("\n  📄 CSV-Zeilen mit '{}':", second);
        let mut count = 0;
        for (i, (first_cols, second_cols, numbers)) in self.raw_data.iter().enumerate() {
            if second_cols.contains(&second.to_string()) {
                println!("    Zeile {}: {:?} → {:?} → {}", 
                    i + 1, first_cols, second_cols, numbers);
                count += 1;
                if count >= 3 {
                    println!("    ...");
                    break;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    println!("🎯 CSV Kompletter Autocomplete (Beide Spalten)\n");
    
    // CSV laden
    let csv_data = CsvData::load()?;
    
    loop {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("MENÜ: Wählen Sie eine Suchrichtung");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("1. Von erster Spalte → zweite Spalte");
        println!("2. Von zweiter Spalte → erste Spalte");
        println!("3. Direkte Suche (beide Spalten)");
        println!("q. Beenden");
        
        let choice = Text::new("Auswahl (1/2/3/q):")
            .with_default("1")
            .prompt()?;
        
        match choice.trim() {
            "1" => {
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("SUCHE: Erste Spalte → Zweite Spalte");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                
                let first_autocomplete = csv_data.get_first_level_autocomplete();
                let first_choice = Text::new("Wort aus erster Spalte:")
                    .with_autocomplete(first_autocomplete)
                    .with_help_message("Tippen Sie für Vorschläge")
                    .prompt()?;
                
                csv_data.show_details_for_first(&first_choice);
            }
            
            "2" => {
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("SUCHE: Zweite Spalte → Erste Spalte");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                
                let second_autocomplete = csv_data.get_second_level_autocomplete();
                let second_choice = Text::new("Wort aus zweiter Spalte:")
                    .with_autocomplete(second_autocomplete)
                    .with_help_message("Tippen Sie für Vorschläge")
                    .prompt()?;
                
                csv_data.show_details_for_second(&second_choice);
            }
            
            "3" => {
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("DIREKTE SUCHE: Beide Spalten");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                
                // Zuerst erste Spalte
                let first_autocomplete = csv_data.get_first_level_autocomplete();
                let first_choice = Text::new("Erstes Wort:")
                    .with_autocomplete(first_autocomplete)
                    .prompt()?;
                
                // Dann zweite Spalte (mit Filterung nach erster Wahl)
                let seconds_for_first = csv_data.find_seconds_for_first(&first_choice);
                if seconds_for_first.is_empty() {
                    println!("⚠️ Keine zweiten Wörter für '{}'", first_choice);
                    continue;
                }
                
                let second_autocomplete = SimpleAutocomplete::new(seconds_for_first.clone());
                let second_choice = Text::new("Zweites Wort:")
                    .with_autocomplete(second_autocomplete)
                    .with_help_message(&format!("{} Optionen verfügbar", seconds_for_first.len()))
                    .prompt()?;
                
                // Ergebnisse anzeigen
                println!("\n✅ GEFUNDEN: {} → {}", first_choice, second_choice);
                
                let numbers = csv_data.find_numbers_for_pair(&first_choice, &second_choice);
                if !numbers.is_empty() {
                    println!("🔢 Zugehörige Nummern: {}", numbers.join(", "));
                }
                
                // Zeige alle CSV-Zeilen mit diesem Paar
                println!("\n📄 Vollständige Einträge:");
                let mut found = false;
                for (i, (first_cols, second_cols, nums)) in csv_data.raw_data.iter().enumerate() {
                    if first_cols.contains(&first_choice) && second_cols.contains(&second_choice) {
                        println!("  Zeile {}: {:?} → {:?} → {}", 
                            i + 1, first_cols, second_cols, nums);
                        found = true;
                    }
                }
                
                if !found {
                    println!("  ⚠️ Keine direkten Einträge gefunden");
                }
            }
            
            "q" | "Q" => {
                break;
            }
            
            _ => {
                println!("⚠️ Ungültige Auswahl. Bitte 1, 2, 3 oder q eingeben.");
            }
        }
        
        println!("\n⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼⎼");
    }
    
    println!("\n🏁 Programm beendet.");
    
    Ok(())
}
