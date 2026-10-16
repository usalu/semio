//! 🌱️ Facade-owned exported macros cut out of the WP1 trees: `transient_root!` and `window_transient_owners!` (`🪟️window/🫧️transient`).
//!
//! 🧭️ WP1 pre-created this empty module (PLAN-PLUGIN-CRATES.md, section 12.5.6): when the crate that receives the defining file is carved, WP1 moves the `#[macro_export]` definitions here verbatim, so `$crate::` keeps naming the facade.

//#region 🔖️TransientRoot
/// 🫧️ Declares a whole-root transient state's one mutation and codecs (audit K3: the ~150 lines flow, fem, remodel, cad,
/// lowpoly, layout, forms, draw, raster, wfc each re-wrote): `$mutation::Snapshot { transient }` (wire
/// `{"kind":"snapshot","transient":…}`) replacing the whole root, its leaf descriptor (`$owner`, `$kind`, `$display`,
/// `$schema`), its sparse per-field diff `$diff` (`fields` lists the state's fields, see [`sparse_record_diff!`]: the slots where the
/// requested root differs from the base) and inverse (the root before), JSON op text and binary, and the state's DSL and pack in the semio envelope
/// `$envelope` (`$extension`). The state stays the plugin's own type: `Clone + Default + PartialEq + ToValue + FromValue`. An
/// artifact-level transient stops here; a window transient adds [`window_transient_owners!`].
///
/// 🎯️ The one exception to "no whole-record mutations": a transient root keeps its single `Snapshot { transient }` mutation. The
/// root is ephemeral and published through the ephemeral ownership transfer ([`window_transient_transfer!`]), which moves a whole
/// root (`footprint` sizes it, `into_state` moves it out), so per-field set variants cannot be transferred. Its diff is still the
/// sparse `$diff::changing(base, root)`, and its inverse restores the base root, whose diff sums to exactly the negative diff.
/// Persisted configs have no such variant (`config_record!` generates per-field set mutations only).
#[macro_export]
macro_rules! transient_root {
    (
        state: $state:ident,
        mutation: $mutation:ident,
        diff: $diff:ident,
        owner: $owner:literal,
        kind: $kind:literal,
        display_name: $display:literal,
        payload_schema: $schema:literal,
        envelope: $envelope:literal,
        extension: $extension:literal,
        $($fields:tt)+
    ) => {
        $crate::__kernel::sparse_record_diff! { record: $state, diff: $diff, $($fields)+ }

        #[doc = concat!("🫧️ The one mutation of [`", stringify!($state), "`]: replace the whole root.")]
        #[derive(Clone, Debug, PartialEq, $crate::ToValue, $crate::FromValue, $crate::__value::RetireOwned)]
        #[value(tag = "kind", rename_all = "kebab-case")]
        pub enum $mutation {
            Snapshot { transient: $state },
        }

        impl $crate::__kernel::Mutation<$state> for $mutation {
            type Diff = $diff;

            const DESCRIPTORS: &'static [$crate::__kernel::MutationLeafDescriptor] = &[$crate::__kernel::MutationLeafDescriptor {
                schema_version: 1,
                owner: $owner,
                semantic_kind: $kind,
                display_name: $display,
                emoji: "🫧️",
                aggregate_variant: "Snapshot",
                payload_schema: $schema,
                text_opcode: None,
                binary_tag: None,
                invertibility: $crate::__kernel::MutationInvertibility::ExplicitMutation,
                diff_participation: $crate::__kernel::MutationDiffParticipation::Detect,
                outcome_classes: &[$crate::__kernel::MutationOutcomeClass::Applied],
                composition: $crate::__kernel::MutationComposition::Atomic,
                required_language_surfaces: &[$crate::__kernel::MutationLanguageSurface::Rust, $crate::__kernel::MutationLanguageSurface::JsonSchema],
            }];

            fn descriptor(&self) -> &'static $crate::__kernel::MutationLeafDescriptor {
                &Self::DESCRIPTORS[0]
            }

            fn diff(&self, base: &$state) -> $crate::__kernel::MutationOutcome<$diff> {
                let Self::Snapshot { transient } = self;
                let changed = <$diff>::changing(base, transient);
                if changed == <$diff as Default>::default() {
                    return $crate::__kernel::MutationOutcome::empty();
                }
                $crate::__kernel::MutationOutcome::new(changed)
            }

            fn inverse(&self, base: &$state) -> Result<Vec<Self>, $crate::__value::ValueError> {
                Ok(vec![Self::Snapshot { transient: base.clone() }])
            }
        }

        impl $crate::__kernel::OpText for $mutation {
            fn parse_op(line: &str) -> Result<Self, $crate::__diagnostic::TextError> {
                $crate::__pack_json::from_json_str(line, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__diagnostic::TextError::from_value_error(error, $crate::__diagnostic::TextSpan::at(1, 1)))
            }

            fn print_op(&self) -> String {
                $crate::__pack_json::to_json_string(self)
            }
        }

        impl $crate::__kernel::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::__kernel::ProtocolError> {
                Ok($crate::__pack_json::to_json_string(self).into_bytes())
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::__kernel::ProtocolError> {
                let text = std::str::from_utf8(bytes).map_err(|error| $crate::__kernel::ProtocolError::from($crate::__kernel::PackError::from($crate::__value::ValueError::from(error))))?;
                $crate::__pack_json::from_json_str(text, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__kernel::ProtocolError::from($crate::__kernel::PackError::from(error)))
            }
        }

        impl pack::snapshot::ArtifactDsl for $state {
            const EXTENSION: &'static str = $extension;

            fn envelope_id() -> &'static str {
                $envelope
            }

            fn parse_dsl(text: &str) -> Result<Self, $crate::__diagnostic::TextError> {
                let body = $crate::__kernel::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
                if body.trim().is_empty() {
                    return Ok(Self::default());
                }
                $crate::__pack_json::from_json_str(body, $crate::__pack_json::JsonMemberPolicy::Reject).map_err(|error| $crate::__diagnostic::TextError::from_value_error(error, $crate::__diagnostic::TextSpan::at(1, 1)))
            }

            fn print_dsl(&self) -> String {
                let envelope = $crate::__kernel::semio_format::SemioEnvelope::from_envelope_id($envelope, $crate::__kernel::semio_format::Component::Dsl, 1).expect(concat!("valid ", $envelope, " envelope"));
                $crate::__kernel::semio_format::wrap_text(&envelope, &$crate::__pack_json::to_json_string(self))
            }
        }

        impl pack::snapshot::ArtifactPack for $state {
            fn encode_pack_with(&self, _options: &$crate::__kernel::PackEncodeOptions) -> Result<Vec<u8>, $crate::__kernel::PackError> {
                let envelope = $crate::__kernel::semio_format::SemioEnvelope::from_envelope_id($envelope, $crate::__kernel::semio_format::Component::Pack, 1).map_err(|error| $crate::__kernel::PackError::from(error.into_value_error()))?;
                Ok($crate::__kernel::semio_format::wrap_binary(&envelope, $crate::__pack_json::to_json_string(self).as_bytes()))
            }

            fn decode_pack_with(bytes: &[u8], _options: &$crate::__kernel::PackDecodeOptions) -> Result<Self, $crate::__kernel::PackError> {
                if bytes.is_empty() {
                    return Ok(Self::default());
                }
                let (envelope, inner) = $crate::__kernel::semio_format::unwrap_binary(bytes).map_err(|error| $crate::__kernel::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity($envelope, $crate::__kernel::semio_format::Component::Pack, 1) {
                    return Err($crate::__kernel::PackError::from($crate::__value::ValueError::new($crate::__value::ValueRefusalKind::InvalidValue, concat!($envelope, " pack envelope mismatch"))));
                }
                let text = std::str::from_utf8(&inner).map_err(|error| $crate::__kernel::PackError::from($crate::__value::ValueError::from(error)))?;
                $crate::__pack_json::from_json_str(text, $crate::__pack_json::JsonMemberPolicy::Reject).map_err($crate::__kernel::PackError::from)
            }
        }
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for $state {

}
    };
}

