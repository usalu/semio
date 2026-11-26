//! 🧮️ Lexical primitives for the existing canonical Poll input.
use semio_plugin_abi::owned_abi::OwnedCorePollLimits;

pub(super) const LIMITS_PREFIX: &[u8] = br#"],"command_page":null,"cold_pair_page":null,"limits":["#;

pub(super) fn byte(rest: &mut &[u8]) -> Option<u8> {
    let (head, tail) = rest.split_first()?;
    *rest = tail;
    Some(*head)
}

pub(super) fn take(rest: &mut &[u8], expected: &[u8]) -> Option<()> {
    for &character in expected {
        if byte(rest)? != character { return None; }
    }
    Some(())
}

pub(super) fn uint(rest: &mut &[u8], end: u8) -> Option<u64> {
    let first = byte(rest)?.checked_sub(b'0').filter(|digit| *digit <= 9)?;
    let mut value = u64::from(first);
    loop {
        let next = byte(rest)?;
        if next == end { return Some(value); }
        let digit = next.checked_sub(b'0').filter(|digit| *digit <= 9 && first != 0)?;
        value = value.checked_mul(10)?.checked_add(u64::from(digit))?;
    }
}

pub(super) fn generation(rest: &mut &[u8]) -> Option<u64> {
    take(rest, b"\"")?;
    let value = uint(rest, b'"')?;
    (value != 0).then_some(value)
}

pub(super) fn request_sequence(rest: &mut &[u8], end: u8) -> Option<u64> {
    let value = uint(rest, end)?;
    (1..(1 << 53)).contains(&value).then_some(value)
}

pub(super) fn limits(rest: &mut &[u8]) -> Option<OwnedCorePollLimits> {
    let mut scalars = [0; OwnedCorePollLimits::ARITY];
    for (index, value) in scalars.iter_mut().enumerate() {
        *value = uint(rest, if index + 1 == OwnedCorePollLimits::ARITY { b']' } else { b',' })?;
    }
    take(rest, b"}}")?;
    if !rest.is_empty() { return None; }
    OwnedCorePollLimits::from_scalars(scalars)
}
