// Although languages have very different constructs, their constructs can often be categorized in to similar groups like Declarations, 
// Definitions, Statements, Expressions, Types, and Patterns.In writing your grammar, a good first step is to create just enough 
// structure to include all of these basic groups of symbols.For a language like Go, you might start with something like this:

module.exports = grammar({
    name: 'octave',

    rules: {
        source_file: $ => repeat($._definition),

        _definition: $ => choice(
            $.function_definition,
            $.variable_definition
            // TODO: other kinds of definitions
        ),

        variable_definition: $ => seq(
            $.identifier,
            optional(seq($.identifier, '=')),
            $._expression,
            optional(';')
        ),

        function_definition: $ => seq(
            'function',
            $.identifier,
            optional($.parameter_list),
            repeat($._statement),
            choice('endfunction', 'end')
        ),

        parameter_list: $ => seq(
            '(',
            optional($.parameters),
            ')'
        ),

        parameters: $ => seq(
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
        ),

        // return_statement: $ => seq(
        //     'return',
        //     $._expression,
        //     ';'
        // ),

        if_statement: $ => seq(
            'if',
            $._expression,
            $.block,
            optional($.else_clause),
            choice('end', 'endif')
        ),

        else_clause: $ => seq(
            'else',
            choice(
                $.if_statement,
                $._expression
            )
        ),

        _expression: $ => choice(
            $.identifier,
            $.number
        ),

        identifier: $ => /[a-z]+/,

        number: $ => /\d+/
    }
});
