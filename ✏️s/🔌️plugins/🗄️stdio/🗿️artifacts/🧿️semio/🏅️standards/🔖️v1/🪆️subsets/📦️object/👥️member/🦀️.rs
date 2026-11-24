//! 👥️ The typed member store of the `object` subset of the `stdio.semio` artifact. One subset is one crate so the typed stores of the whole artifact are never instantiated in a single compilation unit; see the master plan's "Architecture > The semio artifact" section.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;

dsl::space_members! {
    native_producer:&::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer,
    history_limits: ::semio_s_artifact_stdio_semio_base::SEMIO_MEMBER_HISTORY_LIMITS,
    maximum_history_allocation_bytes: ::semio_s_artifact_stdio_semio_base::SEMIO_MEMBER_HISTORY_ALLOCATION_BYTES,
    pub enum SemioObjectMember, SemioObjectMemberOpen {
        Object("s.stdio.semio", "v1", "object", "stdio.semio") => (semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot, semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation, ::semio_s_artifact_stdio_semio_base::SemioMemberStoreProducer<semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot,semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation>, ::semio_s_artifact_stdio_semio_base::SemioMemberStoreProducer::<semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot,semio_s_artifact_stdio_semio_object::schema::mutations::SemioObjectMutation>::new()),
    }
}
