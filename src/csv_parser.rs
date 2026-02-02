use std::collections::{HashMap, HashSet};

// CSV-Daten zur Kompilierzeit einbetten
const CSV_DATA: &str = include_str!("../csv/coordinatesColumnsFirstReliTable.csv");

pub struct CsvParser;

impl CsvParser {
    // Extrahiert ALLE Wörter aus der ersten Spalte
    pub fn parse_first_column_all(text: &str) -> Vec<String> {
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
    pub fn parse_second_column(text: &str) -> Vec<String> {
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
    
    // Lädt alle CSV-Daten
    pub fn load_all_data() -> (HashMap<String, Vec<String>>, Vec<String>, Vec<(Vec<String>, Vec<String>, String)>) {
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
// csv_parser.rs - Entfernen der unbenutzten Variable
// Zeilen 183-195:
println!("\n🔍 Beispiel-Zuordnungen:");
for (first, seconds) in sorted_first_to_seconds.iter().take(3) {
    println!("  '{}' → {} Optionen", first, seconds.len());
    for (i, second) in seconds.iter().take(3).enumerate() {
        println!("     {}. {}", i + 1, second);
    }
    if seconds.len() > 3 {
        println!("     ... und {} weitere", seconds.len() - 3);
    }
}

if sorted_first_to_seconds.len() > 3 {
    println!("  ... und {} weitere Zuordnungen", sorted_first_to_seconds.len() - 3);
}       
        (sorted_first_to_seconds, all_first_columns, raw_data)
    }
}
