//! 🪆️ Generic record field and variant construction under explicit owned controls.
use crate::*;
use semio_framework_dsl::{UnitSpec,unit_by_symbol};

/// 📋️ Constructs the declared native sequence directly from semantic field owners.
pub trait DslSequence<T>: Default + IntoIterator<Item = T> {
    fn from_decoded<I: IntoIterator<Item = Result<T, TextError>>>(values: I) -> Result<Self, TextError>;
    fn begin_controlled(length: usize, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError>;
    fn ensure_controlled_slot(&mut self, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError>;
    fn push_controlled(&mut self, value: T) -> Result<(), ValueError>;
}

impl<T> DslSequence<T> for Vec<T> {
    fn from_decoded<I: IntoIterator<Item = Result<T, TextError>>>(values: I) -> Result<Self, TextError> { values.into_iter().collect() }
    fn begin_controlled(length: usize, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { control.allocate_vec(length) }
    fn ensure_controlled_slot(&mut self, _control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> { Ok(()) }
    fn push_controlled(&mut self, value: T) -> Result<(), ValueError> { self.push(value); Ok(()) }
}

impl<T, const N: usize> DslSequence<T> for semio_framework_value::list::PagedList<T, N> {
    fn from_decoded<I: IntoIterator<Item = Result<T, TextError>>>(values: I) -> Result<Self, TextError> {
        let mut output = Self::default();
        for value in values { output.try_push(value?).map_err(|error| TextError::new(ValueRefusalKind::OwnershipLimit, error.to_string(), TextSpan::at(1, 1)))?; }
        Ok(output)
    }
    fn begin_controlled(_length: usize, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { control.checkpoint()?; Ok(Self::default()) }
    fn ensure_controlled_slot(&mut self, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        while !self.has_reserved_slot() {
            let bytes = self.next_allocation_bytes().map_err(ValueError::from)?;
            control.charge(bytes)?;
            self.reserve_one(bytes).map_err(|error| ValueError::from(error.refusal()))?;
        }
        Ok(())
    }
    fn push_controlled(&mut self, value: T) -> Result<(), ValueError> { self.push_reserved(value).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "paged DSL sequence rejected an admitted slot")) }
}

/// 🧭️ Projects semantic sequences through their actual borrowed native owners.
pub trait DslSequenceView<T> {
    type Iter<'a>: ExactSizeIterator<Item = &'a T> where Self: 'a, T: 'a;
    fn field_items(&self) -> Self::Iter<'_>;
}

impl<T> DslSequenceView<T> for [T] {
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;
    fn field_items(&self) -> Self::Iter<'_> { self.iter() }
}
impl<T> DslSequenceView<T> for Vec<T> {
    type Iter<'a> = std::slice::Iter<'a, T> where T: 'a;
    fn field_items(&self) -> Self::Iter<'_> { self.iter() }
}
impl<T, const N: usize> DslSequenceView<T> for semio_framework_value::list::PagedList<T, N> {
    type Iter<'a> = semio_framework_value::list::PagedIter<'a, T, N> where T: 'a;
    fn field_items(&self) -> Self::Iter<'_> { self.iter() }
}
//#region 🔖️Field
/// 🔗️ Bridges a concrete Rust field type to the engine's `Shape`/`FieldValue` — every
/// primitive implements it directly; `#[derive(DslRecord)]`/`#[derive(DslScalar)]` implement it
/// for technology-declared nested types, so composition (a record field whose type is another
/// derived record or enum) works transparently through the same trait.
pub trait DslField: Sized {
    /// 🔎️ Reads a declared field without materializing a mirror or an unbounded projection.
    fn projection_view(&self,_path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no retained source projection"))}
    /// 🔤️ Borrows a dynamic key from the original ranked or indexed field owner.
    fn projection_key(&self,_path:&[usize],_index:usize)->Result<&str,ValueError>{Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no retained ranked key projection"))}
    // 🚫️async: E4 fn-pointer transitivity — `Shape::Record`/`Table`/`Statements` hold
    // `fn() -> RecordSpec`; every `shape()` implementation ultimately feeds one, directly or
    // through a derived `__dsl_spec` — see R9.
    fn shape() -> Shape;
    /// 🏭️ Constructs only the explicitly declared shape metadata under caller admission.
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native schema implementation"))}
    fn to_value(&self) -> FieldValue;
    /// 🛫️ Projects explicitly owned fields under cumulative output admission and cancellation.
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native projection implementation"))}
    /// 📑️ Projects a record without an intermediate boxed field carrier.
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record projection implementation"))}
    fn from_value(value: &FieldValue) -> Result<Self, String>;
    /// 🧹️ Retires a completed field according to its owner after partial reconstruction fails.
    fn retire_decoded(self) { drop(self); }
    /// 🛬️ Constructs an owned field under the caller's cumulative allocation and work control.
    fn from_value_controlled(_value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled native construction implementation"))
    }
    /// 📑️ Binds a record view without cloning a temporary FieldValue carrier.
    fn from_record_controlled(_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"field owner has no controlled record construction implementation"))
    }
}

/// 📦️ Boxed ownership preserves the inner field's schema, value, and decoding errors.
impl<T: DslField> DslField for Box<T> {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{DslField::projection_view(self.as_ref(),path)}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{DslField::projection_key(self.as_ref(),path,index)}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{T::to_value_controlled(self.as_ref(),control)}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{T::to_record_controlled(self.as_ref(),control)}

    fn retire_decoded(self) { T::retire_decoded(*self); }
    fn shape() -> Shape {
        T::shape()
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{T::shape_controlled(control)}
    fn to_value(&self) -> FieldValue {
        T::to_value(self.as_ref())
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        T::from_value(value).map(Box::new)
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.charge(std::mem::size_of::<T>())?;
        control.scoped_stage(|control|T::from_value_controlled(value, control)).map(Box::new)
    }
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.charge(std::mem::size_of::<T>())?;control.scoped_stage(|control|T::from_record_controlled(record,control)).map(Box::new)}
}

macro_rules! impl_dsl_field_int {
    ($ty:ty, $shape:expr, $variant:ident, $as_ty:ty) => {
        impl DslField for $ty {
            fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::$variant(*self as $as_ty))}
            // 🚫️async: E4 — see `DslField::shape`'s tag above.
            fn shape() -> Shape {
                $shape
            }
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok($shape)}
            fn to_value(&self) -> FieldValue {
                FieldValue::$variant(*self as $as_ty)
            }
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::$variant(*self as $as_ty))}
            fn from_value(value: &FieldValue) -> Result<Self, String> {
                match value {
                    FieldValue::$variant(v) => <$ty>::try_from(*v).map_err(|_| format!("integer {v} out of range for {}", stringify!($ty))),
                    other => Err(format!("expected {}, found {other:?}", stringify!($variant))),
                }
            }
            fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
                control.step()?;
                match value { FieldValue::$variant(number)=><$ty>::try_from(*number).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,format!("integer {number} out of range for {}",stringify!($ty)))),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,concat!("expected ",stringify!($variant)))) }
            }
        }
    };
}

