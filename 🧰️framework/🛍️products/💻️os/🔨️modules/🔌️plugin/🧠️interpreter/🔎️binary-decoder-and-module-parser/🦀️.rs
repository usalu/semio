//! 🔎️ Binary decoder and module parser: the byte decoder, the section order and the parsers of function types, limits, memories, tables, globals, constant expressions, code, elements and data. See [`super`].

use super::*;

//#region 🔎️BinaryDecoder
#[derive(Clone)]
pub(super) struct Decoder<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) position: usize,
}

impl<'a> Decoder<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(super) fn at(bytes: &'a [u8], position: usize) -> Self {
        Self { bytes, position }
    }

    pub(super) fn done(&self) -> bool {
        self.position == self.bytes.len()
    }

    pub(super) fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    pub(super) fn byte(&mut self) -> Result<u8, CoreError> {
        let byte = self.bytes.get(self.position).copied().ok_or_else(|| CoreError::Decode("unexpected end of input".into()))?;
        self.position += 1;
        Ok(byte)
    }

    pub(super) fn bytes(&mut self, length: usize) -> Result<&'a [u8], CoreError> {
        let end = self.position.checked_add(length).ok_or_else(|| CoreError::Decode("byte length overflow".into()))?;
        let result = self.bytes.get(self.position..end).ok_or_else(|| CoreError::Decode("byte range exceeds input".into()))?;
        self.position = end;
        Ok(result)
    }

    pub(super) fn u32(&mut self) -> Result<u32, CoreError> {
        let value = self.u64()?;
        u32::try_from(value).map_err(|_| CoreError::Decode("unsigned LEB128 exceeds u32".into()))
    }

    pub(super) fn u64(&mut self) -> Result<u64, CoreError> {
        if let Some(&byte) = self.bytes.get(self.position).filter(|byte| **byte & 0x80 == 0) {
            self.position += 1;
            return Ok(u64::from(byte));
        }
        let mut value = 0u64;
        let mut shift = 0u32;
        loop {
            let byte = self.byte()?;
            let payload = u64::from(byte & 0x7f);
            if shift >= 64 && payload != 0 {
                return Err(CoreError::Decode("unsigned LEB128 overflow".into()));
            }
            value |= payload.checked_shl(shift).unwrap_or(0);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
            if shift > 70 {
                return Err(CoreError::Decode("unsigned LEB128 is too long".into()));
            }
        }
    }

    pub(super) fn i32(&mut self) -> Result<i32, CoreError> {
        Ok(self.signed(32)? as i32)
    }

    pub(super) fn i64(&mut self) -> Result<i64, CoreError> {
        self.signed(64)
    }

    pub(super) fn signed(&mut self, bits: u32) -> Result<i64, CoreError> {
        if let Some(&byte) = self.bytes.get(self.position).filter(|byte| **byte & 0x80 == 0 && bits > 7) {
            self.position += 1;
            let value = i64::from(byte & 0x7f);
            return Ok(if byte & 0x40 != 0 { value | (!0i64 << 7) } else { value });
        }
        let mut value = 0i64;
        let mut shift = 0u32;
        let mut byte;
        loop {
            byte = self.byte()?;
            value |= i64::from(byte & 0x7f).checked_shl(shift).unwrap_or(0);
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
            if shift > 70 {
                return Err(CoreError::Decode("signed LEB128 is too long".into()));
            }
        }
        if shift < bits && byte & 0x40 != 0 {
            value |= (!0i64) << shift;
        }
        Ok(value)
    }

    pub(super) fn name(&mut self) -> Result<String, CoreError> {
        let length = self.u32()? as usize;
        String::from_utf8(self.bytes(length)?.to_vec()).map_err(|_| CoreError::Decode("name is not UTF-8".into()))
    }

    pub(super) fn vector<T>(&mut self, mut decode: impl FnMut(&mut Decoder<'a>) -> Result<T, CoreError>) -> Result<Vec<T>, CoreError> {
        let count = self.u32()? as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(decode(self)?);
        }
        Ok(values)
    }

    pub(super) fn section(&mut self) -> Result<(u8, Decoder<'a>), CoreError> {
        let id = self.byte()?;
        let length = self.u32()? as usize;
        Ok((id, Decoder::new(self.bytes(length)?)))
    }
}
//#endregion 🔎️BinaryDecoder

//#region 🧱️ModuleParser
type FunctionBody = (Vec<ValueType>, Vec<u8>, BTreeMap<usize, ControlBounds>);

