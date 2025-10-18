use std::collections::HashMap;
use std::io::{self, Write};

mod custom_definitions;
mod botan_parser;

/// A simple decoder for CW beacon messages
pub struct BotanDecoder {
    /// Mapping table for character/pattern decoding
    decode_map: HashMap<String, String>,
}

impl BotanDecoder {
    /// Create a new decoder with default mappings
    pub fn new() -> Self {
        let mut decode_map = HashMap::new();
        
        // Add some common CW/morse patterns - you can customize these based on your definitions
        // These are examples that you should replace with your actual definitions
        decode_map.insert(".-".to_string(), "A".to_string());
        decode_map.insert("-...".to_string(), "B".to_string());
        decode_map.insert("-.-.".to_string(), "C".to_string());
        decode_map.insert("-..".to_string(), "D".to_string());
        decode_map.insert(".".to_string(), "E".to_string());
        decode_map.insert("..-.".to_string(), "F".to_string());
        decode_map.insert("--.".to_string(), "G".to_string());
        decode_map.insert("....".to_string(), "H".to_string());
        decode_map.insert("..".to_string(), "I".to_string());
        decode_map.insert(".---".to_string(), "J".to_string());
        decode_map.insert("-.-".to_string(), "K".to_string());
        decode_map.insert(".-..".to_string(), "L".to_string());
        decode_map.insert("--".to_string(), "M".to_string());
        decode_map.insert("-.".to_string(), "N".to_string());
        decode_map.insert("---".to_string(), "O".to_string());
        decode_map.insert(".--.".to_string(), "P".to_string());
        decode_map.insert("--.-".to_string(), "Q".to_string());
        decode_map.insert(".-.".to_string(), "R".to_string());
        decode_map.insert("...".to_string(), "S".to_string());
        decode_map.insert("-".to_string(), "T".to_string());
        decode_map.insert("..-".to_string(), "U".to_string());
        decode_map.insert("...-".to_string(), "V".to_string());
        decode_map.insert(".--".to_string(), "W".to_string());
        decode_map.insert("-..-".to_string(), "X".to_string());
        decode_map.insert("-.--".to_string(), "Y".to_string());
        decode_map.insert("--..".to_string(), "Z".to_string());
        
        // Numbers
        decode_map.insert(".----".to_string(), "1".to_string());
        decode_map.insert("..---".to_string(), "2".to_string());
        decode_map.insert("...--".to_string(), "3".to_string());
        decode_map.insert("....-".to_string(), "4".to_string());
        decode_map.insert(".....".to_string(), "5".to_string());
        decode_map.insert("-....".to_string(), "6".to_string());
        decode_map.insert("--...".to_string(), "7".to_string());
        decode_map.insert("---..".to_string(), "8".to_string());
        decode_map.insert("----.".to_string(), "9".to_string());
        decode_map.insert("-----".to_string(), "0".to_string());

        BotanDecoder { decode_map }
    }

    /// Create a decoder with custom mappings
    pub fn with_custom_mappings(mappings: HashMap<String, String>) -> Self {
        BotanDecoder {
            decode_map: mappings,
        }
    }

    /// Load decoder from configuration file
    pub fn from_config_file(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use std::fs;
        
        let content = fs::read_to_string(config_path)?;
        let mut decode_map = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse pattern = value format
            if let Some(eq_pos) = line.find('=') {
                let pattern = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                
                if !pattern.is_empty() && !value.is_empty() {
                    decode_map.insert(pattern, value);
                }
            }
        }
        
