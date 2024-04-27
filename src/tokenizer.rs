use std::{path::PathBuf, rc::Rc};

use crate::{
    tokens::{Keyword, Symbol, Token},
    utils::{
        remove_comments, replace_carriage_returns_with_single_space,
        replace_line_breaks_with_single_space, replace_multi_spaces_with_single_space,
        replace_tabs_with_single_space,
    },
};

#[derive(Debug)]
pub struct JackTokenizer {
    /// The input tokenized
    tokens: Vec<Rc<Token>>,
    /// The current token being processed.
    current_token: Option<Rc<Token>>,
    /// The current token index
    current_token_index: usize,
    /// The next token to be processed (since jack is LL1, we only need one lookahead token)
    next_token: Option<Rc<Token>>,
}

impl JackTokenizer {
    pub fn new(path: PathBuf) -> Self {
        let content = std::fs::read_to_string(path).expect("failed to read file");
        let clean_content = Self::clean_input(content);

        let tokens: Vec<_> = Self::into_tokens(clean_content)
            .into_iter()
            .map(Rc::new)
            .collect();
        let current_token = tokens.first().cloned();
        let next_token = tokens.get(1).cloned();

        Self {
            tokens,
            current_token,
            current_token_index: 0,
            next_token,
        }
    }

    /// Removes the comments, replaces lines breaks with a single space,
    /// replaces tabs (\r) with a single space character and finally
    /// replaces multiple space characters with a single space character
    fn clean_input(input: String) -> String {
        let removed_comments = remove_comments(input);
        let removed_line_breaks = replace_line_breaks_with_single_space(removed_comments);
        let removed_tabs = replace_tabs_with_single_space(removed_line_breaks);
        let removed_carriage = replace_carriage_returns_with_single_space(removed_tabs);
        replace_multi_spaces_with_single_space(removed_carriage)
    }

    /// Converts the input to a stream of tokens
    /// This is done by iterating the characters
    /// of the input code and handling 3 cases:
    /// 1. char is a symbol or a space: we check
    ///   if the acc string contains a keyword,
    ///   a digit or a identifier.
    /// 2. char is a quote: we take the chars up
    ///   until we reach the next quote.
    /// 3. char is alphanumeric: we accumulate it
    ///   into a string until point 1. is reached.
    fn into_tokens(input: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut acc = String::new();
        let mut i = 0;

        let chars: Vec<_> = input.chars().collect();

        while i < chars.len() {
            let c = chars[i];
            let is_symbol = Symbol::is_symbol(&c);
            // If there is a space or a symbol, we check acc
            if c == ' ' || is_symbol {
                if !acc.is_empty() {
                    if Keyword::is_keyword(&acc) {
                        tokens.push(Token::Keyword(acc.clone().into()));
                    } else if let Ok(u) = str::parse::<u16>(&acc) {
                        tokens.push(Token::IntConst(u))
                    } else {
                        tokens.push(Token::Identifier(acc.clone()))
                    }
                    acc.clear();
                }
                if is_symbol {
                    tokens.push(Token::Symbol(c.into()));
                }
                i += 1;
            }
            if c == '"' {
                let string_constant: String = chars[i + 1..chars.len()]
                    .iter()
                    .take_while(|c| **c != '"')
                    .collect();
                i += string_constant.len() + 2; // we skip the 2 quotes
                tokens.push(Token::StringConst(string_constant));
            }
            // If c is a digit, letter or _, it can be a identifier,
            // keyword or digit
            if c.is_alphanumeric() || c == '_' {
                acc.push(c);
                i += 1;
            }
        }

        tokens
    }

    pub fn has_more_tokens(&self) -> bool {
        self.current_token.is_some()
    }

    pub fn advance(&mut self) {
        self.current_token = self.next_token.take();
        self.next_token = self.tokens.get(self.current_token_index + 2).cloned();
        self.current_token_index += 1;
    }

    pub fn current_token(&self) -> Rc<Token> {
        self.current_token
            .as_ref()
            .expect("no current token")
            .clone()
    }

    pub fn keyword(&self) -> Keyword {
        match &*self.current_token() {
            Token::Keyword(k) => k.clone(),
            _ => panic!("current token isn't a keyword"),
        }
    }

    pub fn symbol(&self) -> Symbol {
        match &*self.current_token() {
            Token::Symbol(s) => s.clone(),
            _ => panic!("current token isn't a symbol"),
        }
    }

    pub fn identifier(&self) -> String {
        match &*self.current_token() {
            Token::Identifier(s) => s.clone(),
            _ => panic!("current token isn't a identifier"),
        }
    }

    pub fn int_val(&self) -> u16 {
        match &*self.current_token() {
            Token::IntConst(i) => *i,
            _ => panic!("current token isn't a int value"),
        }
    }

