use super::*;
use super::super::{Lexer, Token};

const GRANT: CosGrant = CosGrant { work: 1, allocation_bytes: 64 * 1024, release_bytes: 64 * 1024 };

#[test]
fn content_tokens_preserve_authored_operators_and_operands_under_one_unit_turns() {
    let source = b"% beginning\r\nBT /F1 12 Tf [(A \\(test\\)) -15 <42>] TJ true null BMC ET";
    let mut legacy = Lexer::new(source);
    let mut cursor = ContentTokenCursor::new(0);
    let mut collected = Vec::new();
    for _ in 0..5_000 {
        let step = cursor.advance(source, GRANT, false);
        assert!(step.work <= 1);
        if step.event.unwrap() != ContentTokenEvent::Ready { continue; }
        let token = cursor.take().unwrap();
        let native = match token {
            ContentToken::Value(value) => Token::Object(value),
            ContentToken::Operator { start, end } => Token::Keyword(String::from_utf8_lossy(&source[start..end]).into_owned()),
            ContentToken::End => Token::End,
        };
        assert_eq!(legacy.next_token().unwrap(), native);
        if native == Token::End { break; }
        if let Token::Keyword(word) = native { collected.push(word); }
    }
    assert_eq!(collected, ["BT", "Tf", "TJ", "BMC", "ET"]);
}

#[test]
fn content_tokens_hold_partial_operands_through_cancellation() {
    let input = b"(long retained literal string that halts after its first allocation) Tj";
    let mut cursor = ContentTokenCursor::new(0);
    let mut allocated = 0;
    let mut released = 0;
    while allocated == 0 {
        let step = cursor.advance(input, GRANT, false);
        assert_eq!(step.event, Ok(ContentTokenEvent::More));
        allocated += step.allocated_bytes;
        released += step.released_bytes;
    }
    assert_eq!(cursor.advance(input, GRANT, true).event, Ok(ContentTokenEvent::Cancelled));
    assert!(cursor.take().is_none());
    cursor.begin_close();
    loop {
        let step = cursor.close_step(GRANT);
        assert!(step.work <= 1);
        released += step.released_bytes;
        if step.event == Ok(ContentTokenEvent::Closed) { break; }
    }
    assert_eq!(allocated, released);
}