pub(super) struct ModuleParser {
    pub(super) types: Vec<FunctionType>,
    imported_functions: Vec<FunctionDecl>,
    defined_types: Vec<u32>,
    pub(super) code: Vec<FunctionBody>,
    pub(super) tables: Vec<TableLimits>,
    pub(super) memories: Vec<MemoryLimits>,
    pub(super) globals: Vec<GlobalDecl>,
    pub(super) exports: BTreeMap<String, ExportKind>,
    pub(super) start: Option<u32>,
    pub(super) elements: Vec<ElementDecl>,
    pub(super) data: Vec<DataDecl>,
    data_count: Option<usize>,
    seen: BTreeSet<u8>,
    last_non_custom_section: u8,
}

impl ModuleParser {
    pub(super) fn parse(bytes: &[u8]) -> Result<CoreModule, CoreError> {
        let mut decoder = Decoder::new(bytes);
        if decoder.bytes(4)? != b"\0asm" || decoder.bytes(4)? != [1, 0, 0, 0] {
            return Err(CoreError::Decode("expected WebAssembly core module version 1".into()));
        }
        let mut parser = Self {
            types: Vec::new(),
            imported_functions: Vec::new(),
            defined_types: Vec::new(),
            code: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            exports: BTreeMap::new(),
            start: None,
            elements: Vec::new(),
            data: Vec::new(),
            data_count: None,
            seen: BTreeSet::new(),
            last_non_custom_section: 0,
        };
        while !decoder.done() {
            let (id, mut section) = decoder.section()?;
            if id != 0 {
                if !parser.seen.insert(id) {
                    return Err(CoreError::Validation(format!("duplicate section {id}")));
                }
                let order = core_section_order(id)?;
                if order < parser.last_non_custom_section {
                    return Err(CoreError::Validation(format!("section {id} is out of order")));
                }
                parser.last_non_custom_section = order;
            }
            parser.parse_section(id, &mut section)?;
            if !section.done() {
                return Err(CoreError::Decode(format!("section {id} has {} trailing bytes", section.remaining())));
            }
        }
        parser.finish(bytes)
    }

    fn parse_section(&mut self, id: u8, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        match id {
            0 => {
                let _ = section.name()?;
                let remaining = section.remaining();
                section.bytes(remaining)?;
            }
            1 => self.types = section.vector(parse_function_type)?,
            2 => self.parse_imports(section)?,
            3 => self.defined_types = section.vector(|decoder| decoder.u32())?,
            4 => self.tables.extend(section.vector(parse_table_type)?),
            5 => self.memories.extend(section.vector(parse_memory_type)?),
            6 => self.globals.extend(section.vector(parse_global)?),
            7 => self.parse_exports(section)?,
            8 => self.start = Some(section.u32()?),
            9 => self.elements = section.vector(parse_element)?,
            10 => self.code = section.vector(parse_code)?,
            11 => self.data = section.vector(parse_data)?,
            12 => {
                let declared = section.u32()? as usize;
                self.data_count = Some(declared);
            }
            13 => return Err(CoreError::Validation("exception tags are outside the owned Semio ABI subset".into())),
            _ => return Err(CoreError::Decode(format!("unknown core section {id}"))),
        }
        Ok(())
    }

    fn parse_imports(&mut self, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        let count = section.u32()?;
        for _ in 0..count {
            let module = section.name()?;
            let name = section.name()?;
            match section.byte()? {
                0x00 => self.imported_functions.push(FunctionDecl::Import { module, name, type_index: section.u32()? }),
                0x01 => self.tables.push(parse_table_type(section)?),
                0x02 => self.memories.push(parse_memory_type(section)?),
                0x03 => {
                    let value_type = ValueType::parse(section.byte()?)?;
                    let mutable = match section.byte()? {
                        0 => false,
                        1 => true,
                        other => return Err(CoreError::Decode(format!("invalid global mutability {other}"))),
                    };
                    self.globals.push(GlobalDecl { value_type, mutable, initializer: ConstExpr::Value(value_type.zero()) });
                }
                0x04 => return Err(CoreError::Validation(format!("tag import {module}.{name} is unsupported"))),
                kind => return Err(CoreError::Decode(format!("unknown import kind {kind}"))),
            }
        }
        Ok(())
    }

    fn parse_exports(&mut self, section: &mut Decoder<'_>) -> Result<(), CoreError> {
        let count = section.u32()?;
        for _ in 0..count {
            let name = section.name()?;
            let kind = match section.byte()? {
                0x00 => ExportKind::Function(section.u32()?),
                0x01 => ExportKind::Table(section.u32()?),
                0x02 => ExportKind::Memory(section.u32()?),
                0x03 => ExportKind::Global(section.u32()?),
                other => return Err(CoreError::Decode(format!("unsupported export kind {other}"))),
            };
            if self.exports.insert(name.clone(), kind).is_some() {
                return Err(CoreError::Validation(format!("duplicate export {name}")));
            }
        }
        Ok(())
    }

