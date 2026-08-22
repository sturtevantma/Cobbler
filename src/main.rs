mod lexer;
mod lexer2;

fn main() {
    let test = "var lmao: Array[unsigned short] = func";
    let mut lex = lexer::Lexer::new(test);
    lex.scan_tokens();
    lex.print_tokens();
}
