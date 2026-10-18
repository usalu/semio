//! 🗃️ Retained typed state: the stepping of the retained window config typed state with its retirement and drop. See [`super`].

use super::*;

impl<O: WindowConfigOwner> RetainedWindowConfigTypedState<O> {
    #[doc(hidden)]
    pub fn new(native: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, WindowConfigPackLoadDiagnostic> {
        let producer = <O::State as pack::snapshot::ArtifactPack>::borrowed_record_spec_producer().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let spec = producer.decode(native).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?;
        let stack = Vec::new();
        let wrappers = Vec::new();
        Ok(Self { spec, stack, wrappers, string: None, tag: None, root: None, complete: false, handed_back: false, retirement_original: ManuallyDrop::new(None), retirement: None })
    }

    /// 🎁️ Wrappers pending at the current container depth, outermost first; deeper values never see them.
    fn pending_wrappers(&self) -> &[(ValueWrapper, usize)] {
        let depth = self.stack.len();
        &self.wrappers[self.wrappers.iter().rposition(|(_, at)| *at != depth).map_or(0, |index| index + 1)..]
    }

    pub(super) fn expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        self.pending_wrappers().iter().try_fold(self.parent_expected()?, |expected, (wrapper, _)| {
            Ok(match wrapper {
                ValueWrapper::Block => match expected.shape {
                    Some(semio_framework_dsl_record::BorrowedShape::Block(inner)) => ExpectedValue::field(Some(inner())),
                    _ => ExpectedValue::field(None),
                },
                ValueWrapper::Dynamic => ExpectedValue::dsl(),
            })
        })
    }

    fn parent_expected(&self) -> Result<ExpectedValue, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            None if self.root.is_none() => Ok(ExpectedValue::field(None)),
            Some(ValueFrame::Record { root, spec, field: Some(field), .. }) => Ok(ExpectedValue::field((if *root { Some(&self.spec) } else { spec.as_ref() }).and_then(|spec| spec.fields.iter().find(|candidate| candidate.id == *field)).map(|field| field.shape))),
            Some(ValueFrame::Sequence { element, .. }) => Ok(*element),
            Some(ValueFrame::Map { element, key: Some(_), .. }) => Ok(*element),
            Some(ValueFrame::Statements { variants, keyword: Some(keyword), .. }) => Ok(ExpectedValue::field(variants.iter().find(|(candidate, _)| *candidate == keyword).map(|(_, spec)| semio_framework_dsl_record::BorrowedShape::Record(*spec)))),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn child_record_spec(expected: &ExpectedValue) -> Option<semio_framework_dsl_record::BorrowedRecordSpec> {
        match expected.shape.as_ref() {
            Some(semio_framework_dsl_record::BorrowedShape::Record(spec)) | Some(semio_framework_dsl_record::BorrowedShape::Table(spec)) => Some(spec()),
            _ => None,
        }
    }

    /// 🎯️ The DSL shapes whose VALUE is a [`semio_framework_dsl_record::FieldValue::Tuple`] even though the
    /// pack's packed-numeric wire form (`TAG_PACKED_F64`/`TAG_PACKED_VARINT`, `🎒️pack/🌱️value/📝️record-encode/🦀️.rs`
    /// `encode_seq`) carries no tuple-vs-list marker of its own: a coordinate (`@x,y,z`), a direction,
    /// a dimension triple and a range are each a small tuple of floats. The whole-pack decoder spells
    /// exactly this set in its own `is_tuple_shape`; this RETAINED decoder knew only `Shape::Tuple`, so
    /// a `#[dsl(coord)] [f64; 3]` field was rebuilt as a `FieldValue::List` and the state's
    /// `DslField::from_value` rejected it — `WindowConfigPackLoadDiagnostic::TypedState`, surfaced as
    /// `window-config.typed-state`. Every CAD world window config reload died on `CadCamera::position`
    /// (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, cad-content).
    fn shape_is_tuple_valued(shape: Option<&semio_framework_dsl_record::BorrowedShape>) -> bool {
        matches!(
            shape,
            Some(semio_framework_dsl_record::BorrowedShape::Tuple(_, _)) | Some(semio_framework_dsl_record::BorrowedShape::Coord(_)) | Some(semio_framework_dsl_record::BorrowedShape::Dir) | Some(semio_framework_dsl_record::BorrowedShape::Dim(_)) | Some(semio_framework_dsl_record::BorrowedShape::Range)
        )
    }

