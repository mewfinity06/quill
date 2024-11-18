use santiago::grammar::*;

#[allow(dead_code)]
pub fn grammar() -> Grammar<()> {
    santiago::grammar!(

        "program" => rules "expr" "EOF";

        "expr" => empty;
        "expr" => rules "(" "stmt" ")";
        "expr" => rules "[" "stmt" "]";
        "expr" => rules "{" "stmt" "}";
        "expr" => rules "stmt";
        "expr" => rules "stmt" "expr";

        "stmt" => empty;
        "stmt" => rules "stmt" ";";
        "stmt" => rules "value";
        "stmt" => rules "use_stmt";
        "stmt" => rules "module_stmt";
        "stmt" => rules "variable_stmt";
        "stmt" => rules "variable_reassignment";
        "stmt" => rules "binary_stmt";
        "stmt" => rules "unary_stmt";
        "stmt" => rules "struct_stmt";
        "stmt" => rules "impl_stmt";
        "stmt" => rules "function_stmt";
        "stmt" => rules "anon_function_stmt";
        "stmt" => rules "function_call";

        "use_stmt" => rules "use" "ID" ";";
        "use_stmt" => rules "use" "ID" "as" "ID" ";";
        "use_stmt" => rules "use" "ID_double_colon_notation" ";";
        "use_stmt" => rules "use" "ID_double_colon_notation" "as" "ID" ";";

        "module_stmt" => rules "module" "ID" ";";
        "module_stmt" => rules "module" "ID_dot_notation" ";";

        "variable_stmt" => rules "mut"    "ID" ":=" "stmt" ";";
        "variable_stmt" => rules "const"  "ID" ":=" "stmt" ";";
        "variable_stmt" => rules "static" "ID" ":=" "stmt" ";";
        "variable_stmt" => rules "mut"    "ID" ":"  "type" "=" "stmt" ";";
        "variable_stmt" => rules "const"  "ID" ":"  "type" "=" "stmt" ";";
        "variable_stmt" => rules "static" "ID" ":"  "type" "=" "stmt" ";";
        "variable_stmt" => rules "mut"    "ID" ":=" "ID" ";";
        "variable_stmt" => rules "const"  "ID" ":=" "ID" ";";
        "variable_stmt" => rules "static" "ID" ":=" "ID" ";";
        "variable_stmt" => rules "mut"    "ID" ":"  "type" "=" "ID" ";";
        "variable_stmt" => rules "const"  "ID" ":"  "type" "=" "ID" ";";
        "variable_stmt" => rules "static" "ID" ":"  "type" "=" "ID" ";";

        "variable_reassignment" => rules "ID" "reassignment" "expr";

        "binary_stmt" => rules "ID"    "binary" "ID";
        "binary_stmt" => rules "ID"    "binary" "value";
        "binary_stmt" => rules "value" "binary" "ID";
        "binary_stmt" => rules "value" "binary" "value";

        "unary_stmt" => rules "ID"    "unary" "ID";
        "unary_stmt" => rules "ID"    "unary" "value";
        "unary_stmt" => rules "value" "unary" "ID";
        "unary_stmt" => rules "value" "unary" "value";

        "struct_stmt" => rules "struct" "ID" ":" "struct_type" "{" "struct_body" "}";

        "struct_body" => empty;
        "struct_body" => rules "struct_body" "struct_body";
        "struct_body" => rules "@" "ID" "=" "value" ",";
        "struct_body" => rules "ID" ":" "type" ",";
        "struct_body" => rules "ID" ",";

        "impl_stmt" => rules "ID" ":" "impl" "{" "impl_body" "}";

        "impl_body" => empty;
        "impl_body" => rules "stmt";
        "impl_body" => rules "stmt" "stmt";

        "function_stmt" => rules "privacy" "func" "ID" "(" "func_signature" ")" ":" "type" "{" "func_body" "}";
        "function_stmt" => rules           "func" "ID" "(" "func_signature" ")" ":" "type" "{" "func_body" "}";
        "function_stmt" => rules "privacy" "func" "ID"                          ":" "type" "{" "func_body" "}";
        "function_stmt" => rules           "func" "ID"                          ":" "type" "{" "func_body" "}";

        "anon_function_stmt" => rules "func" "(" "func_signature" ")" ":" "type" "=" "{" "func_body" "}";
        "anon_function_stmt" => rules "func"                          ":" "type" "=" "{" "func_body" "}";

        "func_signature" => empty;
        "func_signature" => rules "ID" ":" "type";
        "func_signature" => rules "ID" ":" "type" "," "func_signature";

        "func_body" => empty;
        "func_body" => rules "stmt";
        "func_body" => rules "stmt" "func_body";

        "function_call" => rules "ID" "(" "func_call_body" ")";

        "func_call_body" => empty;
        "func_call_body" => rules "value";
        "func_call_body" => rules "ID";
        "func_call_body" => rules "ID" ":" "value";
        "func_call_body" => rules "ID" ":" "ID";
        "func_call_body" => rules "value" "," "func_call_body";
        "func_call_body" => rules "ID" "," "func_call_body";
        "func_call_body" => rules "ID" ":" "value" "," "func_call_body";
        "func_call_body" => rules "ID" ":" "ID" "," "func_call_body";

        "ID_double_colon_notation" => rules "ID";
        "ID_double_colon_notation" => rules "ID" "::" "ID_double_colon_notation";

        "ID_dot_notation" => rules "ID";
        "ID_dot_notation" => rules "ID" "." "ID_dot_notation";

        "EOF" => empty;

        // LEXEMES
        "ID" => lexemes "IDENT";
        "INT" => lexemes "INT";
        "FLOAT" => lexemes "FLOAT";
        "STRING" => lexemes "STRING";

        "value" => rules "INT";
        "value" => rules "FLOAT";
        "value" => rules "STRING";

        // Keywords & Types
        "type" => rules   "^" "type";
        "type" => lexemes "TYPE_INT";
        "type" => lexemes "TYPE_FLOAT";
        "type" => lexemes "TYPE_CHAR";
        "type" => lexemes "TYPE_STRING";
        "type" => lexemes "TYPE_BYTE";
        "type" => lexemes "TYPE_BOOL";
        "type" => lexemes "TYPE_VOID";
        "type" => lexemes "TYPE_NONE";
        "type" => lexemes "TYPE_VECTOR";
        "type" => lexemes "TYPE_MATRIX";
        "type" => lexemes "TYPE_ERROR";
        "type" => lexemes "TYPE_COMPLEX";
        "type" => lexemes "TYPE_USIZE";
        "type" => lexemes "TYPE_ISIZE";
        "struct_type" => lexemes "TYPE_STRUCT_ENUM";
        "struct_type" => lexemes "TYPE_STRUCT_COMPACT";
        "struct_type" => lexemes "TYPE_STRUCT_LOOSE";
        "struct_type" => lexemes "TYPE_STRUCT_UNION";
            
        "func" => lexemes "KEYWORD_FUNC";
        "privacy" => lexemes "KEYWORD_PRIVATE";
        "privacy" => lexemes "KEYWORD_PUBLIC";
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
        "loop" => lexemes "KEYWORD_LOOP";
        "signed" => lexemes "KEYWORD_SIGNED";
        "unsigned" => lexemes "KEYWORD_UNSIGNED";
        "use" => lexemes "KEYWORD_USE";
        "as" => lexemes "KEYWORD_AS";
        "static" => lexemes "KEYWORD_STATIC";
        "in" => lexemes "KEYWORD_IN";
        "typeset" => lexemes "KEYWORD_TYPESET";
        "module" => lexemes "KEYWORD_MODULE";

        // Assignment operators
        ":" => lexemes "COLON";
        "=" => lexemes "ASSIGNMENT";
        ":=" => lexemes "ASSIGNMENT_IMPLICIT";
        "reassignment" => lexemes "ASSIGNMENT_PLUS";
        "reassignment" => lexemes "ASSIGNMENT_MINUS";
        "reassignment" => lexemes "ASSIGNMENT_STAR";
        "reassignment" => lexemes "ASSIGNMENT_DIVIDE";
        "reassignment" => lexemes "ASSIGNMENT_MOD";
        "reassignment" => lexemes "ASSIGNMENT_BITWISE_OR";
        "reassignment" => lexemes "ASSIGNMENT_BITWISE_AND";
        "reassignment" => lexemes "ASSIGNMENT_BITWISE_XOR";
        "reassignment" => lexemes "ASSIGNMENT_AND_NOT";
        "reassignment" => lexemes "ASSIGNMENT_SHIFT_LEFT";
        "reassignment" => lexemes "ASSIGNMENT_SHIFT_RIGHT";
        "reassignment" => lexemes "ASSIGNMENT_CONDITIONAL_OR";
        "reassignment" => lexemes "ASSIGNMENT_CONDITIONAL_AND";

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
        "iter" => lexemes "TO_LESS";
        "iter" => lexemes "TO_EQUAL";

        // Comparison tokens
        "comparison" => lexemes "EQUALS";
        "comparison" => lexemes "NOT_EQUALS";
        "comparison" => lexemes "GREATER_EQUALS";
        "comparison" => lexemes "LESSER_EQUALS";
        "comparison" => lexemes "LEFT_ARROW";
        "comparison" => lexemes "RIGHT_ARROW";

        // Logical operators
        "binary" => lexemes "AND";
        "binary" => lexemes "OR";
        "binary" => lexemes "BANG";
        "binary" => lexemes "PIPE";
        "binary" => lexemes "TILDE";
        "binary" => lexemes "BITWISE_AND";

        // Mathematical tokens
        "binary" => lexemes "PLUS";
        "binary" => lexemes "MINUS";
        "binary" => lexemes "STAR";
        "binary" => lexemes "SLASH";
        "binary" => lexemes "MOD";
        "binary" => lexemes "STAR_STAR";
        "unary" => lexemes "PLUS_PLUS";
        "unary" => lexemes "MINUS_MINUS";

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
        "::" => lexemes "DOUBLE_COLON";

        "EOF" => empty;
    )
}
