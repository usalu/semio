//! 📒️ Records and tables of the schema parser: the record body, call record and record field parsers and the table column and cell parsers. See [`super`].

use super::*;

//#region 🔖️Parser
/// 🧾️ Parses one record: its own leading keyword if `spec.keyword` declares one (the
/// `Statements` dispatcher only peeks to choose a variant — consuming it is always this
/// function's job, so a spec is self-contained regardless of whether it's reached via `parse`
/// directly, `Shape::Record`, or a `Statements` variant), positional fields in declaration order,
/// then order-independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a
/// key), until a token that is neither a known key nor an unfilled positional slot — which ends
/// the record (it belongs to whatever comes next: a new statement, a closing brace, or EOF).
pub(super) fn parse_record_body(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    cursor.limits.check_depth(depth, cursor.span())?;
    if spec.layout == RecordLayout::Call {
        return parse_call_record(cursor, spec, depth);
    }
    if let Some(keyword) = &spec.keyword {
        if cursor.at_keyword(keyword) {
            cursor.advance();
        } else {
            return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected keyword '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
        }
    }
    parse_record_fields(cursor, spec, depth)
}

/// 📛️ Parses a `RecordLayout::Call` record: `<name> = <keyword>(args)`. The parenthesized
/// argument list is parsed by the exact same [`parse_record_fields`] loop every other layout uses
/// — it naturally stops at the first token that matches neither a positional slot nor a known
/// key (here, always `)`), so no special "bounded sub-cursor" is needed to keep it from reading
/// past the closing paren.
fn parse_call_record(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let name_field = spec.fields.iter().find(|f| f.is_call_name).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"RecordLayout::Call requires exactly one field marked call_name()",cursor.span()))?;
    let name = ident_like_text(&cursor.expect(TokenKind::Ident)?);
    cursor.expect(TokenKind::Equals)?;
    let keyword = spec.keyword.as_deref().ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,"RecordLayout::Call requires RecordSpec.keyword (the call target)",cursor.span()))?;
    if !cursor.at_keyword(keyword) {
        return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("expected call target '{keyword}', found {:?} '{}'", cursor.peek().kind, cursor.peek().text.as_str()),cursor.span()));
    }
    cursor.advance();
    cursor.expect(TokenKind::LParen)?;
    let mut record = parse_record_fields(cursor, spec, depth)?;
    cursor.expect(TokenKind::RParen)?;
    record.fields.insert(name_field.id, FieldValue::Text(name));
    Ok(record)
}