    /// 📍️ Coordinate/direction/dimension/range literals are fixed-arity tuples of floats; their
    /// elements carry no element shape of their own, so name it here rather than leave every
    /// component shapeless.
    fn child_element(expected: &ExpectedValue) -> ExpectedValue {
        let shape = match expected.shape.as_ref() {
            Some(semio_framework_dsl_record::BorrowedShape::Tuple(inner, _)) | Some(semio_framework_dsl_record::BorrowedShape::List(inner)) | Some(semio_framework_dsl_record::BorrowedShape::Map(inner)) => Some(inner()),
            Some(semio_framework_dsl_record::BorrowedShape::Table(spec)) => Some(semio_framework_dsl_record::BorrowedShape::Record(*spec)),
            Some(semio_framework_dsl_record::BorrowedShape::Coord(_)) | Some(semio_framework_dsl_record::BorrowedShape::Dir) | Some(semio_framework_dsl_record::BorrowedShape::Dim(_)) | Some(semio_framework_dsl_record::BorrowedShape::Range) => Some(semio_framework_dsl_record::BorrowedShape::Float),
            _ => None,
        };
        ExpectedValue { shape, dsl: expected.dsl }
    }

    fn string_target(&self) -> Result<StringTarget, WindowConfigPackLoadDiagnostic> {
        match self.stack.last() {
            Some(ValueFrame::Map { key: None, .. }) => Ok(StringTarget::MapKey),
            Some(ValueFrame::Statements { keyword: None, .. }) => Ok(StringTarget::StatementKeyword),
            _ => Ok(StringTarget::Value(self.expected()?)),
        }
    }

