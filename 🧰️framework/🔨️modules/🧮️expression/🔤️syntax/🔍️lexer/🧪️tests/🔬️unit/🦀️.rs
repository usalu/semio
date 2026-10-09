use super::*;

fn tokens(src: &str) -> Vec<Token> {
    let chars: Vec<char> = src.chars().collect();
    lex(&chars).unwrap().into_iter().map(|l| l.token).collect()
}

fn failure(src: &str) -> ParseError {
    let chars: Vec<char> = src.chars().collect();
    lex(&chars).unwrap_err()
}

#[test]
fn numbers_with_fractions_and_exponents() {
    assert_eq!(tokens("12 3.5 .5 1e3 2.5E-2 4e+1"), vec![Token::Number(12.0), Token::Number(3.5), Token::Number(0.5), Token::Number(1000.0), Token::Number(0.025), Token::Number(40.0)]);
}

#[test]
fn an_exponent_marker_without_digits_is_not_an_exponent() {
    assert_eq!(tokens("2em"), vec![Token::Number(2.0), Token::Ident("em".into())]);
    assert_eq!(tokens("2e"), vec![Token::Number(2.0), Token::Ident("e".into())]);
}

#[test]
fn units_are_identifiers_until_parsed() {
    assert_eq!(tokens("2.4m 90 mm 45°"), vec![Token::Number(2.4), Token::Ident("m".into()), Token::Number(90.0), Token::Ident("mm".into()), Token::Number(45.0), Token::Degree]);
    assert_eq!(tokens("3 m²"), vec![Token::Number(3.0), Token::Ident("m²".into())]);
}

#[test]
fn operators_and_aliases() {
    assert_eq!(tokens("+ - − * × / ÷ ^ ( ) ,"), vec![Token::Plus, Token::Minus, Token::Minus, Token::Star, Token::Star, Token::Slash, Token::Slash, Token::Caret, Token::LParen, Token::RParen, Token::Comma]);
    assert_eq!(tokens("= == != ≠ < <= ≤ > >= ≥"), vec![Token::Eq, Token::Eq, Token::Ne, Token::Ne, Token::Lt, Token::Le, Token::Le, Token::Gt, Token::Ge, Token::Ge]);
}

#[test]
fn identifiers_may_be_unicode() {
    assert_eq!(tokens("Höhe_2 _x"), vec![Token::Ident("Höhe_2".into()), Token::Ident("_x".into())]);
}

#[test]
fn text_and_quoted_names_with_escapes() {
    assert_eq!(tokens(r#""a\"b\\c\nd\te""#), vec![Token::Text("a\"b\\c\nd\te".into())]);
    assert_eq!(tokens(r"`frame \`w\` \\`"), vec![Token::Name("frame `w` \\".into())]);
}

#[test]
fn spans_count_characters_not_bytes() {
    let chars: Vec<char> = "Höhe + 45°".chars().collect();
    let spans: Vec<(usize, usize)> = lex(&chars).unwrap().iter().map(|l| (l.start, l.end)).collect();
    assert_eq!(spans, vec![(0, 4), (5, 6), (7, 9), (9, 10)]);
}

#[test]
fn bad_characters_are_located() {
    let e = failure("1 + #");
    assert_eq!((e.span.start, e.span.end), (4, 5));
    assert_eq!(e.kind, ParseErrorKind::UnexpectedChar { found: '#' });
    assert_eq!(failure("a ! b").kind, ParseErrorKind::UnexpectedChar { found: '!' });
}

#[test]
fn unterminated_and_invalid_literals() {
    assert_eq!(failure("\"abc").kind, ParseErrorKind::UnterminatedText);
    assert_eq!(failure("`abc").kind, ParseErrorKind::UnterminatedName);
    assert_eq!(failure("``").kind, ParseErrorKind::EmptyName);
    assert_eq!(failure("\"a\\q\"").kind, ParseErrorKind::InvalidEscape { found: 'q' });
    assert_eq!(failure("`a\\n`").kind, ParseErrorKind::InvalidEscape { found: 'n' });
    assert_eq!(failure("\"a\\").kind, ParseErrorKind::UnterminatedText);
    assert_eq!(failure("1e999").kind, ParseErrorKind::InvalidNumber { text: "1e999".into() });
}
