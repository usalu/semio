import type { ArtifactBootstrapPair, WireArtifactBootstrap } from "../../🟦️.ts";

//#region 🔖️ArtifactBootstrap
export const ARTIFACT_BOOTSTRAP_FORMAT_VERSION = 1;
export const ARTIFACT_BOOTSTRAP_CHUNK_BYTES = 4 * 1024;
export const ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES = 64 * 1024 * 1024;
export const ARTIFACT_BOOTSTRAP_MAX_CHUNKS = 16 * 1024;

/** 🛡️ Caller-selected assembly ceilings, always constrained by the wire chunk maximum. */
export type ArtifactBootstrapLimits = Readonly<{ maxTotalBytes: number; maxChunks: number; maxChunkBytes: number }>;

/** 📈️ Observable transfer progress; byte and chunk counts never decrease within one assembler. */
export type ArtifactBootstrapProgress = Readonly<{ receivedBytes: number; totalBytes: number; receivedChunks: number; totalChunks: number }>;

/** ⏱️ Host-provided cancellation, monotonic clock, and progress boundary. */
export interface ArtifactBootstrapControl {
  isCancelled(): boolean;
  nowMs(): number;
  onProgress(progress: ArtifactBootstrapProgress): void;
}

export const DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS: ArtifactBootstrapLimits = Object.freeze({ maxTotalBytes: ARTIFACT_BOOTSTRAP_MAX_TOTAL_BYTES, maxChunks: ARTIFACT_BOOTSTRAP_MAX_CHUNKS, maxChunkBytes: ARTIFACT_BOOTSTRAP_CHUNK_BYTES });

/** 🚧️ Constructs the canonical artifact bootstrap representation refusal. */
export function artifactBootstrapError(message: string): Error {
  return new Error(`artifact bootstrap ${message}`);
}

function equalBytes(left: readonly number[] | Readonly<Uint8Array>, right: readonly number[] | Readonly<Uint8Array>): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

function validateBytes(name: string, value: readonly number[]): void {
  if (value.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255)) throw artifactBootstrapError(`${name} contains an invalid byte`);
}

function validateHash(name: string, value: readonly number[], nonzero: boolean): void {
  if (value.length !== 32) throw artifactBootstrapError(`${name} must contain 32 bytes`);
  validateBytes(name, value);
  if (nonzero && value.every((byte) => byte === 0)) throw artifactBootstrapError(`${name} must be nonzero`);
}

function validateNatural(name: string, value: number, minimum: number): void {
  if (!Number.isSafeInteger(value) || value < minimum) throw artifactBootstrapError(`${name} is invalid`);
}

function validateLimits(limits: ArtifactBootstrapLimits): void {
  validateNatural("max total bytes", limits.maxTotalBytes, 1);
  validateNatural("max chunks", limits.maxChunks, 1);
  validateNatural("max chunk bytes", limits.maxChunkBytes, 1);
  if (limits.maxChunkBytes > ARTIFACT_BOOTSTRAP_CHUNK_BYTES) throw artifactBootstrapError("max chunk bytes exceeds wire limit");
}

function artifactBootstrapTotal(bootstrap: WireArtifactBootstrap): number {
  validateNatural("pack length", bootstrap.pack_length, 1);
  validateNatural("SPR length", bootstrap.spr_length, 1);
  const total = bootstrap.pack_length + bootstrap.spr_length;
  if (!Number.isSafeInteger(total)) throw artifactBootstrapError("total bytes overflow");
  return total;
}