impl_dsl_field_int!(i8, Shape::Int, Int, i64);
impl_dsl_field_int!(i16, Shape::Int, Int, i64);
impl_dsl_field_int!(i32, Shape::Int, Int, i64);
impl_dsl_field_int!(i64, Shape::Int, Int, i64);
impl_dsl_field_int!(isize, Shape::Int, Int, i64);
impl_dsl_field_int!(u8, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u16, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u32, Shape::UInt, UInt, u64);
impl_dsl_field_int!(u64, Shape::UInt, UInt, u64);
impl_dsl_field_int!(usize, Shape::UInt, UInt, u64);

impl DslField for bool {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::Bool(*self))}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Bool
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Bool)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Bool(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Bool(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Bool(b) => Ok(*b),
            other => Err(format!("expected Bool, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value {FieldValue::Bool(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Bool"))} }
}

impl DslField for f32 {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}let FieldValue::Float(value)=self.to_value()else{return Err(native_encoding::projection_path_error())};Ok(native_encoding::FieldProjectionView::Float(value))}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        let bits=self.to_bits();let value=if bits&0x7f800000==0x7f800000&&bits&0x7fffff!=0{f64::from_bits(((bits as u64&0x80000000)<<32)|0x7ff0000000000000|((bits as u64&0x7fffff)<<29))}else{*self as f64};FieldValue::Float(value)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(<Self as DslField>::to_value(self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f)=>{let bits=f.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err("NaN word is not exactly representable at binary32 width".into());}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*f as f32)}},
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {FieldValue::Float(value)=>{let bits=value.to_bits();if bits&0x7ff0000000000000==0x7ff0000000000000&&bits&0xfffffffffffff!=0{if bits&0x1fffffff!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN word is not exactly representable at binary32 width"));}Ok(f32::from_bits(((bits>>32)as u32&0x80000000)|0x7f800000|((bits>>29)as u32&0x7fffff)))}else{Ok(*value as f32)}},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))}
    }
}

impl DslField for f64 {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::Float(*self))}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Float
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Float)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Float(*self)
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Float(*self))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Float(f) => Ok(*f),
            other => Err(format!("expected Float, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> { control.step()?;match value{FieldValue::Float(value)=>Ok(*value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Float"))} }
}

