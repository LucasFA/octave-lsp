// Although languages have very different constructs, their constructs can often be categorized in to similar groups like Declarations, 
// Definitions, Statements, Expressions, Types, and Patterns.In writing your grammar, a good first step is to create just enough 
// structure to include all of these basic groups of symbols.For a language like Go, you might start with something like this:

module.exports = grammar({
    name: 'octave',

    rules: {
        source_file: $ => repeat($._statement),

        _definition: $ => choice(
            $.function_definition,
            $.variable_definition
        ),

        variable_definition: $ => seq(
            field("name", $.identifier),
            "=",
            field("value", $._expression),
        ),

        function_definition: $ => seq(
            "function",
            optional(seq($.return_value, "=")),
            $.identifier,
            field("parameters", optional($.parameter_list)),
            repeat($._statement),
            choice('endfunction', 'end')
        ),

        return_value: $ => choice($.identifier, $.ret_list),
        ret_list: $ => seq('[', optional($.list_of_identifiers), ']'),
        
        parameter_list: $ => seq(
            '(',
            optional($.list_of_identifiers),
            ')'
        ),

        list_of_identifiers: $ => seq(
            $.identifier,
            repeat(seq(',', $.identifier))
        ),

        _type: $ => choice(
            'bool'
            // TODO: other kinds of types
        ),

        block: $ => seq(
            '{',
            repeat($._statement),
            '}'
        ),

        _statement: $ => choice(
            $.if_statement,
            seq($._expression, ';'),
            $._definition
        ),

        // return_statement: $ => seq(
        //     'return',
        //     $._expression,
        //     ';'
        // ),

        if_statement: $ => seq(
            'if', $._expression,

            optional(repeat($.else_if_clause)),
            optional($.else_clause),
            choice('end', 'endif')
        ),

        else_if_clause: $ => seq(
            'elseif',
            $._statement
        ),

        else_clause: $ => seq(
            'else',
            $._expression
        ),

        _expression: $ => choice(
            $.identifier,
            $.number,
            $.variable_definition,
        ),

        identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
        
        number: $ => choice(
            $.integer,
            $.float
        ),
        
        float : $ => token(choice(seq(/\d+/, '.', /\d+/), seq(/\d+/, '.'), seq('.', /\d+/))),
        integer: $ => /\d+/,

        // Keywords
        // function_KW : $ => "function",
        // endfunction_KW : $ => "endfunction",
        // if_KW : $ => "if",
        // elseif_KW : $ => "elseif",
        // else_KW : $ => "else",
        // endif_KW : $ => "endif",
        // switch_KW : $ => "switch",
        // case_KW : $ => "case",
        // otherwise_KW : $ => "otherwise",
        // endswitch_KW : $ => "endswitch",
        // while_KW : $ => "while",
        // endwhile_KW : $ => "endwhile",
        // do_KW : $ => "do",
        // until_KW : $ => "until",
        // for_KW : $ => "for",
        // endfor_KW : $ => "endfor",
        // break_KW : $ => "break",
        // continue_KW : $ => "continue",
        // unwind_protect_KW : $ => "unwind_protect",
        // unwind_protect_cleanup_KW : $ => "unwind_protect_cleanup",
        // end_unwind_protect_KW : $ => "end_unwind_protect",
        // try_KW : $ => "try",
        // catch_KW : $ => "catch",
        // end_try_catch_KW : $ => "end_try_catch",
        // end_KW : $ => "end",

        // Operators
        // Plus: $ => token("+"),
        // Minus: $ => token("-"),
        // Asterisk: $ => token("*"),
        // ElmtMult: $ => token(".*"),
        // Slash: $ => token("/"),
        // ElmtDiv: $ => token("./"),
        // LeftDiv: $ => token("\\"),
        // ElmtLeftDiv : $ => token(".\\"),
        // Caret : $ => token("^"),
        // ElmtPow : $ => token(".^"),
        // Transpose : $ => token("'"),
        // ElmtTranspose : $ => token(".'"),
        // Not: $ => token("!"),
        
        // And : $ => token("&&"),
        // Or : $ => token("||"),
        // EqualsEquals : $ => token("=="),
        // NotEquals : $ => token("!="),
        // LessThan : $ => token("<"),
        // GreaterThan : $ => token(">"),
        // LessThanEquals : $ => token("<="),
        // GreaterThanEquals: $ => token(">="),
        
        // Equals : $ => token("="),
        // Colon : $ => token(":"),

        // LBrace : $ => token("{"),
        // RBrace : $ => token("}"),
        // LBracket : $ => token("["),
        // RBracket : $ => token("]"),
        // LParen : $ => token("("),
        // RParen : $ => token(")"),
    }
});
