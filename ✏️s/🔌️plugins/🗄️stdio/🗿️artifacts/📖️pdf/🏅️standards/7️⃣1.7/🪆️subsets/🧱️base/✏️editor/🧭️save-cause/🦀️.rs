//! 🧭️ Why a PDF Save declines, as the stable fault codes the editor's fault catalog localizes; the lowest crate both the Save program and the catalog depend on.

/// 🧭️ The cause a declined Save carries; each variant has one code and one bilingual notice in the PDF fault catalog.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SaveCause {
    /// 🧠️ The whole-document reconciliation needs more memory than the grant carries.
    #[default]
    Memory,
    /// 🧾️ The retained trailer is too large or not shallow, so no candidate graph can be admitted.
    Control,
    /// 📎️ The end of the original file cannot be read.
    Original,
    /// 🔐️ The document is encrypted.
    Encrypted,
    /// 🌱️ The original file names another document catalog.
    Root,
    /// 🧾️ The opened document no longer matches the retained original file.
    Baseline,
    /// 🗃️ The original file is no longer held.
    Retained,
    /// 🌲️ The page tree is not one an appended section can follow.
    Tree,
    /// 📚️ The edits reach the catalog, outlines, destinations or document information beyond an appended section.
    Document,
    /// 📄️ An edit to a page reaches beyond what an appended section can state.
    Page,
    /// 🧩️ An edit uses page resources an appended section cannot bind.
    Resources,
    /// 📏️ The appended section would exceed the available memory.
    Extent,
}

impl SaveCause {
    /// 🚨️ Every code a declined Save can carry, in variant order.
    pub const CODES: [&'static str; 12] = [
        "stdio.pdf.save-declined", "stdio.pdf.save-declined-control", "stdio.pdf.save-declined-original", "stdio.pdf.save-declined-encrypted",
        "stdio.pdf.save-declined-root", "stdio.pdf.save-declined-baseline", "stdio.pdf.save-declined-retained", "stdio.pdf.save-declined-tree",
        "stdio.pdf.save-declined-document", "stdio.pdf.save-declined-page", "stdio.pdf.save-declined-resources", "stdio.pdf.save-declined-extent",
    ];

    /// 🚨️ The fault code of this cause; the English developer detail never reaches the user.
    pub const fn code(self) -> &'static str { Self::CODES[self as usize] }
}
