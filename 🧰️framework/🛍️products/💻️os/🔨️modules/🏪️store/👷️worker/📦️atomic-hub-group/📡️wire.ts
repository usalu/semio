/** 🧷️ Encodes and admits the exact fixed-frame WebSocket carrier before ordinary command parsing. */
import { atomicBackbonePart } from "../../../../../../🔨️modules/📡️replication/🚪️io/💾️binary/🔗️causal/🧮️backbone/🧩️atomic-group/🟦️.ts";

const WIRE_COMMAND_LANE = 0;
const CLIENT_ATOMIC_PART = 8;
const SERVER_ATOMIC_PART = 16;
const FRONTIER_MAXIMUM_BYTES = 640;

export function encodeClientAtomicDocumentPart(batchId: bigint, part: Uint8Array): Uint8Array<ArrayBuffer> {
  if (batchId < 0n || batchId > 0xffff_ffff_ffff_ffffn || atomicBackbonePart(part) === undefined) {
    throw new Error("atomic Hub frame: client binding");
  }
  const bytes = new Uint8Array(10 + part.length);
  bytes.set([WIRE_COMMAND_LANE, CLIENT_ATOMIC_PART], 0);
  new DataView(bytes.buffer).setBigUint64(2, batchId, true);
  bytes.set(part, 10);
  return bytes;
}

export function decodeServerAtomicDocumentPart(
  bytes: Uint8Array,
): Readonly<{ part: Uint8Array; frontier: Uint8Array }> | undefined {
  if (bytes[0] !== WIRE_COMMAND_LANE || bytes[1] !== SERVER_ATOMIC_PART) return undefined;
  if (bytes.length < 4) throw new Error("atomic Hub frame: frontier length");
  const metadataLength = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getUint16(2, true);
  if (metadataLength > FRONTIER_MAXIMUM_BYTES || metadataLength >= bytes.length - 4) {
    throw new Error("atomic Hub frame: frontier extent");
  }
  const part = bytes.subarray(4 + metadataLength);
  const decoded = atomicBackbonePart(part);
  if (!decoded || (decoded.ordinal === 0) !== (metadataLength !== 0)) {
    throw new Error("atomic Hub frame: frontier ordinal");
  }
  return { part, frontier: bytes.subarray(4, 4 + metadataLength) };
}
