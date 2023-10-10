// RTF parser for Text Editor
// Support RTF version 1.9.1

mod utils;
mod lexer;
mod parser;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    PlainText(&'a str),
    OpeningBracket,
    ClosingBracket,
    SemiColon,
    CRLF,  // Line-return \n
    ControlSymbol(ControlSymbol<'a>),
}

// Parameters for a control word
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Property {
    On,  // 1
    Off, // 0
    Value(i32),
    None, // No parameter
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ControlWord<'a> {
    Italic,
    Bold,
    Underline,
    FontNumber,
    Unknown(&'a str),
}

// A control symbol is a pair (control_word, property)
type ControlSymbol<'a> = (ControlWord<'a>, Property);

impl<'a> ControlWord<'a> {
    pub fn from(input: &str) -> ControlSymbol {
        // Loop backward the string to get the number
        let mut it = input.chars().rev();
        let mut suffix_index = 0;
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => { suffix_index += 1; }
                _ => break
            }
        }

        // f0 -> prefix: f, suffix: 0
        let index = input.len() - suffix_index;
        let prefix = &input[..index];
        let suffix = &input[index..];

        let property =
            if suffix == "" { Property::None }
            else { Property::Value(suffix.parse::<i32>().expect(&format!("[Lexer] Unable to parse {}", &suffix))) };

        let control_word = match prefix {
            r"\i" => ControlWord::Italic,
            r"\b" => ControlWord::Bold,
            r"\u" => ControlWord::Underline,
            r"\f" => ControlWord::FontNumber,
            _ => ControlWord::Unknown(prefix),
        };
        return (control_word, property);
    }
}

#[cfg(test)]
mod tests {
    use crate::{ControlWord, Property};

    #[test]
    fn control_word_from_input_test() {
        let input = r"\rtf1";
        assert_eq!(
            ControlWord::from(input),
            (ControlWord::Unknown("\\rtf"), Property::Value(1))
        )
    }
}