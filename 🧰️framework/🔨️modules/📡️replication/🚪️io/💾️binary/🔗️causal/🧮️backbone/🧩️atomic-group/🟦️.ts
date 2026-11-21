/** 📡️ Seals the exact multipart document edit held by one authenticated actor binding. */

export const ATOMIC_BACKBONE_MAXIMUM_BYTES = 2 * 1024 * 1024;
const BODY_BYTES = 64 * 1024;
const PREPARED_TAG = Uint8Array.of(0x80, 0, 0x47, 0);
const PART_TAG = Uint8Array.of(0x80, 0, 0x47, 1);
const HEADER_BYTES = 42;
const MAXIMUM_FRAME_BYTES = BODY_BYTES + HEADER_BYTES;

type Part = Readonly<{ digest: Uint8Array; extent: number; ordinal: number; count: number; body: Uint8Array }>;

export type AtomicDocumentScope = Readonly<{ documentId: string; actor: string; generation: string }>;

export type AtomicDocumentRows<Row> = Readonly<{
  editId: string;
  scope: AtomicDocumentScope;
  envelopes: readonly Row[];
  parts: readonly Uint8Array[];
}>;

type BoundRow = Readonly<{
  mutation_id: string;
  document_id: string;
  actor: string;
  transaction: Readonly<{ id: string; tool: string }> | null;
}>;

function prefix(bytes: Uint8Array, tag: Uint8Array): boolean {
  return bytes.length >= tag.length && tag.every((byte, index) => bytes[index] === byte);
}

function sameDigest(left: Uint8Array, right: Uint8Array): boolean {
  return left.byteLength === right.byteLength && left.every((byte, index) => byte === right[index]);
}

function sameScope(left: AtomicDocumentScope, right: AtomicDocumentScope): boolean {
  return left.documentId === right.documentId && left.actor === right.actor && left.generation === right.generation;
}

/** 🔍️ Selects the reserved inner carrier before the ordinary mutation-batch decoder. */
export function atomicBackbonePart(bytes: Uint8Array): Part | undefined {
  if (!prefix(bytes, PART_TAG)) return undefined;
  if (bytes.byteLength <= HEADER_BYTES || bytes.byteLength > MAXIMUM_FRAME_BYTES) throw new Error("atomic backbone: physical extent");
  const read = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const extent = read.getUint32(36, true);
  const ordinal = read.getUint16(40, true);
  if (extent === 0 || extent > ATOMIC_BACKBONE_MAXIMUM_BYTES) throw new Error("atomic backbone: logical extent");
  const count = Math.ceil(extent / BODY_BYTES);
  if (ordinal >= count || bytes.byteLength - HEADER_BYTES !== Math.min(BODY_BYTES, extent - ordinal * BODY_BYTES)) throw new Error("atomic backbone: part extent");
  return { digest: bytes.subarray(4, 36), extent, ordinal, count, body: bytes.subarray(HEADER_BYTES) };
}

function preparedEdit(bytes: Uint8Array): Readonly<{ id: string; batch: Uint8Array }> {
  if (!prefix(bytes, PREPARED_TAG)) throw new Error("atomic backbone: prepared edit tag");
  let end = PREPARED_TAG.length;
  const first = bytes[end++];
  if (first === undefined) throw new Error("atomic backbone: edit identity");
  let length = first;
  if ((first & 128) !== 0) {
    const high = bytes[end++];
    if (high === undefined || high < 1 || high > 2) throw new Error("atomic backbone: edit identity");
    length = (first & 127) + high * 128;
  }
  if (length === 0 || length > 256 || end + length >= bytes.length) throw new Error("atomic backbone: edit identity");
  const id = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes.subarray(end, end + length));
  return { id, batch: bytes.subarray(end + length) };
}

/** 📬️ Retains one group per mounted actor lease and permits one publication after complete verification. */
export class AtomicDocumentAssembly<Row extends BoundRow> {
  private constScope: AtomicDocumentScope;
  private candidate: { digest: Uint8Array; extent: number; parts: (Uint8Array | undefined)[]; remaining: number } | undefined;
  private sealing = false;
  private retired = false;

  constructor(scope: AtomicDocumentScope, private readonly decode: (batch: Uint8Array) => readonly Row[], private readonly trustedRemoteAuthor = false) {
    if (!scope.documentId || !scope.actor || !scope.generation) throw new Error("atomic backbone: actor binding");
    this.constScope = { ...scope };
  }

  retire(): void {
    this.candidate = undefined;
    this.retired = true;
  }

  async admit(scope: AtomicDocumentScope, bytes: Uint8Array): Promise<AtomicDocumentRows<Row> | undefined> {
    if (this.retired || this.sealing || !sameScope(scope, this.constScope)) throw new Error("atomic backbone: stale binding");
    const part = atomicBackbonePart(bytes);
    if (part === undefined) throw new Error("atomic backbone: frame required");
    const candidate = this.candidate ??= {
      digest: part.digest.slice(), extent: part.extent,
      parts: new Array<Uint8Array | undefined>(part.count), remaining: part.count,
    };
    if (candidate.extent !== part.extent || !sameDigest(candidate.digest, part.digest)) throw new Error("atomic backbone: another group is active");
    if (candidate.parts[part.ordinal] !== undefined) throw new Error("atomic backbone: duplicate part");
    candidate.parts[part.ordinal] = bytes.slice();
    if (--candidate.remaining !== 0) return undefined;
    this.sealing = true;
    try {
      const full = new Uint8Array(candidate.extent);
      candidate.parts.forEach((frame, ordinal) => full.set(frame!.subarray(HEADER_BYTES), ordinal * BODY_BYTES));
      const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", full));
      if (this.retired || !sameScope(scope, this.constScope)) throw new Error("atomic backbone: stale sealed binding");
      if (!sameDigest(digest, candidate.digest)) throw new Error("atomic backbone: digest");
      const edit = preparedEdit(full);
      const envelopes = this.decode(edit.batch);
      const transaction = envelopes[0]?.transaction;
      const author = this.trustedRemoteAuthor ? envelopes[0]?.actor : scope.actor;
      const seen = new Set<string>();
      const separator = transaction?.tool.indexOf("#") ?? -1;
      if (!author || !transaction?.id || separator < 1 || transaction.tool.slice(separator + 1) !== "original-media") throw new Error("atomic backbone: transaction");
      for (const row of envelopes) {
        if (!row.mutation_id || seen.has(row.mutation_id) || row.document_id !== scope.documentId || row.actor !== author ||
          row.transaction?.id !== transaction.id || row.transaction.tool !== transaction.tool) throw new Error("atomic backbone: row binding");
        seen.add(row.mutation_id);
      }
      return { editId: edit.id, scope: { ...this.constScope }, envelopes, parts: candidate.parts as Uint8Array[] };
    } finally {
      this.candidate = undefined;
      this.sealing = false;
    }
  }
}
