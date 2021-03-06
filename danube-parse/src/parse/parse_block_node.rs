use super::*;

pub(super) fn parse_block_node(t: Tokens) -> ParseResult<BlockNode> {
    map(
        tuple((
            parse_symbol(Symbol::LeftBrace),
            many0(parse_statement_kind),
            parse_symbol(Symbol::RightBrace),
        )),
        |(_, statement_list, _)| BlockNode { statement_list },
    )(t)
}
