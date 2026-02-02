// if_is_zeilen_angabe/functions.rs
use regex::Regex;
use lazy_static::lazy_static;
use crate::if_is_zeilen_angabe::split;  // Füge diesen Import hinzu

// Für die i18n Simulation
pub mod i18n {
    pub mod befehle2 {
        pub const V: &str = "v";
    }
}

// Globale Regex-Patterns (einmalig kompiliert)
lazy_static! {
    pub static ref ZEILEN_BRUCH_PATTERN: Regex = Regex::new(r"^(-?\d+/\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$").unwrap();
    pub static ref ZEILEN_PATTERN: Regex = Regex::new(&format!("^({}?-?\\d+)(-\\d+)?((\\+)(\\d+))*$", i18n::befehle2::V)).unwrap();
    pub static ref OPTIMIZED_PATTERN: Regex = Regex::new(r"^(v?-?\d+)(-\d+)?((\+)(\d+))*$").unwrap();
}

// 1. isZeilenBruchAngabe_betweenKommas
pub fn is_zeilen_bruch_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_BRUCH_PATTERN.is_match(g)
}

// 2. isZeilenBruchOrGanzZahlAngabe
pub fn is_zeilen_bruch_or_ganz_zahl_angabe(text: &str) -> bool {
    split::split_with_bracket_balance(text)  // Verwende split:: vor dem Funktionsnamen
        .iter()
        .all(|g| is_zeilen_bruch_angabe_between_kommas(g) || is_zeilen_angabe_between_kommas(g))
}

// 3. isZeilenBruchAngabe
pub fn is_zeilen_bruch_angabe(text: &str) -> bool {
    let stext: Vec<&str> = split::split_with_bracket_balance(text);  // Verwende split::
    let any_at_all = stext.iter().any(|txt: &&str| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_bruch_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 4. isZeilenAngabe
pub fn is_zeilen_angabe(text: &str) -> bool {
    let stext: Vec<&str> = split::split_with_bracket_balance(text);  // Verwende split::
    let any_at_all = stext.iter().any(|txt: &&str| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 5. isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_PATTERN.is_match(g) || 
    str_as_generator_to_list_of_num_strs(g).is_some() ||
    (g.len() > 1 && str_as_generator_to_list_of_num_strs(&g[1..]).is_some())
}

pub fn str_as_generator_to_vec_i64(text: &str) -> Option<Vec<i64>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }

    // erlaubte Klammern
    let (_open, close) = match trimmed.chars().next()? {
        '(' => ('(', ')'),
        '[' => ('[', ']'),
        '{' => ('{', '}'),
        _ => return None,
    };

    if !trimmed.ends_with(close) {
        return None;
    }

    let inner = &trimmed[1..trimmed.len() - 1];

    let mut result = Vec::new();

    for part in inner.split(',') {
        let s = part.trim();

        // explizit verbieten: Dezimalpunkte oder -kommas
        if s.contains('.') {
            return None;
        }

        let value: i64 = s.parse().ok()?;
        result.push(value);
    }

    Some(result)
}

// Hilfsfunktion: strAsGeneratorToListOfNumStrs
pub fn str_as_generator_to_list_of_num_strs(text: &str) -> Option<Vec<String>> {
    if text.is_empty() {
        return None;
    }
    
    let trimmed = text.trim();
    
    // Prüfe auf (a,b,c) oder [a,b,c] oder {a,b,c} Format
    if (trimmed.starts_with('(') && trimmed.ends_with(')')) ||
       (trimmed.starts_with('[') && trimmed.ends_with(']')) ||
       (trimmed.starts_with('{') && trimmed.ends_with('}')) {
        
        let inner = &trimmed[1..trimmed.len()-1];
        let numbers: Result<Vec<i32>, _> = inner.split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();
            
        match numbers {
            Ok(nums) => {
                let strings: Vec<String> = nums.iter()
                    .map(|n| n.to_string())
                    .collect();
                Some(strings)
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

// Optimierte Version für isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas_optimized(g: &str) -> bool {
    // Prüfe zuerst das reguläre Muster
    if OPTIMIZED_PATTERN.is_match(g) {
        return true;
    }
    
    // Prüfe Generator-Notation
    if str_as_generator_to_list_of_num_strs(g).is_some() {
        return true;
    }
    
    // Prüfe ohne erstes Zeichen (falls es ein Sonderzeichen ist)
    if g.len() > 1 {
        if let Some(ch) = g.chars().next() {
            if !ch.is_ascii_digit() && ch != '-' && ch != 'v' {
                return str_as_generator_to_list_of_num_strs(&g[1..]).is_some();
            }
        }
    }
    
    false
}
