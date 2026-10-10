import type { ClientFrame, ServerFrame, WireLane, ExactWireMutationEnvelope, WireArtifactBootstrap, WireBootstrap, WireApplyOutcome, WireAckStage, WireRebootstrapRequired } from "../../../🟦️.ts";
import { writeVarintU64, readVarintU64, writeStr, readStr, writeBytes, readBytes, writeHash32, readHash32, writeBool, readBool, writeVecBytes, readVecBytes } from "../🟦️.ts";
import { writeOptStr, readOptStr, writeOptBytes, readOptBytes, writeOptFrontier, readOptFrontier, writeVecEnvelope, readVecEnvelope, encodeEnvelope, encodeExactEnvelope, decodeEnvelope, encodeFrontier, decodeFrontier } from "../🔗️causal/🟦️.ts";
import { DocumentBackboneBatchError, readDocumentBackboneEnvelopeBatchExact } from "../🔗️causal/🧮️backbone/🟦️.ts";
import { readU8 } from "../👥️presence/🟦️.ts";
import { artifactBootstrapError, ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, ARTIFACT_BOOTSTRAP_CHUNK_BYTES, validateArtifactBootstrapHeader, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, validateArtifactBootstrap } from "../../📥️bootstrap/🟦️.ts";

//#region 🔖️NestedEnums
function encodeArtifactBootstrap(out: number[], bootstrap: WireArtifactBootstrap): void {
  writeVarintU64(out, bootstrap.format_version);
  writeHash32(out, bootstrap.descriptor_hash);
  writeStr(out, bootstrap.artifact_schema);
  writeStr(out, bootstrap.artifact_kind);
  writeHash32(out, bootstrap.pack_schema_hash);
  encodeFrontier(out, bootstrap.baseline_frontier);
  writeHash32(out, bootstrap.pack_hash);
  writeHash32(out, bootstrap.spr_hash);
  writeVarintU64(out, bootstrap.pack_length);
  writeVarintU64(out, bootstrap.spr_length);
  writeVarintU64(out, bootstrap.chunk_count);
  writeHash32(out, bootstrap.aggregate_hash);
  encodeFrontier(out, bootstrap.required_tail_frontier);
  writeBool(out, bootstrap.inline !== null);
  if (bootstrap.inline !== null) {
    writeBytes(out, bootstrap.inline.pack);
    writeBytes(out, bootstrap.inline.spr);
  }
}

function readArtifactBootstrapString(bytes: Uint8Array, pos: [number], name: string): string {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length === 0 || length > 256) throw artifactBootstrapError(`${name} length is invalid`);
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError(`${name} is truncated`);
  pos[0] += length;
  return new TextDecoder("utf-8", { fatal: true }).decode(slice);
}

function readArtifactBootstrapBytes(bytes: Uint8Array, pos: [number], expected: number, name: string): number[] {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length !== expected || length > ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES) throw artifactBootstrapError(`${name} length does not match metadata`);
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError(`${name} is truncated`);
  pos[0] += length;
  return Array.from(slice);
}

function readArtifactBootstrapChunkBytes(bytes: Uint8Array, pos: [number]): number[] {
  const length = readVarintU64(bytes, pos);
  if (!Number.isSafeInteger(length) || length === 0 || length > ARTIFACT_BOOTSTRAP_CHUNK_BYTES) throw artifactBootstrapError("chunk bytes exceed wire limit");
  const slice = bytes.subarray(pos[0], pos[0] + length);
  if (slice.length !== length) throw artifactBootstrapError("chunk bytes are truncated");
  pos[0] += length;
  return Array.from(slice);
}

function decodeArtifactBootstrap(bytes: Uint8Array, pos: [number]): WireArtifactBootstrap {
  const partial: WireArtifactBootstrap = {
    format_version: readVarintU64(bytes, pos),
    descriptor_hash: readHash32(bytes, pos),
    artifact_schema: readArtifactBootstrapString(bytes, pos, "artifact schema"),
    artifact_kind: readArtifactBootstrapString(bytes, pos, "artifact kind"),
    pack_schema_hash: readHash32(bytes, pos),
    baseline_frontier: decodeFrontier(bytes, pos),
    pack_hash: readHash32(bytes, pos),
    spr_hash: readHash32(bytes, pos),
    pack_length: readVarintU64(bytes, pos),
    spr_length: readVarintU64(bytes, pos),
    chunk_count: readVarintU64(bytes, pos),
    aggregate_hash: readHash32(bytes, pos),
    required_tail_frontier: decodeFrontier(bytes, pos),
    inline: null,
  };
  const inline = readBool(bytes, pos);
  validateArtifactBootstrapHeader(partial, inline, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS);
  const decoded = inline ? { ...partial, inline: { pack: readArtifactBootstrapBytes(bytes, pos, partial.pack_length, "inline pack"), spr: readArtifactBootstrapBytes(bytes, pos, partial.spr_length, "inline SPR") } } : partial;
  validateArtifactBootstrap(decoded, DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS);
  return decoded;
}

