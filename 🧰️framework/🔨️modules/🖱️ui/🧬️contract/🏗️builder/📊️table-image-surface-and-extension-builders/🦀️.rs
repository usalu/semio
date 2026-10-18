//! 📊️ Table, image, surface and extension builders: the table and table row builders, the typestate image builder, the surface builder and the extension builder. See [`super`].

use super::*;

//#region 🔖️Builder
//#region 📊️Table
/// 📊️ A column-headed data table — `Component::Table`. Its rows are [`table_row`] children. Build with
/// [`table`].
pub struct TableBuilder {
    pub(super) base: NodeBase,
    pub(super) label: crate::Label,
    columns: crate::UiFixedList<crate::Label>,
    row_label: Option<crate::Label>,
    column_label: Option<crate::Label>,
    actions_label: Option<crate::Label>,
    pub(super) window: Option<crate::TreeWindow>,
    column_window: Option<crate::TreeWindow>,
}

/// 📊️ A table named `label` whose header reads `columns`. Its accessible name defaults to `label`.
pub fn table(label: crate::Label, columns: crate::UiFixedList<crate::Label>) -> TableBuilder {
    let mut base = NodeBase::stack(crate::Axis::Vertical);
    base.accessibility.label = Some(label.clone());
    TableBuilder { base, label, columns, row_label: None, column_label: None, actions_label: None, window: None, column_window: None }
}

impl TableBuilder {
    /// ↕️ Names the row and column domains for range status and navigation.
    pub fn axis_labels(mut self, row_label: crate::Label, column_label: crate::Label) -> Self {
        self.row_label = Some(row_label);
        self.column_label = Some(column_label);
        self
    }

    /// 🎬️ Names the trailing actions column.
    pub fn actions_label(mut self, label: crate::Label) -> Self {
        self.actions_label = Some(label);
        self
    }

    /// 🪟️ Declares which slice of the logical row list the built rows actually are — see
    /// [`crate::TreeWindow`].
    pub fn window(mut self, window: crate::TreeWindow) -> Self {
        self.window = Some(window);
        self
    }

    /// ↔️ Declares which slice of the logical column list the headers and row cells contain.
    pub fn column_window(mut self, window: crate::TreeWindow) -> Self {
        self.column_window = Some(window);
        self
    }
}

impl HasBase for TableBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for TableBuilder {}

impl HasStackLayout for TableBuilder {}

impl From<TableBuilder> for BuiltNode {
    fn from(builder: TableBuilder) -> Self {
        assemble(
            builder.base,
            crate::Component::Table(crate::TableProps {
                label: builder.label,
                columns: builder.columns,
                row_label: builder.row_label,
                column_label: builder.column_label,
                actions_label: builder.actions_label,
                window: builder.window,
                column_window: builder.column_window,
            }),
        )
    }
}

/// 📊️ One table row — `Component::TableRow`. Cells, row actions and target are props, never child
/// records. Build with [`table_row`].
pub struct TableRowBuilder {
    pub(super) base: NodeBase,
    cells: crate::UiFixedList<crate::UiText>,
    pub(super) row_actions: crate::UiFixedList<crate::RowAction>,
    pub(super) target: Option<crate::RowTarget>,
}

/// 📊️ A table row reading `cells`, positional to its table's columns.
pub fn table_row(cells: crate::UiFixedList<crate::UiText>) -> TableRowBuilder {
    TableRowBuilder { base: NodeBase::stack(crate::Axis::Horizontal), cells, row_actions: crate::UiFixedList::default(), target: None }
}

impl TableRowBuilder {
    /// 🎬️ Appends one row action — a verb on the row's [`crate::RowTarget`].
    #[expect(clippy::result_large_err, reason = "A full row-action list returns its builder and original action owner for caller-directed retirement.")]
    pub fn try_row_action(mut self, row_action: crate::RowAction) -> Result<Self, (Self, crate::RowAction)> {
        match self.row_actions.try_push(row_action) {
            Ok(()) => Ok(self),
            Err(row_action) => Err((self, row_action)),
        }
    }

