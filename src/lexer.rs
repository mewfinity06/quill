#[allow(unused_imports)]
use santiago::lexer::*;

#[allow(dead_code)]
pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Numeric patterns
        "DEFAULT" | "INT" = pattern r"\d+";
        "DEFAULT" | "FLOAT" = pattern r"[+-]?(\d*\.\d+|\d+\.\d*)([eE][+-]?\d+)?";

        // Keywords & Types
        "DEFAULT" | "TYPE_INT" = string "Int";
        "DEFAULT" | "TYPE_FLOAT" = string "Float";
        "DEFAULT" | "TYPE_CHAR" = string "Char";
        "DEFAULT" | "TYPE_STRING" = string "String";
        "DEFAULT" | "TYPE_BYTE" = string "Byte";
        "DEFAULT" | "TYPE_BOOL" = string "Bool";
        "DEFAULT" | "TYPE_VOID" = string "Void";
        "DEFAULT" | "TYPE_NONE" = string "None";
        "DEFAULT" | "TYPE_STRUCT_ENUM" = string "Enum";
        "DEFAULT" | "TYPE_STRUCT_COMPACT" = string "Compact";
        "DEFAULT" | "TYPE_STRUCT_LOOSE" = string "Loose";
        "DEFAULT" | "TYPE_STRUCT_UNION" = string "Union";
        "DEFAULT" | "TYPE_VECTOR" = string "Vector";
        "DEFAULT" | "TYPE_MATRIX" = string "Matrix";
        "DEFAULT" | "TYPE_ERROR" = string "Error";
        "DEFAULT" | "TYPE_COMPLEX" = string "Complex";
        "DEFAULT" | "TYPE_USIZE" = string "Usize";
        "DEFAULT" | "TYPE_ISIZE" = string "Isize";

        "DEFAULT" | "KEYWORD_FUNC" = string "func";
        "DEFAULT" | "KEYWORD_PRIVATE" = string "private";
        "DEFAULT" | "KEYWORD_PUBLIC" = string "public";
        "DEFAULT" | "KEYWORD_STATIC" = string "static";
        "DEFAULT" | "KEYWORD_STRUCT" = string "struct";
        "DEFAULT" | "KEYWORD_SWITCH" = string "switch";
        "DEFAULT" | "KEYWORD_RETURN" = string "return";
        "DEFAULT" | "KEYWORD_IF" = string "if";
        "DEFAULT" | "KEYWORD_ELSE" = string "else";
        "DEFAULT" | "KEYWORD_FINALLY" = string "finally";
        "DEFAULT" | "KEYWORD_IMPL" = string "impl";
        "DEFAULT" | "KEYWORD_CONST" = string "const";
        "DEFAULT" | "KEYWORD_MUT" = string "mut";
        "DEFAULT" | "KEYWORD_DEFER" = string "defer";
        "DEFAULT" | "KEYWORD_BREAK" = string "break";
        "DEFAULT" | "KEYWORD_CONTINUE" = string "continue";
        "DEFAULT" | "KEYWORD_FOR" = string "for";
        "DEFAULT" | "KEYWORD_WHILE" = string "while";
        "DEFAULT" | "KEYWORD_LOOP" = string "loop";
        "DEFAULT" | "KEYWORD_SIGNED" = string "signed";
        "DEFAULT" | "KEYWORD_UNSIGNED" = string "unsigned";
        "DEFAULT" | "KEYWORD_USE" = string "use";
        "DEFAULT" | "KEYWORD_AS" = string "as";
        "DEFAULT" | "KEYWORD_IN" = string "in";
        "DEFAULT" | "KEYWORD_TYPESET" = string "typeset";
        "DEFAULT" | "KEYWORD_MODULE" = string "module";
 
        // Identifier pattern
        "DEFAULT" | "IDENT" = pattern r"[a-zA-Z_][a-zA-Z0-9_]*";

        // String patterns
        "DEFAULT" | "STRING" = pattern r#""([^"\\\n]|\\.)*"|'([^'\\\n]|\\.)*'"#;

        // Assignment operators
        "DEFAULT" | "COLON" = string ":";
        "DEFAULT" | "ASSIGNMENT" = string "=";
        "DEFAULT" | "ASSIGNMENT_IMPLICIT" = string ":=";
        "DEFAULT" | "ASSIGNMENT_PLUS" = string "+=";
        "DEFAULT" | "ASSIGNMENT_MINUS" = string "-=";
        "DEFAULT" | "ASSIGNMENT_STAR" = string "*=";
        "DEFAULT" | "ASSIGNMENT_DIVIDE" = string "/=";
        "DEFAULT" | "ASSIGNMENT_MOD" = string "%=";
        "DEFAULT" | "ASSIGNMENT_BITWISE_OR" = string "|=";
        "DEFAULT" | "ASSIGNMENT_BITWISE_AND" = string "&=";
        "DEFAULT" | "ASSIGNMENT_BITWISE_XOR" = string "~=";
        "DEFAULT" | "ASSIGNMENT_AND_NOT" = string "&~=";
        "DEFAULT" | "ASSIGNMENT_SHIFT_LEFT" = string "<<=";
        "DEFAULT" | "ASSIGNMENT_SHIFT_RIGHT" = string ">>=";
        "DEFAULT" | "ASSIGNMENT_CONDITIONAL_OR" = string "||=";
        "DEFAULT" | "ASSIGNMENT_CONDITIONAL_AND" = string "&&=";

        // Separator tokens
        "DEFAULT" | "OPEN_PAREN" = string "(";
        "DEFAULT" | "CLOSE_PAREN" = string ")";
        "DEFAULT" | "OPEN_BRACKET" = string "[";
        "DEFAULT" | "CLOSE_BRACKET" = string "]";
        "DEFAULT" | "OPEN_CURLY" = string "{";
        "DEFAULT" | "CLOSE_CURLY" = string "}";
        "DEFAULT" | "SEMICOLON" = string ";";
        "DEFAULT" | "COMMA" = string ",";

        // Range tokens
        "DEFAULT" | "TO_LESS" = string "..<";
        "DEFAULT" | "TO_EQUAL" = string "..=";

        // Comparison tokens
        "DEFAULT" | "EQUALS" = string "==";
        "DEFAULT" | "NOT_EQUALS" = string "!=";
        "DEFAULT" | "GREATER_EQUALS" = string ">=";
        "DEFAULT" | "LESSER_EQUALS" = string "<=";
        "DEFAULT" | "LEFT_ARROW" = string "<";
        "DEFAULT" | "RIGHT_ARROW" = string ">";

        // Logical operators
        "DEFAULT" | "AND" = string "&&";
        "DEFAULT" | "OR" = string "||";
        "DEFAULT" | "BANG" = string "!";
        "DEFAULT" | "PIPE" = string "|";
        "DEFAULT" | "TILDE" = string "~";
        "DEFAULT" | "BITWISE_AND" = string "&";

        // Mathematical tokens
        "DEFAULT" | "PLUS" = string "+";
        "DEFAULT" | "MINUS" = string "-";
        "DEFAULT" | "STAR" = string "*";
        "DEFAULT" | "SLASH" = string "/";
        "DEFAULT" | "MOD" = string "%";
        "DEFAULT" | "PLUS_PLUS" = string "++";
        "DEFAULT" | "MINUS_MINUS" = string "--";
        "DEFAULT" | "STAR_STAR" = string "**";

        // Arrow tokens
        "DEFAULT" | "FAT_ARROW" = string "=>";
        "DEFAULT" | "THIN_ARROW" = string "->";
        "DEFAULT" | "PIPE_LINE" = string "|>";

        // Misc
        "DEFAULT" | "AT" = string "@";
        "DEFAULT" | "POUND" = string "#";
        "DEFAULT" | "DOLLAR" = string "$";
        "DEFAULT" | "CARROT" = string "^";
        "DEFAULT" | "QUESTION" = string "?";
        "DEFAULT" | "FULL_STOP" = string ".";
        "DEFAULT" | "BACK_TICK" = string "`";
        "DEFAULT" | "BACK_SLASH" = string "\\";
        "DEFAULT" | "DOUBLE_COLON" = string "::";

        // Whitespace (skip)
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
        "DEFAULT" | "COMMENT" = pattern r"//[^\n]*|/\*[^*]*\*+(?:[^/*][^*]*\*+)*/" => |lexer| lexer.skip();
    )
}