/// 🔤️ `String` binds as `Shape::Text` — the one string shape. The parser accepts either a
/// bare `Ident` token or a quoted `Text` token wherever `Text` is expected; the printer emits bare
/// (unquoted) whenever `crate::os_dsl::is_bare_ident` holds for the value, quoted+escaped otherwise —
/// so bare-vs-quoted is entirely a printing decision now, not a separate shape a field opts into.
impl DslField for String {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::Text(self))}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Text
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Text)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Text(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Text(control.copy_text(self)?))}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Text(s) => Ok(s.clone()),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value { FieldValue::Text(text)=>control.copy_text(text),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Text")) }
    }
}

/// 📝️ Shared actors expose their original immutable text and require full authority for construction.
impl DslField for semio_framework_value::SharedUtf8 {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if !path.is_empty(){return Err(native_encoding::projection_path_error())}Ok(native_encoding::FieldProjectionView::Text(self.as_str()))}
    fn shape()->Shape{Shape::Text}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Text)}
    fn to_value(&self)->FieldValue{FieldValue::Text(self.as_str().to_owned())}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.step()?;Ok(FieldValue::Text(control.copy_text(self.as_str())?))}
    fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Text(text)=>Ok(Self::from(text.clone())),_=>Err("expected Text".into())}}
    fn from_value_controlled(_value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"shared UTF8 construction requires original full retained authority"))}
}
#[cfg(test)]
#[path="📝️shared-utf8/🧪️tests/🦀️.rs"]
mod shared_utf8_tests;

impl<const N: usize> DslField for semio_framework_value::paged::PagedUtf8<N> {
    fn shape() -> Shape { Shape::Text }
    fn shape_controlled<C: NativeSchemaControl>(control: &mut C) -> Result<Shape, ValueError> { control.checkpoint()?; Ok(Shape::Text) }
    fn to_value(&self) -> FieldValue { FieldValue::Text(self.to_string_owner()) }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> { Ok(FieldValue::Text(self.to_string_owner_controlled(control)?)) }
    fn from_value(value: &FieldValue) -> Result<Self, String> { match value { FieldValue::Text(text) => Self::try_from_str(text).map_err(ValueError::into_message), _ => Err("expected Text".into()) } }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { match value { FieldValue::Text(text) => Self::try_from_str_controlled(text, control), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected Text")) } }
}

impl<T: DslField, const N: usize> DslField for semio_framework_value::list::PagedList<T, N> {
    fn shape() -> Shape { Shape::List(Box::new(T::shape())) }
    fn shape_controlled<C: NativeSchemaControl>(control: &mut C) -> Result<Shape, ValueError> { control.scoped_depth(64, |control| Ok(Shape::List(crate::producer::boxed(T::shape_controlled(control)?, control)?))) }
    fn to_value(&self) -> FieldValue { FieldValue::List(self.iter().map(T::to_value).collect()) }
    fn to_value_controlled(&self, control: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> { Ok(FieldValue::List(native_encoding::project_list(self, control)?)) }
    fn from_value(value: &FieldValue) -> Result<Self, String> { let FieldValue::List(items) = value else { return Err("expected List".into()); }; let mut output = Self::default(); for value in items { output.try_push(T::from_value(value)?).map_err(|error| error.to_string())?; } Ok(output) }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { match value { FieldValue::List(items) => __rt::decode_list_controlled(items, control), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected List")) } }
    fn projection_view(&self, path: &[usize]) -> Result<native_encoding::FieldProjectionView<'_>, ValueError> { if path.is_empty() { return Ok(native_encoding::FieldProjectionView::List(self.len())); } self.get(path[0]).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..]) }
    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> { let (row, rest) = path.split_first().ok_or_else(native_encoding::projection_path_error)?; self.get(*row).ok_or_else(native_encoding::projection_path_error)?.projection_key(rest, index) }
    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
}

impl<T: DslField, const N: usize> DslField for semio_framework_value::paged::PagedMap<T, N> {
    fn shape() -> Shape { Shape::Map(Box::new(T::shape())) }
    fn shape_controlled<C: NativeSchemaControl>(control: &mut C) -> Result<Shape, ValueError> { control.scoped_depth(64, |control| Ok(Shape::Map(crate::producer::boxed(T::shape_controlled(control)?, control)?))) }
    fn to_value(&self) -> FieldValue { FieldValue::Map(self.iter().map(|(key, value)| (key.to_string_owner(), value.to_value())).collect()) }
    fn from_value(value: &FieldValue) -> Result<Self, String> { match value { FieldValue::Map(entries) => Self::try_from_fallible_entries(entries.iter().map(|(key, value)| Ok((semio_framework_value::paged::PagedUtf8::try_from_str(key)?, T::from_value(value).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message))?)))).map_err(ValueError::into_message), _ => Err("expected Map".into()) } }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        let FieldValue::Map(entries) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected Map")); };
        let mut output = __rt::DecodedFieldOwner::new(semio_framework_value::list::PagedList::<(semio_framework_value::paged::PagedUtf8<{usize::MAX}>, T), N>::default(), |entries| { for (_, value) in entries { T::retire_decoded(value); } });
        for (key, value) in entries {
            control.step()?;
            if output.as_mut().iter().any(|entry| entry.0.eq_str(key)) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "duplicate paged map field key")); }
            while !output.as_mut().has_reserved_slot() { let bytes = output.as_mut().next_allocation_bytes()?; control.charge(bytes)?; output.as_mut().reserve_one(bytes).map_err(|error| ValueError::from(error.refusal()))?; }
            let key = semio_framework_value::paged::PagedUtf8::try_from_str_controlled(key, control)?;
            let value = T::from_value_controlled(value, control)?;
            output.as_mut().push_reserved((key, value)).map_err(|_| ValueError::new(ValueRefusalKind::InvariantViolated, "paged map decoder lost its admitted slot"))?;
        }
        Self::from_retained_entries(output.take())
    }
    fn projection_view(&self, path: &[usize]) -> Result<native_encoding::FieldProjectionView<'_>, ValueError> { if path.is_empty() { return Ok(native_encoding::FieldProjectionView::Map(self.len())); } self.entry_at(path[0]).ok_or_else(native_encoding::projection_path_error)?.1.projection_view(&path[1..]) }
    fn retire_decoded(self) { for (_, value) in self.into_retained_entries() { T::retire_decoded(value); } }
}

