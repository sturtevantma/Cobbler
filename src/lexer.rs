use std::sync::LazyLock;
use regex::{Regex, RegexSet};

// Get index from TokenType macro
macro_rules! usize {
    ($name:ident) => { TokenType::$name as usize }
}

// Generate enum, and both regex pattern maps
macro_rules! generate_enum {
    ($( ($name:ident, $pattern:expr) ),*) => {
        #[allow(dead_code)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum TokenType { // TokenType - Enum containing the Kind of token
            $( $name ),*
        }

        impl TokenType {
            // Get enum from index returns None if out of range
            pub fn from(index: usize) -> Option<Self> {
                // Generate big match for each token type and pattern match it with its corresponding usize (achieved with cast)
                match index {
                    $( i if i == TokenType::$name as usize => return Some(TokenType::$name), )*
                    _ => return None, // Index out of bounds
                }

            }

            #[allow(dead_code)]
            // Returns number of entries in TokenType
            pub fn len() -> usize {
                usize!(EOF) + 1
            }
        }

        #[allow(dead_code)]
        // RegexSet used for mass check
        pub static PATTERNSET: LazyLock<RegexSet> = LazyLock::new(|| {
            RegexSet::new(&[
                $( $pattern ),*
            ]).unwrap()
        });

        #[allow(dead_code)]
        // Individual patterns for single checks
        pub static PATTERNS: [LazyLock<Regex>; usize!(EOF) + 1] = [
            $( LazyLock::new(|| {
                Regex::new($pattern).unwrap()
            }) ),*
        ];
    }
}

// Create enums takes the form (EnumVariant, Pattern)
generate_enum!(
    /* Groupings */
    (LeftParen,    r"^(\()"),
    (RightParen,   r"^(\))"),
    (LeftBrace,    r"^(\{)"),
    (RightBrace,   r"^(\})"),
    (LeftBracket,  r"^(\[)"),
    (RightBracket, r"^(\])"),
    
    /* Delimiters */
    (Comma,      r"^(,)"),
    (Colon,      r"^(:)"),
    (RightArrow, r"^(->)"),
    (Dollar,     r"^(\$)"),
    (Dot,        r"^(\.)"),
    
    /* Operators */
    (Plus,            r"^(\+)"),
    (Minus,           r"^(-)"),
    (Star,            r"^(\*)"),
    (StarStar,        r"^(\*\*)"),
    (Slash,           r"^(/)"),
    (SlashSlash,      r"^(//)"),
    (Percent,         r"^(%)"),
    (Bang,            r"^(!)"),
    (BangEqual,       r"^(!=)"),
    (Less,            r"^(<)"),
    (LessEqual,       r"^(<=)"),
    (Greater,         r"^(>)"),
    (GreaterEqual,    r"^(>=)"),
    (EqualEqual,      r"^(==)"),
    (And,             r"^((and)|(&&))"),
    (Or,              r"^((or)|(\|\|))"),
    (In,              r"^(in)"),
    (As,              r"^(as)"),
    (Equal,           r"^(=)"),
    (PlusEqual,       r"^(\+=)"),
    (MinusEqual,      r"^(-=)"),
    (StarEqual,       r"^(\*=)"),
    (StarStarEqual,   r"^(\*\*=)"),
    (SlashEqual,      r"^(/=)"),
    (SlashSlashEqual, r"^(//=)"),
    (PercentEqual,    r"^(%=)"),
    
    /* Targets and Subscriptions */
    (AtLoad, r"^(@load)"),
    (AtTick, r"^(@tick)"),
    (AtE,    r"^(@e)"),
    (AtS,    r"^(@s)"),
    (AtR,    r"^(@r)"),
    (AtP,    r"^(@p)"),
    (AtN,    r"^(@n)"),
    (AtA,    r"^(@a)"),
    
    /* Literals */
    (Identifier,    r"^([a-zA-Z_][a-zA-Z0-9_]*)"),
    (Null,          r"^(NULL)"),
    (True,          r"^(TRUE)"),
    (False,         r"^(FALSE)"),
    (StringLiteral, r#"^("[^"]*"|'[^']*')"#),
    (IntLiteral,    r"^(\d+)"),
    (FloatLiteral,  r"^((\d*\.?\d+(f|F))|(\d*\.\d+))"),
    (DoubleLiteral, r"^((\d*\.?\d+(d|D)))"),
    
    /* Types */
    (Uint8,  r"^((uint8)|(unsigned byte))"),
    (Uint16, r"^((uint16)|(unsigned short))"),
    (Int8,   r"^((int8)|(byte))"),
    (Int16,  r"^((int16)|(short))"),
    (Int32,  r"^((int32)|(int))"),
    (Int64,  r"^((int64)|(long))"),
    (Float,  r"^(float)"),
    (Double, r"^(double)"),
    (Bool,   r"^(bool)"),
    (String, r"^(String)"),
    (Array,  r"^(Array)"),
    (Object, r"^(Object)"),
    (Void,   r"^(void)"),
    
    /* Control */
    (If,       r"^(if)"),
    (Else,     r"^(else)"),
    (For,      r"^(for)"),
    (While,    r"^(while)"),
    (Break,    r"^(break)"),
    (Continue, r"^(continue)"),
    (Return,   r"^(return)"),
    
    /* Keywords */
    (Var,     r"^(var)"),
    (Func,    r"^f(unc)"),
    (Include, r"^(include)"),
    (Public,  r"^(public)"),
    (Private, r"^(private)"),

    // Special
    (Backtick, r"^(\\)"),
    (Raw,      r"^(RAW)"),
    (Name,     r"^(PACK)"),
    (Comment,  r"^(#.*)"),
    (Blank,    r"^([^\S\n])"),
    (EOL,      r"^(\n)"),
    (EOF,      r"")
);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    line: usize, // Potentially more performant sizes available for line and col.
    col: usize,  // Notably, it would make sense to need far fewer column bits
    kind: TokenType,
    lexeme: String
}

