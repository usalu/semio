//! 🌱️ Store native ArtifactPack genesis admission capability.
use crate::os_store::{ArtifactPack,PackEncodeOptions};
use crate::os_vcs::io::binary::genesis::ArtifactGenesisCodec;
use semio_framework_value::ValueError;
impl<P: ArtifactPack> ArtifactGenesisCodec for P {
    fn encode_genesis_pack(&self) -> Result<Vec<u8>, ValueError> {
        self.encode_pack_with(&PackEncodeOptions::default()).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
    fn decode_genesis_pack(pack: &[u8]) -> Result<Self, ValueError> {
        Self::decode_pack(pack).map_err(|error| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))
    }
}

use crate::os_store::{OwnedSchemaToken,OwnedSchemaTokenKind,OwnedSchemaDecodeDiagnostic,OwnedSchemaRecordCursor,ArtifactEnvelopeFreshVcsAuthority};
pub(super) struct ArtifactGenesisPackCapture {
    pub(super) operation: semio_framework_job::OperationId,
    pub(super) generation: semio_framework_job::Generation,
    pub(super) token: OwnedSchemaToken,
    pub(super) relative: usize,
    pub(super) bytes: Vec<u8>,
    pub(super) hasher: semio_framework_hash::Hasher,
    pub(super) complete: bool,
    pub(super) high: Option<u8>,
}

impl ArtifactGenesisPackCapture {
    pub(super) fn admit<P>(self,snapshot:P)->crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>{let digest=*self.hasher.finalize().as_bytes();crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis::from_verified_pack(snapshot,self.bytes,digest)}
    pub(super) fn new(token: OwnedSchemaToken, cx: &semio_framework_job::StepContext<'_>) -> Result<Self, OwnedSchemaDecodeDiagnostic> {
        let extent = token.end.saturating_sub(token.start).saturating_sub(2) as usize;
        if token.kind != OwnedSchemaTokenKind::String || extent == 0 || extent & 1 != 0 { return Err(ArtifactEnvelopeFreshVcsAuthority::<(), ()>::diagnostic("artifact-envelope.genesis-pack-hex")); }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(extent / 2).map_err(|_| ArtifactEnvelopeFreshVcsAuthority::<(), ()>::diagnostic("artifact-envelope.genesis-pack-capacity"))?;
        Ok(Self { operation: cx.operation(), generation: cx.generation(), token, relative: 1, bytes, hasher: semio_framework_hash::Hasher::new(), complete: false, high: None })
    }
    pub(super) fn step(&mut self, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation || cx.is_cancelled() { return Err(ArtifactEnvelopeFreshVcsAuthority::<(), ()>::diagnostic("artifact-envelope.genesis-pack-stale")); }
        cx.set_stage("artifact-genesis-pack");
        while !self.complete && !cx.should_yield() && cx.fuel_remaining() != 0 {
            if self.token.start + self.relative as u64 + 1 >= self.token.end { self.complete = true; break; }
            let mut pair = [0; 1];
            if source.copy_token_bytes(self.token, self.relative, &mut pair) != 1 { return Err(ArtifactEnvelopeFreshVcsAuthority::<(), ()>::diagnostic("artifact-envelope.genesis-pack-truncated")); }
            let nibble = |byte| match byte { b'0'..=b'9' => Some(byte-b'0'), b'a'..=b'f' => Some(byte-b'a'+10), _ => None };
            let Some(value) = nibble(pair[0]) else { return Err(ArtifactEnvelopeFreshVcsAuthority::<(), ()>::diagnostic("artifact-envelope.genesis-pack-hex")); };
            if let Some(high) = self.high.take() { let byte = high << 4 | value; self.bytes.push(byte); self.hasher.update(&[byte]); } else { self.high = Some(value); }
            self.relative += 1;
            cx.consume_fuel(1);
        }
        Ok(self.complete)
    }
}
