use std::collections::HashMap;
use crate::autocomplete::SimpleAutocomplete;

pub struct CsvData {
    // Für jede erste Spalte (alle Varianten) speichern wir die zugehörigen zweiten Spalten
    pub first_to_seconds_map: HashMap<String, Vec<String>>,
    // Alle ersten Spalten für Autocomplete
    pub all_first_columns: Vec<String>,
    // Rohdaten für Detailsuche
    pub raw_data: Vec<(Vec<String>, Vec<String>, String)>,
}

impl CsvData {
    pub fn new() -> Self {
        use crate::csv_parser::CsvParser;
        
        let (first_to_seconds_map, all_first_columns, raw_data) = CsvParser::load_all_data();
        
        Self {
            first_to_seconds_map,
            all_first_columns,
            raw_data,
        }
    }
    
    // Hole Autocomplete für erste Spalte
    pub fn get_first_level_autocomplete(&self) -> SimpleAutocomplete {
        SimpleAutocomplete::new(self.all_first_columns.clone())
    }
    
    // Hole Autocomplete für zweite Spalte basierend auf erster Auswahl
    pub fn get_second_level_autocomplete(&self, first: &str) -> Option<SimpleAutocomplete> {
        self.first_to_seconds_map
            .get(first)
            .map(|seconds| SimpleAutocomplete::new(seconds.clone()))
    }
    
    // Hole zweite Spalten für eine erste Spalte
    pub fn get_seconds_for_first(&self, first: &str) -> Option<&Vec<String>> {
        self.first_to_seconds_map.get(first)
    }
    
    // Finde zugehörige Zahlen für ein Paar
    pub fn find_numbers_for_pair(&self, first: &str, second: &str) -> Vec<String> {
        let mut results = Vec::new();
        
        for (first_cols, second_cols, numbers) in &self.raw_data {
            if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
                results.push(numbers.clone());
            }
        }
        
        results
    }
    
    // Zeige alle Details für ein erstes Wort
    pub fn show_details_for_first(&self, first: &str) {
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
    pub fn show_pair_details(&self, first: &str, second: &str) {
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