function encodeBootstrap(out: number[], bootstrap: WireBootstrap): void {
  if (bootstrap === "None") {
    out.push(0);
    return;
  }
  if (bootstrap === "Tail") {
    out.push(2);
    return;
  }
  if ("Snapshot" in bootstrap) {
    out.push(1);
    writeHash32(out, bootstrap.Snapshot.pack_hash);
    writeOptBytes(out, bootstrap.Snapshot.inline);
    return;
  }
  out.push(3);
  encodeArtifactBootstrap(out, bootstrap.ArtifactBootstrap);
}
function decodeBootstrap(bytes: Uint8Array, pos: [number]): WireBootstrap {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire bootstrap tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "None";
  if (tag === 2) return "Tail";
  if (tag === 1) return { Snapshot: { pack_hash: readHash32(bytes, pos), inline: readOptBytes(bytes, pos) } };
  if (tag === 3) return { ArtifactBootstrap: decodeArtifactBootstrap(bytes, pos) };
  throw new Error(`wire bootstrap tag: unknown tag ${tag}`);
}

function encodeApplyOutcome(out: number[], outcome: WireApplyOutcome): void {
  if (outcome === "Accepted") {
    out.push(0);
    return;
  }
  if ("Transformed" in outcome) {
    out.push(1);
    encodeEnvelope(out, outcome.Transformed.envelope);
    return;
  }
  out.push(2);
  writeStr(out, outcome.Rejected.reason);
  writeBytes(out, outcome.Rejected.messages);
}
function decodeApplyOutcome(bytes: Uint8Array, pos: [number]): WireApplyOutcome {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire apply-outcome tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Accepted";
  if (tag === 1) return { Transformed: { envelope: decodeEnvelope(bytes, pos) } };
  if (tag === 2) return { Rejected: { reason: readStr(bytes, pos), messages: readBytes(bytes, pos) } };
  throw new Error(`wire apply-outcome tag: unknown tag ${tag}`);
}

function encodeAckStage(out: number[], stage: WireAckStage): void {
  if (stage === "Received") {
    out.push(0);
    return;
  }
  if (stage === "Persisted") {
    out.push(1);
    return;
  }
  out.push(2);
  encodeApplyOutcome(out, stage.Applied.outcome);
}
function decodeAckStage(bytes: Uint8Array, pos: [number]): WireAckStage {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire ack-stage tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Received";
  if (tag === 1) return "Persisted";
  if (tag === 2) return { Applied: { outcome: decodeApplyOutcome(bytes, pos) } };
  throw new Error(`wire ack-stage tag: unknown tag ${tag}`);
}
function writeVecAckStage(out: number[], values: readonly WireAckStage[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) encodeAckStage(out, value);
}
function readVecAckStage(bytes: Uint8Array, pos: [number]): WireAckStage[] {
  const count = readVarintU64(bytes, pos);
  const result: WireAckStage[] = [];
  for (let i = 0; i < count; i++) result.push(decodeAckStage(bytes, pos));
  return result;
}
//#endregion 🔖️NestedEnums

const WIRE_LANE_BYTES: Record<WireLane, number> = { command: 0, preview: 1 };
const WIRE_BYTE_LANES: readonly WireLane[] = ["command", "preview"];

/** 📤️ Encodes one `ClientFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_client_frame` (see that module's doc comment: W5 flipped the whole wire
 * codec from a JSON body to this hand-rolled binary layout, byte-for-byte with the Rust side). */
