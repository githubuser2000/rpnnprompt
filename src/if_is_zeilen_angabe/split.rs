// if_is_zeilen_angabe/split.rs

// Implementierung von Lookahead: r",(?![^\[\]\{\}\(\)]*[\]\}\)])"
// Diese Regex sucht Kommas, die NICHT gefolgt werden von einem schließenden Bracket/Klammer ohne vorher ein öffnendes gesehen zu haben
pub fn split_with_lookahead(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    
    let mut i = 0;
    while i < len {
        if chars[i] == ',' {
            // Prüfe Lookahead: nach diesem Komma darf nicht direkt ein schließendes Bracket/Klammer kommen,
            // ohne vorher ein entsprechendes öffnendes zu haben
            if !has_unmatched_closing_bracket_ahead(&chars[i+1..]) {
                result.push(&text[start..i]);
                start = i + 1;
            }
        }
        i += 1;
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}

// Prüft ob nach aktueller Position ein schließendes Bracket/Klammer kommt,
// ohne dass vorher ein entsprechendes öffnendes im aktuellen Kontext war
pub fn has_unmatched_closing_bracket_ahead(chars: &[char]) -> bool {
    if chars.is_empty() {
        return false;
    }
    
    // Zähle die Balance für jede Bracket-Art
    let mut bracket_balance = 0;
    let mut brace_balance = 0;
    let mut paren_balance = 0;
    
    // Gehe durch alle verbleibenden Zeichen
    for &c in chars {
        match c {
            '[' => bracket_balance += 1,
            ']' => {
                if bracket_balance > 0 {
                    bracket_balance -= 1;
                } else {
                    // Unmatched closing bracket gefunden
                    return true;
                }
            }
            '{' => brace_balance += 1,
            '}' => {
                if brace_balance > 0 {
                    brace_balance -= 1;
                } else {
                    // Unmatched closing brace gefunden
                    return true;
                }
            }
            '(' => paren_balance += 1,
            ')' => {
                if paren_balance > 0 {
                    paren_balance -= 1;
                } else {
                    // Unmatched closing paren gefunden
                    return true;
                }
            }
            ',' => {
                // Wenn wir ein Komma erreichen und alle Balances sind 0,
                // dann war das vorherige Komma gültig
                if bracket_balance == 0 && brace_balance == 0 && paren_balance == 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    
    // Am Ende prüfen ob unmatchede schließende Brackets existieren
    false
}

// Alternative: Split mit vollständiger Bracket-Balance Berechnung
pub fn split_with_bracket_balance(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    
    let mut bracket_balance = 0;
    let mut brace_balance = 0;
    let mut paren_balance = 0;
    
    let chars: Vec<char> = text.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        match c {
            '[' => bracket_balance += 1,
            ']' => bracket_balance -= 1,
            '{' => brace_balance += 1,
            '}' => brace_balance -= 1,
            '(' => paren_balance += 1,
            ')' => paren_balance -= 1,
            ',' => {
                // Komma ist nur ein Trenner wenn alle Balances 0 sind
                if bracket_balance == 0 && brace_balance == 0 && paren_balance == 0 {
                    result.push(&text[start..i]);
                    start = i + 1;
                }
            }
            _ => {}
        }
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}

// Optimierte Version mit Lookahead-Simulation
pub fn split_with_lookahead_optimized(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut paren_depth = 0;
    
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    
    let mut i = 0;
    while i < len {
        match chars[i] {
            '[' => bracket_depth += 1,
            ']' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1;
                }
            }
            '{' => brace_depth += 1,
            '}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
            }
            ',' => {
                // Prüfe Lookahead: nach diesem Komma darf kein schließendes Bracket kommen,
                // ohne dass wir in einem Bracket-Kontext sind
                
                // Wenn wir nicht in einem Bracket-Kontext sind, ist das Komma gültig
                if bracket_depth == 0 && brace_depth == 0 && paren_depth == 0 {
                    result.push(&text[start..i]);
                    start = i + 1;
                } else {
                    // Wir sind in einem Bracket-Kontext, prüfe ob danach ein schließendes Bracket kommt
                    let mut j = i + 1;
                    let mut found_closing = false;
                    
                    while j < len {
                        match chars[j] {
                            ']' | '}' | ')' => {
                                found_closing = true;
                                break;
                            }
                            '[' | '{' | '(' | ',' => {
                                // Neues Bracket oder Komma bricht die Suche
                                break;
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    
                    // Wenn kein schließendes Bracket gefunden wurde, ist das Komma gültig
                    if !found_closing {
                        result.push(&text[start..i]);
                        start = i + 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}
