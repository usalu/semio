//! 🧬️ Exports every wire-facing type of `semio-framework-ui-contract` to TypeScript via ts-rs'
//! `TS::export_all_to`, writing each type plus its transitive dependencies into
//! `$CARGO_MANIFEST_DIR/bindings` — a scratch directory `📜️script.ts`'s `generate` command
//! consolidates into `🛂️manifest/🤖️generated/🟦️ui-contract.ts` and then deletes.
//!
//! A genuine `tests/*.rs` integration test: it depends on `semio-framework-ui-contract` as an
//! ordinary external crate, so it never touches the crate's own `🦀️*.rs` files — those belong to a
//! different packet (`ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`, packet
//! `manifest-typegen`'s registrar-requests ask for `#[ts(export_to = …)]` there instead; this test is
//! the "meanwhile" workaround named in that packet's brief). Entirely gated behind the optional
//! `typegen` feature, so a plain `cargo test`/`cargo check` never links `ts-rs`.
//!
//! Every call below is redundant with `export_all_to`'s own transitive-dependency walk — a handful of
//! root types (e.g. [`contract::UiSnapshot`], [`contract::UiPatch`], [`contract::BuiltNode`]) would
//! already reach the rest — but listing every `#[derive(TS)]` type explicitly means completeness
//! never depends on which types happen to be reachable from a chosen root, and a type accidentally
//! dropped from every root's dependency graph still gets caught by its own line here.
#![cfg(feature = "typegen")]

use semio_framework_ui_contract as contract;
use ts_rs::TS;

macro_rules! export_all {
    ($dir:expr, $($ty:ty),+ $(,)?) => {
        $(
            <$ty as TS>::export_all_to($dir)
                .unwrap_or_else(|error| panic!("export {} -> {:?}: {error}", stringify!($ty), $dir));
        )+
    };
}

#[test]
fn exports_typescript_bindings() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("bindings");

    export_all!(
        &dir,
        // 🦀️accessibility.rs
        contract::Liveness,
        contract::AccessibilitySpec,
        // 🦀️action.rs
        contract::ActionId,
        contract::Trigger,
        contract::ActionBinding,
        contract::MenuRef,
        contract::UiIntent,
        contract::UiValue,
        // 🦀️builder.rs
        contract::BuiltNode,
        // 🦀️component.rs
        contract::Label,
        contract::ContainerRole,
        contract::InputKind,
        contract::RowActionPlacement,
        contract::DropOverlaySpec,
        contract::SelectItem,
        contract::KeyValueEntry,
        contract::RowAction,
        contract::ContainerProps,
        contract::TextProps,
        contract::ButtonProps,
        contract::SeparatorProps,
        contract::InputProps,
        contract::SelectProps,
        contract::ToggleProps,
        contract::KeyValueListProps,
        contract::SliderProps,
        contract::NumberStepperProps,
        contract::RingProps,
        contract::IconSelectProps,
        contract::TreeProps,
        contract::TreeSectionProps,
        contract::TreeItemProps,
        contract::ImageProps,
        contract::ExtensionProps,
        contract::Component,
        // 🦀️document.rs
        contract::SurfaceId,
        contract::UiNodeId,
        contract::UiRevision,
        contract::TransitionHint,
        contract::UiNodeRecord,
        contract::UiSnapshot,
        contract::UiPatchOp,
        contract::UiPatch,
        // 🦀️layout.rs
        contract::SpaceToken,
        contract::Sizing,
        contract::Axis,
        contract::Align,
        contract::Justify,
        contract::GridTrack,
        contract::ScrollAxes,
        contract::Anchor,
        contract::EdgeSpace,
        contract::StackLayout,
        contract::GridLayout,
        contract::OverlayLayout,
        contract::ScrollLayout,
        contract::AbsoluteLayout,
        contract::LeafLayout,
        contract::LayoutSpec,
        contract::WindowStackCorner,
        contract::WindowLayoutNode,
        contract::WindowLayout,
        // 🦀️limits.rs
        contract::UiDocumentLimits,
        contract::UiContractViolation,
        contract::PatchRejection,
        contract::QuotaKind,
        // 🦀️presence.rs
        contract::Activity,
        contract::PeerMark,
        contract::OwnPresence,
        contract::PresenceUpdate,
        // 🦀️style.rs
        contract::Variant,
        contract::SizeToken,
        contract::Density,
        contract::Tone,
        contract::Emphasis,
        contract::StyleSpec,
        // 🦀️surface.rs
        contract::SurfaceKind,
        contract::SurfaceDoc,
        contract::SurfaceProps,
    );
}
