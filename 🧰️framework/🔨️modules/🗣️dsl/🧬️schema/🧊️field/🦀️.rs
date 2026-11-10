//! 🧊️ A field type whose native receiver builds it from the moved field needs one declaration for both receiving paths.
/// 🧬️ Declares owned and borrowed receiving for field types whose native receiver builds them from the field's own source.
#[macro_export]
macro_rules! native_field_receiving{($($ty:ty),+$(,)?)=>{$(
 impl $crate::DslOwnedReceivingField for $ty{type Cursor=$crate::receiving::owned::NativeFieldReceiving<$ty>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
 impl $crate::DslReceivingField for $ty{fn receive_value(value:&$crate::FieldValue,output:&mut ::core::option::Option<Self>,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::ReceivingOutcome{$crate::receiving::receive_decoded(value,output,owner)}}
)+};}