/// 🔌️ A wire literal as a plain struct field (or inside a `#[dsl(table)]` `Vec` as a
/// `WIRE`-typed column) — thin `DslField` wrapper around `crate::WireValue` so adopter
/// technologies never need to hand-roll their own `Shape::Wire` binding.
#[derive(Clone, Debug, PartialEq)]
pub struct Wire(pub WireValue);

impl Wire {
    fn structure_controlled<C:NativeSchemaControl>(value:&WireValue,control:&mut C)->Result<WireValue,ValueError>{
        fn node<C:NativeSchemaControl>(value:&WireNode,control:&mut C)->Result<WireNode,ValueError>{Ok(WireNode{id:control.copy_text(&value.id)?,kind:value.kind.as_deref().map(|text|control.copy_text(text)).transpose()?,port:value.port.as_deref().map(|text|control.copy_text(text)).transpose()?})}
        control.step()?;
        Ok(WireValue{from:node(&value.from,control)?,edge:value.edge.as_ref().map(|(directed,to)|node(to,control).map(|to|(*directed,to))).transpose()?,edge_label:WireEdgeLabel{id:value.edge_label.id.as_deref().map(|text|control.copy_text(text)).transpose()?,kind:value.edge_label.kind.as_deref().map(|text|control.copy_text(text)).transpose()?},properties:DslValue::Null})
    }
}

