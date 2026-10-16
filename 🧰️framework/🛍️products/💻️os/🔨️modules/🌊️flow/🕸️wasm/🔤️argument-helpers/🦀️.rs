//! 🔤️ Typed accessors over preflighted Flow arguments and the shared failure constructors.

use super::*;

pub fn text<'a>(args: &'a FlowArguments, key: &str) -> Result<&'a str, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Text) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    std::str::from_utf8(args.bytes(span)).map_err(|_| FlowFailure::new(AbiErrorCode::InvalidUtf8, key))
}

pub fn optional_text<'a>(args: &'a FlowArguments, key: &str) -> Result<Option<&'a str>, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::OptionalText) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    span.present.then(|| std::str::from_utf8(args.bytes(span)).map_err(|_| FlowFailure::new(AbiErrorCode::InvalidUtf8, key))).transpose()
}

pub fn number(args: &FlowArguments, key: &str) -> Result<f64, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Number) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    let value = f64::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?);
    value.is_finite().then_some(value).ok_or_else(|| FlowFailure::new(AbiErrorCode::MalformedTag, key))
}

pub fn integer(args: &FlowArguments, key: &str) -> Result<u64, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    match span.kind {
        FlowArgumentKind::Integer => Ok(u64::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?)),
        FlowArgumentKind::U32 => Ok(u32::from_le_bytes(args.bytes(span).try_into().map_err(|_| FlowFailure::new(AbiErrorCode::MalformedLength, key))?) as u64),
        FlowArgumentKind::U8 => Ok(u64::from(args.bytes(span)[0])),
        _ => Err(FlowFailure::new(AbiErrorCode::MalformedTag, key)),
    }
}

pub fn u32_integer(args: &FlowArguments, key: &str) -> Result<u32, FlowFailure> {
    u32::try_from(integer(args, key)?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))
}

pub fn u8_integer(args: &FlowArguments, key: &str) -> Result<u8, FlowFailure> {
    u8::try_from(integer(args, key)?).map_err(|_| abi_failure(AbiErrorCode::LimitExceeded))
}

pub fn boolean(args: &FlowArguments, key: &str) -> Result<bool, FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Boolean) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    match args.bytes(span)[0] {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(FlowFailure::new(AbiErrorCode::MalformedTag, key)),
    }
}

pub fn bytes<'a>(args: &'a FlowArguments, key: &str) -> Result<&'a [u8], FlowFailure> {
    let span = args.get(key).ok_or_else(|| FlowFailure::new(AbiErrorCode::MissingField, key))?;
    if !matches!(span.kind, FlowArgumentKind::Bytes) {
        return Err(FlowFailure::new(AbiErrorCode::MalformedTag, key));
    }
    Ok(args.bytes(span))
}

pub fn ok() -> Result<Vec<u8>, FlowFailure> {
    Ok(Vec::new())
}
pub fn domain_error(error: impl std::fmt::Display) -> FlowFailure {
    FlowFailure::new(AbiErrorCode::MalformedTag, error.to_string())
}
pub fn abi_failure(code: AbiErrorCode) -> FlowFailure {
    FlowFailure::new(code, code.to_string())
}

fn dag_cursor_failure(fault: dag_selection_io::DagSelectionTextFault) -> FlowFailure {
    abi_failure(match fault {
        dag_selection_io::DagSelectionTextFault::Cancelled => AbiErrorCode::Cancelled,
        dag_selection_io::DagSelectionTextFault::Interrupted => AbiErrorCode::Interrupted,
        dag_selection_io::DagSelectionTextFault::Deadline => AbiErrorCode::DeadlineExceeded,
        dag_selection_io::DagSelectionTextFault::NoFuel => AbiErrorCode::NoCredit,
        dag_selection_io::DagSelectionTextFault::Limit => AbiErrorCode::LimitExceeded,
        dag_selection_io::DagSelectionTextFault::Sealed => AbiErrorCode::Sealed,
    })
}

pub(super) fn surface_abi_failure(code: SurfaceAbiErrorCode) -> FlowFailure {
    let code = match code {
        SurfaceAbiErrorCode::MalformedTag => AbiErrorCode::MalformedTag,
        SurfaceAbiErrorCode::MalformedLength => AbiErrorCode::MalformedLength,
        SurfaceAbiErrorCode::InvalidUtf8 => AbiErrorCode::InvalidUtf8,
        SurfaceAbiErrorCode::MissingField => AbiErrorCode::MissingField,
        SurfaceAbiErrorCode::LimitExceeded => AbiErrorCode::LimitExceeded,
        SurfaceAbiErrorCode::UnknownOperation => AbiErrorCode::UnknownOperation,
        SurfaceAbiErrorCode::UnknownHandle => AbiErrorCode::UnknownHandle,
        SurfaceAbiErrorCode::StaleGeneration => AbiErrorCode::StaleGeneration,
        SurfaceAbiErrorCode::AbaHandle => AbiErrorCode::AbaHandle,
        SurfaceAbiErrorCode::DuplicateAcknowledgement => AbiErrorCode::DuplicateAcknowledgement,
        SurfaceAbiErrorCode::Interrupted => AbiErrorCode::Interrupted,
        SurfaceAbiErrorCode::Cancelled => AbiErrorCode::Cancelled,
        SurfaceAbiErrorCode::Sealed => AbiErrorCode::Sealed,
        SurfaceAbiErrorCode::LateReply => AbiErrorCode::LateReply,
        SurfaceAbiErrorCode::DuplicateReply => AbiErrorCode::DuplicateReply,
        SurfaceAbiErrorCode::OutOfOrderPage => AbiErrorCode::OutOfOrderPage,
        SurfaceAbiErrorCode::DeadlineExceeded => AbiErrorCode::DeadlineExceeded,
        SurfaceAbiErrorCode::NoCredit => AbiErrorCode::NoCredit,
        SurfaceAbiErrorCode::Busy => AbiErrorCode::Busy,
        SurfaceAbiErrorCode::Closed => AbiErrorCode::Closed,
        SurfaceAbiErrorCode::GenerationExhausted => AbiErrorCode::GenerationExhausted,
    };
    abi_failure(code)
}
