mod parse_attribute_node;
mod parse_attributed;
mod parse_block_node;
mod parse_closure_argument_node;
mod parse_closure_node;
mod parse_compound_assign_kind;
mod parse_compound_assign_node;
mod parse_condition_node;
mod parse_conditional_node;
mod parse_constant_node;
mod parse_enum_named_variant_node;
mod parse_enum_node;
mod parse_enum_unnamed_variant_node;
mod parse_enum_variant_kind;
mod parse_expression_kind;
mod parse_expression_named_struct_node;
mod parse_expression_unnamed_struct_node;
mod parse_float;
mod parse_for_node;
mod parse_function_argument_node;
mod parse_function_argument_node_list;
mod parse_function_node;
mod parse_generic_node;
mod parse_generic_node_list;
mod parse_ident_node;
mod parse_identifier;
mod parse_immutablity_kind;
mod parse_implement_item_kind;
mod parse_implement_node;
mod parse_implement_output_type_node;
mod parse_implement_trait_node;
mod parse_infix_operator_kind;
mod parse_int;
mod parse_item_kind;
mod parse_keyword;
mod parse_let_node;
mod parse_literal_kind;
mod parse_loop_node;
mod parse_module_node;
mod parse_output_type_node;
mod parse_path_node;
mod parse_pattern_kind;
mod parse_pattern_match_node;
mod parse_program;
mod parse_program_node;
mod parse_return_node;
mod parse_statement_kind;
mod parse_static_node;
mod parse_string;
mod parse_struct_field_kind;
mod parse_struct_node;
mod parse_symbol;
mod parse_trait_item_constant_node;
mod parse_trait_item_function_node;
mod parse_trait_item_kind;
mod parse_trait_node;
mod parse_type_alias_node;
mod parse_type_array_node;
mod parse_type_kind;
mod parse_unary_operator_kind;
mod parse_use_extra_kind;
mod parse_use_kind;
mod parse_use_node;
mod parse_use_root_ident_kind;
mod parse_use_root_node;
mod parse_value_kind;
mod parse_visibility_kind;
mod parse_while_node;

use crate::*;
use danube_lex::lex;
use danube_lex::{Keyword, Symbol, Token, Tokens};
use nom::{branch::*, bytes::complete::*, combinator::*, multi::*, sequence::*};
use parse_attribute_node::parse_attribute_node;
use parse_attributed::parse_attributed;
use parse_block_node::parse_block_node;
use parse_closure_argument_node::parse_closure_argument_node;
use parse_closure_node::parse_closure_node;
use parse_compound_assign_kind::parse_compound_assign_kind;
use parse_compound_assign_node::parse_compound_assign_node;
use parse_condition_node::parse_condition_node;
use parse_conditional_node::parse_conditional_node;
use parse_constant_node::parse_constant_node;
use parse_enum_named_variant_node::parse_enum_named_variant_node;
use parse_enum_node::parse_enum_node;
use parse_enum_unnamed_variant_node::parse_enum_unnamed_variant_node;
use parse_enum_variant_kind::parse_enum_variant_kind;
use parse_expression_kind::parse_expression_kind;
use parse_expression_named_struct_node::parse_expression_named_struct_node;
use parse_expression_unnamed_struct_node::parse_expression_unnamed_struct_node;
use parse_float::parse_float;
use parse_for_node::parse_for_node;
use parse_function_argument_node::parse_function_argument_node;
use parse_function_argument_node_list::parse_function_argument_node_list;
use parse_function_node::parse_function_node;
use parse_generic_node::parse_generic_node;
use parse_generic_node_list::parse_generic_node_list;
use parse_ident_node::parse_ident_node;
use parse_identifier::parse_identifier;
use parse_immutablity_kind::parse_immutablity_kind;
use parse_implement_item_kind::parse_implement_item_kind;
use parse_implement_node::parse_implement_node;
use parse_implement_output_type_node::parse_implement_output_type_node;
use parse_implement_trait_node::parse_implement_trait_node;
use parse_infix_operator_kind::parse_infix_operator_kind;
use parse_int::parse_int;
use parse_item_kind::parse_item_kind;
use parse_keyword::parse_keyword;
use parse_let_node::parse_let_node;
use parse_literal_kind::parse_literal_kind;
use parse_loop_node::parse_loop_node;
use parse_module_node::parse_module_node;
use parse_output_type_node::parse_output_type_node;
use parse_path_node::parse_path_node;
use parse_pattern_kind::parse_pattern_kind;
use parse_pattern_match_node::parse_pattern_match_node;
use parse_program::parse_program;
use parse_program_node::parse_program_node;
use parse_return_node::parse_return_node;
use parse_statement_kind::parse_statement_kind;
use parse_static_node::parse_static_node;
use parse_string::parse_string;
use parse_struct_field_kind::parse_struct_field_kind;
use parse_struct_node::parse_struct_node;
use parse_symbol::parse_symbol;
use parse_trait_item_constant_node::parse_trait_item_constant_node;
use parse_trait_item_function_node::parse_trait_item_function_node;
use parse_trait_item_kind::parse_trait_item_kind;
use parse_trait_node::parse_trait_node;
use parse_type_alias_node::parse_type_alias_node;
use parse_type_array_node::parse_type_array_node;
use parse_type_kind::parse_type_kind;
use parse_unary_operator_kind::parse_unary_operator_kind;
use parse_use_extra_kind::parse_use_extra_kind;
use parse_use_kind::parse_use_kind;
use parse_use_node::parse_use_node;
use parse_use_root_ident_kind::parse_use_root_ident_kind;
use parse_use_root_node::parse_use_root_node;
use parse_value_kind::parse_value_kind;
use parse_visibility_kind::parse_visibility_kind;
use parse_while_node::parse_while_node;

type ParseResult<'a, T> = nom::IResult<Tokens<'a>, T, ParseError>;

impl std::str::FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token_list = lex(s)?;

        match parse_program(Tokens::new(&token_list)) {
            Ok((_, program)) => Ok(program),
            Err(error) => Err(error.into()),
        }
    }
}

impl Program {
    pub fn from_tokens(t: Tokens) -> ParseResult<Program> {
        parse_program(t)
    }
}
