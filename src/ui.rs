// ui.rs - Vollständig korrigierte Version
use crate::csv_data::CsvData;
use inquire::Text;
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("🔍 CSV Zwei-Stufen Autocomplete mit Zeilenangabe-Validierung\n");
    
    // CSV laden
    let csv_data = CsvData::new();
    
    loop {
        println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ SCHRITT 1: Wählen Sie eine erste Spalte                                ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        
        // Autocomplete für erste Spalte
        let first_autocomplete = csv_data.get_first_level_autocomplete();
        let first_choice = Text::new("Erste Spalte auswählen:")
            .with_autocomplete(first_autocomplete)
            .with_help_message("Beginnen Sie zu tippen für Vorschläge")
            .prompt()?;
        
        println!("✅ Ausgewählt: '{}'", first_choice);
        
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
        
        println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ SCHRITT 2: Wählen Sie eine zweite Spalte                                ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        println!("Verfügbare Optionen für '{}':", first_choice);
        
        // Autocomplete für zweite Spalte (abhängig von erster Wahl)
        let second_autocomplete = csv_data.get_second_level_autocomplete(&first_choice)
            .expect("Sollte existieren da seconds vorhanden sind");
        
        let second_choice = Text::new("Zweite Spalte auswählen:")
            .with_autocomplete(second_autocomplete)
            .with_help_message(&format!("{} Optionen verfügbar", seconds.len()))
            .prompt()?;
        
        println!("✅ Ausgewählt: '{}' → '{}'", first_choice, second_choice);
        
        // Zeige vollständige Details zum Paar
        csv_data.show_pair_details(&first_choice, &second_choice);
        
        println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ SCHRITT 3: Zeilenangabe eingeben und validieren                         ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        println!("Beispiele für gültige Zeilenangaben:");
        println!("  • Einzelne Zeilen: 1,2,3");
        println!("  • Bereiche: 3-8,12");
        println!("  • Mit v: v1, v2-5");
        println!("  • Gemischt: 1-5,10,12-15");
        println!("  • Generatoren: (1,3,5), [2,4,6]");
        println!("  • Bruchangaben: 1/2, 3/4-5/6");
        println!("  • Leer lassen für alle Zeilen");
        
        let mut zeilen_history = Vec::new();
        
        loop {
            let zeilen_input = Text::new("Zeilenangabe eingeben (oder 'fertig' zum Beenden):")
                .with_help_message("Drücken Sie Enter ohne Eingabe für alle Zeilen")
                .prompt()?;
            
            if zeilen_input.trim().eq_ignore_ascii_case("fertig") {
                break;
            }
            
            // Validierung durchführen
            let result = validate_and_process_zeilenangabe(
                &zeilen_input, 
                &first_choice, 
                &second_choice, 
                &csv_data
            );
            
            if let Ok(zeilen_numbers) = result {
                if !zeilen_numbers.is_empty() {
                    zeilen_history.extend(zeilen_numbers);
                }
            }
            
            println!("\nWeitere Zeilenangabe eingeben? (j/N)");
            let again = Text::new("")
                .with_default("n")
                .prompt()?;
            
            if !again.to_lowercase().starts_with('j') {
                break;
            }
        }
        
        // Zeige Zusammenfassung
        if !zeilen_history.is_empty() {
            show_ergebnis_zusammenfassung(&first_choice, &second_choice, &zeilen_history, &csv_data);
        }
        
        println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ Möchten Sie eine weitere Suche durchführen? (j/N)                       ┃");
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
        let again = Text::new("Weitersuchen?")
            .with_default("n")
            .prompt()?;
            
        if !again.to_lowercase().starts_with('j') {
            break;
        }
        
        println!("\n🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄🔄");
    }
    
    println!("\n👋 Programm beendet.");
    
    Ok(())
}