        Ok(BotanDecoder { decode_map })
    }

    /// Add or update a mapping
    pub fn add_mapping(&mut self, pattern: String, decoded: String) {
        self.decode_map.insert(pattern, decoded);
    }

    /// Decode a CW beacon string
    /// Expects patterns to be separated by spaces, words by multiple spaces or special delimiters
    pub fn decode(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        let mut result = String::new();
        
        // Split by double spaces to separate words
        let words: Vec<&str> = input.split("  ").collect();
        
        for (word_idx, word) in words.iter().enumerate() {
            if word_idx > 0 {
                result.push(' ');
            }
            
            // Split each word by single spaces to get individual patterns
            let patterns: Vec<&str> = word.split(' ').filter(|s| !s.is_empty()).collect();
            
            for pattern in patterns {
                match self.decode_map.get(pattern) {
                    Some(decoded_char) => result.push_str(decoded_char),
                    None => {
                        return Err(format!("Unknown pattern: '{}'", pattern));
                    }
                }
            }
        }

        Ok(result)
    }

    /// Get all available patterns
    pub fn get_patterns(&self) -> Vec<String> {
        self.decode_map.keys().cloned().collect()
    }

    /// Print available mappings
    pub fn print_mappings(&self) {
        println!("Available mappings:");
        let mut mappings: Vec<_> = self.decode_map.iter().collect();
        mappings.sort_by_key(|(pattern, _)| pattern.as_str());
        
        for (pattern, decoded) in mappings {
            println!("  '{}' -> '{}'", pattern, decoded);
        }
    }
}

/// Decode a BOTAN beacon message - main entry point for BOTAN decoding
pub fn decode_botan_beacon(input: &str) -> Result<String, String> {
    match botan_parser::parse_botan_beacon(input) {
        Ok(beacon_data) => Ok(format!("{}", beacon_data)),
        Err(error) => Err(error),
    }
}

impl Default for BotanDecoder {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("BOTAN Satellite Beacon Decoder");
    println!("==============================");
    println!("This decoder processes BOTAN satellite beacon messages.");
    println!("Expected format: BOTAN JS1YPT (Optional<RSSI>) <16-hex-digit-data>");
    println!("Example: BOTAN JS1YPT SI8640 A67C8D5E2AA13608");
    println!();
    
    // Interactive mode
    loop {
        print!("Enter BOTAN beacon to decode (or 'quit' to exit): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_ascii_uppercase();
                
                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                // Check if it's a BOTAN beacon format
                if input.starts_with("BOTAN") {
                    match botan_parser::parse_botan_beacon(&input) {
                        Ok(beacon_data) => {
                            println!("\n{}", beacon_data);
                        },
                        Err(error) => {
                            println!("BOTAN Parsing Error: {}", error);
                        }
                    }
                } else {
                    // Fall back to legacy morse code decoder for non-BOTAN inputs
                    let decoder = BotanDecoder::new();
                    match decoder.decode(&input) {
                        Ok(decoded) => println!("Legacy Morse Decoded: {}", decoded),
                        Err(error) => println!("Legacy Decoding Error: {}", error),
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_decoding() {
        let decoder = BotanDecoder::new();
        
        // Test single letters
        assert_eq!(decoder.decode(".-").unwrap(), "A");
        assert_eq!(decoder.decode("-...").unwrap(), "B");
        assert_eq!(decoder.decode("-.-.").unwrap(), "C");
        
        // Test words (patterns separated by spaces)
        assert_eq!(decoder.decode(".- -...").unwrap(), "AB");
        assert_eq!(decoder.decode("... --- ...").unwrap(), "SOS");
        
        // Test multiple words (separated by double spaces)  
        assert_eq!(decoder.decode(".... ..  .-- --- .-. .-.. -..").unwrap(), "HI WORLD");
    }

    #[test]
    fn test_numbers() {
        let decoder = BotanDecoder::new();
        assert_eq!(decoder.decode(".---- ..--- ...--").unwrap(), "123");
    }

    #[test]
    fn test_unknown_pattern() {
        let decoder = BotanDecoder::new();
        assert!(decoder.decode(".-.-.-").is_err());
    }

    #[test]
    fn test_empty_input() {
        let decoder = BotanDecoder::new();
        assert_eq!(decoder.decode("").unwrap(), "");
        assert_eq!(decoder.decode("   ").unwrap(), "");
    }

    #[test]
    fn test_custom_mappings() {
        let mut custom_map = HashMap::new();
        custom_map.insert("X".to_string(), "SPECIAL".to_string());
        custom_map.insert("Y".to_string(), "CODE".to_string());
        
        let decoder = BotanDecoder::with_custom_mappings(custom_map);
        assert_eq!(decoder.decode("X Y").unwrap(), "SPECIALCODE");
    }
}