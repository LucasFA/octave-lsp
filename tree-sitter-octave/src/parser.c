#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 45
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 31
#define ALIAS_COUNT 0
#define TOKEN_COUNT 17
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_EQ = 1,
  anon_sym_SEMI = 2,
  anon_sym_function = 3,
  anon_sym_endfunction = 4,
  anon_sym_end = 5,
  anon_sym_LPAREN = 6,
  anon_sym_RPAREN = 7,
  anon_sym_COMMA = 8,
  anon_sym_bool = 9,
  anon_sym_LBRACE = 10,
  anon_sym_RBRACE = 11,
  anon_sym_if = 12,
  anon_sym_endif = 13,
  anon_sym_else = 14,
  sym_identifier = 15,
  sym_number = 16,
  sym_source_file = 17,
  sym__definition = 18,
  sym_variable_definition = 19,
  sym_function_definition = 20,
  sym_parameter_list = 21,
  sym_parameters = 22,
  sym_block = 23,
  sym__statement = 24,
  sym_if_statement = 25,
  sym_else_clause = 26,
  sym__expression = 27,
  aux_sym_source_file_repeat1 = 28,
  aux_sym_function_definition_repeat1 = 29,
  aux_sym_parameters_repeat1 = 30,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_EQ] = "=",
  [anon_sym_SEMI] = ";",
  [anon_sym_function] = "function",
  [anon_sym_endfunction] = "endfunction",
  [anon_sym_end] = "end",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_COMMA] = ",",
  [anon_sym_bool] = "bool",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_if] = "if",
  [anon_sym_endif] = "endif",
  [anon_sym_else] = "else",
  [sym_identifier] = "identifier",
  [sym_number] = "number",
  [sym_source_file] = "source_file",
  [sym__definition] = "_definition",
  [sym_variable_definition] = "variable_definition",
  [sym_function_definition] = "function_definition",
  [sym_parameter_list] = "parameter_list",
  [sym_parameters] = "parameters",
  [sym_block] = "block",
  [sym__statement] = "_statement",
  [sym_if_statement] = "if_statement",
  [sym_else_clause] = "else_clause",
  [sym__expression] = "_expression",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_function_definition_repeat1] = "function_definition_repeat1",
  [aux_sym_parameters_repeat1] = "parameters_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_function] = anon_sym_function,
  [anon_sym_endfunction] = anon_sym_endfunction,
  [anon_sym_end] = anon_sym_end,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_bool] = anon_sym_bool,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_endif] = anon_sym_endif,
  [anon_sym_else] = anon_sym_else,
  [sym_identifier] = sym_identifier,
  [sym_number] = sym_number,
  [sym_source_file] = sym_source_file,
  [sym__definition] = sym__definition,
  [sym_variable_definition] = sym_variable_definition,
  [sym_function_definition] = sym_function_definition,
  [sym_parameter_list] = sym_parameter_list,
  [sym_parameters] = sym_parameters,
  [sym_block] = sym_block,
  [sym__statement] = sym__statement,
  [sym_if_statement] = sym_if_statement,
  [sym_else_clause] = sym_else_clause,
  [sym__expression] = sym__expression,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_function_definition_repeat1] = aux_sym_function_definition_repeat1,
  [aux_sym_parameters_repeat1] = aux_sym_parameters_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_endfunction] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_end] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bool] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_endif] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__definition] = {
    .visible = false,
    .named = true,
  },
  [sym_variable_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_function_definition] = {
    .visible = true,
    .named = true,
  },
  [sym_parameter_list] = {
    .visible = true,
    .named = true,
  },
  [sym_parameters] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym__statement] = {
    .visible = false,
    .named = true,
  },
  [sym_if_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_else_clause] = {
    .visible = true,
    .named = true,
  },
  [sym__expression] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_function_definition_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_parameters_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 12,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 17,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 14,
  [37] = 13,
  [38] = 33,
  [39] = 34,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(33);
      if (lookahead == '(') ADVANCE(42);
      if (lookahead == ')') ADVANCE(43);
      if (lookahead == ',') ADVANCE(44);
      if (lookahead == ';') ADVANCE(35);
      if (lookahead == '=') ADVANCE(34);
      if (lookahead == 'b') ADVANCE(24);
      if (lookahead == 'e') ADVANCE(15);
      if (lookahead == 'f') ADVANCE(30);
      if (lookahead == 'i') ADVANCE(10);
      if (lookahead == '{') ADVANCE(46);
      if (lookahead == '}') ADVANCE(47);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      END_STATE();
    case 1:
      if (lookahead == '(') ADVANCE(42);
      if (lookahead == 'e') ADVANCE(21);
      if (lookahead == 'i') ADVANCE(10);
      if (lookahead == '}') ADVANCE(47);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == ')') ADVANCE(43);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 3:
      if (lookahead == 'c') ADVANCE(28);
      END_STATE();
    case 4:
      if (lookahead == 'c') ADVANCE(29);
      END_STATE();
    case 5:
      if (lookahead == 'd') ADVANCE(40);
      END_STATE();
    case 6:
      if (lookahead == 'd') ADVANCE(39);
      END_STATE();
    case 7:
      if (lookahead == 'd') ADVANCE(41);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(51);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(16);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(9)
      END_STATE();
    case 10:
      if (lookahead == 'f') ADVANCE(48);
      END_STATE();
    case 11:
      if (lookahead == 'f') ADVANCE(50);
      END_STATE();
    case 12:
      if (lookahead == 'i') ADVANCE(53);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(12)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 13:
      if (lookahead == 'i') ADVANCE(25);
      END_STATE();
    case 14:
      if (lookahead == 'i') ADVANCE(26);
      END_STATE();
    case 15:
      if (lookahead == 'l') ADVANCE(27);
      if (lookahead == 'n') ADVANCE(5);
      END_STATE();
    case 16:
      if (lookahead == 'l') ADVANCE(27);
      if (lookahead == 'n') ADVANCE(7);
      END_STATE();
    case 17:
      if (lookahead == 'l') ADVANCE(45);
      END_STATE();
    case 18:
      if (lookahead == 'n') ADVANCE(3);
      END_STATE();
    case 19:
      if (lookahead == 'n') ADVANCE(36);
      END_STATE();
    case 20:
      if (lookahead == 'n') ADVANCE(38);
      END_STATE();
    case 21:
      if (lookahead == 'n') ADVANCE(6);
      END_STATE();
    case 22:
      if (lookahead == 'n') ADVANCE(4);
      END_STATE();
    case 23:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 24:
      if (lookahead == 'o') ADVANCE(23);
      END_STATE();
    case 25:
      if (lookahead == 'o') ADVANCE(19);
      END_STATE();
    case 26:
      if (lookahead == 'o') ADVANCE(20);
      END_STATE();
    case 27:
      if (lookahead == 's') ADVANCE(8);
      END_STATE();
    case 28:
      if (lookahead == 't') ADVANCE(13);
      END_STATE();
    case 29:
      if (lookahead == 't') ADVANCE(14);
      END_STATE();
    case 30:
      if (lookahead == 'u') ADVANCE(18);
      END_STATE();
    case 31:
      if (lookahead == 'u') ADVANCE(22);
      END_STATE();
    case 32:
      if (eof) ADVANCE(33);
      if (lookahead == ';') ADVANCE(35);
      if (lookahead == 'f') ADVANCE(59);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(32)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_function);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_endfunction);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_end);
      if (lookahead == 'f') ADVANCE(31);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_end);
      if (lookahead == 'f') ADVANCE(31);
      if (lookahead == 'i') ADVANCE(11);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_end);
      if (lookahead == 'i') ADVANCE(11);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_bool);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_if);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_endif);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'c') ADVANCE(58);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(49);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'i') ADVANCE(57);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(52);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'n') ADVANCE(37);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'o') ADVANCE(56);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 't') ADVANCE(54);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'u') ADVANCE(55);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_identifier);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(60);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(61);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 32},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 32},
  [5] = {.lex_state = 32},
  [6] = {.lex_state = 1},
  [7] = {.lex_state = 1},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 12},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 9},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 9},
  [16] = {.lex_state = 32},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 2},
  [20] = {.lex_state = 9},
  [21] = {.lex_state = 32},
  [22] = {.lex_state = 32},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 1},
  [26] = {.lex_state = 32},
  [27] = {.lex_state = 1},
  [28] = {.lex_state = 32},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 9},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 9},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 9},
  [36] = {.lex_state = 9},
  [37] = {.lex_state = 9},
  [38] = {.lex_state = 9},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 2},
  [41] = {.lex_state = 2},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [anon_sym_endfunction] = ACTIONS(1),
    [anon_sym_end] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_bool] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_endif] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(42),
    [sym__definition] = STATE(4),
    [sym_variable_definition] = STATE(4),
    [sym_function_definition] = STATE(4),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_function] = ACTIONS(5),
    [sym_identifier] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(9), 1,
      anon_sym_endfunction,
    ACTIONS(11), 1,
      anon_sym_end,
    ACTIONS(13), 1,
      anon_sym_LPAREN,
    ACTIONS(15), 1,
      anon_sym_if,
    STATE(6), 1,
      sym_parameter_list,
    STATE(7), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [21] = 4,
    ACTIONS(19), 1,
      anon_sym_end,
    ACTIONS(21), 1,
      anon_sym_if,
    ACTIONS(17), 2,
      anon_sym_endfunction,
      anon_sym_RBRACE,
    STATE(3), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [37] = 4,
    ACTIONS(5), 1,
      anon_sym_function,
    ACTIONS(7), 1,
      sym_identifier,
    ACTIONS(24), 1,
      ts_builtin_sym_end,
    STATE(5), 4,
      sym__definition,
      sym_variable_definition,
      sym_function_definition,
      aux_sym_source_file_repeat1,
  [53] = 4,
    ACTIONS(26), 1,
      ts_builtin_sym_end,
    ACTIONS(28), 1,
      anon_sym_function,
    ACTIONS(31), 1,
      sym_identifier,
    STATE(5), 4,
      sym__definition,
      sym_variable_definition,
      sym_function_definition,
      aux_sym_source_file_repeat1,
  [69] = 4,
    ACTIONS(15), 1,
      anon_sym_if,
    ACTIONS(34), 1,
      anon_sym_endfunction,
    ACTIONS(36), 1,
      anon_sym_end,
    STATE(8), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [84] = 4,
    ACTIONS(15), 1,
      anon_sym_if,
    ACTIONS(34), 1,
      anon_sym_endfunction,
    ACTIONS(36), 1,
      anon_sym_end,
    STATE(3), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [99] = 4,
    ACTIONS(15), 1,
      anon_sym_if,
    ACTIONS(38), 1,
      anon_sym_endfunction,
    ACTIONS(40), 1,
      anon_sym_end,
    STATE(3), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [114] = 4,
    ACTIONS(42), 1,
      anon_sym_if,
    ACTIONS(44), 1,
      sym_identifier,
    ACTIONS(46), 1,
      sym_number,
    STATE(35), 2,
      sym_if_statement,
      sym__expression,
  [128] = 3,
    ACTIONS(15), 1,
      anon_sym_if,
    ACTIONS(48), 1,
      anon_sym_RBRACE,
    STATE(3), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [140] = 3,
    ACTIONS(15), 1,
      anon_sym_if,
    ACTIONS(50), 1,
      anon_sym_RBRACE,
    STATE(10), 3,
      sym__statement,
      sym_if_statement,
      aux_sym_function_definition_repeat1,
  [152] = 4,
    ACTIONS(52), 1,
      anon_sym_end,
    ACTIONS(54), 1,
      anon_sym_endif,
    ACTIONS(56), 1,
      anon_sym_else,
    STATE(38), 1,
      sym_else_clause,
  [165] = 2,
    ACTIONS(60), 1,
      anon_sym_end,
    ACTIONS(58), 3,
      anon_sym_endfunction,
      anon_sym_RBRACE,
      anon_sym_if,
  [174] = 2,
    ACTIONS(64), 1,
      anon_sym_end,
    ACTIONS(62), 3,
      anon_sym_endfunction,
      anon_sym_RBRACE,
      anon_sym_if,
  [183] = 4,
    ACTIONS(56), 1,
      anon_sym_else,
    ACTIONS(66), 1,
      anon_sym_end,
    ACTIONS(68), 1,
      anon_sym_endif,
    STATE(33), 1,
      sym_else_clause,
  [196] = 3,
    ACTIONS(70), 1,
      ts_builtin_sym_end,
    ACTIONS(72), 1,
      anon_sym_SEMI,
    ACTIONS(74), 2,
      anon_sym_function,
      sym_identifier,
  [207] = 2,
    STATE(34), 1,
      sym__expression,
    ACTIONS(76), 2,
      sym_identifier,
      sym_number,
  [215] = 3,
    ACTIONS(78), 1,
      anon_sym_RPAREN,
    ACTIONS(80), 1,
      anon_sym_COMMA,
    STATE(18), 1,
      aux_sym_parameters_repeat1,
  [225] = 2,
    STATE(39), 1,
      sym__expression,
    ACTIONS(83), 2,
      sym_identifier,
      sym_number,
  [233] = 2,
    ACTIONS(85), 1,
      anon_sym_end,
    ACTIONS(87), 2,
      anon_sym_endif,
      anon_sym_else,
  [241] = 2,
    ACTIONS(89), 1,
      ts_builtin_sym_end,
    ACTIONS(91), 2,
      anon_sym_function,
      sym_identifier,
  [249] = 2,
    ACTIONS(93), 1,
      ts_builtin_sym_end,
    ACTIONS(95), 2,
      anon_sym_function,
      sym_identifier,
  [257] = 2,
    STATE(16), 1,
      sym__expression,
    ACTIONS(97), 2,
      sym_identifier,
      sym_number,
  [265] = 3,
    ACTIONS(99), 1,
      anon_sym_RPAREN,
    ACTIONS(101), 1,
      anon_sym_COMMA,
    STATE(18), 1,
      aux_sym_parameters_repeat1,
  [275] = 2,
    ACTIONS(105), 1,
      anon_sym_end,
    ACTIONS(103), 2,
      anon_sym_endfunction,
      anon_sym_if,
  [283] = 2,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
    ACTIONS(109), 2,
      anon_sym_function,
      sym_identifier,
  [291] = 2,
    ACTIONS(113), 1,
      anon_sym_end,
    ACTIONS(111), 2,
      anon_sym_endfunction,
      anon_sym_if,
  [299] = 2,
    ACTIONS(115), 1,
      ts_builtin_sym_end,
    ACTIONS(117), 2,
      anon_sym_function,
      sym_identifier,
  [307] = 3,
    ACTIONS(119), 1,
      anon_sym_RPAREN,
    ACTIONS(121), 1,
      sym_identifier,
    STATE(44), 1,
      sym_parameters,
  [317] = 3,
    ACTIONS(101), 1,
      anon_sym_COMMA,
    ACTIONS(123), 1,
      anon_sym_RPAREN,
    STATE(24), 1,
      aux_sym_parameters_repeat1,
  [327] = 2,
    ACTIONS(125), 1,
      anon_sym_end,
    ACTIONS(127), 2,
      anon_sym_endif,
      anon_sym_else,
  [335] = 1,
    ACTIONS(78), 2,
      anon_sym_RPAREN,
      anon_sym_COMMA,
  [340] = 2,
    ACTIONS(129), 1,
      anon_sym_end,
    ACTIONS(131), 1,
      anon_sym_endif,
  [347] = 2,
    ACTIONS(133), 1,
      anon_sym_LBRACE,
    STATE(15), 1,
      sym_block,
  [354] = 2,
    ACTIONS(135), 1,
      anon_sym_end,
    ACTIONS(137), 1,
      anon_sym_endif,
  [361] = 2,
    ACTIONS(62), 1,
      anon_sym_endif,
    ACTIONS(64), 1,
      anon_sym_end,
  [368] = 2,
    ACTIONS(58), 1,
      anon_sym_endif,
    ACTIONS(60), 1,
      anon_sym_end,
  [375] = 2,
    ACTIONS(139), 1,
      anon_sym_end,
    ACTIONS(141), 1,
      anon_sym_endif,
  [382] = 2,
    ACTIONS(133), 1,
      anon_sym_LBRACE,
    STATE(12), 1,
      sym_block,
  [389] = 1,
    ACTIONS(143), 1,
      sym_identifier,
  [393] = 1,
    ACTIONS(145), 1,
      sym_identifier,
  [397] = 1,
    ACTIONS(147), 1,
      ts_builtin_sym_end,
  [401] = 1,
    ACTIONS(149), 1,
      anon_sym_EQ,
  [405] = 1,
    ACTIONS(151), 1,
      anon_sym_RPAREN,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 21,
  [SMALL_STATE(4)] = 37,
  [SMALL_STATE(5)] = 53,
  [SMALL_STATE(6)] = 69,
  [SMALL_STATE(7)] = 84,
  [SMALL_STATE(8)] = 99,
  [SMALL_STATE(9)] = 114,
  [SMALL_STATE(10)] = 128,
  [SMALL_STATE(11)] = 140,
  [SMALL_STATE(12)] = 152,
  [SMALL_STATE(13)] = 165,
  [SMALL_STATE(14)] = 174,
  [SMALL_STATE(15)] = 183,
  [SMALL_STATE(16)] = 196,
  [SMALL_STATE(17)] = 207,
  [SMALL_STATE(18)] = 215,
  [SMALL_STATE(19)] = 225,
  [SMALL_STATE(20)] = 233,
  [SMALL_STATE(21)] = 241,
  [SMALL_STATE(22)] = 249,
  [SMALL_STATE(23)] = 257,
  [SMALL_STATE(24)] = 265,
  [SMALL_STATE(25)] = 275,
  [SMALL_STATE(26)] = 283,
  [SMALL_STATE(27)] = 291,
  [SMALL_STATE(28)] = 299,
  [SMALL_STATE(29)] = 307,
  [SMALL_STATE(30)] = 317,
  [SMALL_STATE(31)] = 327,
  [SMALL_STATE(32)] = 335,
  [SMALL_STATE(33)] = 340,
  [SMALL_STATE(34)] = 347,
  [SMALL_STATE(35)] = 354,
  [SMALL_STATE(36)] = 361,
  [SMALL_STATE(37)] = 368,
  [SMALL_STATE(38)] = 375,
  [SMALL_STATE(39)] = 382,
  [SMALL_STATE(40)] = 389,
  [SMALL_STATE(41)] = 393,
  [SMALL_STATE(42)] = 397,
  [SMALL_STATE(43)] = 401,
  [SMALL_STATE(44)] = 405,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_function_definition_repeat1, 2),
  [19] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_function_definition_repeat1, 2),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_function_definition_repeat1, 2), SHIFT_REPEAT(17),
  [24] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [28] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(40),
  [31] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(43),
  [34] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [36] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [40] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [42] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [44] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [52] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 5),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 5),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 4),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 4),
  [66] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_definition, 3),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_definition, 3),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_parameters_repeat1, 2),
  [80] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_parameters_repeat1, 2), SHIFT_REPEAT(41),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_definition, 4),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_definition, 4),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_definition, 4),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_variable_definition, 4),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameters, 2),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [103] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 3),
  [105] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parameter_list, 3),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_definition, 3),
  [109] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_definition, 3),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameter_list, 2),
  [113] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_parameter_list, 2),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_definition, 5),
  [117] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_definition, 5),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_parameters, 1),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [129] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_else_clause, 2),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_else_clause, 2),
  [139] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [147] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [151] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_octave(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