    fn finish(self, bytes: &[u8]) -> Result<CoreModule, CoreError> {
        if self.defined_types.len() != self.code.len() {
            return Err(CoreError::Validation(format!("function section declares {} bodies but code has {}", self.defined_types.len(), self.code.len())));
        }
        if self.data_count.is_some_and(|declared| declared != self.data.len()) {
            return Err(CoreError::Validation("data-count section disagrees with data section".into()));
        }
        let mut functions = self.imported_functions;
        for (type_index, (locals, body, controls)) in self.defined_types.into_iter().zip(self.code) {
            if type_index as usize >= self.types.len() {
                return Err(CoreError::Validation(format!("function type index {type_index} is out of bounds")));
            }
            functions.push(FunctionDecl::Defined { type_index, locals, body: body.into(), controls: Arc::new(controls) });
        }
        for export in self.exports.values() {
            match *export {
                ExportKind::Function(index) if index as usize >= functions.len() => return Err(CoreError::Validation(format!("exported function {index} is out of bounds"))),
                ExportKind::Table(index) if index as usize >= self.tables.len() => return Err(CoreError::Validation(format!("exported table {index} is out of bounds"))),
                ExportKind::Memory(index) if index as usize >= self.memories.len() => return Err(CoreError::Validation(format!("exported memory {index} is out of bounds"))),
                ExportKind::Global(index) if index as usize >= self.globals.len() => return Err(CoreError::Validation(format!("exported global {index} is out of bounds"))),
                _ => {}
            }
        }
        Ok(CoreModule {
            bytes_fingerprint: stable_fingerprint(bytes),
            types: self.types,
            functions,
            tables: self.tables,
            memories: self.memories,
            globals: self.globals,
            exports: self.exports,
            start: self.start,
            elements: self.elements,
            data: self.data,
        })
    }
}

fn core_section_order(id: u8) -> Result<u8, CoreError> {
    match id {
        1..=9 => Ok(id),
        12 => Ok(10),
        10 => Ok(11),
        11 => Ok(12),
        13 => Ok(13),
        _ => Err(CoreError::Decode(format!("unknown core section {id}"))),
    }
}

fn parse_function_type(decoder: &mut Decoder<'_>) -> Result<FunctionType, CoreError> {
    if decoder.byte()? != 0x60 {
        return Err(CoreError::Decode("expected function type".into()));
    }
    Ok(FunctionType { parameters: decoder.vector(|decoder| ValueType::parse(decoder.byte()?))?, results: decoder.vector(|decoder| ValueType::parse(decoder.byte()?))? })
}

fn parse_limits(decoder: &mut Decoder<'_>) -> Result<(u64, Option<u64>, bool, bool), CoreError> {
    let flags = decoder.u32()?;
    if flags & !0x07 != 0 || flags & 0x04 != 0 && flags & 0x02 == 0 {
        return Err(CoreError::Decode(format!("invalid limits flags {flags}")));
    }
    let memory64 = flags & 0x04 != 0;
    let minimum = if memory64 { decoder.u64()? } else { u64::from(decoder.u32()?) };
    let maximum = if flags & 0x01 != 0 { Some(if memory64 { decoder.u64()? } else { u64::from(decoder.u32()?) }) } else { None };
    if maximum.is_some_and(|maximum| maximum < minimum) {
        return Err(CoreError::Validation("limits maximum is below minimum".into()));
    }
    Ok((minimum, maximum, flags & 0x02 != 0, memory64))
}

fn parse_memory_type(decoder: &mut Decoder<'_>) -> Result<MemoryLimits, CoreError> {
    let (minimum_pages, maximum_pages, shared, memory64) = parse_limits(decoder)?;
    if shared {
        return Err(CoreError::Validation("shared memory is outside the single-threaded Semio plugin sandbox".into()));
    }
    Ok(MemoryLimits { minimum_pages, maximum_pages, memory64 })
}

fn parse_table_type(decoder: &mut Decoder<'_>) -> Result<TableLimits, CoreError> {
    let element = ValueType::parse(decoder.byte()?)?;
    if !matches!(element, ValueType::FuncRef | ValueType::ExternRef) {
        return Err(CoreError::Validation("table element is not a reference type".into()));
    }
    let (minimum, maximum, shared, memory64) = parse_limits(decoder)?;
    if shared || memory64 {
        return Err(CoreError::Validation("table limits cannot be shared or memory64".into()));
    }
    Ok(TableLimits { element, minimum, maximum })
}

