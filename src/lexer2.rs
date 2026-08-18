

macro_rules! generate_match_and_enum {
    ($( ($name:ident, $( $pattern:expr ),+) ),* $(,)?) => {
        pub enum TokenType {
            $( $name ),*
        }

        impl TokenType {
            pub fn get_token(input_string: &str) -> Option<(TokenType, usize) {
                match input_string {
                    ${ 
                        $pattern =>
                    }
                    _ => None
                }
            }
        }

}}

// Create enums takes the form (EnumVariant, Pattern)
generate_match_and_enum!(
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

TokenType;