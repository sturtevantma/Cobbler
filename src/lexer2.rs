use std::collections::HashMap as Map;

macro_rules! new_trie_node {
    () => {TrieNode { value: None, children: Map::new()}}
}

macro_rules! some_digit {
    () => {
        Some('0')
        | Some('9')
        | Some('8')
        | Some('7')
        | Some('6')
        | Some('5')
        | Some('4')
        | Some('3')
        | Some('2')
        | Some('1')
    };
}


macro_rules! generate_match_and_enum {
    ($( 
        ( $name:ident : $( 
                        $pattern:expr
                       ),+ )
    ),*
    $(,)?) => 
    {
        
        #[allow(dead_code)]
        pub enum TokenType {
            $( $name ),*
        }

        #[allow(dead_code)]
        impl Lexer {
            pub fn new() -> Lexer {
                let mut l = Lexer {
                    trie: Trie::new()
                };
                $(
                    $( l.trie.insert( $pattern, TokenType::$name ); )+
                )*
                return l
            }
        }

}}

// Create enums takes the form (EnumVariant, $(Pattern,)*)
// '_' is used for keys which need to be overwritten
generate_match_and_enum!(
    /* Groupings */
    (LeftParen:    "("),
    (RightParen:   ")"),
    (LeftBrace:    "{"),
    (RightBrace:   "}"),
    (LeftBracket:  "["),
    (RightBracket: "]"),

    /* Delimiters */
    (Comma:      ","),
    (Colon:      ":"),
    (RightArrow: "->"),
    (Dollar:     "$"),
    (Dot:        "."),

    /* Operators */
    (Plus:            "+"),
    (Minus:           "-"),
    (Star:            "*"),
    (StarStar:        "**"),
    (Slash:           "/"),
    (SlashSlash:      "//"),
    (Percent:         "%"),
    (Bang:            "!"),
    (BangEqual:       "!="),
    (Less:            "<"),
    (LessEqual:       "<="),
    (Greater:         ">"),
    (GreaterEqual:    ">="),
    (EqualEqual:      "=="),
    (And:             "and", "&&"),
    (Or:              "or", "||"),
    (In:              "in"),
    (As:              "as"),
    (Equal:           "="),
    (PlusEqual:       "+="),
    (MinusEqual:      "-="),
    (StarEqual:       "*="),
    (StarStarEqual:   "**="),
    (SlashEqual:      "/="),
    (SlashSlashEqual: "//="),
    (PercentEqual:    "%="),

    /* Targets and Subscriptions */
    (AtLoad: "@load"),
    (AtTick: "@tick"),
    (AtE:    "@e"),
    (AtS:    "@s"),
    (AtR:    "@"),
    (AtP:    "@p"),
    (AtN:    "@n"),
    (AtA:    "@a"),

    /* Literals */
    (Identifier:    ""),
    (Null:          "NULL"),
    (True:          "TRUE"),
    (False:         "FALSE"),
    (StringLiteral: "\"", "'"), 
    (IntLiteral:    ""),
    (FloatLiteral:  ""),
    (DoubleLiteral: ""),

    /* Types */
    (Uint8:  "uint8", "unsigned byte"),
    (Uint16: "uint16", "unsigned short"),
    (Int8:   "int8", "byte"),
    (Int16:  "int16", "short"),
    (Int32:  "int32", "int"),
    (Int64:  "int64", "long"),
    (Float:  "float"),
    (Double: "double"),
    (Bool:   "bool"),
    (String: "String"),
    (Array:  "Array"),
    (Object: "Object"),
    (Void:   "void"),

    /* Control */
    (If:       "if"),
    (Else:     "else"),
    (For:      "for"),
    (While:    "while"),
    (Break:    "break"),
    (Continue: "continue"),
    (Return:   "return"),

    /* Keywords */
    (Var:     "var"),
    (Func:    "func"),
    (Include: "include"),
    (Public:  "public"),
    (Private: "private"),

    // Special
    (Backtick: r"\"),
    (Raw:      "RAW"),
    (Name:     "PACK"),
    (Comment:  "#"),
    (Blank:    ""),
    (EOL:      "\n"),
    (EOF:      "")
);


pub struct Token {
    line: usize,
    col: usize,
    lexeme: String,
    kind: TokenType,
}

pub struct Lexer {
    trie: Trie,
}

pub struct TrieNode {
    value: Option<TokenType>,
    children: Map<char, Box<TrieNode>>,
}

pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: new_trie_node!()
        }
    }

    pub fn insert(&mut self, key: &str, val: TokenType) {
        if key.eq("") { // empty key -> do not add to Trie
            return;
        }
        let mut node: &mut TrieNode = &mut self.root;
        for c in key.chars() { // For each character in the string
            node = node.children.entry(c).or_insert(Box::new(new_trie_node!())); // add a node with key char and set current node to this
        }
        node.value = Some(val) // set value of terminal node
    }
}

impl Lexer {
    pub fn lex(&self, string: &String, tokens: &Vec<Token> ) {
        let mut current: usize = 0; // Index of current char
        let mut substr: (usize, usize) = (0, 1); // Current Lexeme range on [Start, End)
        let mut char_stream = string.chars(); // Iterable
        let mut current_node: &TrieNode = &self.trie.root; // Reference to root (gets updated to current prefix node as we traverse)
        let mut line: (usize, usize) = (1, 1); // (line, column)
        let mut tokens: Vec<Token> = vec![];
        let size = string.len(); // Size of char stream, immutable
        while current < size { // Do this until end of string

            // Check if current char is a key in children
            // If it is advance node

            // Else che
            
        }
    }
}