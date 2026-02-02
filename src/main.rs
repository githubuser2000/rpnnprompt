// main.rs - aktualisiert
mod autocomplete;
mod csv_parser;
mod csv_data;
mod ui;
mod zeilen_parser;  // Neue Modul-Deklaration
mod if_is_zeilen_angabe;  // Falls noch nicht vorhanden

use anyhow::Result;

fn main() -> Result<()> {
    // Optional: Test der Zeilenangabe-Validierung
    #[cfg(debug_assertions)]
    test_zeilen_validation();
    
    ui::run()
}

#[cfg(debug_assertions)]
fn test_zeilen_validation() {
    use if_is_zeilen_angabe::*;
    
    println!("Testing Zeilenangabe Validation...");
    
    let test_cases = vec![
        ("1,2,3", true),
        ("3-8,12", true),
        ("v1,v2-5", true),
        ("(1,3,5)", true),
        ("1/2-3/4", true),
        ("abc", false),
        ("1.5,2", false),
    ];
    
    for (input, expected) in test_cases {
        let result = is_zeilen_angabe(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} '{}' -> {}", status, input, result);
    }
}