/// 🧾️ Parses a record's fields: positional fields in declaration order, then order-
/// independent `key=value` attributes (LL(2): an `Ident` followed by `=` is always a key), until a
/// token that is neither a known key nor an unfilled positional slot — which ends the record (it
/// belongs to whatever comes next: a new statement, a closing brace/paren, or EOF). Excludes any
/// field marked `call_name()` from both candidate sets: that field is consumed by the caller
/// (`RecordLayout::Call`'s `<name> =` prefix) before this function ever runs, for a Call-layout
/// spec, and no field is ever marked `call_name()` under any other layout.
fn parse_record_fields(cursor: &mut Cursor, spec: &RecordSpec, depth: usize) -> Result<RecordValue, TextError> {
    let mut record = RecordValue::default();
    let positional: Vec<&FieldSpec> = {
        let mut p: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_some() && !f.is_call_name).collect();
        p.sort_by_key(|f| f.position.unwrap());
        p
    };
    for field in &positional {
        if field.optional {
            // An explicit `_` placeholder always means "absent, but consume the slot" — this is
            // what keeps LATER positionals aligned when an earlier optional one is skipped (see
            // `print_record`'s matching print-side logic). Only positional contexts ever see a
            // `Placeholder` token; keyed optionals are simply omitted instead.
            if cursor.peek().kind == TokenKind::Placeholder {
                cursor.advance();
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
            if !can_start_positional(cursor, &field.shape) {
                record.fields.insert(field.id, FieldValue::Absent);
                continue;
            }
        }
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    // `Statements` fields have no field-level key at all — they're recognized purely by matching
    // one of their own variants' keywords, so at most one such field may appear per record.
    // `Block` fields are also excluded from the `key=value` loop below: their own key acts as a
    // bare leading keyword (`children { ... }`, no `=`) — `Table` fields (bare `key [...] {...}`
    // SoA form) are handled the same way, via their own lookahead branch below.
    let statements_field = spec.fields.iter().find(|f| f.position.is_none() && matches!(f.shape, Shape::Statements(_)));
    let mut keyed: Vec<&FieldSpec> = spec.fields.iter().filter(|f| f.position.is_none() && !f.key.is_empty() && !f.is_call_name && !matches!(f.shape, Shape::Statements(_))).collect();

    loop {
        if let Some(key) = cursor.dynamic_attr_key()? {
            let Some(index) = keyed.iter().position(|f| !matches!(f.shape, Shape::Block(_)) && f.key == key) else { break };
            let field = keyed.remove(index);
            cursor.advance();
            cursor.expect(TokenKind::Equals)?;
            let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        // `Table`'s bare SoA form: the keyword directly followed by `[` (no `=`) — distinct from
        // the AoS-verbose `key=[...]` form already handled by the `at_attr_key` branch above.
        if let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Table(_)) && cursor.at_keyword(&f.key) && cursor.peek_at(1).kind == TokenKind::LBracket) {
            let field = keyed.remove(index);
            let Shape::Table(spec_fn) = &field.shape else { unreachable!() };
            let spec_fn = *spec_fn;
            cursor.advance();
            let value = parse_table_soa(cursor, spec_fn, depth + 1)?;
            record.fields.insert(field.id, value);
            continue;
        }
        let Some(index) = keyed.iter().position(|f| matches!(f.shape, Shape::Block(_)) && cursor.at_keyword(&f.key)) else { break };
        let field = keyed.remove(index);
        cursor.advance();
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }
    for field in keyed {
                if !field.optional { return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("missing required field '{}'",field.key),cursor.span())); }
if !record.fields.contains_key(&field.id){record.fields.insert(field.id,FieldValue::Absent);}
    }

    if let Some(field) = statements_field {
        let value = parse_field_shape(cursor, field, spec, &record, depth + 1)?;
        record.fields.insert(field.id, value);
    }

    Ok(record)
}

// 🚫️async: E1 pure lookahead over the now-sync `Cursor`, called inline in a plain `if` with no
// await anywhere at its one call site — see R9
fn can_start_positional(cursor: &Cursor, shape: &Shape) -> bool {
    match shape {
        Shape::Bool | Shape::Enum(_) => cursor.peek().kind == TokenKind::Ident,
        Shape::Int | Shape::UInt => cursor.peek().kind == TokenKind::Int,
        Shape::Float | Shape::Quantity(_) | Shape::Angle(_) | Shape::Dim(_) => matches!(cursor.peek().kind, TokenKind::Float | TokenKind::Int),
        Shape::Ref(_) => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Count => cursor.peek().kind == TokenKind::Ident,
        Shape::Coord(_) => cursor.peek().kind == TokenKind::At,
        Shape::Dir => cursor.peek().kind == TokenKind::Caret,
        Shape::Range | Shape::Expr => cursor.peek().kind == TokenKind::LParen,
        Shape::Embed(_) | Shape::EmbedFrom(_) => matches!(cursor.peek().kind, TokenKind::Fence | TokenKind::Text | TokenKind::Placeholder),
        // Only `Text|Placeholder` — NOT bare `Ident` — may start an optional positional `Text`
        // field: an unquoted bare-ident value here would be indistinguishable from the next
        // statement's leading keyword, so this deliberately narrower check (versus `Shape::Text`
        // parsing `Ident|Text` everywhere else) resolves that ambiguity.
        Shape::Text => matches!(cursor.peek().kind, TokenKind::Text | TokenKind::Placeholder),
        Shape::Bytes64 => cursor.peek().kind == TokenKind::Text,
        Shape::List(_) => cursor.peek().kind == TokenKind::LBracket,
        Shape::Block(_) | Shape::Map(_) => cursor.peek().kind == TokenKind::LBrace,
        _ => true,
    }
}