/// 🪟️ Declares the window-transient owners of one whole-root state ([`transient_root!`]) — its ephemeral transfer
/// ([`window_transient_transfer!`]), one [`WindowTransientOwner`] per window kind — and, in the invoking module, `register`
/// (every owner), `from_snapshot` / `current` (the window's root, else the default) and `addressed` (the mutation installing a
/// root in the view's own window, refused `window-transient.window-required` / `.window-stale` / `.kind-unknown`), audit K3.
#[macro_export]
macro_rules! window_transient_owners {
    (state: $state:ident, mutation: $mutation:ident, windows: { $($owner:ident => $window:expr),+ $(,)? } $(,)?) => {
        $crate::window_transient_transfer! { state: $state, mutation: $mutation }

        $(
            #[doc = concat!("🪟️ The [`", stringify!($state), "`] owner of window kind `", stringify!($window), "`.")]
            pub struct $owner;

            impl $crate::WindowTransientOwner for $owner {
                const WINDOW_KIND_ID: &'static str = $window;
                type State = $state;
                type Mutation = $mutation;

                fn build_owners() -> $crate::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                    let state = std::sync::Arc::new($crate::__value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                    let mutation = std::sync::Arc::new($crate::__value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                    let preparation = std::sync::Arc::new($crate::__kernel::ArtifactEphemeralTransferPreparationFactory::new($mutation::footprint, $mutation::into_state, state.clone(), mutation.clone()));
                    $crate::WindowTransientOwnerBundle::new(preparation, state, mutation)
                }
            }
        )+

        #[doc = "🗂️ Registers every window-transient owner of this state."]
        pub fn register(registry: &mut $crate::WindowTransientOwnerRegistry) -> Result<(), $crate::Fault> {
            $(registry.register::<$owner>()?;)+
            Ok(())
        }

        #[doc = "📖️ The root a window-transient snapshot holds for this state, else the default root."]
        pub fn from_snapshot(snapshot: Option<&$crate::WindowTransientSnapshot>) -> $state {
            snapshot.and_then(|snapshot| None $(.or_else(|| snapshot.get::<$owner>()))+).cloned().unwrap_or_default()
        }

        #[doc = "📖️ The root of the window a transient view reads, else the default root."]
        pub fn current<T>(view: &$crate::TransientView<'_, T>) -> $state {
            from_snapshot(view.window)
        }

        #[doc = "📬️ The mutation installing `transient` as the root of the view's own window."]
        pub fn addressed(view: &$crate::ViewModel, transient: $state) -> Result<$crate::WindowTransientMutation, $crate::Fault> {
            let refuse = |code: &str, message: &str| $crate::Fault::new($crate::FaultOrigin::Framework, $crate::FaultCode::new(code), message);
            let id = view.window_id.as_deref().ok_or_else(|| refuse("window-transient.window-required", "a window transient needs the view's own window"))?;
            let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| refuse("window-transient.window-stale", "the view's window is absent from its window roster"))?;
            let mutation = $mutation::Snapshot { transient };
            $(
                if kind == <$owner as $crate::WindowTransientOwner>::WINDOW_KIND_ID {
                    return Ok($crate::WindowTransientMutation::of::<$owner>(id, mutation));
                }
            )+
            Err(refuse("window-transient.kind-unknown", "the view's window kind holds no transient of this state"))
        }
    };
}
//#endregion 🔖️TransientRoot