// Funktion zur Validierung und Verarbeitung von Zeilenangaben
fn validate_and_process_zeilenangabe(
    input: &str,
    first: &str,
    second: &str,
    csv_data: &CsvData,
) -> Result<Vec<i32>, String> {
    let trimmed = input.trim();
    
    println!("\n🔎 Validierung der Zeilenangabe '{}'...", trimmed);
    
    // Wenn leer, alle Zeilen nehmen
    if trimmed.is_empty() {
        println!("✅ Alle Zeilen werden ausgewählt");
        let all_rows = get_all_row_numbers_for_pair(first, second, csv_data);
        show_selected_rows(first, second, &all_rows, csv_data);
        return Ok(all_rows);
    }
    
    // Prüfe verschiedene Formate
    if crate::if_is_zeilen_angabe::is_zeilen_angabe(trimmed) {
        println!("✅ Gültige Zeilenangabe erkannt!");
        
        // Zeilen in einzelne Zahlen umwandeln
        if let Some(zeilen_numbers) = parse_zeilen_angabe_to_numbers(trimmed) {
            println!("✅ Extrahierte Zeilennummern: {:?}", zeilen_numbers);
            
            // Überprüfe, ob die Zeilen existieren
            let valid_rows = validate_row_numbers(&zeilen_numbers, csv_data);
            
            // Zeige die entsprechenden CSV-Zeilen an
            show_selected_rows(first, second, &valid_rows, csv_data);
            
            Ok(valid_rows)
        } else {
            let err = "Konnte Zeilenangabe nicht parsen".to_string();
            println!("⚠️ {}", err);
            Err(err)
        }
    } else if crate::if_is_zeilen_angabe::is_zeilen_bruch_angabe(trimmed) {
        println!("✅ Gültige Bruchangabe erkannt!");
        println!("✅ Bruchangabe: {}", trimmed);
        
        // Hier könnten Sie spezielle Bruch-Verarbeitung implementieren
        process_bruch_angabe(trimmed, first, second, csv_data);
        Ok(Vec::new())
    } else {
        let err = format!("Ungültige Eingabe: '{}'", trimmed);
        println!("⚠️ {}", err);
        println!("  Erlaubte Formate:");
        println!("    - Einzelne Zahlen: 1,2,3");
        println!("    - Bereiche: 1-5,10-15");
        println!("    - Mit 'v': v1, v2-5");
        println!("    - Generatoren: (1,3,5), [2,4,6]");
        println!("    - Brüche: 1/2, 3/4-5/6");
        Err(err)
    }
}

