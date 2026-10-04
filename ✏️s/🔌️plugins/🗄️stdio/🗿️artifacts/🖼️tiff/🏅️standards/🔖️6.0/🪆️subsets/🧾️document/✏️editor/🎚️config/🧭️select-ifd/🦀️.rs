//! 🧭️ Schema leaf for selecting one TIFF image-file directory in local editor state.

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetSelectedIfd {
    pub selected_ifd: usize,
}
