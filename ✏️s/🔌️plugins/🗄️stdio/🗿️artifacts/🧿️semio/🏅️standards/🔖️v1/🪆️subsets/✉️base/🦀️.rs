//! 🧱️ Foundation of the semio artifact family: the vocabulary, codec primitives and editor plumbing every subset shares.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
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
pub fn semio_written(table: &'static [semio_plugin::io::ComposerEntry], cache: &'static std::sync::OnceLock<Vec<semio_plugin::io::ComposerEntry>>) -> &'static [semio_plugin::io::ComposerEntry] {
    cache.get_or_init(|| table.iter().filter(|entry| entry.writes.artifact_kind == SEMIO_ARTIFACT_SCHEMA_ID).map(|entry| semio_plugin::io::ComposerEntry { writes: entry.writes, reads: entry.reads, compose: entry.compose }).collect())
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
        impl ::semio_os_kernel::SnapshotRetirementFactory<$snapshot> for $crate::SemioSnapshotRetirementFactory<$snapshot> {
            fn retirement_birth_bytes(&self, snapshot: &::semio_framework_value::factory::SharedOriginalStorage<$snapshot>) -> usize {
                ::semio_framework_value::SnapshotRetirementFactory::retirement_birth_bytes(&::semio_framework_value::SharedValueRetirementFactory::<$snapshot>::default(), snapshot)
            }

            fn retire_demands(&self, snapshot: &::semio_framework_value::factory::SharedOriginalStorage<$snapshot>) -> ::std::result::Result<::semio_framework_value::RetirementDemand, ::semio_framework_value::ValueError> {
                ::semio_framework_value::SnapshotRetirementFactory::retire_demands(&::semio_framework_value::SharedValueRetirementFactory::<$snapshot>::default(), snapshot)
            }

            fn retire_work_demand(&self, snapshot: &::semio_framework_value::factory::SharedOriginalStorage<$snapshot>) -> ::std::result::Result<u64, ::semio_framework_value::ValueError> {
                ::semio_framework_value::SnapshotRetirementFactory::retire_work_demand(&::semio_framework_value::SharedValueRetirementFactory::<$snapshot>::default(), snapshot)
            }

            fn retire<'reply>(&self, snapshot: &mut ::std::option::Option<::semio_framework_value::factory::SharedOriginalStorage<$snapshot>>, effect: &mut ::semio_framework_value::RetainedAllocationEffect<'_>, reply: ::semio_framework_value::FactoryProducerReply<'reply, dyn ::semio_framework_value::ErasedSnapshotRetirement, ::semio_framework_value::OriginalRetirementFailure>) -> ::semio_framework_value::FactoryProducerReplyComplete<'reply, ::semio_framework_value::OriginalRetirementFailure> {
                ::semio_framework_value::SnapshotRetirementFactory::retire(&::semio_framework_value::SharedValueRetirementFactory::<$snapshot>::default(), snapshot, effect, reply)
            }

            fn recover_retire<'reply>(&self, snapshot: &mut ::std::option::Option<::semio_framework_value::factory::SharedOriginalStorage<$snapshot>>, effect: &mut ::semio_framework_value::RetainedAllocationEffect<'_>, reply: ::semio_framework_value::FactoryProducerReply<'reply, dyn ::semio_framework_value::ErasedSnapshotRetirement, ::semio_framework_value::OriginalRetirementFailure>) -> ::semio_framework_value::FactoryProducerReplyComplete<'reply, ::semio_framework_value::OriginalRetirementFailure> {
                ::semio_framework_value::SnapshotRetirementFactory::recover_retire(&::semio_framework_value::SharedValueRetirementFactory::<$snapshot>::default(), snapshot, effect, reply)
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
        impl ::semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue for $owner {
            fn default_demand() -> ::semio_framework_value::RetirementDemand {
                ::semio_framework_value_native_owned::native_decoding::owned::tree::fixed(2 * ::std::mem::size_of::<Self>())
            }

            fn default_work_demand()->Result<u64,::semio_framework_value::ValueError>{Ok(1)}
fn admit_default<'reply>(output:&mut Option<Self>,effect:&mut ::semio_framework_value::RetainedAllocationEffect<'_>,reply: ::semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<::semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>,::semio_framework_value::ValueError>{::semio_framework_value_native_owned::native_decoding::owned::native_admit_default(output,<Self as ::semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValue>::default_demand(),||<Self as ::std::default::Default>::default(),effect,reply)}
fn recover_default<'reply>(_:&mut Option<Self>,_:&mut ::semio_framework_value::RetainedAllocationEffect<'_>,reply: ::semio_framework_value_native_owned::native_decoding::OriginalEffectReply<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<::semio_framework_value_native_owned::native_decoding::OriginalEffectReplyComplete<'reply,::semio_framework_value::RetainedCloneStep,::semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>,::semio_framework_value::ValueError>{Ok(reply.pending())}
        }

        impl ::semio_framework_value_native_owned::native_decoding::owned::NativeDefault for $owner {
            type Cursor = ::semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor<Self>;
            fn default_cursor() -> Self::Cursor { ::semio_framework_value_native_owned::native_decoding::owned::NativeDefaultValueCursor::new() }
        }
    )+};
}