// Parst eine Zeilenangabe in eine Liste von Zahlen
fn parse_zeilen_angabe_to_numbers(input: &str) -> Option<Vec<i32>> {
    use crate::if_is_zeilen_angabe::split;
    
    let parts = split::split_with_bracket_balance(input);
    let mut numbers = Vec::new();
    
    for part in parts {
        if part.is_empty() {
            continue;
        }
        
        // Entferne führendes 'v' falls vorhanden
        let clean_part = if part.starts_with('v') || part.starts_with('V') {
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
        
        // Prüfe auf Generator-Notation mit führendem Zeichen
        if part.len() > 1 {
            let first_char = part.chars().next().unwrap();
            if !first_char.is_ascii_digit() && first_char != '-' && first_char != 'v' && first_char != 'V' {
                if let Some(generator_nums) = crate::if_is_zeilen_angabe::str_as_generator_to_list_of_num_strs(&part[1..]) {
                    for num_str in generator_nums {
                        if let Ok(num) = num_str.parse::<i32>() {
                            numbers.push(num);
                        }
                    }
                    continue;
                }
            }
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
    
    Some(numbers)
}

// Verarbeitet Bruchangaben
fn process_bruch_angabe(input: &str, first: &str, second: &str, _csv_data: &CsvData) {
    println!("ℹ️  Bruchangabe-Verarbeitung für '{}' → '{}'", first, second);
    println!("Bruch: {}", input);
    
    // Einfache Implementierung - kann erweitert werden
    let parts: Vec<&str> = input.split(',').collect();
    for part in parts {
        if part.contains('/') {
            println!("  Bruch '{}' erkannt", part);
        }
    }
}

// Holt alle Zeilennummern für ein Paar
fn get_all_row_numbers_for_pair(first: &str, second: &str, csv_data: &CsvData) -> Vec<i32> {
    let mut row_numbers = Vec::new();
    
    for (i, (first_cols, second_cols, _)) in csv_data.raw_data.iter().enumerate() {
        if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
            row_numbers.push((i + 1) as i32); // 1-basierte Zeilennummern
        }
    }
    
    row_numbers
}

// Validiert Zeilennummern gegen CSV-Daten
fn validate_row_numbers(zeilen_numbers: &[i32], csv_data: &CsvData) -> Vec<i32> {
    let max_row = csv_data.raw_data.len() as i32;
    let mut valid_rows = Vec::new();
    
    for &row_num in zeilen_numbers {
        if row_num >= 1 && row_num <= max_row {
            valid_rows.push(row_num);
        } else {
            println!("⚠️  Zeile {} existiert nicht (max: {})", row_num, max_row);
        }
    }
    
    valid_rows
}

// Zeigt ausgewählte Zeilen an
fn show_selected_rows(first: &str, second: &str, zeilen_numbers: &[i32], csv_data: &CsvData) {
    if zeilen_numbers.is_empty() {
        println!("ℹ️  Keine Zeilen ausgewählt");
        return;
    }
    
    println!("\n📋 Ausgewählte CSV-Zeilen für '{}' → '{}':", first, second);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let mut matching_rows = 0;
    let mut total_shown = 0;
    
    for &row_num in zeilen_numbers {
        let index = (row_num - 1) as usize; // 1-basiert zu 0-basiert
        
        if index < csv_data.raw_data.len() {
            let (first_cols, second_cols, numbers) = &csv_data.raw_data[index];
            
            if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
                matching_rows += 1;
                total_shown += 1;
                
                if total_shown <= 20 { // Begrenze die Ausgabe
                    println!("Zeile {:3}: {}", row_num, format_csv_row(first_cols, second_cols, numbers));
                }
            }
        }
    }
    
    if total_shown > 20 {
        println!("... und {} weitere Zeilen", total_shown - 20);
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Gesamt: {} von {} ausgewählten Zeilen passen", matching_rows, zeilen_numbers.len());
}

// Formatiert eine CSV-Zeile für die Ausgabe
fn format_csv_row(first_cols: &[String], second_cols: &[String], numbers: &str) -> String {
    let first_fmt = if first_cols.len() == 1 {
        first_cols[0].clone()
    } else {
        format!("({})", first_cols.join(", "))
    };
    
    let second_fmt = if second_cols.len() == 1 {
        second_cols[0].clone()
    } else {
        format!("({})", second_cols.join(", "))
    };
    
    format!("{} → {} → {}", first_fmt, second_fmt, numbers)
}

// Zeigt eine Ergebnis-Zusammenfassung
// ui.rs - Erweiterung der show_ergebnis_zusammenfassung Funktion
fn show_ergebnis_zusammenfassung(
    first: &str,
    second: &str,
    zeilen_history: &[i32],
    csv_data: &CsvData,
) {
    println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ 📊 ERGEBNIS-ZUSAMMENFASSUNG                                           ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    
    // Sortiere und entferne Duplikate
    let mut sorted_rows = zeilen_history.to_vec();
    sorted_rows.sort();
    sorted_rows.dedup();
    
    println!("Paar: '{}' → '{}'", first, second);
    println!("Ausgewählte Zeilen: {}", sorted_rows.len());
    
    if !sorted_rows.is_empty() {
        if sorted_rows.len() <= 10 {
            println!("Zeilennummern: {:?}", sorted_rows);
        } else {
            println!("Zeilennummern: {:?} ... und {} weitere", 
                &sorted_rows[..10], sorted_rows.len() - 10);
        }
        
        // Zeige Statistiken
        let total_matching = count_matching_rows(first, second, &sorted_rows, csv_data);
        println!("Davon passende Zeilen: {}", total_matching);
        
        // GENERIERE UND ZEIGE DEN KOMMANDO-STRING
        generate_and_show_command_string(first, second, &sorted_rows);
    }
}

// Neue Funktion: Generiert und zeigt den Kommando-String
fn generate_and_show_command_string(first: &str, second: &str, zeilen_numbers: &[i32]) {
    println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ 🚀 GENERIERTER KOMMANDO-AUFRUF                                        ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    
    // 1. Konvertiere Zeilennummern in das benötigte Format
    let zeilen_string = format_zeilen_fuer_kommando(zeilen_numbers);
    
    // 2. Baue den Kommando-String
    let command = format!(
        "../target/debug/mein-rpnn --vorhervonausschnitt {} --spaltenname {} {}",
        zeilen_string, first, second
    );
    
    println!("📋 Vollständiger Befehl:");
    println!("{}", command);
    
    // 3. Kopierbare Version (ohne Pfad für einfachere Nutzung)
    let simplified_command = format!(
        "mein-rpnn --vorhervonausschnitt {} --spaltenname {} {}",
        zeilen_string, first, second
    );
    
    println!("\n📝 Vereinfachte Version (zum Kopieren):");
    println!("{}", simplified_command);
    
    // 4. Option zum Kopieren in Zwischenablage (falls unterstützt)
    offer_copy_option(&command);
}

// Formatierte Zeilen für den Kommando-Aufruf
fn format_zeilen_fuer_kommando(zeilen_numbers: &[i32]) -> String {
    if zeilen_numbers.is_empty() {
        return String::new();
    }
    
    // Sortiere die Zahlen
    let mut sorted = zeilen_numbers.to_vec();
    sorted.sort();
    
    // Gruppiere zusammenhängende Bereiche
    let mut result = String::new();
    let mut start = sorted[0];
    let mut prev = sorted[0];
    
    for i in 1..sorted.len() {
        if sorted[i] == prev + 1 {
            // Fortlaufender Bereich
            prev = sorted[i];
        } else {
            // Bereich beenden und neuen starten
            if start == prev {
                result.push_str(&format!("{},", start));
            } else {
                result.push_str(&format!("{}-{},", start, prev));
            }
            start = sorted[i];
            prev = sorted[i];
        }
    }
    
    // Letzten Bereich hinzufügen
    if start == prev {
        result.push_str(&format!("{}", start));
    } else {
        result.push_str(&format!("{}-{}", start, prev));
    }
    
    result
}

// Bietet Option zum Kopieren an
fn offer_copy_option(command: &str) {
    use std::io::{self, Write};
    
    println!("\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ 📋 KOPIER-OPTIONEN                                                    ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    
    println!("1. Den obigen Befehl manuell kopieren");
    println!("2. Befehl in Datei speichern");
    println!("3. Direkt ausführen (experimentell)");
    
    print!("\nIhre Wahl (1-3, Enter für keine Aktion): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    let choice = input.trim();
    
    match choice {
        "2" => save_command_to_file(command),
        "3" => execute_command_experimental(command),
        _ => println!("ℹ️  Befehl kann manuell kopiert werden."),
    }
}

// Speichert den Befehl in eine Datei
fn save_command_to_file(command: &str) {
    use std::fs::File;
    use std::io::Write;
    
    let filename = "generated_command.sh";
    
    match File::create(filename) {
        Ok(mut file) => {
            // Unix/Linux Shell-Skript
            writeln!(file, "#!/bin/bash").unwrap();
            writeln!(file, "# Generierter Befehl").unwrap();
            writeln!(file, "{}", command).unwrap();
            
            // Machen Sie es ausführbar (Unix)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = file.metadata().unwrap().permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(filename, perms).unwrap();
            }
            
            println!("✅ Befehl gespeichert in: {}", filename);
            println!("   Ausführen mit: bash {}", filename);
        }
        Err(e) => {
            println!("⚠️  Fehler beim Speichern: {}", e);
        }
    }
}

// Versucht den Befehl auszuführen (experimentell)
fn execute_command_experimental(command: &str) {
    println!("⚠️  EXPERIMENTELL: Versuche Befehl auszuführen...");
    
    // Entferne den relativen Pfad für die Ausführung
    let cmd_without_path = command.replace("../target/debug/", "");
    
    println!("Ausführe: {}", cmd_without_path);
    
    // ACHTUNG: Dies ist nur ein Beispiel - in der Praxis möchten Sie
    // wahrscheinlich den Benutzer fragen, bevor Sie etwas ausführen
    println!("ℹ️  Ausführung deaktiviert (Sicherheitsfeature)");
    println!("   Befehl kann manuell ausgeführt werden.");
}

// Zählt passende Zeilen
fn count_matching_rows(first: &str, second: &str, zeilen_numbers: &[i32], csv_data: &CsvData) -> i32 {
    let mut count = 0;
    
    for &row_num in zeilen_numbers {
        let index = (row_num - 1) as usize;
        
        if index < csv_data.raw_data.len() {
            let (first_cols, second_cols, _) = &csv_data.raw_data[index];
            
            if first_cols.contains(&first.to_string()) && second_cols.contains(&second.to_string()) {
                count += 1;
            }
        }
    }
    
    count
}