fn parse_global(decoder: &mut Decoder<'_>) -> Result<GlobalDecl, CoreError> {
    let value_type = ValueType::parse(decoder.byte()?)?;
    let mutable = match decoder.byte()? {
        0 => false,
        1 => true,
        other => return Err(CoreError::Decode(format!("invalid global mutability {other}"))),
    };
    Ok(GlobalDecl { value_type, mutable, initializer: parse_const_expr(decoder)? })
}

fn parse_const_expr(decoder: &mut Decoder<'_>) -> Result<ConstExpr, CoreError> {
    let expression = match decoder.byte()? {
        0x23 => ConstExpr::Global(decoder.u32()?),
        0x41 => ConstExpr::Value(Value::I32(decoder.i32()?)),
        0x42 => ConstExpr::Value(Value::I64(decoder.i64()?)),
        0x43 => ConstExpr::Value(Value::F32(u32::from_le_bytes(decoder.bytes(4)?.try_into().expect("four bytes")))),
        0x44 => ConstExpr::Value(Value::F64(u64::from_le_bytes(decoder.bytes(8)?.try_into().expect("eight bytes")))),
        0xd0 => {
            let reference = ValueType::parse(decoder.byte()?)?;
            ConstExpr::Value(reference.zero())
        }
        0xd2 => ConstExpr::RefFunction(decoder.u32()?),
        opcode => return Err(CoreError::Decode(format!("unsupported constant opcode 0x{opcode:02x}"))),
    };
    if decoder.byte()? != 0x0b {
        return Err(CoreError::Decode("constant expression has no end".into()));
    }
    Ok(expression)
}

fn parse_code(decoder: &mut Decoder<'_>) -> Result<FunctionBody, CoreError> {
    let size = decoder.u32()? as usize;
    let mut body = Decoder::new(decoder.bytes(size)?);
    let groups = body.u32()?;
    let mut locals = Vec::new();
    for _ in 0..groups {
        let count = body.u32()? as usize;
        let value_type = ValueType::parse(body.byte()?)?;
        let new_length = locals.len().checked_add(count).ok_or_else(|| CoreError::Validation("local count overflow".into()))?;
        locals.resize(new_length, value_type);
    }
    let code = body.bytes(body.remaining())?.to_vec();
    let controls = analyze_controls(&code)?;
    Ok((locals, code, controls))
}

fn parse_element(decoder: &mut Decoder<'_>) -> Result<ElementDecl, CoreError> {
    let flags = decoder.u32()?;
    let mode = if flags & 0x01 == 0 {
        SegmentMode::Active
    } else if flags & 0x03 == 0x03 {
        SegmentMode::Declarative
    } else {
        SegmentMode::Passive
    };
    let table = if mode == SegmentMode::Active && flags & 0x02 != 0 { decoder.u32()? } else { 0 };
    let offset = if mode == SegmentMode::Active { Some(parse_const_expr(decoder)?) } else { None };
    let expressions = flags & 0x04 != 0;
    if flags & 0x03 != 0 {
        let element_type = decoder.byte()?;
        if element_type != 0x00 && element_type != 0x70 {
            return Err(CoreError::Decode(format!("unsupported element type 0x{element_type:02x}")));
        }
    }
    let values = if expressions {
        decoder.vector(|decoder| match parse_const_expr(decoder)? {
            ConstExpr::RefFunction(index) => Ok(Some(index)),
            ConstExpr::Value(Value::FuncRef(None)) => Ok(None),
            _ => Err(CoreError::Validation("element expression is not funcref".into())),
        })?
    } else {
        decoder.vector(|decoder| decoder.u32().map(Some))?
    };
    Ok(ElementDecl { mode, table, offset, values })
}

fn parse_data(decoder: &mut Decoder<'_>) -> Result<DataDecl, CoreError> {
    let flags = decoder.u32()?;
    let mode = match flags {
        0 | 2 => SegmentMode::Active,
        1 => SegmentMode::Passive,
        _ => return Err(CoreError::Decode(format!("unsupported data segment flags {flags}"))),
    };
    let memory = if flags == 2 { decoder.u32()? } else { 0 };
    let offset = if mode == SegmentMode::Active { Some(parse_const_expr(decoder)?) } else { None };
    let length = decoder.u32()? as usize;
    Ok(DataDecl { mode, memory, offset, bytes: decoder.bytes(length)?.to_vec() })
}
//#endregion 🧱️ModuleParser