    fn begin_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        if self.string.is_some() {
            return Err(WindowConfigPackLoadDiagnostic::TypedState);
        }
        self.string = Some(RetainedString { target: self.string_target()?, value: String::new(), remaining: None, symbol: None });
        Ok(())
    }

    fn begin_symbol(&mut self, symbol: u64, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        self.begin_string()?;
        let chars = catalog.symbol_chars(symbol).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?;
        let owner = self.string.as_mut().expect("retained config string was just created");
        owner.value.try_reserve_exact(chars.saturating_mul(4)).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        owner.symbol = Some((symbol, 0, chars));
        if chars == 0 {
            self.finish_string()?;
        }
        Ok(())
    }

    pub(super) fn grant_symbol(&mut self, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<bool, WindowConfigPackLoadDiagnostic> {
        let Some(owner) = self.string.as_mut() else { return Ok(false) };
        let Some((symbol, index, chars)) = owner.symbol else { return Ok(false) };
        let value = catalog.symbol_char(symbol, index).map_err(|_| WindowConfigPackLoadDiagnostic::Pack)?.ok_or(WindowConfigPackLoadDiagnostic::Pack)?;
        owner.value.push(value);
        if index + 1 == chars {
            self.finish_string()?;
        } else {
            self.string.as_mut().expect("retained config symbol remains owned").symbol = Some((symbol, index + 1, chars));
        }
        Ok(true)
    }

    fn finish_string(&mut self) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let owner = self.string.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        match owner.target {
            StringTarget::MapKey => match self.stack.last_mut() {
                Some(ValueFrame::Map { key, .. }) if key.is_none() => *key = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::StatementKeyword => match self.stack.last_mut() {
                Some(ValueFrame::Statements { keyword, .. }) if keyword.is_none() => *keyword = Some(owner.value),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            StringTarget::Value(expected) => {
                let value = if expected.dsl { BuiltValue::Dsl(semio_framework_value::DslValue::String(owner.value)) } else { BuiltValue::Field(semio_framework_dsl_record::FieldValue::Text(owner.value)) };
                self.emit(value)?;
            }
        }
        Ok(())
    }

    fn into_dsl(value: BuiltValue) -> Result<semio_framework_value::DslValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Dsl(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn into_field(value: BuiltValue) -> Result<semio_framework_dsl_record::FieldValue, WindowConfigPackLoadDiagnostic> {
        match value {
            BuiltValue::Field(value) => Ok(value),
            _ => Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
    }

    fn emit(&mut self, mut value: BuiltValue) -> Result<(), WindowConfigPackLoadDiagnostic> {
        while let Some(&(wrapper, at)) = self.wrappers.last() {
            if at != self.stack.len() {
                break;
            }
            self.wrappers.pop();
            value = match wrapper {
                ValueWrapper::Block => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Block(Box::new(Self::into_field(value)?))),
                ValueWrapper::Dynamic => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Value(Self::into_dsl(value)?)),
            };
        }
        match self.stack.last_mut() {
            Some(ValueFrame::Record { fields, field, .. }) => {
                let id = field.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if fields.insert(id, Self::into_field(value)?).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
            Some(ValueFrame::Sequence { values, .. }) => values.push(value),
            Some(ValueFrame::Map { values, key, .. }) => values.push((key.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?, value)),
            Some(ValueFrame::Statements { values, keyword, .. }) => {
                let keyword = keyword.take().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                let record = match Self::into_field(value)? {
                    semio_framework_dsl_record::FieldValue::Record(record) => record,
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                values.push((keyword, record));
            }
            Some(ValueFrame::Bytes { .. }) => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            None => {
                let field = Self::into_field(value)?;
                let state = <O::State as semio_framework_dsl_record::DslField>::from_value(&field).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?;
                if self.root.replace(state).is_some() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
            }
        }
        Ok(())
    }

    #[doc(hidden)]
    pub fn begin_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer, count: u64) -> Result<(), WindowConfigPackLoadDiagnostic> {
        let expected = self.expected()?;
        let count = usize::try_from(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
        match kind {
            store::mounted_pack_rt::RetainedValueContainer::Record => {
                let root = self.stack.is_empty() && self.root.is_none();
                let spec = if root { None } else { Self::child_record_spec(&expected) };
                let mut entries = Vec::new();
                entries.try_reserve_exact(count.max((if root { Some(&self.spec) } else { spec.as_ref() }).map_or(0, |spec| spec.fields.len()))).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                let fields = semio_framework_dsl_record::RecordFields::from_empty_slots(entries);
                self.stack.push(ValueFrame::Record { kind, root, spec, fields, field: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Tuple | store::mounted_pack_rt::RetainedValueContainer::List | store::mounted_pack_rt::RetainedValueContainer::PackedF64 | store::mounted_pack_rt::RetainedValueContainer::PackedVarint => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Sequence { kind, element: Self::child_element(&expected), values });
            }
            store::mounted_pack_rt::RetainedValueContainer::Map => {
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Map { dsl: expected.dsl, element: Self::child_element(&expected), values, key: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::Statements => {
                let variants = match expected.shape {
                    Some(semio_framework_dsl_record::BorrowedShape::Statements(variants)) => variants,
                    _ => &[],
                };
                let mut values = Vec::new();
                values.try_reserve_exact(count).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                self.stack.push(ValueFrame::Statements { variants, values, keyword: None });
            }
            store::mounted_pack_rt::RetainedValueContainer::ChunkedBytes | store::mounted_pack_rt::RetainedValueContainer::Table | store::mounted_pack_rt::RetainedValueContainer::Wire => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    fn end_container(&mut self, kind: store::mounted_pack_rt::RetainedValueContainer) -> Result<(), WindowConfigPackLoadDiagnostic> {
        if !self.pending_wrappers().is_empty() {
            return Err(WindowConfigPackLoadDiagnostic::TypedState);
        }
        let frame = self.stack.pop().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
        let value = match frame {
            ValueFrame::Record { kind: expected, root, spec, mut fields, field: None } if expected == kind => {
                if let Some(spec) = if root { Some(&self.spec) } else { spec.as_ref() } {
                    for field in spec.fields {
                        if !fields.contains_key(&field.id) {
                            fields.insert(field.id, semio_framework_dsl_record::FieldValue::Absent);
                        }
                    }
                }
                BuiltValue::Field(semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue { fields }))
            }
            ValueFrame::Sequence { kind: expected, values, .. } if expected == kind => {
                let values = values.into_iter().map(Self::into_field).collect::<Result<Vec<_>, _>>()?;
                let tuple = matches!(kind, store::mounted_pack_rt::RetainedValueContainer::Tuple) || Self::shape_is_tuple_valued(self.expected()?.shape.as_ref());
                BuiltValue::Field(if tuple { semio_framework_dsl_record::FieldValue::Tuple(values) } else { semio_framework_dsl_record::FieldValue::List(values) })
            }
            ValueFrame::Map { dsl: true, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Dsl(semio_framework_value::DslValue::Object(values.into_iter().map(|(key, value)| Self::into_dsl(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?))
            }
            ValueFrame::Map { dsl: false, values, key: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Map => {
                BuiltValue::Field(semio_framework_dsl_record::FieldValue::Map(values.into_iter().map(|(key, value)| Self::into_field(value).map(|value| (key, value))).collect::<Result<Vec<_>, _>>()?))
            }
            ValueFrame::Statements { values, keyword: None, .. } if kind == store::mounted_pack_rt::RetainedValueContainer::Statements => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Statements(values)),
            _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        };
        self.emit(value)
    }

    fn scalar_signed(&self, value: i64) -> BuiltValue {
        let expected = self.expected().ok();
        if expected.as_ref().is_some_and(|expected| expected.dsl) {
            BuiltValue::Dsl(semio_framework_value::DslValue::int(value))
        } else if expected.as_ref().and_then(|expected| expected.shape.as_ref()).is_some_and(|shape| matches!(shape, semio_framework_dsl_record::BorrowedShape::UInt | semio_framework_dsl_record::BorrowedShape::Count)) && value >= 0 {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::UInt(value as u64))
        } else if let Some(semio_framework_dsl_record::BorrowedShape::Enum(_)) = expected.as_ref().and_then(|expected| expected.shape.as_ref()) {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::Enum(value as u32))
        } else {
            BuiltValue::Field(semio_framework_dsl_record::FieldValue::Int(value))
        }
    }

    pub(super) fn accept(&mut self, token: Mounted, catalog: &store::mounted_pack_rt::RetainedPackCatalogCursor) -> Result<(), WindowConfigPackLoadDiagnostic> {
        use store::mounted_pack_rt::{RetainedValueRole as Role, RetainedValueToken as Token};
        match token {
            Token::Tag { value: tag @ (0x00..=0x02), .. } => {
                let expected = self.expected()?;
                let value = match (tag, expected.dsl) {
                    (0x00, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Absent),
                    (0x01, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bool(false)),
                    (0x02, false) => BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bool(true)),
                    (0x01, true) => BuiltValue::Dsl(semio_framework_value::DslValue::Bool(false)),
                    (0x02, true) => BuiltValue::Dsl(semio_framework_value::DslValue::Bool(true)),
                    _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
                };
                self.emit(value)?;
            }
            Token::Tag { value: 0x12, .. } if self.expected()?.dsl => self.emit(BuiltValue::Dsl(semio_framework_value::DslValue::Null))?,
            Token::Tag { value: 0x0e, .. } => self.wrappers.push((ValueWrapper::Block, self.stack.len())),
            Token::Tag { value: 0x11, .. } => self.wrappers.push((ValueWrapper::Dynamic, self.stack.len())),
            Token::Tag { value, .. } if matches!(value, 0x03..=0x0d | 0x0f..=0x10 | 0x15..=0x17) => self.tag = Some(value),
            Token::Begin { kind, count } => {
                self.tag.take();
                self.begin_container(kind, count)?;
            }
            Token::Unsigned { role: Role::FieldId, value } => match self.stack.last_mut() {
                Some(ValueFrame::Record { field, .. }) if field.is_none() && value <= u16::MAX as u64 => *field = Some(value as u16),
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Unsigned { role: Role::Symbol, value } => {
                self.tag.take();
                self.begin_symbol(value, catalog)?;
            }
            Token::Unsigned { role: Role::StringLength, value } => {
                self.tag.take();
                self.begin_string()?;
                let owner = self.string.as_mut().expect("retained inline string was just created");
                owner.value.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                owner.remaining = Some(value);
                if value == 0 {
                    self.finish_string()?;
                }
            }
            Token::Unsigned { role: Role::BytesLength, value } => {
                self.tag.take();
                let mut bytes = Vec::new();
                bytes.try_reserve_exact(usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)?;
                if value == 0 {
                    self.emit(BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bytes64(bytes)))?;
                } else {
                    self.stack.push(ValueFrame::Bytes { values: bytes, remaining: usize::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::Capacity)? });
                }
            }
            Token::StringChar(value) => {
                let owner = self.string.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                owner.value.push(value);
                let remaining = owner.remaining.as_mut().ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                *remaining = remaining.checked_sub(value.len_utf8() as u64).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                if *remaining == 0 {
                    self.finish_string()?;
                }
            }
            Token::Byte(value) => match self.stack.last_mut() {
                Some(ValueFrame::Bytes { values, remaining }) => {
                    values.push(value);
                    *remaining = remaining.checked_sub(1).ok_or(WindowConfigPackLoadDiagnostic::TypedState)?;
                    if *remaining == 0 {
                        let ValueFrame::Bytes { values, remaining: 0 } = self.stack.pop().expect("retained byte owner remains topmost") else { unreachable!() };
                        self.emit(BuiltValue::Field(semio_framework_dsl_record::FieldValue::Bytes64(values)))?;
                    }
                }
                _ => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            },
            Token::Signed(value) => {
                self.tag.take();
                let scalar = self.scalar_signed(value);
                self.emit(scalar)?;
            }
            Token::Unsigned { role: Role::Unsigned | Role::Integer | Role::Enum, value } => {
                let expected = self.expected()?;
                let built = if expected.dsl {
                    BuiltValue::Dsl(semio_framework_value::DslValue::uint(value))
                } else if matches!(expected.shape, Some(semio_framework_dsl_record::BorrowedShape::Enum(_))) || self.tag == Some(0x0a) {
                    BuiltValue::Field(semio_framework_dsl_record::FieldValue::Enum(u32::try_from(value).map_err(|_| WindowConfigPackLoadDiagnostic::TypedState)?))
                } else {
                    BuiltValue::Field(semio_framework_dsl_record::FieldValue::UInt(value))
                };
                self.tag.take();
                self.emit(built)?;
            }
            Token::F64(bits) => {
                self.tag.take();
                let expected = self.expected()?;
                let built = if expected.dsl { BuiltValue::Dsl(semio_framework_value::DslValue::float(f64::from_bits(bits))) } else { BuiltValue::Field(semio_framework_dsl_record::FieldValue::Float(f64::from_bits(bits))) };
                self.emit(built)?;
            }
            Token::End(kind) => self.end_container(kind)?,
            Token::Complete { .. } => {
                if !self.stack.is_empty() || !self.wrappers.is_empty() || self.string.is_some() || self.root.is_none() {
                    return Err(WindowConfigPackLoadDiagnostic::TypedState);
                }
                self.complete = true;
            }
            Token::Unsigned { role: Role::TableRows | Role::TableField | Role::Chunk, .. }
            | Token::WirePresence(_)
            | Token::WireNodePresence(_)
            | Token::WireLabelPresence(_)
            | Token::TablePresence { .. }
            | Token::TableBitmap { .. }
            | Token::Tag { .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
            Token::Unsigned { role: Role::Count, .. } => return Err(WindowConfigPackLoadDiagnostic::TypedState),
        }
        Ok(())
    }

    #[doc(hidden)]
    pub fn take(&mut self) -> Option<O::State> {
        if !self.complete || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.root.take()
    }

    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.as_ref().map_or(Ok(RetirementDemand { copy_bytes: size_of::<RetainedTypedOwners<O>>(), depth: 1, ..Default::default() }), |owner| Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }))
    }
    #[doc(hidden)]
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if !super::super::registry::fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(owner) = self.retirement.as_mut() { return owner.step(grant); }
        let owners = self.retirement_original.take().unwrap_or_else(|| RetainedTypedOwners::<O> {
            spec: std::mem::replace(&mut self.spec, semio_framework_dsl_record::BorrowedRecordSpec { keyword: None, layout: semio_framework_dsl_record::RecordLayout::Inline, fields: &[] }),
            stack: std::mem::take(&mut self.stack), wrappers: std::mem::take(&mut self.wrappers), string: self.string.take(), root: self.root.take(),
        });
        match ControlledRetirement::new(owners) {
            Ok(owner) => { self.retirement = Some(owner); self.handed_back = true; Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })) },
            Err((error, owners)) => { *self.retirement_original = Some(owners); Err(error) },
        }
    }

    #[doc(hidden)]
    pub fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.root.is_none() && self.stack.capacity() == 0 && self.wrappers.capacity() == 0 && self.string.is_none() && self.retirement_original.is_none() && self.retirement.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)
    }
}

impl<O:WindowConfigOwner> RetireOwned for RetainedWindowConfigTypedState<O>{
 fn retirement(self)->Box<dyn RetirementCursor>{
  use semio_framework_value::retirement::{sequence,deferred};let original=ManuallyDrop::new(self);
  unsafe{sequence(vec![deferred(std::ptr::read(&original.spec)),deferred(std::ptr::read(&original.stack)),deferred(std::ptr::read(&original.wrappers)),deferred(std::ptr::read(&original.string)),deferred(original.tag),deferred(std::ptr::read(&original.root)),deferred(original.complete),deferred(original.handed_back)])}
 }
 fn retirement_birth_bytes(&self)->Option<usize>{use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.spec),deferred_birth_bytes_for(&self.stack),deferred_birth_bytes_for(&self.wrappers),deferred_birth_bytes_for(&self.string),deferred_birth_bytes_for(&self.tag),deferred_birth_bytes_for(&self.root),deferred_birth_bytes_for(&self.complete),deferred_birth_bytes_for(&self.handed_back)])}
 fn controlled_retirement_supported()->bool{true}
}

impl<O: WindowConfigOwner> Drop for RetainedWindowConfigTypedState<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained typed window config reached Drop before handoff or terminal-empty retirement");
    }
}
