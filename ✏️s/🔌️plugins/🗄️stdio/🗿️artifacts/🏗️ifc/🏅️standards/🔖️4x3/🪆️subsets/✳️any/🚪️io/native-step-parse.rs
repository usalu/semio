//! 🧱️ Funded list census and retained value frames construct actual IFC headers, complex rows and reference indexes.
use super::{
    IfcSnapshot, IfcValue, MAX_DEPTH, Port, TxtSnapshot, invalid,
    lex::{Lexer, Token},
    limit,
};
use semio_framework_value::{RetireOwned, ValueError};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcComplexType, IfcEntity, IfcHeader, IfcTypedValue};
use std::mem::{size_of, take};
#[derive(Default, Clone, Copy, RetireOwned)]
struct List {
    items: usize,
    words: usize,
}
#[derive(RetireOwned)]
struct Frame {
    items: Vec<IfcValue>,
    name: Option<String>,
    mode: u8,
}
#[derive(Default, RetireOwned)]
pub(super) struct Receiving {
    pub snapshot: Option<IfcSnapshot>,
    pub error_text: String,
    header_offsets: [usize; 3],
    lists: Vec<List>,
    frames: Vec<Frame>,
    finished: Option<Frame>,
    pending: Option<IfcValue>,
    args: Vec<IfcValue>,
    entity: Option<IfcEntity>,
    component: Option<IfcComplexType>,
    text: String,
    ids: Vec<(u64, usize)>,
    refs: Vec<(u64, usize)>,
}
fn census(source: &TxtSnapshot, state: &mut Receiving, port: &mut Port<'_, '_, '_>) -> Result<(usize, usize), ValueError> {
    let (mut lexer, mut lists, mut entities, mut references, mut tokens, mut previous) = (Lexer::default(), 0usize, 0usize, 0usize, 0usize, 0u8);
    loop {
        let t = lexer.token(source, port)?;
        if t.kind == 0 {
            break;
        }
        tokens = tokens.checked_add(1).filter(|n| *n <= 8_000_000).ok_or_else(|| limit("STEP source exceeds token admission"))?;
        if t.kind == b'(' {
            lists = lists.checked_add(1).filter(|n| *n <= 2_000_000).ok_or_else(|| limit("STEP source exceeds list admission"))?;
        }
        if t.kind == b'R' {
            references += 1;
        }
        if t.kind == b'=' && previous == b'R' {
            entities += 1;
            if entities > 1_000_000 {
                return Err(limit("STEP source exceeds entity admission"));
            }
        }
        previous = t.kind;
    }
    port.vector(lists, &mut state.lists)?;
    let mut lexer = Lexer::default();
    let mut stack = [0usize; MAX_DEPTH + 3];
    let mut depth = 0;
    loop {
        let t = lexer.token(source, port)?;
        if t.kind == 0 {
            break;
        }
        if t.kind == b')' {
            if depth == 0 {
                return Err(invalid("unbalanced STEP closing list"));
            }
            depth -= 1;
            continue;
        }
        if depth > 0 {
            let entry = &mut state.lists[stack[depth - 1]];
            if entry.items == 0 {
                port.effect(size_of::<usize>(), || entry.items = 1)?;
            }
            if t.kind == b',' {
                port.effect(size_of::<usize>(), || entry.items += 1)?;
            }
            if t.kind == b'K' {
                port.effect(size_of::<usize>(), || entry.words += 1)?;
            }
        }
        if t.kind == b'(' {
            if depth == stack.len() {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "STEP source exceeds nesting admission"));
            }
            stack[depth] = state.lists.len();
            depth += 1;
            port.effect(size_of::<List>(), || state.lists.push(List::default()))?;
        }
    }
    if depth != 0 {
        return Err(invalid("unclosed STEP source list"));
    }
    Ok((entities, references.checked_sub(entities).ok_or_else(|| invalid("STEP identity census differs from references"))?))
}
struct Parser<'a, 'b, 'c, 'd, 'e, 'f> {
    source: &'a TxtSnapshot,
    state: &'b mut Receiving,
    port: &'c mut Port<'d, 'e, 'f>,
    lexer: Lexer,
    list: usize,
    look: Option<Token>,
}
impl Parser<'_, '_, '_, '_, '_, '_> {
    fn next(&mut self) -> Result<Token, ValueError> {
        match self.look.take() {
            Some(t) => {
                self.port.at(t.start.offset);
                Ok(t)
            }
            None => self.lexer.token(self.source, self.port),
        }
    }
    fn expected(&mut self, kind: u8) -> Result<Token, ValueError> {
        let t = self.next()?;
        if t.kind != kind {
            return Err(invalid("unexpected STEP token for declared grammar"));
        }
        Ok(t)
    }
    fn word(&mut self, word: &str) -> Result<(), ValueError> {
        let t = self.next()?;
        if !t.is(word) {
            return Err(invalid("mandatory STEP record is missing or out of order"));
        }
        Ok(())
    }
    fn command(&mut self, word: &str) -> Result<(), ValueError> {
        self.word(word)?;
        self.expected(b';')?;
        Ok(())
    }
    fn open(&mut self) -> Result<List, ValueError> {
        let t = self.expected(b'(')?;
        self.port.at(t.start.offset);
        let entry = self.state.lists.get(self.list).copied().ok_or_else(|| invalid("STEP list census differs from grammar"))?;
        self.port.effect(size_of::<usize>(), || self.list += 1)?;
        Ok(entry)
    }
    fn text(&mut self, token: Token) -> Result<(), ValueError> {
        self.lexer.decode(self.source, token, &mut self.state.text, self.port)
    }
    fn frame(&mut self, typed: bool) -> Result<(), ValueError> {
        let entry = self.open()?;
        if self.state.frames.len() > MAX_DEPTH {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "STEP value exceeds nesting admission"));
        }
        self.port.effect(size_of::<Frame>(), || self.state.frames.push(Frame { items: Vec::new(), name: None, mode: 0 }))?;
        let frame = self.state.frames.last_mut().unwrap();
        self.port.vector(entry.items, &mut frame.items)?;
        if typed {
            self.port.effect(size_of::<String>(), || frame.name = Some(take(&mut self.state.text)))?;
        }
        Ok(())
    }
    fn list(&mut self) -> Result<(), ValueError> {
        self.frame(false)?;
        loop {
            if self.state.pending.is_some() {
                let frame = self.state.frames.last_mut().unwrap();
                if frame.items.len() == frame.items.capacity() {
                    return Err(invalid("STEP argument count differs from its admitted list"));
                }
                self.port.effect(size_of::<IfcValue>(), || {
                    frame.items.push(self.state.pending.take().unwrap());
                    frame.mode = 1;
                })?;
            }
            let t = self.next()?;
            let mode = self.state.frames.last().unwrap().mode;
            if t.kind == b')' {
                if mode == 2 {
                    return Err(invalid("STEP list ends after a comma"));
                }
                self.port.effect(size_of::<Frame>(), || self.state.finished = self.state.frames.pop())?;
                self.port.effect(size_of::<IfcValue>(), || {
                    let frame = self.state.finished.as_mut().unwrap();
                    self.state.pending = Some(match frame.name.take() {
                        Some(name) => IfcValue::TypedValue(IfcTypedValue { name, items: take(&mut frame.items) }),
                        None => IfcValue::Aggregate(take(&mut frame.items)),
                    });
                    self.state.finished = None;
                })?;
                if self.state.frames.is_empty() {
                    self.port.effect(size_of::<Vec<IfcValue>>(), || {
                        let Some(IfcValue::Aggregate(items)) = self.state.pending.take() else { unreachable!() };
                        self.state.args = items;
                    })?;
                    return Ok(());
                }
                continue;
            }
            if mode == 1 {
                if t.kind != b',' {
                    return Err(invalid("STEP arguments require comma separators"));
                }
                self.port.effect(1, || self.state.frames.last_mut().unwrap().mode = 2)?;
                continue;
            }
            match t.kind {
                b'(' => {
                    self.look = Some(t);
                    self.frame(false)?;
                }
                b'K' => {
                    if !t.name {
                        return Err(invalid("invalid STEP typed value name"));
                    }
                    self.text(t)?;
                    self.frame(true)?;
                }
                b'$' | b'*' => self.port.effect(size_of::<IfcValue>(), || self.state.pending = Some(if t.kind == b'$' { IfcValue::Unset } else { IfcValue::Derived }))?,
                b'R' => {
                    if t.id == 0 {
                        return Err(invalid("STEP references must be positive"));
                    }
                    if self.state.refs.len() == self.state.refs.capacity() {
                        return Err(invalid("STEP reference census differs from grammar"));
                    }
                    self.port.effect(size_of::<(u64, usize)>(), || self.state.refs.push((t.id, t.start.offset)))?;
                    self.port.effect(size_of::<IfcValue>(), || self.state.pending = Some(IfcValue::Reference(t.id)))?;
                }
                b'S' | b'B' | b'E' => {
                    self.text(t)?;
                    self.port.effect(size_of::<IfcValue>(), || {
                        self.state.pending = Some(match t.kind {
                            b'S' => IfcValue::String(take(&mut self.state.text)),
                            b'B' => IfcValue::Binary(take(&mut self.state.text)),
                            _ => IfcValue::Enum(take(&mut self.state.text)),
                        })
                    })?;
                }
                b'N' => {
                    self.text(t)?;
                    if t.real {
                        let number = self.port.effect(t.length, || self.state.text.parse::<f64>().ok().filter(|v| v.is_finite()))?.ok_or_else(|| invalid("STEP real must be a finite binary64 value"))?;
                        self.port.effect(size_of::<IfcValue>(), || self.state.pending = Some(IfcValue::Real(number)))?;
                    } else {
                        let number = self.port.effect(t.length, || self.state.text.parse::<i64>().ok())?.ok_or_else(|| invalid("STEP integer exceeds signed64"))?;
                        self.port.effect(size_of::<IfcValue>(), || self.state.pending = Some(IfcValue::Integer(number)))?;
                    }
                    self.port.release_text(&mut self.state.text)?;
                }
                _ => return Err(invalid("STEP argument requires a value")),
            }
        }
    }
    fn header(&mut self, index: usize, word: &str) -> Result<(), ValueError> {
        let token = self.next()?;
        if !token.is(word) {
            return Err(invalid("mandatory STEP HEADER record is missing or out of order"));
        }
        self.port.effect(size_of::<usize>(), || self.state.header_offsets[index] = token.start.offset)?;
        self.list()?;
        self.expected(b';')?;
        let header = &mut self.state.snapshot.as_mut().unwrap().header;
        let destination = match index {
            0 => &mut header.file_description,
            1 => &mut header.file_name,
            _ => &mut header.file_schema,
        };
        self.port.effect(size_of::<Vec<IfcValue>>(), || *destination = take(&mut self.state.args))?;
        Ok(())
    }
    fn entity(&mut self, token: Token) -> Result<(), ValueError> {
        if token.id == 0 {
            return Err(invalid("STEP instance identifiers must be positive"));
        }
        if self.state.ids.len() == self.state.ids.capacity() {
            return Err(invalid("STEP instance count differs from admission"));
        }
        self.port.effect(size_of::<(u64, usize)>(), || self.state.ids.push((token.id, token.start.offset)))?;
        self.expected(b'=')?;
        self.port.effect(size_of::<IfcEntity>(), || self.state.entity = Some(IfcEntity { id: token.id, name: String::new(), args: Vec::new(), complex: Vec::new() }))?;
        let t = self.next()?;
        if t.kind == b'(' {
            self.look = Some(t);
            let entry = self.open()?;
            self.port.vector(entry.words.saturating_sub(1), &mut self.state.entity.as_mut().unwrap().complex)?;
            if entry.words == 0 {
                return Err(invalid("STEP complex instance requires components"));
            }
            loop {
                let name = self.next()?;
                if name.kind == b')' {
                    break;
                }
                if name.kind != b'K' || !name.name {
                    return Err(invalid("STEP complex component requires a valid name"));
                }
                self.text(name)?;
                self.port.effect(size_of::<IfcComplexType>(), || self.state.component = Some(IfcComplexType { name: take(&mut self.state.text), args: Vec::new() }))?;
                self.list()?;
                self.port.effect(size_of::<Vec<IfcValue>>(), || self.state.component.as_mut().unwrap().args = take(&mut self.state.args))?;
                let entity = self.state.entity.as_mut().unwrap();
                if entity.name.is_empty() {
                    self.port.effect(size_of::<IfcComplexType>(), || {
                        let component = self.state.component.as_mut().unwrap();
                        entity.name = take(&mut component.name);
                        entity.args = take(&mut component.args);
                        self.state.component = None;
                    })?;
                    continue;
                }
                if entity.complex.len() == entity.complex.capacity() {
                    return Err(invalid("STEP complex component count exceeds admission"));
                }
                self.port.effect(size_of::<IfcComplexType>(), || entity.complex.push(self.state.component.take().unwrap()))?;
            }
        } else {
            if t.kind != b'K' || !t.name {
                return Err(invalid("STEP entity requires a valid name"));
            }
            self.text(t)?;
            self.port.effect(size_of::<String>(), || self.state.entity.as_mut().unwrap().name = take(&mut self.state.text))?;
            self.list()?;
            self.port.effect(size_of::<Vec<IfcValue>>(), || self.state.entity.as_mut().unwrap().args = take(&mut self.state.args))?;
        }
        self.expected(b';')?;
        self.port.effect(size_of::<IfcEntity>(), || self.state.snapshot.as_mut().unwrap().entities.push(self.state.entity.take().unwrap()))?;
        Ok(())
    }
}
fn strings(values: &[IfcValue], port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    if values.is_empty() {
        return Err(invalid("STEP header string lists must be nonempty"));
    }
    for value in values {
        if !port.effect(size_of::<IfcValue>(), || matches!(value, IfcValue::String(_)))? {
            return Err(invalid("STEP header list must contain only strings"));
        }
    }
    Ok(())
}
fn header(source: &IfcHeader, origins: &[usize; 3], port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    port.at(origins[0]);
    match source.file_description.as_slice() {
        [IfcValue::Aggregate(values), IfcValue::String(_)] => strings(values, port)?,
        _ => return Err(invalid("FILE_DESCRIPTION requires its two typed attributes")),
    }
    port.at(origins[1]);
    match source.file_name.as_slice() {
        [IfcValue::String(_), IfcValue::String(_), IfcValue::Aggregate(a), IfcValue::Aggregate(b), IfcValue::String(_), IfcValue::String(_), IfcValue::String(_)] => {
            strings(a, port)?;
            strings(b, port)?;
        }
        _ => return Err(invalid("FILE_NAME requires its seven typed attributes")),
    }
    port.at(origins[2]);
    if !port.effect(size_of::<IfcValue>(), || matches!(source.file_schema.as_slice(),[IfcValue::Aggregate(v)]if matches!(v.as_slice(),[IfcValue::String(name)]if name==super::super::IFC4X3_STEP_SCHEMA)))? {
        return Err(invalid("FILE_SCHEMA must contain exactly IFC4X3_ADD2"));
    }
    Ok(())
}
fn identities(state: &mut Receiving, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    fn sift(values: &mut [(u64, usize)], mut root: usize, end: usize, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
        loop {
            let left = root.saturating_mul(2).saturating_add(1);
            if left >= end {
                return Ok(());
            }
            let mut child = left;
            if left + 1 < end && port.effect(32, || values[left].0 < values[left + 1].0)? {
                child += 1;
            }
            if !port.effect(32, || values[root].0 < values[child].0)? {
                return Ok(());
            }
            port.effect(32, || values.swap(root, child))?;
            root = child;
        }
    }
    let length = state.ids.len();
    for root in (0..length / 2).rev() {
        sift(&mut state.ids, root, length, port)?;
    }
    for end in (1..length).rev() {
        port.effect(32, || state.ids.swap(0, end))?;
        sift(&mut state.ids, 0, end, port)?;
    }
    for pair in state.ids.windows(2) {
        port.at(pair[1].1);
        if port.effect(32, || pair[0].0 == pair[1].0)? {
            return Err(invalid("STEP instance identifiers must be unique"));
        }
    }
    for &(id, offset) in &state.refs {
        port.at(offset);
        let (mut low, mut high, mut found) = (0, state.ids.len(), false);
        while low < high {
            let middle = low + (high - low) / 2;
            match port.effect(16, || state.ids[middle].0.cmp(&id))? {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return Err(invalid("STEP reference has no instance target"));
        }
    }
    Ok(())
}
pub(super) fn receive(source: &TxtSnapshot, state: &mut Receiving, port: &mut Port<'_, '_, '_>) -> Result<(), ValueError> {
    let (entities, references) = census(source, state, port)?;
    port.vector(MAX_DEPTH + 1, &mut state.frames)?;
    port.vector(entities, &mut state.ids)?;
    port.vector(references, &mut state.refs)?;
    port.effect(size_of::<IfcSnapshot>(), || state.snapshot = Some(IfcSnapshot { schema: String::new(), header: IfcHeader { file_description: Vec::new(), file_name: Vec::new(), file_schema: Vec::new() }, entities: Vec::new() }))?;
    let graph = state.snapshot.as_mut().unwrap();
    port.text("stdio.ifc".len(), &mut graph.schema)?;
    port.effect("stdio.ifc".len(), || graph.schema.push_str("stdio.ifc"))?;
    port.vector(entities, &mut graph.entities)?;
    let mut parser = Parser { source, state, port, lexer: Lexer::default(), list: 0, look: None };
    parser.command("ISO-10303-21")?;
    parser.command("HEADER")?;
    for (index, word) in ["FILE_DESCRIPTION", "FILE_NAME", "FILE_SCHEMA"].into_iter().enumerate() {
        parser.header(index, word)?;
    }
    header(&parser.state.snapshot.as_ref().unwrap().header, &parser.state.header_offsets, parser.port)?;
    parser.command("ENDSEC")?;
    parser.command("DATA")?;
    loop {
        let token = parser.next()?;
        if token.is("ENDSEC") {
            parser.expected(b';')?;
            break;
        }
        if token.kind != b'R' {
            return Err(invalid("STEP DATA requires an instance or ENDSEC"));
        }
        parser.entity(token)?;
    }
    parser.command("END-ISO-10303-21")?;
    if parser.next()?.kind != 0 {
        return Err(invalid("unexpected source after STEP exchange structure"));
    }
    if parser.list != parser.state.lists.len() {
        return Err(invalid("STEP list census was not consumed"));
    }
    identities(parser.state, parser.port)
}
