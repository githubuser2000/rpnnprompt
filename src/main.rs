use inquire::{autocompletion::Replacement, Autocomplete};
use std::error::Error;

// Einfacher Autocomplete für Farben
#[derive(Clone)]
struct ColorAutocomplete {
    colors: Vec<String>,
}

impl ColorAutocomplete {
    fn new() -> Self {
        Self {
            colors: vec![
                "rot".to_string(),
                "grün".to_string(),
                "blau".to_string(),
                "gelb".to_string(),
                "orange".to_string(),
                "lila".to_string(),
                "rosa".to_string(),
                "braun".to_string(),
                "schwarz".to_string(),
                "weiß".to_string(),
                "grau".to_string(),
                "türkis".to_string(),
                "magenta".to_string(),
                "cyan".to_string(),
                "silber".to_string(),
                "gold".to_string(),
            ],
        }
    }
}

impl Autocomplete for ColorAutocomplete {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let input_lower = input.to_lowercase();
        let suggestions: Vec<String> = self
            .colors
            .iter()
            .filter(|color| color.to_lowercase().contains(&input_lower))
            .take(5)
            .cloned()
            .collect();
        
        Ok(suggestions)
    }

    fn get_completion(
        &mut self,
        _input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, Box<dyn Error + Send + Sync>> {
        match highlighted_suggestion {
            Some(suggestion) => Ok(Replacement::Some(suggestion)),
            None => Ok(Replacement::None),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Farb-Autocomplete Demo ===\n");
    
    let color = inquire::Text::new("Was ist Ihre Lieblingsfarbe?")
        .with_autocomplete(ColorAutocomplete::new())
        .prompt()?;
    
    println!("Ihre Lieblingsfarbe ist: {}", color);
    
    Ok(())
}
