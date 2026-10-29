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
pub fn applied<P: Clone, M: protocol::ColdMutation<P>>(base: &P, mutation: &M) -> (P, protocol::MutationOutcome<M::ReferenceDiff>) {
    let outcome = mutation.diff(base);
    let next = protocol::apply_diff_cold(outcome.diff(), base).unwrap_or_else(|_| base.clone());
    (next, outcome)
}

/// 🧹️ Retires a subset's shared snapshot through the bounded shared-retirement admission; `semio_snapshot_retirement!`
/// gives each subset snapshot its implementation.
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SemioSnapshotRetirementFactory<P>(pub std::marker::PhantomData<fn() -> P>);

/// 🧸️ Implements one subset snapshot's retirement factory; the crate that owns the snapshot type invokes it.
#[macro_export]
macro_rules! semio_snapshot_retirement {
    ($snapshot:ty) => {
        impl ::semio_framework_os_kernel::SnapshotRetirementFactory<$snapshot> for $crate::SemioSnapshotRetirementFactory<$snapshot> {
            fn retirement_birth_bytes(&self, _snapshot: &::std::sync::Arc<$snapshot>) -> usize {
                ::semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<$snapshot>()
            }

            fn retire(
                &self,
                snapshot: ::std::sync::Arc<$snapshot>,
                grant: ::semio_framework_value::retained_clone::RetainedCloneGrant,
            ) -> Result<(Box<dyn ::semio_framework_os_kernel::ErasedSnapshotRetirement>, ::semio_framework_value::retained_clone::RetainedCloneProgress), (::semio_framework_value::ValueError, ::std::sync::Arc<$snapshot>)> {
                ::semio_framework_value::retirement::arc_custody::admit_shared_retirement(snapshot, grant, true)
            }
        }
    };
}

/// 🗝️ Implements one subset's member store owner: a retained snapshot opens by receiving its sealed pack record natively
/// into the persisted owner unless `open = <operation>` names the subset's own opening operation; the crate that owns the
/// mutation type invokes it.
pub struct SemioMemberStoreProducer<P,M>(::std::marker::PhantomData<fn()->(P,M)>);
impl<P,M> Copy for SemioMemberStoreProducer<P,M> {}
impl<P,M> Clone for SemioMemberStoreProducer<P,M> {fn clone(&self)->Self{*self}}
impl<P,M> SemioMemberStoreProducer<P,M> {pub const fn new()->Self{Self(::std::marker::PhantomData)}}

/// 🌱️ Implements the native default of snapshot types whose omitted field receives their `Default` value; it copies no bytes
/// beyond the value itself. The crate that owns each type invokes it.
#[macro_export]
macro_rules! semio_native_default {
    ($($owner:ty),+ $(,)?) => {$(
        impl ::semio_framework_value_native_owned::native_decoding::owned::NativeDefault for $owner {
            fn native_default_demand() -> ::semio_framework_value::RetirementDemand {
                ::semio_framework_value_native_owned::native_decoding::owned::tree::fixed(2 * ::std::mem::size_of::<Self>())
            }

            fn native_default(grant: ::semio_framework_value::RetainedCloneGrant) -> Result<(Self, ::semio_framework_value::RetainedCloneProgress), ::semio_framework_value::ValueError> {
                let demand = <Self as ::semio_framework_value_native_owned::native_decoding::owned::NativeDefault>::native_default_demand();
                if !::semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) {
                    return Err(::semio_framework_value_native_owned::native_decoding::owned::tree::invalid());
                }
                Ok((<Self as ::std::default::Default>::default(), ::semio_framework_value_native_owned::native_decoding::owned::tree::receipt(demand)))
            }
        }
    )+};
}

/// 📦️ Implements one subset snapshot's binary pack as its derived record document inside the semio pack envelope, so a
/// member opens by receiving that record natively; the crate that owns the snapshot invokes it.
#[macro_export]
macro_rules! semio_snapshot_record_pack {
    ($snapshot:ty) => {
        impl ::pack::snapshot::ArtifactPack for $snapshot {
            fn encode_pack_with(&self, options: &::semio_framework_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, ::semio_framework_os_kernel::os_store::PackError> {
                let inner = ::semio_framework_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = ::semio_framework_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), ::semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| ::semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
                Ok(::semio_framework_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
            }

            fn decode_pack_with(bytes: &[u8], options: &::semio_framework_os_kernel::os_store::PackDecodeOptions) -> Result<Self, ::semio_framework_os_kernel::os_store::PackError> {
                let (envelope, inner) = ::semio_framework_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| ::semio_framework_os_kernel::os_store::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity(<Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), ::semio_framework_os_kernel::os_store::semio_format::Component::Pack, 1) {
                    return Err(::semio_framework_os_kernel::os_store::PackError::from(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
                }
                let (record, _report) = ::semio_framework_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(::semio_framework_os_kernel::os_store::serialization::text_error_to_pack_error)
            }

            fn record_spec() -> Option<::semio_framework_dsl_record::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }
    };
}

#[macro_export]
macro_rules! semio_member_owner {
    ($snapshot:ty, $mutation:ty) => {
        $crate::semio_member_owner!($snapshot, $mutation, open = ::semio_framework_os_kernel::os_store::OriginalRecordMemberSnapshotOpen<$snapshot>);
    };
    ($snapshot:ty, $mutation:ty, open = $open:ty) => {
        impl ::semio_framework_os_kernel::os_store::document_store_owners::MemberStoreProducer<$snapshot,$mutation> for $crate::SemioMemberStoreProducer<$snapshot,$mutation> {
            type SnapshotOpen = $open;

            fn member_store_owners_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                Ok(::semio_framework_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<$snapshot, $mutation>()?.copy_bytes)
            }

            fn member_store_owners_birth_demand(&self) -> Result<::semio_framework_value::retained_clone::RetainedCloneBirthDemand, ::semio_framework_value::ValueError> {
                ::semio_framework_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<$snapshot, $mutation>()
            }

            fn member_store_owners(&self,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(::semio_framework_os_kernel::os_store::document_store_owners::DocumentStoreOwners<$snapshot, $mutation>, ::semio_framework_value::retained_clone::RetainedCloneProgress), ::semio_framework_os_kernel::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<$snapshot, $mutation>> {
                ::semio_framework_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners::<$snapshot, $mutation>(grant)
            }
        }
    };
}


/// 🔗️ One direction of one subset's conversions: `declare` adds the composer rows that write a semio dialect to an
/// artifact declaration, `register` publishes all of the direction's rows into the process-wide io registry.
#[derive(Clone, Copy)]
pub struct SemioConversion {
    pub declare: fn(semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>,
    pub register: fn(),
}
