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
            $.equals,
            $._expression,
            optional(';')
        ),
        
        equals: $ => token('='),

        function_definition: $ => seq(
            'function',
            optional(seq($.return_value, $.equals)),
            $.identifier,
            optional($.parameter_list),
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

        identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
        
        number: $ => choice(
            $.integer,
            $.float
        ),

        float : $ => choice(seq(/\d+/, '.', /\d+/), seq(/\d+/, '.'), seq('.', /\d+/)),
        integer: $ => /\d+/
    }
});