export function encodeClientFrame(frame: ClientFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if (frame === "Bye") {
    out.push(6);
    return new Uint8Array(out);
  }
  if ("SocketHelloV1" in frame) {
    out.push(7);
    const hello = frame.SocketHelloV1;
    writeVarintU64(out, hello.wire_version);
    writeVarintU64(out, hello.protocol_version);
    writeStr(out, hello.schema);
    writeHash32(out, hello.pack_schema_hash);
    writeOptStr(out, hello.resume_token);
    writeOptFrontier(out, hello.frontier);
  } else if ("Commands" in frame) {
    out.push(1);
    writeVarintU64(out, frame.Commands.batch_id);
    writeVecEnvelope(out, frame.Commands.envelopes);
  } else if ("FrontierAdvertise" in frame) {
    out.push(2);
    encodeFrontier(out, frame.FrontierAdvertise.frontier);
  } else if ("PreviewPublish" in frame) {
    out.push(3);
    writeStr(out, frame.PreviewPublish.key);
    writeVarintU64(out, frame.PreviewPublish.seq);
    writeBytes(out, frame.PreviewPublish.payload);
  } else if ("Presence" in frame) {
    out.push(4);
    writeBytes(out, frame.Presence.peer);
  } else if ("CreditGrant" in frame) {
    out.push(5);
    writeVarintU64(out, frame.CreditGrant.n);
  } else {
    throw new Error("encodeClientFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 🚚️ One `ClientFrame::Commands` of exact envelopes — {@link encodeClientFrame}'s `Commands` bytes, every HLC exact: a relay
 * keeps each envelope's authored `(hlc, id)`, the TS twin of the Rust actors' relay. */
export function encodeClientCommandsFrameExact(batchId: number, envelopes: readonly ExactWireMutationEnvelope[], lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane], 1];
  writeVarintU64(out, batchId);
  writeVarintU64(out, envelopes.length);
  for (const envelope of envelopes) encodeExactEnvelope(out, envelope);
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ClientFrame` — the TS twin of `protocol_wire::decode_client_frame`. */
export function decodeClientFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ClientFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]!];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire client-frame tag: truncated");
  pos[0] += 1;
  let frame: ClientFrame;
  switch (tag) {
    case 1:
      frame = { Commands: { batch_id: readVarintU64(bytes, pos), envelopes: readVecEnvelope(bytes, pos) } };
      break;
    case 2:
      frame = { FrontierAdvertise: { frontier: decodeFrontier(bytes, pos) } };
      break;
    case 3:
      frame = { PreviewPublish: { key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 4:
      frame = { Presence: { peer: readBytes(bytes, pos) } };
      break;
    case 5:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 6:
      frame = "Bye";
      break;
    case 7:
      frame = {
        SocketHelloV1: {
          wire_version: readVarintU64(bytes, pos),
          protocol_version: readVarintU64(bytes, pos),
          schema: readStr(bytes, pos),
          pack_schema_hash: readHash32(bytes, pos),
          resume_token: readOptStr(bytes, pos),
          frontier: readOptFrontier(bytes, pos),
        },
      };
      break;
    default:
      throw new Error(`wire client-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 📤️ Encodes one `ServerFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_server_frame`. */
export function encodeServerFrame(frame: ServerFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if ("Welcome" in frame) {
    out.push(0);
    writeStr(out, frame.Welcome.session_id);
    writeStr(out, frame.Welcome.resume_token);
    encodeFrontier(out, frame.Welcome.server_frontier);
    encodeBootstrap(out, frame.Welcome.bootstrap);
  } else if ("SnapshotChunk" in frame) {
    out.push(1);
    writeVarintU64(out, frame.SnapshotChunk.seq);
    writeBytes(out, frame.SnapshotChunk.bytes);
  } else if ("SnapshotDone" in frame) {
    out.push(2);
    writeVarintU64(out, frame.SnapshotDone.seq_count);
  } else if ("Commands" in frame) {
    out.push(3);
    writeVecEnvelope(out, frame.Commands.envelopes);
    writeStr(out, frame.Commands.origin);
    encodeFrontier(out, frame.Commands.frontier);
  } else if ("Ack" in frame) {
    out.push(4);
    writeVarintU64(out, frame.Ack.batch_id);
    writeVecAckStage(out, frame.Ack.stages);
    encodeFrontier(out, frame.Ack.frontier);
  } else if ("Preview" in frame) {
    out.push(5);
    writeStr(out, frame.Preview.actor);
    writeStr(out, frame.Preview.key);
    writeVarintU64(out, frame.Preview.seq);
    writeBytes(out, frame.Preview.payload);
  } else if ("Presence" in frame) {
    out.push(6);
    writeVecBytes(out, frame.Presence.peers);
  } else if ("CreditGrant" in frame) {
    out.push(7);
    writeVarintU64(out, frame.CreditGrant.n);
  } else if ("Error" in frame) {
    out.push(8);
    writeStr(out, frame.Error.code);
    writeStr(out, frame.Error.message);
  } else if ("Session" in frame) {
    out.push(9);
    writeStr(out, frame.Session.actor);
    out.push(frame.Session.color);
  } else if ("ArtifactBootstrapChunk" in frame) {
    out.push(10);
    writeHash32(out, frame.ArtifactBootstrapChunk.descriptor_hash);
    writeVarintU64(out, frame.ArtifactBootstrapChunk.index);
    writeBytes(out, frame.ArtifactBootstrapChunk.bytes);
  } else if ("ArtifactBootstrapDone" in frame) {
    out.push(11);
    writeHash32(out, frame.ArtifactBootstrapDone.descriptor_hash);
    writeVarintU64(out, frame.ArtifactBootstrapDone.chunk_count);
  } else if ("RebootstrapRequired" in frame) {
    out.push(12);
    writeStr(out, frame.RebootstrapRequired.control.space_id);
    writeStr(out, frame.RebootstrapRequired.control.document_id);
    writeHash32(out, frame.RebootstrapRequired.control.checkpoint_id);
    writeHash32(out, frame.RebootstrapRequired.control.descriptor_hash);
    encodeFrontier(out, frame.RebootstrapRequired.control.baseline_frontier);
  } else {
    throw new Error("encodeServerFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ServerFrame` — the TS twin of `protocol_wire::decode_server_frame`. */
export function decodeServerFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ServerFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]!];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire server-frame tag: truncated");
  pos[0] += 1;
  let frame: ServerFrame;
  switch (tag) {
    case 0:
      frame = { Welcome: { session_id: readStr(bytes, pos), resume_token: readStr(bytes, pos), server_frontier: decodeFrontier(bytes, pos), bootstrap: decodeBootstrap(bytes, pos) } };
      break;
    case 1:
      frame = { SnapshotChunk: { seq: readVarintU64(bytes, pos), bytes: readBytes(bytes, pos) } };
      break;
    case 2:
      frame = { SnapshotDone: { seq_count: readVarintU64(bytes, pos) } };
      break;
    case 3:
      frame = { Commands: { envelopes: readVecEnvelope(bytes, pos), origin: readStr(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 4:
      frame = { Ack: { batch_id: readVarintU64(bytes, pos), stages: readVecAckStage(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 5:
      frame = { Preview: { actor: readStr(bytes, pos), key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 6:
      frame = { Presence: { peers: readVecBytes(bytes, pos) } };
      break;
    case 7:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 8:
      frame = { Error: { code: readStr(bytes, pos), message: readStr(bytes, pos) } };
      break;
    case 9:
      frame = { Session: { actor: readStr(bytes, pos), color: readU8(bytes, pos) } };
      break;
    case 10:
      frame = { ArtifactBootstrapChunk: { descriptor_hash: readHash32(bytes, pos), index: readVarintU64(bytes, pos), bytes: readArtifactBootstrapChunkBytes(bytes, pos) } };
      break;
    case 11:
      frame = { ArtifactBootstrapDone: { descriptor_hash: readHash32(bytes, pos), chunk_count: readVarintU64(bytes, pos) } };
      break;
    case 12: {
      const control: WireRebootstrapRequired = {
        space_id: readStr(bytes, pos),
        document_id: readStr(bytes, pos),
        checkpoint_id: readHash32(bytes, pos),
        descriptor_hash: readHash32(bytes, pos),
        baseline_frontier: decodeFrontier(bytes, pos),
      };
      if (!control.space_id || !control.document_id || control.space_id.length > 256 || control.document_id.length > 256 || new TextEncoder().encode(control.space_id).length > 256 || new TextEncoder().encode(control.document_id).length > 256 || control.checkpoint_id.every((byte) => byte === 0) || control.descriptor_hash.every((byte) => byte === 0) || control.baseline_frontier.document_id !== control.document_id || (!control.baseline_frontier.head_edit_id && (control.baseline_frontier.head_edit_ordinal !== 0 || control.baseline_frontier.last_commit_seq !== 0))) {
        throw new Error("artifact bootstrap: rebootstrap control identity is invalid");
      }
      frame = { RebootstrapRequired: { control } };
      break;
    }
    default:
      throw new Error(`wire server-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 🪡 Extracts and validates the exact causal batch embedded in a server `Commands` frame. */
export function extractServerCommandsDocumentBackboneBatchExact(bytes: Uint8Array): Uint8Array | null {
  if (!(bytes instanceof Uint8Array) || bytes.length < 2) throw new DocumentBackboneBatchError("malformed", "server-frame");
  if (WIRE_BYTE_LANES[bytes[0]!] === undefined) throw new DocumentBackboneBatchError("malformed", "server-frame");
  if (bytes[1] !== 3) return null;
  const position: [number] = [2];
  return readDocumentBackboneEnvelopeBatchExact(bytes, position).bytes;
}