    /// 🎯️ Sets the ONE target every row action and the activation of this row fire on.
    pub fn target(mut self, target: crate::RowTarget) -> Self {
        self.target = Some(target);
        self
    }
}

impl HasBase for TableRowBuilder {
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl HasChildren for TableRowBuilder {}

impl From<TableRowBuilder> for BuiltNode {
    fn from(builder: TableRowBuilder) -> Self {
        assemble(builder.base, crate::Component::TableRow(crate::TableRowProps { cells: builder.cells, row_actions: builder.row_actions, target: builder.target }))
    }
}
//#endregion 📊️Table

//#region 🖼️Image
/// 🖼️ Typestate marker: [`ImageBuilder::alt`]/[`ImageBuilder::decorative`] not yet called. An
/// `ImageBuilder<NoAlt>` has no `build()` at all — there is no inherent `build` on this state, and the
/// blanket [`Buildable`] impl only applies to `T: Into<BuiltNode>`, which `ImageBuilder<NoAlt>` is not
/// (only `ImageBuilder<HasAlt>` implements [`From`] into [`BuiltNode`]). Calling `.build()` on this
/// state is therefore a COMPILE error (`E0599: no method named build`), never a runtime panic — see
/// this module's own accessible-UI mandate in [`crate`]'s docs and the compile-fail example below.
pub struct NoAlt;

/// 🖼️ Typestate marker: [`ImageBuilder::alt`] or [`ImageBuilder::decorative`] was called, so the image
/// carries a decided accessible name (either real alt text or an explicit decorative opt-out). Only
/// `ImageBuilder<HasAlt>` implements `Into<BuiltNode>`, which is what makes [`Buildable::build`]
/// available on it and not on `ImageBuilder<NoAlt>`.
pub struct HasAlt(ImageAlt);

/// 🖼️ Whether an [`ImageBuilder`] carries real accessible text or has deliberately opted out —
/// `ImageBuilder<HasAlt>` stores this value directly in its typestate payload, so no optional runtime
/// invariant or panicking owner transition exists.
#[expect(clippy::large_enum_variant, reason = "Image accessibility typestate owns its bounded label inline without an extra allocation during construction.")]
enum ImageAlt {
    Text(crate::Label),
    Decorative,
}

/// 🖼️ An image — `Component::Image`. Build with [`image`], which returns `ImageBuilder<NoAlt>`; call
/// [`ImageBuilder::alt`] or [`ImageBuilder::decorative`] to obtain an `ImageBuilder<HasAlt>` before
/// [`Buildable::try_build`] is even callable — CLAUDE.md's accessible-UI mandate
/// makes an image's accessible name non-optional, and a **typestate**, not a runtime panic, is the
/// cheapest place to hold that line: a plugin that forgets `.alt(..)`/`.decorative()` fails to compile,
/// it never reaches a running actor to crash.
///
#[doc = concat!("```compile_fail\n", include_str!("../../🧪️tests/🚫️image-description-required/🦀️.rs"), "\n```\n\n")]
#[doc = concat!("```\n", include_str!("../../🧪️tests/🖼️image-described/🦀️.rs"), "\n```")]
pub struct ImageBuilder<State = NoAlt> {
    pub(super) base: NodeBase,
    src: crate::UiText,
    pub(super) alt: State,
}

/// 🖼️ An image loaded from `src`, in state [`NoAlt`] — call [`ImageBuilder::alt`] or
/// [`ImageBuilder::decorative`] before `.try_build()` becomes callable at all.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn image(src: crate::UiText) -> ImageBuilder<NoAlt> {
    ImageBuilder { base: NodeBase::leaf(), src, alt: NoAlt }
}