/** 🛂️ Validates an original bootstrap header before wire payload acquisition. */
export function validateArtifactBootstrapHeader(bootstrap: WireArtifactBootstrap, inline: boolean, limits: ArtifactBootstrapLimits): number {
  validateLimits(limits);
  if (bootstrap.format_version !== ARTIFACT_BOOTSTRAP_FORMAT_VERSION) throw artifactBootstrapError(`version ${bootstrap.format_version} is unsupported`);
  validateHash("descriptor hash", bootstrap.descriptor_hash, true);
  validateHash("pack schema hash", bootstrap.pack_schema_hash, true);
  validateHash("pack hash", bootstrap.pack_hash, true);
  validateHash("SPR hash", bootstrap.spr_hash, true);
  validateHash("aggregate hash", bootstrap.aggregate_hash, true);
  validateHash("baseline frontier chain hash", bootstrap.baseline_frontier.chain_hash, false);
  validateHash("required tail frontier chain hash", bootstrap.required_tail_frontier.chain_hash, false);
  const schemaBytes = new TextEncoder().encode(bootstrap.artifact_schema).length;
  const kindBytes = new TextEncoder().encode(bootstrap.artifact_kind).length;
  if (schemaBytes === 0 || schemaBytes > 256) throw artifactBootstrapError("artifact schema length is invalid");
  if (kindBytes === 0 || kindBytes > 256) throw artifactBootstrapError("artifact kind length is invalid");
  if (bootstrap.baseline_frontier.document_id.length === 0 || bootstrap.baseline_frontier.document_id !== bootstrap.required_tail_frontier.document_id) throw artifactBootstrapError("frontier document mismatch");
  validateNatural("baseline head", bootstrap.baseline_frontier.head_edit_ordinal, 0);
  validateNatural("baseline commit", bootstrap.baseline_frontier.last_commit_seq, 0);
  validateNatural("required tail head", bootstrap.required_tail_frontier.head_edit_ordinal, 0);
  validateNatural("required tail commit", bootstrap.required_tail_frontier.last_commit_seq, 0);
  if (bootstrap.required_tail_frontier.head_edit_ordinal < bootstrap.baseline_frontier.head_edit_ordinal || bootstrap.required_tail_frontier.last_commit_seq < bootstrap.baseline_frontier.last_commit_seq) throw artifactBootstrapError("required tail frontier precedes baseline");
  const total = artifactBootstrapTotal(bootstrap);
  if (total > limits.maxTotalBytes) throw artifactBootstrapError("total bytes exceed assembler budget");
  validateNatural("chunk count", bootstrap.chunk_count, 0);
  if (inline) {
    if (bootstrap.chunk_count !== 0) throw artifactBootstrapError("inline pair must declare zero chunks");
  } else {
    if (bootstrap.chunk_count === 0 || bootstrap.chunk_count > limits.maxChunks) throw artifactBootstrapError("chunk count exceeds assembler budget");
    if (bootstrap.chunk_count > total || total > bootstrap.chunk_count * limits.maxChunkBytes) throw artifactBootstrapError("chunk count cannot cover declared bytes");
  }
  return total;
}

/** 🧾️ Validates declared bootstrap metadata and any inline pair. */
export function validateArtifactBootstrap(bootstrap: WireArtifactBootstrap, limits: ArtifactBootstrapLimits): number {
  const inline = bootstrap.inline !== null;
  const total = validateArtifactBootstrapHeader(bootstrap, inline, limits);
  if (bootstrap.inline !== null) {
    if (bootstrap.inline.pack.length !== bootstrap.pack_length || bootstrap.inline.spr.length !== bootstrap.spr_length) throw artifactBootstrapError("inline pair lengths do not match metadata");
    validateBytes("inline pack", bootstrap.inline.pack);
    validateBytes("inline SPR", bootstrap.inline.spr);
  }
  return total;
}

/** #️⃣ Browser-safe SHA-256 backed by the host Web Crypto implementation. */
export async function artifactBootstrapSha256(bytes: Uint8Array): Promise<Uint8Array> {
  const owned = Uint8Array.from(bytes);
  return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", owned.buffer));
}

/** 🔗️ SHA-256 over the exact declared `pack || spr` content stream. */
export async function artifactBootstrapAggregateHash(pack: Uint8Array, spr: Uint8Array): Promise<Uint8Array> {
  const content = new Uint8Array(pack.byteLength + spr.byteLength);
  content.set(pack, 0);
  content.set(spr, pack.byteLength);
  return artifactBootstrapSha256(content);
}

/** 🧱️ Bounded, cancellable staging owner that yields a pair only after complete integrity validation. */
export class ArtifactBootstrapAssembler {
  readonly bootstrap: WireArtifactBootstrap;
  readonly limits: ArtifactBootstrapLimits;
  readonly deadlineMs: number | null;
  #storage: Uint8Array | null;
  #received = 0;
  #nextIndex = 0;

  constructor(bootstrap: WireArtifactBootstrap, expectedDescriptorHash: readonly number[], limits: ArtifactBootstrapLimits = DEFAULT_ARTIFACT_BOOTSTRAP_LIMITS, deadlineMs: number | null = null, control: ArtifactBootstrapControl) {
    const total = validateArtifactBootstrap(bootstrap, limits);
    validateHash("expected descriptor hash", expectedDescriptorHash, true);
    if (!equalBytes(bootstrap.descriptor_hash, expectedDescriptorHash)) throw artifactBootstrapError("descriptor mismatch");
    if (control.isCancelled()) throw artifactBootstrapError("cancelled");
    if (deadlineMs !== null && control.nowMs() >= deadlineMs) throw artifactBootstrapError("deadline exceeded");
    this.bootstrap = bootstrap;
    this.limits = limits;
    this.deadlineMs = deadlineMs;
    this.#storage = new Uint8Array(total);
    control.onProgress(this.progress);
    if (bootstrap.inline !== null) {
      this.#storage.set(bootstrap.inline.pack, 0);
      this.#storage.set(bootstrap.inline.spr, bootstrap.pack_length);
      this.#received = total;
      control.onProgress(this.progress);
    }
  }

