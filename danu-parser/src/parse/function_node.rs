use super::*;

pub(super) fn parse_function_node(s: Tokens) -> ParseResult<FunctionNode> {
  map(
    tuple((
      parse_keyword(Keyword::Function),
      parse_ident_node,
      parse_function_argument_list,
      opt(parse_function_type),
      parse_function_body,
    )),
    |(_, ident, argument_list, return_type, body)| FunctionNode {
      ident,
      argument_list,
      return_type,
      body,
    },
  )(s)
}

fn parse_function_argument_list(s: Tokens) -> ParseResult<Vec<FunctionArgumentNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftParens),
      separated_list(parse_symbol(Symbol::Comma), parse_function_argument_node),
      opt(parse_symbol(Symbol::Comma)),
      parse_symbol(Symbol::RightParens),
    )),
    |(_, argument_list, _, _)| argument_list,
  )(s)
}

fn parse_function_type(s: Tokens) -> ParseResult<TypeNode> {
  map(
    tuple((parse_symbol(Symbol::ReturnArrow), parse_type_node)),
    |(_, ty)| ty,
  )(s)
}

fn parse_function_body(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  alt((parse_function_body_shortcut, parse_function_body_longcut))(s)
}

fn parse_function_body_shortcut(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  map(
    tuple((
      parse_symbol(Symbol::Assign),
      parse_expression_node,
      parse_symbol(Symbol::Semicolon),
    )),
    |(_, expression, _)| vec![StatementNode::Expression(expression)],
  )(s)
}

fn parse_function_body_longcut(s: Tokens) -> ParseResult<Vec<StatementNode>> {
  map(
    tuple((
      parse_symbol(Symbol::LeftBrace),
      many0(parse_statement_node),
      parse_symbol(Symbol::RightBrace),
    )),
    |(_, statement_list, _)| statement_list,
  )(s)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compile(s: &str) -> FunctionNode {
    let (_, token_list) = lex(s).unwrap();
    match parse_function_node(Tokens::new(&token_list)) {
      Ok((_, node)) => node,
      Err(error) => {
        dbg!(error);
        panic!()
      }
    }
  }

  #[test]
  fn no_argument() {
    let source = "fn foo() { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![],
        return_type: None,
        body: vec![]
      }
    );
  }

  #[test]
  fn a_argument() {
    let source = "fn foo(bar: Bar) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![FunctionArgumentNode {
          ident: IdentNode {
            raw: "bar".to_owned()
          },
          ty: TypeNode::Ident(IdentNode {
            raw: "Bar".to_owned()
          })
        }],
        return_type: None,
        body: vec![]
      }
    );
  }

  #[test]
  fn two_argument() {
    let source = "fn foo(bar: Bar, baz: Baz) { }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![
          FunctionArgumentNode {
            ident: IdentNode {
              raw: "bar".to_owned()
            },
            ty: TypeNode::Ident(IdentNode {
              raw: "Bar".to_owned()
            })
          },
          FunctionArgumentNode {
            ident: IdentNode {
              raw: "baz".to_owned()
            },
            ty: TypeNode::Ident(IdentNode {
              raw: "Baz".to_owned()
            })
          }
        ],
        return_type: None,
        body: vec![]
      }
    );
  }

  #[test]
  fn function_conditional_if() {
    let source = "fn foo() {
      if true { }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![],
        return_type: None,
        body: vec![StatementNode::Conditional(StatementConditionalNode {
          main_branch: Box::new((
            ExpressionNode::Literal(LiteralValueNode::Bool(true)),
            vec![],
          )),
          branch_list: vec![],
          other: None
        })]
      }
    );
  }

  #[test]
  fn function_conditional_else() {
    let source = "fn foo() {
      if true { } else { }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![],
        return_type: None,
        body: vec![StatementNode::Conditional(StatementConditionalNode {
          main_branch: Box::new((
            ExpressionNode::Literal(LiteralValueNode::Bool(true)),
            vec![],
          )),
          branch_list: vec![],
          other: Some(vec![])
        })]
      }
    );
  }

  #[test]
  fn function_pattern_match() {
    let source = "fn foo() {
      match true {
        true => { },
      }
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![],
        return_type: None,
        body: vec![StatementNode::PatternMatch(PatternMatchNode {
          condition: Box::new(ExpressionNode::Literal(LiteralValueNode::Bool(true))),
          branch_list: vec![(
            vec![PatternNode::Literal(LiteralValueNode::Bool(true))],
            vec![]
          )],
        })]
      }
    );
  }

  #[test]
  fn function_expression_let_mut_pattern() {
    let source = "fn foo() {
      let mut Foo::Bar = true;
    }";
    assert_eq!(
      compile(source),
      FunctionNode {
        ident: IdentNode {
          raw: "foo".to_owned()
        },
        argument_list: vec![],
        return_type: None,
        body: vec![StatementNode::LetMut(LetMutNode {
          pattern: PatternNode::Path(PathNode {
            ident_list: vec![
              IdentNode {
                raw: "Foo".to_owned()
              },
              IdentNode {
                raw: "Bar".to_owned()
              }
            ]
          }),
          ty: None,
          value: ExpressionNode::Literal(LiteralValueNode::Bool(true)),
        })]
      }
    );
  }
}