//#region 🔖️Table
/// 🚧️ Which shapes have a fixed/bounded token extent and may therefore be a `Table`
/// column: an unbounded `Tuple` (`len: None`, comma-separated until... forever) and `Statements`
/// (repeats until a non-matching keyword) both need an external delimiter to know where they end
/// — fine inside `[ ]`/`{ }` brackets, fatal inside a table row where the ONLY thing marking a
/// row boundary is "we've now read exactly `columns.len()` values".
// 🚫️async: E1 pure, inlined directly into `format!` args alongside `shape_type_name` (Display,
// not Future) — see R9
fn shape_is_self_delimiting(shape: &Shape) -> bool {
    !matches!(shape, Shape::Statements(_) | Shape::Tuple(_, None))
}

/// 🚧️ Spec-build-time validation for a `Table`'s element `RecordSpec` — called wherever a
/// `Shape::Table(spec_fn)` is first evaluated (both parse paths, and printing), since `spec_fn` is
/// a lazy pointer rather than an eagerly-built value there is no earlier moment to check it at.
pub(super) fn validate_table_columns(spec: &RecordSpec) -> Result<(), TextError> {
    for field in &spec.fields {
        if !shape_is_self_delimiting(&field.shape) {
            return Err(TextError::new(ValueRefusalKind::UnsupportedOwner,format!("table column '{}' has a non-self-delimiting shape ({}) and cannot be a table column", field.key, shape_type_name(&field.shape)),TextSpan::at(1, 1)));
        }
    }
    Ok(())
}

/// 🏷️ UPPERCASE schema type tag for a `Shape` — what a `Table` header prints per column
/// (`id:TEXT`), per the unified syntax law (`UPPERCASE` for engine shapes, `PascalCase` reserved
/// for technology-declared domain kinds).
// 🚫️async: E1 pure, inlined directly into `format!` args at both call sites (Display, not Future) — see R9
/// 📊️ Parses the bare SoA form of a `Table` field: `[col:TYPE ...] { v11 v12 ...  v21 v22
/// ... }`, cursor positioned right after the field's own keyword has already been consumed. The
/// header names columns (in the order values then appear per row); a `:TYPE` suffix is accepted
/// but not required to resolve a column (it's a human/printer-facing tag, not load-bearing for
/// parsing — the column's real shape always comes from the element `RecordSpec`), which is what
/// lets a hand-written header omit types the engine can already infer. Rows have NO separator —
/// reading exactly `columns.len()` values per row is what makes a row self-delimiting, which is
/// also why every column shape must itself be self-delimiting (`validate_table_columns`).
fn parse_table_soa(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    let element_spec = (spec_fn.ordinary)();
    validate_table_columns(&element_spec)?;
    cursor.expect(TokenKind::LBracket)?;
    let mut columns: Vec<&FieldSpec> = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        let key_token = cursor.expect(TokenKind::Ident)?;
        let key = key_token.text.as_str().to_string();
        if cursor.peek().kind == TokenKind::Colon {
            cursor.advance();
            cursor.expect(TokenKind::Ident)?; // type tag — documentation only, not re-validated here
        }
        let field_spec = element_spec.fields.iter().find(|f| f.key == key).ok_or_else(|| TextError::new(ValueRefusalKind::InvalidValue,format!("unknown table column '{key}'"),key_token.span))?;
        columns.push(field_spec);
    }
    cursor.expect(TokenKind::RBracket)?;
    cursor.expect(TokenKind::LBrace)?;
    let mut rows = Vec::new();
    while cursor.peek().kind != TokenKind::RBrace {
                if columns.is_empty(){return Err(TextError::new(ValueRefusalKind::InvalidValue,"table requires columns",cursor.span()));}
let mut record = RecordValue::default();
        for field_spec in &columns {
            if cursor.peek().kind == TokenKind::Placeholder {
                                if !field_spec.optional{return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("missing required table field '{}'",field_spec.key),cursor.span()));}
cursor.advance();
                record.fields.insert(field_spec.id, FieldValue::Absent);
                continue;
            }
            let value = parse_table_cell(cursor, &field_spec.shape, depth + 1)?;
            record.fields.insert(field_spec.id, value);
        }
        for field_spec in &element_spec.fields {
            if !record.fields.contains_key(&field_spec.id){
                if !field_spec.optional{return Err(TextError::new(ValueRefusalKind::InvalidValue,format!("missing required table column '{}'",field_spec.key),cursor.span()));}
                record.fields.insert(field_spec.id,FieldValue::Absent);
            }
        }
        rows.push(FieldValue::Record(record));
        cursor.limits.check_nodes(rows.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBrace)?;
    Ok(FieldValue::List(rows))
}