  /** 💾️ Bytes currently owned by the assembler; zero after abort or completion. */
  get retainedBytes(): number {
    return this.#storage?.byteLength ?? 0;
  }

  /** 📈️ Current monotonic progress snapshot. */
  get progress(): ArtifactBootstrapProgress {
    return { receivedBytes: this.#received, totalBytes: this.bootstrap.pack_length + this.bootstrap.spr_length, receivedChunks: this.#nextIndex, totalChunks: this.bootstrap.chunk_count };
  }

  /** 🧹️ Drops all staged bytes without producing a completion value. */
  abort(): void {
    this.#storage?.fill(0);
    this.#storage = null;
  }

  #fail(message: string): never {
    this.abort();
    throw artifactBootstrapError(message);
  }

  #guard(control: ArtifactBootstrapControl): void {
    if (control.isCancelled()) this.#fail("cancelled");
    if (this.deadlineMs !== null && control.nowMs() >= this.deadlineMs) this.#fail("deadline exceeded");
    if (this.#storage === null) throw artifactBootstrapError("is not active");
  }

  /** 🧩️ Appends exactly the next descriptor-bound, nonempty bounded chunk. */
  push(chunk: Readonly<{ descriptor_hash: readonly number[]; index: number; bytes: readonly number[] }>, control: ArtifactBootstrapControl): ArtifactBootstrapProgress {
    this.#guard(control);
    if (this.bootstrap.inline !== null) this.#fail("inline transfer cannot accept chunks");
    if (!equalBytes(chunk.descriptor_hash, this.bootstrap.descriptor_hash)) this.#fail("chunk descriptor mismatch");
    if (chunk.index !== this.#nextIndex) this.#fail(`chunk index ${chunk.index} does not equal expected ${this.#nextIndex}`);
    if (chunk.bytes.length === 0 || chunk.bytes.length > this.limits.maxChunkBytes) this.#fail("chunk bytes exceed assembler budget");
    try {
      validateBytes("chunk", chunk.bytes);
    } catch {
      this.#fail("chunk contains an invalid byte");
    }
    if (chunk.index >= this.bootstrap.chunk_count) this.#fail("chunk index exceeds declared count");
    const end = this.#received + chunk.bytes.length;
    if (end > this.progress.totalBytes) this.#fail("chunk bytes exceed declared total");
    this.#storage!.set(chunk.bytes, this.#received);
    this.#received = end;
    this.#nextIndex += 1;
    control.onProgress(this.progress);
    return this.progress;
  }

  /** ✅️ Verifies completion and all hashes, transfers ownership of the pair, then retires staging. */
  async finish(done: Readonly<{ descriptor_hash: readonly number[]; chunk_count: number }> | null, control: ArtifactBootstrapControl): Promise<ArtifactBootstrapPair> {
    try {
      this.#guard(control);
      if (this.bootstrap.inline === null) {
        if (done === null) this.#fail("is incomplete without done frame");
        if (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash)) this.#fail("done descriptor mismatch");
        if (done.chunk_count !== this.bootstrap.chunk_count) this.#fail("done chunk count mismatch");
      } else if (done !== null && (!equalBytes(done.descriptor_hash, this.bootstrap.descriptor_hash) || done.chunk_count !== 0)) {
        this.#fail("inline done metadata mismatch");
      }
      if (this.#nextIndex !== this.bootstrap.chunk_count || this.#received !== this.progress.totalBytes) this.#fail("is incomplete");
      const storage = this.#storage!;
      const pack = storage.subarray(0, this.bootstrap.pack_length);
      const spr = storage.subarray(this.bootstrap.pack_length);
      const [packHash, sprHash, aggregateHash] = await Promise.all([artifactBootstrapSha256(pack), artifactBootstrapSha256(spr), artifactBootstrapSha256(storage)]);
      this.#guard(control);
      if (!equalBytes(packHash, this.bootstrap.pack_hash)) this.#fail("pack hash mismatch");
      if (!equalBytes(sprHash, this.bootstrap.spr_hash)) this.#fail("SPR hash mismatch");
      if (!equalBytes(aggregateHash, this.bootstrap.aggregate_hash)) this.#fail("aggregate hash mismatch");
      this.#storage = null;
      return { pack, spr };
    } catch (error) {
      this.abort();
      throw error;
    }
  }
}
//#endregion 🔖️ArtifactBootstrap
