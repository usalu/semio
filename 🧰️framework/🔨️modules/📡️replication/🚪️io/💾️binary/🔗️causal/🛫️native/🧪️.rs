//! 🧪️ Native causal fragments match an independent Protocol Buffers-varint fixture.
use super::*;
use crate::{MutationEnvelope,ArtifactDiff,InverseMutation};
use semio_framework_replication_ids::{MutationId,ArtifactId,ActorId,SchemaId};
fn hex(text:&str)->Vec<u8>{text.trim().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}
fn law()->serde_json::Value{serde_json::from_str(include_str!("🔣️.json")).unwrap()}
fn rows(f:&serde_json::Value)->Vec<MutationEnvelope>{f["rows"].as_array().unwrap().iter().map(|row|MutationEnvelope{
    mutation_id:MutationId(row["mutationId"].as_str().unwrap().into()),document_id:ArtifactId(row["documentId"].as_str().unwrap().into()),actor:ActorId(row["actor"].as_str().unwrap().into()),dependencies:row["dependencies"].as_array().unwrap().iter().map(|value|MutationId(value.as_str().unwrap().into())).collect(),observed:row["observed"].as_str().map(|value|MutationId(value.into())),target:row["target"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().into()).collect(),diff:ArtifactDiff{schema:SchemaId(row["schema"].as_str().unwrap().into()),payload:crate::CausalPayload::try_from_slice(&hex(row["payloadHex"].as_str().unwrap())).unwrap()},inverse:InverseMutation{schema:SchemaId(row["schema"].as_str().unwrap().into()),payload:crate::CausalPayload::try_from_slice(&hex(row["inverseHex"].as_str().unwrap())).unwrap()},timestamp:HybridLogicalTimestamp{actor:row["timestamp"][0].as_str().unwrap().parse().unwrap(),physical_ms:row["timestamp"][1].as_str().unwrap().parse().unwrap(),logical:row["timestamp"][2].as_str().unwrap().parse().unwrap()},transaction:row["transaction"].as_object().map(|_|semio_framework_replication_edit::TransactionRef{id:row["transaction"]["id"].as_str().unwrap().into(),tool:row["transaction"]["tool"].as_str().unwrap().into()}),verb:row["verb"].as_str().map(Into::into),line:row["line"].as_str().map(Into::into),
}).collect()}
#[test]
fn causal_original_field_fragments_match_neutral_wire_and_refused_turns_allocate_nothing(){
    let f=law();let original=rows(&f);let expected=hex(include_str!("📬️.hex"));assert_eq!(crate::binary::encode_envelopes(&original),expected);
    for maximum in f["fragmentSizes"].as_array().unwrap(){let maximum=maximum.as_u64().unwrap()as usize;let mut cursor=CausalEnvelopeWireCursor::default();let mut actual=Vec::with_capacity(expected.len());
        for turn in 0..10000{let d=cursor.demand(maximum);assert!(d.copy_bytes<=f["quantum"].as_u64().unwrap()as usize);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_depth:1,..Default::default()};
            for axis in f["deniedAxes"].as_array().unwrap(){let mut denied=grant;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes-=1,_=>denied.maximum_depth=0}let mut output=[0xa5;64];let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&original,&mut output[..maximum],denied).unwrap());assert_eq!((born,released),(0,0));assert_eq!(step.progress,Default::default());assert_eq!(output,[0xa5;64]);}
            let mut output=[0xa5;64];let(step,born,released)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&original,&mut output[..maximum],grant).unwrap());assert_eq!((born,released),(0,0));assert!(step.progress.fits(grant));assert!(step.written_bytes<=maximum);assert!(output[step.written_bytes..].iter().all(|byte|*byte==0xa5));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break}assert!(turn<9999);
        }assert!(cursor.is_complete());assert_eq!(actual,expected);
    }
}

#[test]
fn canonical_reference_reader_reads_original_paged_wire_without_a_message_gather(){
 let f=law();let original=rows(&f);let expected=hex(include_str!("📬️.hex"));let source=crate::CausalPayload::try_from_slice(&expected).unwrap();let pointer=source.retained_bytes().backing_ptr(0).unwrap();
 let decoded=crate::binary::decode_document_backbone_envelopes_exact(&source).unwrap();assert_eq!(decoded,original);assert_eq!(source.retained_bytes().backing_ptr(0).unwrap(),pointer);
 let mut wide=rows(&f);let law=&f["source"];let pattern=law["pattern"].as_array().unwrap();let bytes:Vec<_>=(0..law["payloadBytes"].as_u64().unwrap()as usize).map(|i|pattern[i%pattern.len()].as_u64().unwrap()as u8).collect();wide[0].diff.payload=crate::CausalPayload::try_from_slice(&bytes).unwrap();let expected=crate::binary::encode_envelopes(&wide);assert_eq!(expected.len(),law["wireBytes"].as_u64().unwrap()as usize);use sha2::Digest;assert_eq!(format!("{:x}",sha2::Sha256::digest(&expected)),law["wireSha256"].as_str().unwrap());let source=crate::CausalPayload::try_from_slice(&expected).unwrap();
 struct Original<'a>(&'a crate::CausalPayload);impl semio_framework_value::ByteReadSource for Original<'_>{fn byte_len(&self)->usize{self.0.len()}fn byte_at(&self,index:usize)->Option<u8>{self.0.byte_at(index)}fn read_bytes(&self,offset:usize,output:&mut[u8])->usize{assert!(output.len()<=256,"reference receiver must not gather a whole wire or payload");semio_framework_value::ByteReadSource::read_bytes(self.0,offset,output)}}
 let decoded=crate::binary::decode_document_backbone_envelopes_exact(&Original(&source)).unwrap();assert_eq!(decoded,wide);let mut malformed=expected.clone();malformed.push(0);let source=crate::CausalPayload::try_from_slice(&malformed).unwrap();assert!(crate::binary::decode_document_backbone_envelopes_exact(&source).is_err());
}
