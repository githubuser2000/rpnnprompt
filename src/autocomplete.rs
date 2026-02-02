use inquire::{autocompletion::Replacement, Autocomplete};
use std::error::Error;

// Einfache Autocomplete-Implementierung
#[derive(Clone)]
pub struct SimpleAutocomplete {
    items: Vec<String>,
}

impl SimpleAutocomplete {
    pub fn new(items: Vec<String>) -> Self {
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