/// 🧷️ A snapshot whose binary pack is its derived record document: the sqlite native codecs read and write that document
/// body directly under their caller-owned allocation control.
pub trait SemioRecordPack: Sized {
    /// 📤️ Encodes the record document body that sits inside the semio pack envelope, charging the caller's encode control.
    fn record_binary(&self, control: &mut semio_framework_value::native_encoding::NativeEncodeControl<'_>) -> Result<Vec<u8>, semio_framework_value::ValueError>;
    /// 📥️ Receives the record document body into its typed owner under the caller's decode control and database limits.
    fn from_record_binary(body: &[u8], control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, limits: store::sqlite_snapshot::SqliteDatabaseLimits) -> Result<Self, semio_framework_value::ValueError>;
}

/// 📦️ Implements one subset snapshot's binary pack as its derived record document inside the semio pack envelope, so a
/// member opens by receiving that record natively; the crate that owns the snapshot invokes it.
#[macro_export]
macro_rules! semio_snapshot_record_pack {
    ($snapshot:ty) => {
        impl ::pack::snapshot::ArtifactPack for $snapshot {
            fn encode_pack_with(&self, options: &::semio_os_kernel::os_store::PackEncodeOptions) -> Result<Vec<u8>, ::semio_os_kernel::os_store::PackError> {
                let inner = ::semio_os_kernel::os_pack::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
                let envelope = ::semio_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(<Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), ::semio_os_kernel::os_store::semio_format::Component::Pack, 1).map_err(|error| ::semio_os_kernel::os_store::PackError::from(error.into_value_error()))?;
                Ok(::semio_os_kernel::os_store::semio_format::wrap_binary(&envelope, &inner))
            }

            fn decode_pack_with(bytes: &[u8], options: &::semio_os_kernel::os_store::PackDecodeOptions) -> Result<Self, ::semio_os_kernel::os_store::PackError> {
                let (envelope, inner) = ::semio_os_kernel::os_store::semio_format::unwrap_binary(bytes).map_err(|error| ::semio_os_kernel::os_store::PackError::from(error.into_value_error()))?;
                if !envelope.matches_identity(<Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), ::semio_os_kernel::os_store::semio_format::Component::Pack, 1) {
                    return Err(::semio_os_kernel::os_store::PackError::from(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ::pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
                }
                let (record, _report) = ::semio_os_kernel::os_pack::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
                Self::__dsl_from_record(&record).map_err(::semio_os_kernel::os_store::serialization::text_error_to_pack_error)
            }

            fn record_spec() -> Option<::semio_framework_dsl_record::RecordSpec> {
                Some(Self::__dsl_spec())
            }
        }

        impl $crate::SemioRecordPack for $snapshot {
            fn record_binary(&self, control: &mut ::semio_framework_value::native_encoding::NativeEncodeControl<'_>) -> Result<Vec<u8>, ::semio_framework_value::ValueError> {
                ::semio_os_kernel::os_pack::pack_rt::encode_document_controlled(&Self::__dsl_spec(), &self.__dsl_to_record(), &::semio_os_kernel::os_store::PackEncodeOptions::default(), control).map_err(|error| error.into_value_error())
            }

            fn from_record_binary(body: &[u8], control: &mut ::semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, limits: ::semio_os_kernel::sqlite_snapshot::SqliteDatabaseLimits) -> Result<Self, ::semio_framework_value::ValueError> {
                let (record, _report) = ::semio_os_kernel::os_pack::pack_rt::decode_document_controlled(body, &Self::__dsl_spec(), &$crate::io::sqlite::snapshot::native_decoding::record_decode_options(limits), control).map_err(|error| error.into_value_error())?;
                $crate::io::sqlite::snapshot::native_decoding::admit_record_rows(&record, control, limits)?;
                Self::__dsl_from_record_controlled(&record, control)
            }
        }
    };
}

#[macro_export]
macro_rules! semio_member_owner {
    ($snapshot:ty, $mutation:ty) => {
        $crate::semio_member_owner!($snapshot, $mutation, open = ::semio_os_kernel::os_store::OriginalRecordMemberSnapshotOpen<$snapshot>);
    };
    ($snapshot:ty, $mutation:ty, open = $open:ty) => {
        impl ::semio_os_kernel::os_store::document_store_owners::MemberStoreProducer<$snapshot,$mutation> for $crate::SemioMemberStoreProducer<$snapshot,$mutation> {
            type SnapshotOpen = $open;

            fn member_store_owners_copy_byte_demand(&self) -> Result<usize, ::semio_framework_value::ValueError> {
                Ok(::semio_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners_source_demands::<$snapshot, $mutation>()?.copy_bytes)
            }

            fn member_store_owners_birth_demand(&self) -> Result<::semio_framework_value::retained_clone::RetainedCloneBirthDemand, ::semio_framework_value::ValueError> {
                ::semio_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners_birth_demand::<$snapshot, $mutation>()
            }

            fn member_store_owners(&self,grant: ::semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(::semio_os_kernel::os_store::document_store_owners::DocumentStoreOwners<$snapshot, $mutation>, ::semio_framework_value::retained_clone::RetainedCloneProgress), ::semio_os_kernel::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<$snapshot, $mutation>> {
                ::semio_os_kernel::os_store::document_store_owners::bounded_artifact_store_owners::<$snapshot, $mutation>(grant)
            }
        }
    };
}


static NATURAL_MOUNTS: std::sync::Mutex<std::collections::BTreeMap<&'static str, Result<(), String>>> = std::sync::Mutex::new(std::collections::BTreeMap::new());

/// 🧭️ Mounts one subset's pack envelope on the natural-file dispatcher exactly once and remembers why a refusal happened; mounting again returns the first outcome.
pub fn mount_natural_pack(schema: &'static str, envelope_id: &str, format: ::semio_os_kernel::os_store::semio_format::SemioHandler) -> Result<(), String> {
    let mut mounts = NATURAL_MOUNTS.lock().map_err(|_| "the semio natural-file mount table is unavailable".to_string())?;
    if let Some(outcome) = mounts.get(schema) {
        return outcome.clone();
    }
    let outcome = ::semio_os_kernel::os_store::semio_format::SemioEnvelope::from_envelope_id(envelope_id, ::semio_os_kernel::os_store::semio_format::Component::Pack, 1).and_then(|envelope| ::semio_os_kernel::os_store::semio_format::register_format(envelope, format)).map_err(|error| error.to_string());
    mounts.insert(schema, outcome.clone());
    outcome
}

/// 🚧️ The refusal a natural-file call reports when its subset is not mounted: the mount's own reason when registration was refused, otherwise that the subset editor was never mounted (English and German).
pub fn natural_mount_refusal(schema: &'static str) -> Option<semio_plugin::MediaError> {
    let outcome = NATURAL_MOUNTS.lock().ok().and_then(|mounts| mounts.get(schema).cloned());
    let (english, german) = match outcome {
        Some(Ok(())) => return None,
        Some(Err(reason)) => (format!("subset {schema} could not be mounted: {reason}"), format!("Teilmenge {schema} konnte nicht eingebunden werden: {reason}")),
        None => (format!("subset editor {schema} is not mounted"), format!("Teilmengen-Editor {schema} ist nicht eingebunden")),
    };
    Some(semio_plugin::MediaError::Payload("artifact:native".into(), format!("[semio.natural-file.unmounted] {english} / {german}")))
}

/// 📄️ The `.semio` natural-file codec of one subset: Open and Save route through the semio dispatcher. The editor mounts its pack envelope once, when its codec is first read; the codec is always declared, and a refused or missing mount is reported by every encode and decode with its real reason.
#[macro_export]
macro_rules! semio_natural_file {
    ($snapshot:ty, $schema:literal) => {
        fn natural_file_codec() -> Option<semio_plugin::NaturalFileCodec> {
            static FORMAT: ::semio_os_kernel::os_store::semio_format::SemioFormat = ::semio_os_kernel::os_store::semio_format::SemioFormat {
                token: concat!($schema, "#natural-pack-v1"),
                verify: |bytes| <$snapshot as ::pack::snapshot::ArtifactPack>::decode_pack(bytes).map(|_| ()).map_err(|error| error.to_string()),
            };
            let _ = $crate::mount_natural_pack($schema, <$snapshot as ::pack::snapshot::ArtifactDsl>::envelope_id(), &FORMAT);
            Some(semio_plugin::NaturalFileCodec { format_kind: concat!($schema, "@v1"), extension: ".semio", media_type: "application/vnd.semio", binary: true })
        }

        fn encode_natural_file(snapshot: &Self::Snapshot) -> Result<Vec<u8>, semio_plugin::MediaError> {
            if let Some(refusal) = $crate::natural_mount_refusal($schema) {
                return Err(refusal);
            }
            <$snapshot as ::pack::snapshot::ArtifactPack>::encode_pack_with(snapshot, &::semio_os_kernel::os_store::PackEncodeOptions::default()).map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
        }

        fn decode_natural_file(bytes: &[u8]) -> Result<Self::Snapshot, semio_plugin::MediaError> {
            if let Some(refusal) = $crate::natural_mount_refusal($schema) {
                return Err(refusal);
            }
            ::semio_os_kernel::os_store::semio_format::dispatch(bytes).map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))?;
            <$snapshot as ::pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| semio_plugin::MediaError::Payload("artifact:native".into(), error.to_string()))
        }
    };
}

/// 🔗️ One direction of one subset's conversions: `declare` adds the composer rows that write a semio dialect to an
/// artifact declaration, `register` publishes all of the direction's rows into the process-wide io registry.
#[derive(Clone, Copy)]
pub struct SemioConversion {
    pub declare: fn(semio_plugin::app::ArtifactDeclarationBuilder<semio_plugin::app::DeclarationReady>) -> semio_plugin::app::ArtifactDeclarationBuilder<semio_plugin::app::DeclarationReady>,
    pub register: fn(),
}