impl DslField for Wire {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{use native_encoding::{FieldProjectionView as V,projection_path_error};if path.is_empty(){return Ok(V::Wire(self.0.edge.as_ref().map(|(directed,_)|*directed)))}if path[0]==8{return DslField::projection_view(&self.0.properties,&path[1..])}if path.len()!=1{return Err(projection_path_error())}let text=match path[0]{0=>Some(self.0.from.id.as_str()),1=>self.0.from.kind.as_deref(),2=>self.0.from.port.as_deref(),3=>self.0.edge.as_ref().map(|(_,to)|to.id.as_str()),4=>self.0.edge.as_ref().and_then(|(_,to)|to.kind.as_deref()),5=>self.0.edge.as_ref().and_then(|(_,to)|to.port.as_deref()),6=>self.0.edge_label.id.as_deref(),7=>self.0.edge_label.kind.as_deref(),_=>return Err(projection_path_error())};Ok(text.map(V::Text).unwrap_or(V::Absent))}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[8,rest @ ..]=>DslField::projection_key(&self.0.properties,rest,index),_=>Err(native_encoding::projection_path_error())}}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Wire
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Wire)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Wire(self.0.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{let mut output=Self::structure_controlled(&self.0,control)?;output.properties=<DslValue as semio_framework_value::ToValue>::to_value_controlled(&self.0.properties,control)?;Ok(FieldValue::Wire(output))}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let FieldValue::Wire(value)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Wire"))};let mut output=Self::structure_controlled(value,control)?;output.properties=<DslValue as semio_framework_value::FromValue>::from_value_controlled(&value.properties,control)?;Ok(Self(output))}
    fn retire_decoded(self){<DslValue as semio_framework_value::FromValue>::retire_decoded(self.0.properties)}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Wire(w) => Ok(Wire(w.clone())),
            other => Err(format!("expected Wire, found {other:?}")),
        }
    }
}
/// 📚️ General recursion seam: `#[derive(DslRecord)]`/`#[derive(DslScalar)]` fields classify
/// `Vec<T>`/`[T; N]` directly (so their own printed shape stays field-specific), but a NESTED
/// collection — `Vec<Vec<T>>`, a fixed-size array field, ... — needs its inner element type to
/// satisfy `DslField` itself. These two blanket impls close that gap generically instead of adding
/// a special-cased `FieldKind` for every depth of nesting.
impl<T: DslField> DslField for Vec<T> {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::List(self.len()))}self.get(path[0]).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let child=*path.first().ok_or_else(native_encoding::projection_path_error)?;self.get(child).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self,control).map(FieldValue::List)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::List(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::List(crate::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 `Iterator::map` cannot await per-element (residue shape 1) and `T::to_value`/`from_value`
    // are AFIT over an arbitrary implementor, so — unlike a known-pure leaf fn — R9 does not apply;
    // the fix is a plain sequential loop that awaits each element in turn.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(self.len());
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::List(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::List(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(T::from_value(item)?);
                }
                Ok(out)
            }
            other => Err(format!("expected List, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::List(items)=>__rt::decode_list_controlled(items,control),
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected List")),
        }
    }
}

/// 🗺️ Same recursion seam as `Vec<T>`, for a `BTreeMap<String, T>` that's itself nested
/// (e.g. `Option<BTreeMap<String, T>>`) rather than a bare top-level field — `#[derive(DslRecord)]`
/// classifies a *bare* `BTreeMap<String, T>` field directly via its own dedicated `FieldKind`
/// (same `Shape::Map` this produces), so the two never conflict.
impl<T: DslField> DslField for std::collections::BTreeMap<String, T> {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::Map(self.len()))}self.iter().nth(path[0]).map(|(_,value)|value).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.is_empty(){return self.keys().nth(index).map(String::as_str).ok_or_else(native_encoding::projection_path_error)}self.iter().nth(path[0]).map(|(_,value)|value).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_map(self,control)}

    fn retire_decoded(self) { for (_,value) in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Map(Box::new(T::shape()))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Map(crate::producer::boxed(T::shape_controlled(control)?,control)?)))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut entries = Vec::with_capacity(self.len());
        for (k, v) in self {
            entries.push((k.clone(), v.to_value()));
        }
        FieldValue::Map(entries)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Map(entries) => {
                let mut out = Self::new();
                for (k, v) in entries {
                    out.insert(k.clone(), T::from_value(v)?);
                }
                Ok(out)
            }
            other => Err(format!("expected Map, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Map(entries)=>{
                let slot=std::mem::size_of::<(String,T)>().checked_add(128).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map slot size overflow"))?;
                control.charge(entries.len().checked_mul(slot).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"map ownership size overflow"))?)?;
                let mut output=__rt::DecodedFieldOwner::new(Self::new(),Self::retire_decoded);
                for (key,value) in entries {if let Some(previous)=output.as_mut().insert(control.copy_text(key)?,control.scoped_stage(|control|T::from_value_controlled(value,control))?){T::retire_decoded(previous);}}
                Ok(output.take())
            },
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected Map")),
        }
    }
}

