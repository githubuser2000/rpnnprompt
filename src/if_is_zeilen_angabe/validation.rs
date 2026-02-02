// if_is_zeilen_angabe/validation.rs
use super::split::*;

// Test der Lookahead-Implementierung
pub fn test_lookahead_implementation() {
    println!("Testing Lookahead Implementation");
    println!("=================================");
    
    let test_cases = vec![
        ("1,2,3", vec!["1", "2", "3"]),
        ("(1,2),3", vec!["(1,2)", "3"]),
        ("[1,2],3", vec!["[1,2]", "3"]),
        ("{1,2},3", vec!["{1,2}", "3"]),
        ("(1,2),[3,4],5", vec!["(1,2)", "[3,4]", "5"]),
        ("a,b,c", vec!["a", "b", "c"]),
        ("", vec![""]),
        ("(a,b),c,(d,e)", vec!["(a,b)", "c", "(d,e)"]),
        ("[1,2,(3,4)],5", vec!["[1,2,(3,4)]", "5"]),
        ("1,(2,3),4", vec!["1", "(2,3)", "4"]),
    ];
    
    for (input, expected) in test_cases {
        let result1 = split_with_lookahead(input);
        let result2 = split_with_bracket_balance(input);
        let result3 = split_with_lookahead_optimized(input);
        
        println!("\nInput: '{}'", input);
        println!("Expected: {:?}", expected);
        println!("split_with_lookahead: {:?}", result1);
        println!("split_with_bracket_balance: {:?}", result2);
        println!("split_with_lookahead_optimized: {:?}", result3);
        
        // Alle sollten gleich sein
        assert_eq!(result1, expected, "split_with_lookahead failed");
        assert_eq!(result2, expected, "split_with_bracket_balance failed");
        assert_eq!(result3, expected, "split_with_lookahead_optimized failed");
    }
    
    println!("\n✅ All lookahead tests passed!");
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn test_split_functions() {
        assert_eq!(split_with_bracket_balance("1,2,3"), vec!["1", "2", "3"]);
        assert_eq!(split_with_bracket_balance("(1,2),3"), vec!["(1,2)", "3"]);
        assert_eq!(split_with_bracket_balance("[1,2],3"), vec!["[1,2]", "3"]);
        assert_eq!(split_with_bracket_balance("{1,2},3"), vec!["{1,2}", "3"]);
        assert_eq!(split_with_bracket_balance(""), vec![""]);
    }
    
    #[test]
    fn test_is_zeilen_bruch_angabe_between_kommas() {
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2"));
        assert!(is_zeilen_bruch_angabe_between_kommas("-3/4"));
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2-3/4"));
        assert!(!is_zeilen_bruch_angabe_between_kommas("abc"));
    }
    
    #[test]
    fn test_is_zeilen_angabe() {
        assert!(is_zeilen_angabe("1,2,3"));
        assert!(is_zeilen_angabe("1-10,20-30"));
        assert!(is_zeilen_angabe("(1,2,3),[4,5]"));
        assert!(!is_zeilen_angabe("abc,def"));
    }
}
