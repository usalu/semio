//! 🔢️ PDF page numbers: the one place where the one-based page number a person reads and types becomes the zero-based page index the model and its mutations keep.
//! [Page labels](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)

//#region 🔢️PageNumber
/// 🔢️ A typed page number outside `1..=last`: the number as typed and the highest valid number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageNumberRange {
    pub number: u64,
    pub last: usize,
}

/// 📖️ The zero-based index of a one-based page number among `count` slots; `count` is the page count, or one more where a new page may be appended.
pub fn page_index(count: usize, number: u64) -> Result<usize, PageNumberRange> {
    usize::try_from(number).ok().and_then(|number| number.checked_sub(1)).filter(|index| *index < count).ok_or(PageNumberRange { number, last: count })
}

/// 📄️ The one-based page number a person reads for a zero-based page index.
pub const fn page_number(index: usize) -> usize {
    index + 1
}
//#endregion 🔢️PageNumber
