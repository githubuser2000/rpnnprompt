// zeilen_parser.rs - Korrigierte Version
use anyhow::Result;
// Entfernen Sie den nicht benötigten Context Import
// use anyhow::Context;

pub struct ZeilenParser;

impl ZeilenParser {
    // Korrektur: Result benötigt zwei generische Typ-Parameter
    pub fn parse_to_numbers(input: &str) -> Result<Vec<i32>> {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }
        
        // Verwende die split-Funktion aus if_is_zeilen_angabe
        use crate::if_is_zeilen_angabe::split;
        let parts = split::split_with_bracket_balance(trimmed);
        let mut numbers = Vec::new();
        
        for part in parts {
            if part.is_empty() {
                continue;
            }
            
            // Entferne führendes 'v' falls vorhanden
            let clean_part = if part.starts_with('v') {
                &part[1..]
            } else {
                part
            };
            
            // Prüfe auf Generator-Notation
            if let Some(generator_nums) = crate::if_is_zeilen_angabe::str_as_generator_to_list_of_num_strs(clean_part) {
                for num_str in generator_nums {
                    if let Ok(num) = num_str.parse::<i32>() {
                        numbers.push(num);
                    }
                }
                continue;
            }
            
            // Prüfe auf Bereich (z.B. "3-8")
            if let Some((start_str, end_str)) = clean_part.split_once('-') {
                if let (Ok(start), Ok(end)) = (start_str.parse::<i32>(), end_str.parse::<i32>()) {
                    for i in start..=end {
                        numbers.push(i);
                    }
                    continue;
                }
            }
            
            // Einzelne Zahl
            if let Ok(num) = clean_part.parse::<i32>() {
                numbers.push(num);
            }
        }
        
        // Duplikate entfernen und sortieren
        numbers.sort();
        numbers.dedup();
        
        Ok(numbers)
    }
    
    // Korrektur: Result benötigt zwei generische Typ-Parameter
    pub fn parse_bruch_to_numbers(input: &str) -> Result<Vec<(i32, i32)>> {
        let trimmed = input.trim();
        let mut brueche = Vec::new();
        
        // Einfache Implementierung für Brüche wie "1/2-3/4"
        let parts: Vec<&str> = trimmed.split(',').collect();
        for part in parts {
            let part = part.trim();
            if part.contains('/') {
                let fractions: Vec<&str> = part.split('/').collect();
                if fractions.len() == 2 {
                    if let (Ok(z1), Ok(n1)) = (fractions[0].parse::<i32>(), fractions[1].parse::<i32>()) {
                        brueche.push((z1, n1));
                    }
                }
            }
        }
        
        Ok(brueche)
    }
}