/// 🧱️ Reads one table cell's value. Every table-safe shape is bounded by its own bracket or
/// a fixed token count (`validate_table_columns`/`shape_is_self_delimiting`) — EXCEPT a bare
/// `Shape::Record` column, which prints as a flat run of `key=value` tokens with no bracket of its
/// own (a table row has no `field=` prefix to give it one, unlike a Record-shaped field elsewhere).
/// Two adjacent columns of the SAME record type (or any two types sharing a field name) are then
/// genuinely ambiguous: `parse_record_body`'s keyed loop for column N keeps matching `key=value`
/// tokens for as long as the key is one of ITS OWN not-yet-filled fields, so a column-N field left
/// absent (never printed) silently lets column N's parse run on and swallow column N+1's
/// same-named token instead of stopping at the column boundary. Braced here for exactly that
/// reason — every other shape already round-trips through the ordinary `parse_shape`.
fn parse_table_cell(cursor: &mut Cursor, shape: &Shape, depth: usize) -> Result<FieldValue, TextError> {
    if let Shape::Record(spec_fn) = shape {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        return Ok(FieldValue::Record(record));
    }
    parse_shape(cursor, shape, depth)
}

/// 📋️ The AoS-list form for a `Table` value reached anywhere other than a record's own
/// leading keyword-prefixed field: `[ {row-fields} {row-fields} ... ]`. Each row is brace-wrapped
/// for the same reason a `Shape::Record` table COLUMN is (`parse_table_cell` above) — a table row
/// type is commonly declared with no keyword of its own (a header already gives every row its
/// column order, so SoA rows don't need one), so without a bracket of its own, one row's absent
/// field could let its parse run on into the next row's same-named token exactly like the
/// column-vs-column case. Bracing every row here removes that ambiguity regardless of whether the
/// row type happens to declare a keyword or not.
pub(super) fn parse_table_list(cursor: &mut Cursor, spec_fn: RecordSpecProducer, depth: usize) -> Result<FieldValue, TextError> {
    cursor.expect(TokenKind::LBracket)?;
    let mut items = Vec::new();
    while cursor.peek().kind != TokenKind::RBracket {
        cursor.expect(TokenKind::LBrace)?;
        let record = parse_record_body(cursor, &(spec_fn.ordinary)(), depth + 1)?;
        cursor.expect(TokenKind::RBrace)?;
        items.push(FieldValue::Record(record));
        cursor.limits.check_nodes(items.len(), cursor.span())?;
    }
    cursor.expect(TokenKind::RBracket)?;
    Ok(FieldValue::List(items))
}
//#endregion 🔖️Table
//#endregion 🔖️Parser
