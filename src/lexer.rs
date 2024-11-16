#[allow(unused_imports)]
use santiago::{grammar::*, lexer::*};

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
        "DEFAULT" | "KEYWORD_SIGNED" = string "signed";
        "DEFAULT" | "KEYWORD_UNSIGNED" = string "unsigned";
        "DEFAULT" | "KEYWORD_USE" = string "use";
        "DEFAULT" | "KEYWORD_AS" = string "as";

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

        // Whitespace (skip)
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
        "DEFAULT" | "COMMENT" = pattern r"//[^\n]*|/\*[^*]*\*+(?:[^/*][^*]*\*+)*/" => |lexer| lexer.skip();
    )
}

#[allow(dead_code)]
pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
            "ID" => lexemes "IDENT";
            "INT" => lexemes "INT";
            "FLOAT" => lexemes "FLOAT";
            "STRING" => lexemes "STRING";

            // Keywords & Types
            "type" => lexemes "TYPE_INT";
            "type" => lexemes "TYPE_FLOAT";
            "type" => lexemes "TYPE_CHAR";
            "type" => lexemes "TYPE_STRING";
            "type" => lexemes "TYPE_BYTE";
            "type" => lexemes "TYPE_BOOL";
            "type" => lexemes "TYPE_VOID";
            "type" => lexemes "TYPE_NONE";
            "type" => lexemes "TYPE_STRUCT_ENUM";
            "type" => lexemes "TYPE_STRUCT_COMPACT";
            "type" => lexemes "TYPE_STRUCT_LOOSE";
            "type" => lexemes "TYPE_STRUCT_UNION";
            "type" => lexemes "TYPE_VECTOR";
            "type" => lexemes "TYPE_MATRIX";
            "type" => lexemes "TYPE_ERROR";
            "type" => lexemes "TYPE_COMPLEX";
            "type" => lexemes "TYPE_USIZE";
            "type" => lexemes "TYPE_ISIZE";
            "func" => lexemes "KEYWORD_FUNC";
            "private" => lexemes "KEYWORD_PRIVATE";
            "public" => lexemes "KEYWORD_PUBLIC";
            "struct" => lexemes "KEYWORD_STRUCT";
            "switch" => lexemes "KEYWORD_SWITCH";
            "return" => lexemes "KEYWORD_RETURN";
            "if" => lexemes "KEYWORD_IF";
            "else" => lexemes "KEYWORD_ELSE";
            "finally" => lexemes "KEYWORD_FINALLY";
            "impl" => lexemes "KEYWORD_IMPL";
            "const" => lexemes "KEYWORD_CONST";
            "mut" => lexemes "KEYWORD_MUT";
            "defer" => lexemes "KEYWORD_DEFER";
            "break" => lexemes "KEYWORD_BREAK";
            "continue" => lexemes "KEYWORD_CONTINUE";
            "for" => lexemes "KEYWORD_FOR";
            "while" => lexemes "KEYWORD_WHILE";
            "signed" => lexemes "KEYWORD_SIGNED";
            "unsigned" => lexemes "KEYWORD_UNSIGNED";
            "use" => lexemes "KEYWORD_USE";
            "as" => lexemes "KEYWORD_AS";

            // Assignment operators
            ":" => lexemes "COLON";
            "=" => lexemes "ASSIGNMENT";
            ":=" => lexemes "ASSIGNMENT_IMPLICIT";
            "+=" => lexemes "ASSIGNMENT_PLUS";
            "-=" => lexemes "ASSIGNMENT_MINUS";
            "*=" => lexemes "ASSIGNMENT_STAR";
            "/=" => lexemes "ASSIGNMENT_DIVIDE";
            "%=" => lexemes "ASSIGNMENT_MOD";
            "|=" => lexemes "ASSIGNMENT_BITWISE_OR";
            "&=" => lexemes "ASSIGNMENT_BITWISE_AND";
            "~=" => lexemes "ASSIGNMENT_BITWISE_XOR";
            "&~=" => lexemes "ASSIGNMENT_AND_NOT";
            "<<=" => lexemes "ASSIGNMENT_SHIFT_LEFT";
            ">>=" => lexemes "ASSIGNMENT_SHIFT_RIGHT";
            "||=" => lexemes "ASSIGNMENT_CONDITIONAL_OR";
            "&&=" => lexemes "ASSIGNMENT_CONDITIONAL_AND";

            // Separator tokens
            "(" => lexemes "OPEN_PAREN";
            ")" => lexemes "CLOSE_PAREN";
            "[" => lexemes "OPEN_BRACKET";
            "]" => lexemes "CLOSE_BRACKET";
            "{" => lexemes "OPEN_CURLY";
            "}" => lexemes "CLOSE_CURLY";
            ";" => lexemes "SEMICOLON";
            "," => lexemes "COMMA";

            // Range tokens
            "..<" => lexemes "TO_LESS";
            "..=" => lexemes "TO_EQUAL";

            // Comparison tokens
            "==" => lexemes "EQUALS";
            "!=" => lexemes "NOT_EQUALS";
            ">=" => lexemes "GREATER_EQUALS";
            "<=" => lexemes "LESSER_EQUALS";
            "<" => lexemes "LEFT_ARROW";
            ">" => lexemes "RIGHT_ARROW";

            // Logical operators
            "&&" => lexemes "AND";
            "||" => lexemes "OR";
            "!" => lexemes "BANG";
            "|" => lexemes "PIPE";
            "~" => lexemes "TILDE";
            "&" => lexemes "BITWISE_AND";

            // Mathematical tokens
            "+" => lexemes "PLUS";
            "-" => lexemes "MINUS";
            "*" => lexemes "STAR";
            "/" => lexemes "SLASH";
            "++" => lexemes "PLUS_PLUS";
            "--" => lexemes "MINUS_MINUS";
            "**" => lexemes "STAR_STAR";

            // Arrow tokens
            "=>" => lexemes "FAT_ARROW";
            "->" => lexemes "THIN_ARROW";
            "|>" => lexemes "PIPE_LINE";

            // Misc
            "@" => lexemes "AT";
            "#" => lexemes "POUND";
            "$" => lexemes "DOLLAR";
            "^" => lexemes "CARROT";
            "?" => lexemes "QUESTION";
            "." => lexemes "FULL_STOP";
            "`" => lexemes "BACK_TICK";
            "\\" => lexemes "BACK_SLASH";

            // Associativity
            Associativity::Left => rules "+" "-";
            Associativity::Left => rules "*" "/";
            Associativity::Right => rules "=";
            Associativity::None => rules "==" "!=";
            Associativity::Right => rules "&&" "||";
        )
}
