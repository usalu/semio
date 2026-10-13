//! 🧱️ Foundation of the semio artifact family: the vocabulary, codec primitives and editor plumbing every subset shares.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🪆️child/🦀️.rs"]
    pub mod child;
    #[path = "🧬️schema/🧮️geometry/🦀️.rs"]
    pub mod geometry;
    #[path = "🧬️schema/🧰️triples/🦀️.rs"]
    pub mod triples;
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🎯️outcome/🦀️.rs"]
        pub mod outcome;
    }
}

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod text {
        #[path = "🚪️io/📝️text/🔤️codec/🦀️.rs"]
        pub mod codec;
    }
    #[path = "."]
    pub mod binary {
        #[path = "🚪️io/💾️binary/🔤️codec/🦀️.rs"]
        pub mod codec;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "."]
        pub mod snapshot {
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/📏️native/🦀️.rs"]
            pub mod native;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🛬️native/🦀️.rs"]
            pub mod native_decoding;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🛫️native/🦀️.rs"]
            pub mod native_encoding;
            #[path = "🚪️io/🪶️sqlite/📸️snapshot/🫙️projection/🦀️.rs"]
            pub mod projection;
        }
    }
}

#[path = "."]
pub mod editor {
    #[path = "✏️editor/🧭️edit-plumbing/🦀️.rs"]
    pub mod edit_plumbing;
}

#[path = "."]
pub mod examples {
    #[path = "📚️examples/🏢️building/🦀️.rs"]
    pub mod building;
    #[path = "📚️examples/🎥️clip/🦀️.rs"]
    pub mod clip;
    #[path = "📚️examples/📽️deck/🦀️.rs"]
    pub mod deck;
    #[path = "📚️examples/🎬️demo/🦀️.rs"]
    pub mod demo;
    #[path = "📚️examples/📐️drawing/🦀️.rs"]
    pub mod drawing;
    #[path = "📚️examples/🌐️envelope/🦀️.rs"]
    pub mod envelope;
    #[path = "📚️examples/🕸️graph/🦀️.rs"]
    pub mod graph;
    #[path = "📚️examples/🗒️memo/🦀️.rs"]
    pub mod memo;
    #[path = "📚️examples/📃️note/🦀️.rs"]
    pub mod note;
    #[path = "📚️examples/🌊️pipeline/🦀️.rs"]
    pub mod pipeline;
    #[path = "📚️examples/🖍️sketch/🦀️.rs"]
    pub mod sketch;
    #[path = "📚️examples/🧊️solid/🦀️.rs"]
    pub mod solid;
    #[path = "📚️examples/🖼️swatch/🦀️.rs"]
    pub mod swatch;
    #[path = "📚️examples/🎵️tone/🦀️.rs"]
    pub mod tone;
    #[path = "📚️examples/🚶️walk/🦀️.rs"]
    pub mod walk;
}

/// 🧬️ Artifact schema descriptor id of the semio artifact.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

/// 🎹️ The rows of `table` that write a `s.stdio.semio` dialect, copied once into `cache`. A composer's capability claims the
/// dialect it writes and a dialect belongs to exactly one kind, so only these rows are this artifact's to declare; the bridge
/// serializers that write another kind (semio → step, png, …) are that kind's, and stay reachable only through `register()`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_written(table: &'static [semio_framework_plugin::io::ComposerEntry], cache: &'static std::sync::OnceLock<Vec<semio_framework_plugin::io::ComposerEntry>>) -> &'static [semio_framework_plugin::io::ComposerEntry] {
    cache.get_or_init(|| table.iter().filter(|entry| entry.writes.artifact_kind == SEMIO_ARTIFACT_SCHEMA_ID).map(|entry| semio_framework_plugin::io::ComposerEntry { writes: entry.writes, reads: entry.reads, compose: entry.compose }).collect())
}

/// 🧪️ Diff then central apply: the next snapshot (the base when the diff is refused) and the mutation's own outcome.
pub fn applied<P: Clone, M: protocol::Mutation<P>>(base: &P, mutation: &M) -> (P, protocol::MutationOutcome<M::Diff>) {
    let outcome = mutation.diff(base);
    let next = protocol::apply_diff(outcome.diff(), base).unwrap_or_else(|_| base.clone());
    (next, outcome)
}

/// 🧹️ Retires a subset's shared snapshot through the bounded shared-retirement admission; `semio_member_owner!` gives
/// each subset snapshot its implementation.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SemioSnapshotRetirementFactory<P>(pub std::marker::PhantomData<fn() -> P>);

/// 🧸️ Implements one subset's snapshot retirement factory and member store owner: retained snapshots open through
/// the framework's bounded pack decoder unless `open = <decoder>` names the subset's own streaming decoder.
#[macro_export]
macro_rules! semio_member_owner {
    ($snapshot:ty, $mutation:ty) => {
        $crate::semio_member_owner!($snapshot, $mutation, open = ::semio_framework_os_kernel::PackMemberSnapshotOpen<$snapshot>);
    };
    ($snapshot:ty, $mutation:ty, open = $open:ty) => {
        impl ::semio_framework_os_kernel::SnapshotRetirementFactory<$snapshot> for $crate::SemioSnapshotRetirementFactory<$snapshot> {
            fn retirement_birth_bytes(&self, _snapshot: &::std::sync::Arc<$snapshot>) -> usize {
                ::semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<$snapshot>()
            }

            fn retire(
                &self,
                snapshot: ::std::sync::Arc<$snapshot>,
                grant: ::semio_framework_value::retained_clone::RetainedCloneGrant,
            ) -> Result<(Box<dyn ::semio_framework_os_kernel::ErasedSnapshotRetirement>, ::semio_framework_value::retained_clone::RetainedCloneProgress), (::semio_framework_value::ValueError, ::std::sync::Arc<$snapshot>)> {
                ::semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
            }
        }

        impl ::semio_framework_os_kernel::os_store::document_store_owners::MemberStoreOwner<$mutation> for $snapshot {
            type SnapshotOpen = $open;

            fn member_store_owners_birth_demand() -> Result<::semio_framework_value::retained_clone::RetainedCloneBirthDemand, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<Self, $mutation>()
            }

            fn member_store_owners(grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(::semio_framework_os_kernel::os_store::document_store_owners::DocumentStoreOwners<Self, $mutation>, ::semio_framework_value::retained_clone::RetainedCloneProgress), ::semio_framework_os_kernel::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self, $mutation>> {
                ::semio_framework_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners::<Self, $mutation>(grant)
            }
        }
    };
}

semio_framework_value::artifact_retire_leaf!(schema::geometry::SemioPoint2, schema::geometry::SemioPoint3, schema::geometry::SemioQuaternion, schema::geometry::SemioRgba, schema::geometry::SemioTransform, schema::geometry::SemioUv);

/// 🔗️ One direction of one subset's conversions: `declare` adds the composer rows that write a semio dialect to an
/// artifact declaration, `register` publishes all of the direction's rows into the process-wide io registry.
#[derive(Clone, Copy)]
pub struct SemioConversion {
    pub declare: fn(semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>,
    pub register: fn(),
}