    pub fn string_val(&self) -> String {
        match &*self.current_token() {
            Token::StringConst(s) => s.clone(),
            _ => panic!("current token isn't a string value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        // Given
        let lines = r"// File name: projects/10/Square/SquareGame.jack

        // (same as projects/9/Square/SquareGame.jack)
        /**
         * Implements the Square game.
         * This simple game allows the user to move a black square around
         * the screen, and change the square's size during the movement.
         * When the game starts, a square of 30 by 30 pixels is shown at the
         * top-left corner of the screen. The user controls the square as follows.
         * The 4 arrow keys are used to move the square up, down, left, and right.
         * The 'z' and 'x' keys are used, respectively, to decrement and increment
         * the square's size. The 'q' key is used to quit the game.
         */
        class SquareGame {
           field Square square; // the square of this game
           field int direction; // the square's current direction: 
                                // 0=none, 1=up, 2=down, 3=left, 4=right"
            .to_string();

        // When
        let lines_without_comments = JackTokenizer::clean_input(lines);

        // Then
        assert_eq!(
            lines_without_comments,
            "class SquareGame { field Square square; field int direction;"
        )
    }

    #[test]
    fn test_into_tokens() {
        // Given
        let input = r"/**
        * Implements the Square game.
        * This simple game allows the user to move a black square around
        * the screen, and change the square's size during the movement.
        * When the game starts, a square of 30 by 30 pixels is shown at the
        * top-left corner of the screen. The user controls the square as follows.
        * The 4 arrow keys are used to move the square up, down, left, and right.
        * The 'z' and 'x' keys are used, respectively, to decrement and increment
        * the square's size. The 'q' key is used to quit the game.
        */
       class SquareGame {
          field Square square; // the square of this game
          field int direction; // the square's current direction: 
                               // 0=none, 1=up, 2=down, 3=left, 4=right
       
          /** Constructs a new Square Game. */
          constructor SquareGame new() {
             // Creates a 30 by 30 pixels square and positions it at the top-left
             // of the screen.
             let square = Square.new(0, 0, 30);
             let direction = 0;  // initial state is no movement
             return this;
          }"
        .to_string();

        // When
        let cleaned_input = JackTokenizer::clean_input(input);
        let tokens = JackTokenizer::into_tokens(cleaned_input);

        // Then
        pretty_assertions::assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Class),
                Token::Identifier(String::from("SquareGame")),
                Token::Symbol(Symbol::CurlLeft),
                Token::Keyword(Keyword::Field),
                Token::Identifier(String::from("Square")),
                Token::Identifier(String::from("square")),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Field),
                Token::Keyword(Keyword::Int),
                Token::Identifier(String::from("direction")),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Constructor),
                Token::Identifier(String::from("SquareGame")),
                Token::Identifier(String::from("new")),
                Token::Symbol(Symbol::ParenthesisLeft),
                Token::Symbol(Symbol::ParenthesisRight),
                Token::Symbol(Symbol::CurlLeft),
                Token::Keyword(Keyword::Let),
                Token::Identifier(String::from("square")),
                Token::Symbol(Symbol::Equal),
                Token::Identifier(String::from("Square")),
                Token::Symbol(Symbol::Dot),
                Token::Identifier(String::from("new")),
                Token::Symbol(Symbol::ParenthesisLeft),
                Token::IntConst(0),
                Token::Symbol(Symbol::Comma),
                Token::IntConst(0),
                Token::Symbol(Symbol::Comma),
                Token::IntConst(30),
                Token::Symbol(Symbol::ParenthesisRight),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Identifier(String::from("direction")),
                Token::Symbol(Symbol::Equal),
                Token::IntConst(0),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Return),
                Token::Keyword(Keyword::This),
                Token::Symbol(Symbol::Semicolon),
                Token::Symbol(Symbol::CurlRight),
            ]
        );
    }

    #[test]
    fn test_into_tokens_complex() {
        // Given
        let input = r#"
            let length = Keyboard.readInt("HOW MANY NUMBERS? ");"#
            .to_string();

        // When
        let cleaned_input = JackTokenizer::clean_input(input);
        let tokens = JackTokenizer::into_tokens(cleaned_input);

        // Then
        pretty_assertions::assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Let),
                Token::Identifier(String::from("length")),
                Token::Symbol(Symbol::Equal),
                Token::Identifier(String::from("Keyboard")),
                Token::Symbol(Symbol::Dot),
                Token::Identifier(String::from("readInt")),
                Token::Symbol(Symbol::ParenthesisLeft),
                Token::StringConst(String::from("HOW MANY NUMBERS? ")),
                Token::Symbol(Symbol::ParenthesisRight),
                Token::Symbol(Symbol::Semicolon),
            ]
        );
    }

    #[test]
    fn test_advance() {
        // Given
        let mut tokenizer = JackTokenizer::new(PathBuf::from("test_data/Square/SquareGame.jack"));

        // When
        tokenizer.advance();

        // Then
        assert_eq!(tokenizer.current_token_index, 1);
        assert_eq!(
            &*tokenizer.current_token.unwrap(),
            &Token::Identifier(String::from("SquareGame"))
        );
        assert_eq!(
            &*tokenizer.next_token.unwrap(),
            &Token::Symbol(Symbol::CurlLeft)
        );
    }
}