/// 📐️ Fixed-arity `Shape::Tuple(_, Some(N))` — a packed `x,y,z`-style literal for any `N`.
impl<T: DslField, const N: usize> DslField for [T; N] {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{if path.is_empty(){return Ok(native_encoding::FieldProjectionView::Tuple(N))}self.get(path[0]).ok_or_else(native_encoding::projection_path_error)?.projection_view(&path[1..])}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let child=*path.first().ok_or_else(native_encoding::projection_path_error)?;self.get(child).ok_or_else(native_encoding::projection_path_error)?.projection_key(&path[1..],index)}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{native_encoding::project_list(self.as_slice(),control).map(FieldValue::Tuple)}

    fn retire_decoded(self) { for value in self { T::retire_decoded(value); } }
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Tuple(Box::new(T::shape()), Some(N))
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_depth(64,|control|Ok(Shape::Tuple(crate::producer::boxed(T::shape_controlled(control)?,control)?,Some(N))))}
    // 🔁 Same R9-doesn't-apply reasoning as `Vec<T>` above: sequential loop, not `.map().collect()`.
    fn to_value(&self) -> FieldValue {
        let mut items = Vec::with_capacity(N);
        for item in self {
            items.push(item.to_value());
        }
        FieldValue::Tuple(items)
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Tuple(items) if items.len() == N => {
                let mut converted: Vec<T> = Vec::with_capacity(N);
                for item in items {
                    converted.push(T::from_value(item)?);
                }
                converted.try_into().map_err(|_| format!("expected {N} items, got a length mismatch"))
            }
            other => Err(format!("expected a {N}-item Tuple, found {other:?}")),
        }
    }
    fn from_value_controlled(value: &FieldValue, control: &mut NativeDecodeControl<'_>) -> Result<Self,ValueError> {
        control.step()?;
        match value {
            FieldValue::Tuple(items) if items.len()==N=>{let mut output=__rt::DecodedFieldOwner::new(control.allocate_vec::<T>(N)?,<Vec<T> as DslField>::retire_decoded);for item in items {output.as_mut().push(control.scoped_stage(|control|T::from_value_controlled(item,control))?);}output.take().try_into().map_err(|values:Vec<T>|{<Vec<T> as DslField>::retire_decoded(values);ValueError::new(ValueRefusalKind::InvariantViolated,"tuple arity mismatch")})},
            _=>Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("expected a {N}-item Tuple"))),
        }
    }
}

/// 🌱️ Schema-less dynamic literal — binds as `Shape::Value`.
impl DslField for DslValue {
    fn projection_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>{use native_encoding::FieldProjectionView as V;Ok(match intrinsic_projection_node(self,path)?{Self::Null=>V::IntrinsicNull,Self::Bool(v)=>V::IntrinsicBool(*v),Self::Number(v)=>V::IntrinsicNumber(*v),Self::String(v)=>V::IntrinsicText(v),Self::Bytes(v)=>V::IntrinsicBytes(v),Self::Array(v)=>V::IntrinsicArray(v.len()),Self::Object(v)=>V::IntrinsicObject(v.len())})}
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match intrinsic_projection_node(self,path)?{Self::Object(v)=>v.get(index).map(|(key,_)|key.as_str()).ok_or_else(native_encoding::projection_path_error),_=>Err(native_encoding::projection_path_error())}}
    // 🚫️async: E4 — see `DslField::shape`'s tag above.
    fn shape() -> Shape {
        Shape::Value
    }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Value)}
    fn to_value(&self) -> FieldValue {
        FieldValue::Value(self.clone())
    }
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{<Self as semio_framework_value::ToValue>::to_value_controlled(self,control).map(FieldValue::Value)}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{FieldValue::Value(value)=><Self as semio_framework_value::FromValue>::from_value_controlled(value,control),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected an intrinsic Value field"))}}
    fn retire_decoded(self){<Self as semio_framework_value::FromValue>::retire_decoded(self)}
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        match value {
            FieldValue::Value(dsl_value) => Ok(dsl_value.clone()),
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}
fn intrinsic_projection_node<'a>(mut value:&'a DslValue,path:&[usize])->Result<&'a DslValue,ValueError>{for index in path{value=match value{DslValue::Array(items)=>items.get(*index),DslValue::Object(items)=>items.get(*index).map(|(_,value)|value),_=>None}.ok_or_else(native_encoding::projection_path_error)?;}Ok(value)}
//#endregion 🔖️Field