impl<State> ImageBuilder<State> {
    /// ♿️ Supplies the accessible alt text, unlocking `.try_build()` by transitioning to [`HasAlt`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn alt(self, alt: crate::Label) -> ImageBuilder<HasAlt> {
        ImageBuilder { base: self.base, src: self.src, alt: HasAlt(ImageAlt::Text(alt)) }
    }

    /// 🙈️ Explicitly opts out: the image is decorative and hidden from the accessibility tree.
    /// Unlocks `.try_build()` by transitioning to [`HasAlt`], the same as [`ImageBuilder::alt`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn decorative(self) -> ImageBuilder<HasAlt> {
        ImageBuilder { base: self.base, src: self.src, alt: HasAlt(ImageAlt::Decorative) }
    }
}

impl<State> HasBase for ImageBuilder<State> {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ImageBuilder<HasAlt>> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ImageBuilder<HasAlt>) -> Self {
        let ImageBuilder { mut base, src, alt, .. } = builder;
        let HasAlt(alt) = alt;
        let alt_label = match alt {
            ImageAlt::Text(label) => {
                base.accessibility.label.get_or_insert_with(|| label.clone());
                Some(label)
            }
            ImageAlt::Decorative => {
                base.accessibility.hidden = true;
                None
            }
        };
        assemble(base, crate::Component::Image(crate::ImageProps { src, alt: alt_label }))
    }
}
//#endregion 🖼️Image

//#region 🗺️Surface
/// 🗺️ An embedded product surface — `Component::Surface`. Build with [`surface`].
pub struct SurfaceBuilder {
    pub(super) base: NodeBase,
    pub(super) props: crate::SurfaceProps,
}

/// 🗺️ A surface carrying `props` verbatim — every field of [`crate::SurfaceProps`] is the product's
/// own concern, not something this builder second-guesses.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn surface(props: crate::SurfaceProps) -> SurfaceBuilder {
    SurfaceBuilder { base: NodeBase::leaf(), props }
}

impl HasBase for SurfaceBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

/// 👶️ A surface's children are NOT rendered chrome — a renderer draws `props` and nothing else. They
/// are the surface's own out-of-doc payload carriers: `SurfaceDoc.bytes` is [`crate::UiFixedBytes`], a
/// hard [`crate::UI_FIXED_BYTES`] ceiling that cannot page, so a scene whose payload scales with its
/// document (a world of 10 000 instances, say) can only publish by leaving that payload OUTSIDE the
/// doc, as retained text-leaf subtrees the reconciler pages and diffs per node like any other subtree.
/// The layout stays [`crate::LayoutSpec::Leaf`] precisely because these children occupy no space.
impl HasChildren for SurfaceBuilder {}

impl From<SurfaceBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: SurfaceBuilder) -> Self {
        assemble(builder.base, crate::Component::Surface(builder.props))
    }
}
//#endregion 🗺️Surface

//#region 🧩️Extension
/// 🧩️ An external plugin slot — `Component::Extension`. Build with [`extension`].
pub struct ExtensionBuilder {
    pub(super) base: NodeBase,
    extension: crate::UiText,
    pub(super) props: crate::UiValue,
}

/// 🧩️ An extension slot addressed by `name`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub fn extension(name: crate::UiText) -> ExtensionBuilder {
    ExtensionBuilder { base: NodeBase::leaf(), extension: name, props: crate::UiValue::Null }
}

impl ExtensionBuilder {
    /// 📦️ Sets the opaque props payload passed to the slot.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn props(mut self, props: crate::UiValue) -> Self {
        self.props = props;
        self
    }
}

impl HasBase for ExtensionBuilder {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn base_mut(&mut self) -> &mut NodeBase {
        &mut self.base
    }
}

impl From<ExtensionBuilder> for BuiltNode {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from(builder: ExtensionBuilder) -> Self {
        assemble(builder.base, crate::Component::Extension(crate::ExtensionProps { extension: builder.extension, props: builder.props }))
    }
}
//#endregion 🧩️Extension
//#endregion 🔖️Builder
