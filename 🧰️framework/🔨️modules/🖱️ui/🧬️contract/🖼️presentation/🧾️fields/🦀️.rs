//! 🧬️ Canonical presentation field order for typed ownership operations.
#[macro_export]
macro_rules! ui_presentation_field_catalog {
    ($visitor:ident) => {
        $visitor!(AccessibilitySpec { 0 => label: Option<Label>, 1 => description: Option<Label>, 2 => live: Liveness, 3 => shortcut: Option<UiText>, 4 => hidden: bool });
        $visitor!(SurfaceProps { 0 => kind: SurfaceKind, 1 => doc_schema: UiText, 2 => doc: SurfaceDoc, 3 => bindings: UiNodeBindings });
        $visitor!(SurfaceDoc { 0 => bytes: UiFixedBytes });
        $visitor!(GridLayout { 0 => columns: UiGridTracks, 1 => rows: UiGridTracks, 2 => column_gap: SpaceToken, 3 => row_gap: SpaceToken, 4 => padding: EdgeSpace, 5 => align: Align, 6 => justify: Justify });
        $visitor!(StackLayout { 0 => axis: Axis, 1 => gap: SpaceToken, 2 => padding: EdgeSpace, 3 => align: Align, 4 => justify: Justify, 5 => grow: bool, 6 => wrap: bool });
        $visitor!(OverlayLayout { 0 => anchor: Anchor, 1 => inset: EdgeSpace, 2 => dismissible: bool });
        $visitor!(ScrollLayout { 0 => axes: ScrollAxes, 1 => padding: EdgeSpace, 2 => sizing: Sizing });
        $visitor!(AbsoluteLayout { 0 => sizing_width: Sizing, 1 => sizing_height: Sizing });
        $visitor!(LeafLayout { 0 => width: Sizing, 1 => height: Sizing });
    };
}