//#region 🔖️Variants
/// 🌿️ Bridges an enum whose variants are each their own keyword-tagged record — the type
/// bound for `#[dsl(statements)] Vec<T>` collection fields and for `#[derive(DslOps)]` operation
/// enums. `#[derive(DslEnum)]`-with-struct-variants and `#[derive(DslOps)]` both implement this.
pub trait DslVariants: Sized {
    /// 🏷️ Reads the current authored variant and its literal ordinal without allocating labels.
    fn projected_variant_identity(&self)->(&'static str,usize,RecordSpecProducer);
    /// 🫳️ Borrows one original variant field by its declared ordinal path.
    fn projected_variant_view(&self,path:&[usize])->Result<native_encoding::FieldProjectionView<'_>,ValueError>;
    /// 🔑️ Borrows a key from the original variant's ranked field source.
    fn projected_variant_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>;

    /// 🐌️ Lazy: each entry is a zero-capture `fn` pointer, not an eagerly-built `RecordSpec`
    /// — a self-referential grammar's own `variants()` would otherwise need to recurse infinitely
    /// just to construct this list. See [`Shape::Statements`]'s doc comment for the full rationale.
    // 🚫️async: E4 — the returned `Vec<(String, fn() -> RecordSpec)>` IS a fn-pointer table, and
    // `Shape::Statements(<T>::variants())` is itself called from inside a sync `__dsl_spec` — see R9.
    fn variants() -> Vec<(String, RecordSpecProducer)>;
    /// 🌿️ Owns literal variant labels and their lazy controlled schema producers.
    fn variants_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Vec<(String,RecordSpecProducer)>,ValueError>{control.checkpoint()?;Err(ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native schema implementation"))}
    fn to_named_record(&self) -> (String, RecordValue);
    /// 🌿️ Projects a declared tagged variant under the same cumulative output control.
    fn to_named_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<(String,RecordValue),ValueError>{control.checkpoint()?;Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native projection implementation"))}
    /// ⚠️ Returns `TextError` (not `String`, unlike [`DslField::from_value`]) so
    /// generated bodies can `?`-propagate it directly — this is the same error type
    /// `crate::os_spr::OpText::parse_op`/`crate::os_store::ArtifactDsl::parse_dsl` already return, and the derive's
    /// `#[dsl(statements)]` field codegen composes it without any conversion at every nesting depth.
    fn from_named_record(keyword: &str, record: &RecordValue) -> Result<Self, TextError>;
    /// 🌲️ Retires a completed tagged value through its domain owner.
    fn retire_decoded_variant(self) { drop(self); }
    /// 🌿️ Constructs a declared variant without invoking an unchecked owner binding.
    fn from_named_record_controlled(_keyword:&str,_record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.checkpoint()?;
        Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,"variant owner has no controlled native construction implementation"))
    }
}
//#endregion 🔖️Variants

//#region 🔖️Runtime
/// ⚙️ Helpers remaining after P6 flag day — DslField/DslVariants derive bodies only (codec paths deleted).
pub mod __rt {
    use super::*;

    /// 🧹️ Holds a completed typed field until construction commits or invokes its actual owner retirement.
    pub struct DecodedFieldOwner<T> { value:Option<T>, retire:fn(T) }
    impl<T> DecodedFieldOwner<T> {
        /// 📥️ Adopts one owned field with its declared retirement function.
        pub fn new(value:T,retire:fn(T))->Self { Self{value:Some(value),retire} }
        /// 🌿️ Allows bounded construction inside the guarded owned collection.
        pub fn as_mut(&mut self)->&mut T { self.value.as_mut().expect("decoded owner already transferred") }
        /// 📤️ Transfers ownership only after every required constructor succeeds.
        pub fn take(mut self)->T { self.value.take().expect("decoded owner already transferred") }
    }
    impl<T> Drop for DecodedFieldOwner<T> { fn drop(&mut self){if let Some(value)=self.value.take(){(self.retire)(value);}} }

    /// 📋️ Binds declared list elements with one known collection workload and cumulative ownership.
    pub fn decode_list_controlled<T:DslField,C:DslSequence<T>>(items:&[FieldValue],control:&mut NativeDecodeControl<'_>)->Result<C,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(C::begin_controlled(items.len(),control)?,|values:C|{for value in values{T::retire_decoded(value);}});for item in items{output.as_mut().ensure_controlled_slot(control)?;let value=control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(item,control)})?;output.as_mut().push_controlled(value)?;control.step()?;}Ok(output.take())})
    }
    /// 🌿️ Binds tagged variants with exact collection progress and declared variant retirement.
    pub fn decode_statements_controlled<T:DslVariants,C:DslSequence<T>>(items:&[(String,RecordValue)],control:&mut NativeDecodeControl<'_>)->Result<C,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(items.len())?;let mut output=DecodedFieldOwner::new(C::begin_controlled(items.len(),control)?,|values:C|{for value in values{T::retire_decoded_variant(value);}});for(keyword,record)in items{output.as_mut().ensure_controlled_slot(control)?;let value=control.scoped_stage(|control|{control.begin_stage(0)?;T::from_named_record_controlled(keyword,record,control)})?;output.as_mut().push_controlled(value)?;control.step()?;}Ok(output.take())})
    }

    /// 📐️ Resolves a `#[dsl(unit = "...")]`/`#[dsl(angle = "...")]` symbol at spec-build
    /// time. An unknown symbol is a derive-time misuse (a typo'd unit string, caught the first time
    /// the generated `__dsl_spec` runs — every RecordSpec-law test exercises this), so it panics
    /// rather than threading a `Result` through the whole spec-building call chain, matching
    /// `newtype_variant_spec`'s convention above.
    pub fn unit_for_derive(symbol: &'static str) -> &'static UnitSpec {
        unit_by_symbol(symbol).unwrap_or_else(|| panic!("dsl: unknown unit symbol '{symbol}' in #[dsl(unit = ...)]/#[dsl(angle = ...)]"))
    }

    /// 📦️ Single-field tuple ("newtype") enum variant support — `Variant(Body)` delegates its
    /// whole `RecordSpec`/value to `Body`'s own `DslField` impl rather than wrapping it in one
    /// positional field, so `Body` prints/parses identically whether reached through the enum or on
    /// its own. `Body` must have `Shape::Record` (i.e. itself come from `#[derive(DslRecord)]` or
    /// `#[derive(DslArtifact)]`) — anything else is a derive-time misuse, hence the panic rather than
    /// a `Result` (there is no sensible recoverable path for a grammar that's wrong at compile time).
    // 🚫️async: E4 — this fn's VALUE is cast `as fn() -> RecordSpec` at every newtype-variant call
    // site (`✨️derive/🦀️.rs`'s `dsl_variants_codegen`), and it calls the now-sync `DslField::shape`.
    pub fn newtype_variant_spec<T: DslField>() -> RecordSpec {
        match T::shape() {
            Shape::Record(spec_fn) => (spec_fn.ordinary)(),
            other => panic!("newtype variant's inner type must have Record shape, found {other:?}"),
        }
    }

    /// 🪆️ Delegates a declared record variant through its explicit lazy schema producer.
    pub fn newtype_variant_producer<T:DslField>()->RecordSpecProducer{
        RecordSpecProducer{ordinary:newtype_variant_spec::<T>,decoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.decode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}},encoding:|control|{match T::shape_controlled(control)?{Shape::Record(producer)=>producer.encode(control),_=>Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"newtype variant requires a controlled Record schema"))}}}
    }

    pub fn newtype_variant_to_record<T: DslField>(inner: &T) -> RecordValue {
        match inner.to_value() {
            FieldValue::Record(record) => record,
            other => panic!("newtype variant's inner type must produce a Record value, found {other:?}"),
        }
    }

    pub fn newtype_variant_from_record<T: DslField>(record: &RecordValue) -> Result<T, TextError> {
        T::from_value(&FieldValue::Record(record.clone())).map_err(|message|TextError::new(ValueRefusalKind::InvalidValue,message,TextSpan::at(1,1)))
    }
}