#[allow(dead_code)]
/* MUST BE MUTABLE TO SCAN TOKENS */
pub struct Lexer {
    tokens: Vec<Token>,
    string: String,
    current: usize,
    size: usize,
}

#[allow(dead_code)]
// The lexer takes a string (from a file or elsewhere)
// and converts it to a vector of tokens, 
// seperating out keywords, identifiers, operators, etc.
impl Lexer {
    // Create a new Lexer object
    pub fn new(s: &str) -> Lexer {
        Lexer {
            tokens : Vec::new(),
            string : String::from(s),
            current: 0,
            size   : s.len()
        }
    }

    // Set new string to lexer
    pub fn set(&mut self, s: &str) {
        self.current = 0;
        self.size = s.len();
        self.tokens = Vec::new();
        self.string = String::from(s);
    }

    // Prints currently scanned tokens
    pub fn print_tokens(&self) {
        println!("{:?}", self.tokens);
    }

    // Finds the longest matching TokenType
    fn get_longest_match(&self) -> Option<(TokenType, String)> {
        // If at the end of the file match to EOF
        if self.current >= self.size {
            return Some((TokenType::EOF, "".to_string()));
        }
        // Check against pattern set
        let matches = PATTERNSET.matches(&self.string[self.current..]);
        if !matches.matched_any() {
            // If no match return None
            return None;
        }

        let mut longest = (TokenType::EOF, "".to_string());
        for index in matches.iter() {
            // For each pattern that succeeded
            if matches.matched(index) {
                let pat = &PATTERNS[index];
                // If match is longer than current match update longest
                if let Some(pat) = pat.find(&self.string[self.current..]) {
                    if pat.as_str().len() >= longest.1.len() {
                        longest = (TokenType::from(index)?, pat.as_str().to_string());
                    }
                }
            }
        }
        // Return longest match
        return Some(longest);

        
    }

    // Scan tokens from string into tokens
    pub fn scan_tokens(&mut self) {
        // Counters are mutable
        let mut l: usize = 1;
        let mut c: usize = 1;
        // While we can match tokens into variable m
        while let Some(m) = self.get_longest_match() {
            // match on m for special tokens, increment current
            match m.0 {
                // break on EOF
                TokenType::EOF => {
                    self.tokens.push(Token { line: l, col: c, kind: m.0, lexeme: m.1 });
                    break;
                }
                // End of line -> increment line counter, reset column counter
                TokenType::EOL => {
                    self.current += 1;
                    l += 1;
                    c = 0;
                }
                // blank or comment -> increment column by length of match
                TokenType::Blank | TokenType::Comment => {
                    c += m.1.len();
                    self.current += m.1.len();
                }
                // Default case -> add token to tokens, increment column
                _ => {
                    let length = m.1.len();
                    self.tokens.push(Token { line: l, col: c, kind: m.0, lexeme: m.1 });
                    c += length;
                    self.current += length;
                }
            }

        }
    }
}