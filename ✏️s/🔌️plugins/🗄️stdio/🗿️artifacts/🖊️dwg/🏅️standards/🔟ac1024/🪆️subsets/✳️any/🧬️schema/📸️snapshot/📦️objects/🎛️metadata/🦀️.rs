//! 🎛️ Record-metadata macros shared by every hand-written `DslRecord` of the AC1024 snapshot.

macro_rules! dwg_ordinary_shape {
    ([primitive $kind:ident]) => { semio_framework_dsl_record::Shape::$kind };
    ([typed $child:ty]) => { <$child as semio_framework_dsl_record::DslField>::shape() };
    ([tuple $kind:ident $count:literal]) => { semio_framework_dsl_record::Shape::Tuple(Box::new(semio_framework_dsl_record::Shape::$kind), Some($count)) };
    ([enum $(($label:literal, $ordinal:literal)),+]) => { semio_framework_dsl_record::Shape::Enum(vec![$(($label.into(), $ordinal)),+]) };
}
macro_rules! dwg_controlled_shape {
    ([primitive $kind:ident], $control:ident) => {{ $control.checkpoint()?; Ok::<_, ValueError>(semio_framework_dsl_record::Shape::$kind) }};
    ([typed $child:ty], $control:ident) => { <$child as semio_framework_dsl_record::DslField>::shape_controlled($control) };
    ([tuple $kind:ident $count:literal], $control:ident) => { semio_framework_dsl_record::producer::boxed(semio_framework_dsl_record::Shape::$kind, $control).map(|shape| semio_framework_dsl_record::Shape::Tuple(shape, Some($count))) };
    ([enum $(($label:literal, $ordinal:literal)),+], $control:ident) => {{
        $control.scoped_stage(|control| {
            const LABELS: &[(&str, u32)] = &[$(($label, $ordinal)),+];
            control.begin_stage(LABELS.len())?;
            let mut labels = control.allocate_vec::<(String, u32)>(LABELS.len())?;
            for &(label, ordinal) in LABELS {
                control.checkpoint()?;
                labels.push((control.copy_text(label)?, ordinal));
                control.step()?;
            }
            Ok(semio_framework_dsl_record::Shape::Enum(labels))
        })
    }};
}
macro_rules! dwg_borrowed_shape {
    ([primitive $kind:ident]) => {semio_framework_dsl_record::BorrowedShape::$kind};
    ([typed $child:ty]) => {<$child as semio_framework_dsl_record::BorrowedDslField>::SHAPE};
    ([tuple $kind:ident $count:literal]) => {semio_framework_dsl_record::BorrowedShape::Tuple(||semio_framework_dsl_record::BorrowedShape::$kind,Some($count))};
    ([enum $(($label:literal,$ordinal:literal)),+]) => {semio_framework_dsl_record::BorrowedShape::Enum(&[$(($label,$ordinal)),+])};
}
macro_rules! dwg_borrowed_metadata {
    ([] [$($generic:ident)?]; $($fields:tt)*) => {};
    ([$owner:ident] []; $($fields:tt)*) => {
        impl semio_framework_dsl_record::BorrowedDslRecord for $owner {
            dwg_borrowed_metadata!(@spec; $($fields)*);
        }
        impl semio_framework_dsl_record::BorrowedDslField for $owner {
            const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
        }
    };
    ([$owner:ident] [$generic:ident]; $($fields:tt)*) => {
        impl<$generic:semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslRecord for $owner<$generic> {
            dwg_borrowed_metadata!(@spec; $($fields)*);
        }
        impl<$generic:semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslField for $owner<$generic> {
            const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
        }
    };
    (@spec; $($id:literal,$key:literal,$optional:literal,$shape:tt;)+) => {
        const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec {
            keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,
            fields:&[$({let mut field=semio_framework_dsl_record::BorrowedFieldSpec::new($id,$key,dwg_borrowed_shape!($shape));field.optional=$optional;field}),+],
        };
    };
}
macro_rules! dwg_metadata {
    ($spec:ident $(<$generic:ident>)?, $controlled:ident, $producer:ident $(=> $owner:ident)?; $($id:literal, $key:literal, $optional:literal, $shape:tt;)+) => {
        dwg_borrowed_metadata!([$($owner)?] [$($generic)?]; $($id,$key,$optional,$shape;)+);
        pub fn $spec $(<$generic: semio_framework_dsl_record::DslField>)?() -> semio_framework_dsl_record::RecordSpec {
            let fields = vec![$({ let mut field = semio_framework_dsl_record::FieldSpec::new($id, $key, dwg_ordinary_shape!($shape)); field.optional = $optional; field }),+];
            semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Inline, fields)
        }
        pub fn $controlled<C: semio_framework_dsl_record::NativeSchemaControl $(, $generic: semio_framework_dsl_record::DslField)?>(control: &mut C) -> Result<semio_framework_dsl_record::RecordSpec, ValueError> {
            control.scoped_stage(|control| {
                const COUNT: usize = [$(stringify!($id)),+].len();
                control.begin_stage(COUNT)?;
                let mut fields = control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(COUNT)?;
                $(control.checkpoint()?;
                let shape = dwg_controlled_shape!($shape, control)?;
                let mut field = semio_framework_dsl_record::producer::field($id, $key, shape, control)?;
                field.optional = $optional;
                fields.push(field);
                control.step()?;)+
                semio_framework_dsl_record::producer::record(None, semio_framework_dsl_record::RecordLayout::Inline, fields, control)
            })
        }
        pub fn $producer $(<$generic: semio_framework_dsl_record::DslField>)?() -> semio_framework_dsl_record::RecordSpecProducer {
            semio_framework_dsl_record::RecordSpecProducer {
                ordinary: $spec $(::<$generic>)?,
                decoding: |control| $controlled::<_ $(, $generic)?>(control),
                encoding: |control| $controlled::<_ $(, $generic)?>(control),
            }
        }
    };
}
macro_rules! dwg_controlled_payloads {
    ($tag_field:literal; units[$($unit:ident = $unit_tag:literal),*]; $($tag:literal, $field:literal, $variant:ident, $ty:ty;)+) => {
        fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{
            control.scoped_stage(|control|{
                let count=match self{$(Self::$unit=>1,)*_=>2};
                control.begin_stage(count)?;
                let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(count,control)?;
                match self{
                    $(Self::$unit=>{record.insert($tag_field,semio_framework_dsl_record::FieldValue::Enum($unit_tag))?;control.step()?;},)*
                    $(Self::$variant(value)=>{
                        record.insert($tag_field,semio_framework_dsl_record::FieldValue::Enum($tag))?;control.step()?;
                        let field=control.scoped_stage(|control|{control.begin_stage(0)?;<$ty as semio_framework_dsl_record::DslField>::to_value_controlled(value,control)})?;
                        record.insert($field,field)?;control.step()?;
                    },)+
                }
                Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))
            })
        }
        fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{
            let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed payload requires a record"))};
            Self::from_record_controlled(record,control)
        }
        fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,ValueError>{
            control.scoped_stage(|control|{
                control.begin_stage(record.fields.len())?;
                let kind=match record.get($tag_field){Some(semio_framework_dsl_record::FieldValue::Enum(value))=>*value,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed payload requires its authored kind"))};
                let payload=match kind{$($unit_tag=>None,)*$($tag=>Some($field),)+_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed payload kind is unknown"))};
                for(field,value)in &record.fields{
                    if *field!=$tag_field&&Some(*field)!=payload&&!matches!(value,semio_framework_dsl_record::FieldValue::Absent){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed payload has an unowned field"))}
                    control.step()?;
                }
                match kind{
                    $($unit_tag=>Ok(Self::$unit),)*
                    $($tag=>Ok(Self::$variant(control.scoped_stage(|control|{control.begin_stage(0)?;<$ty as semio_framework_dsl_record::DslField>::from_value_controlled(record.get($field).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"DWG typed payload is missing"))?,control)})?)),)+
                    _=>unreachable!()
                }
            })
        }
        fn retire_decoded(self){match self{$(Self::$unit=>{},)*$(Self::$variant(value)=><$ty as semio_framework_dsl_record::DslField>::retire_decoded(value),)+}}
    };
}