//#endregion 🔖️Runtime

//#region 🔖️OpTextRt
/// 🔤️ Handcrafted `OpText` helper — the text twin of [`variants_binary`].
///
/// An operation line is ONE terminal keyword-tagged record, so it parses through
/// [`parse_exact`], which rejects every token outside the variant's own schema body: a trailing
/// `unknown-field 1` is not a second statement, it is garbage the line must refuse. Plain
/// [`parse`] stops at the end of the record it recognises and silently drops the rest, which is
/// the document-mode contract, not the op-line one.
pub mod variants_text {
    use super::{print, DslVariants, JoinMode, Limits, ParseOptions, SourceMode, TextError};

    pub fn parse_op<T: DslVariants>(line: &str) -> Result<T, TextError> {
        let variants = T::variants();
        for (keyword, spec_fn) in &variants {
            if line == keyword.as_str() || line.starts_with(&format!("{keyword} ")) {
                let record = super::parse_exact(line, &(spec_fn.ordinary)(), &ParseOptions { limits: Limits::default(), mode: SourceMode::Inline })?;
                return T::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown operation line '{line}'"),semio_framework_diagnostic::TextSpan::at(1,1)))
    }

    pub fn print_op<T: DslVariants>(op: &T) -> String {
        let (keyword, record) = op.to_named_record();
        let variants = T::variants();
        let spec_fn = variants.iter().find(|(key, _)| key == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
        print(&record, &(spec_fn.ordinary)(), JoinMode::Inline)
    }
}
//#endregion 🔖️OpTextRt

#[path = "🏷️type/🦀️.rs"]
mod value_type_binding;

#[path = "🪆️optional/🦀️.rs"]
mod optional_field;
