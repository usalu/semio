import { fetchWithTimeout } from "@semio-tech/framework";
import type { PersistenceBinding } from "../../../../../🟦️";
import { decodeDocumentArchiveBytes, DOCUMENT_ARCHIVE_MAXIMUM_BYTES } from "../../../../../🟦️";
import { FOLDER_ENDPOINT_PATH, FOLDER_FETCH_TIMEOUT_MS } from "../../⚙️constants/🟦️.ts";
import type { ArtifactState } from "../../🧾️document-state/🟦️.ts";
import { retireFolderCanonicalBootstrapMirror } from "../../🌉️wire-bridge/🟦️.ts";
import { folderBinding, setStatus } from "../../🎭️actor-lifecycle/🟦️.ts";
import { equalByteArrays, releaseDocumentBackboneOwnership } from "../../📮️hub-outbox/🟦️.ts";
import type { MutationEnvelope } from "@semio-tech/framework-replication";

/** ✍️ Folder envelope writes: the envelope URL, the retirement of the held canonical bootstrap mirror and the archive write that every folder lane (watch, config facet, lifecycle) shares. */
export function folderEnvelopeUrl(binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, documentId: string): string {
  return `${FOLDER_ENDPOINT_PATH}?uri=${encodeURIComponent(`folder://${binding.path}`)}&documentId=${encodeURIComponent(documentId)}`;
}

export async function retireCurrentFolderCanonicalBootstrapMirror(state: ArtifactState): Promise<void> {
  const owner = state.canonicalFolderMirror;
  if (!owner) return;
  if (!(await retireFolderCanonicalBootstrapMirror(owner))) throw new Error("folder canonical bootstrap retirement failed");
  if (state.canonicalFolderMirror === owner) state.canonicalFolderMirror = null;
}

export async function writeFolder(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, archive: readonly number[]): Promise<void> {
  await retireCurrentFolderCanonicalBootstrapMirror(state);
  const bytes = Uint8Array.from(archive);
  if (bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");
  decodeDocumentArchiveBytes(bytes);
  if (state.folderArchive !== null && equalByteArrays(state.folderArchive, bytes)) return;
  const response = await fetchWithTimeout(
    folderEnvelopeUrl(binding, state.config.documentId),
    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: bytes },
    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },
  );
  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);
  state.folderArchive = bytes;
  state.folderBootstrapped = true;
}

type ArchiveWrite = Readonly<{ archive: readonly number[]; covered: readonly MutationEnvelope[] }>;
type ArchiveWriteQueue = { running: Promise<void> | null; next: ArchiveWrite | null; failures: number; retry: ReturnType<typeof setTimeout> | null };
const archiveWriteQueues = new WeakMap<ArtifactState, ArchiveWriteQueue>();
const ARCHIVE_RETRY_BASE_MS = 200;
const ARCHIVE_RETRY_MAXIMUM_MS = 5_000;

/** 💾️ The one acknowledged archive write of a folder document. Writes of one document run one at a time in submission order and a newer archive supersedes a queued older one, so a slower older PUT can never land after a newer one. Only the mutations the archive covered leave `pendingMutations`, and `persisted` turns true only when none remain; an edit admitted while the write was in flight stays unsaved. A refused write keeps the edit pending, reports `persisted: false` and retries the newest archive with backoff until the folder acknowledges it. */
export function persistFolderArchive(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, archive: readonly number[], covered: readonly MutationEnvelope[] = [...state.pendingMutations]): Promise<void> {
  const queue = archiveWriteQueues.get(state) ?? { running: null, next: null, failures: 0, retry: null };
  archiveWriteQueues.set(state, queue);
  if (queue.retry !== null) clearTimeout(queue.retry);
  queue.retry = null;
  queue.next = { archive, covered };
  queue.running ??= drainArchiveWrites(state, binding, queue).finally(() => {
    queue.running = null;
  });
  return queue.running;
}

async function drainArchiveWrites(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>, queue: ArchiveWriteQueue): Promise<void> {
  while (queue.next !== null && !state.closed) {
    const write = queue.next;
    queue.next = null;
    try {
      if (await folderDivergedFromHeld(state, binding)) {
        queue.next ??= write;
        setStatus(state, { writeFailure: { code: "external-divergence" } });
        return;
      }
      await writeFolder(state, binding, write.archive);
    } catch (error) {
      if (state.closed || state.docAbort.signal.aborted) return;
      console.error("[backbone-worker] folder write failed", state.config.documentId, error);
      queue.failures += 1;
      queue.next ??= write;
      setStatus(state, { writeFailure: { code: "folder-write-failed" } });
      queue.retry = setTimeout(() => {
        queue.retry = null;
        const retained = queue.next;
        queue.next = null;
        if (retained !== null && !state.closed) void persistFolderArchive(state, binding, retained.archive, retained.covered);
      }, Math.min(ARCHIVE_RETRY_BASE_MS * 2 ** (queue.failures - 1), ARCHIVE_RETRY_MAXIMUM_MS));
      return;
    }
    if (state.closed) return;
    queue.failures = 0;
    const written = new Set(write.covered);
    const released = state.pendingMutations.filter((envelope) => written.has(envelope));
    state.pendingMutations = state.pendingMutations.filter((envelope) => !written.has(envelope));
    releaseDocumentBackboneOwnership(state, released);
    setStatus(state, { pendingMutations: state.pendingMutations.length, writeFailure: undefined });
  }
}

/** 🔎️ Whether the folder holds an archive other than the one this document last read or wrote: its content changed under unsaved edits, so the write is refused instead of overwriting it. */
async function folderDivergedFromHeld(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>): Promise<boolean> {
  const response = await fetchWithTimeout(folderEnvelopeUrl(binding, state.config.documentId), undefined, { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal });
  if (response.status === 204 || response.status === 404) return false;
  if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);
  const bytes = new Uint8Array(await response.arrayBuffer());
  return state.folderArchive === null || !equalByteArrays(state.folderArchive, bytes);
}

/** 🔁️ Writes the archive that waited behind an `external-divergence` or `folder-unreadable` failure once the folder matches again. */
export function resumeBlockedFolderArchive(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }>): void {
  const queue = archiveWriteQueues.get(state);
  const code = state.status.writeFailure?.code;
  if (queue === undefined || queue.running !== null || queue.next === null || (code !== "external-divergence" && code !== "folder-unreadable")) return;
  const waiting = queue.next;
  queue.next = null;
  void persistFolderArchive(state, binding, waiting.archive, waiting.covered);
}

/** 💾️ Whether a folder document still holds work the folder has not acknowledged: admitted mutations, a queued or in-flight archive or a retry waiting. */
export function folderWorkUnacknowledged(state: ArtifactState): boolean {
  if (folderBinding(state.config) === null) return false;
  const queue = archiveWriteQueues.get(state);
  return state.pendingMutations.length > 0 || (queue !== undefined && (queue.running !== null || queue.next !== null || queue.retry !== null));
}

/** 🛑️ Waits until the folder acknowledged everything admitted so far; `false` when the deadline passed, the caller cancelled or the work cannot be saved (nothing was ever offered to the folder). Status events report the progress; nothing is discarded here. */
export async function settleFolderArchive(state: ArtifactState, timeoutMs: number, signal?: AbortSignal): Promise<boolean> {
  const deadline = Date.now() + timeoutMs;
  while (folderWorkUnacknowledged(state)) {
    const queue = archiveWriteQueues.get(state);
    const idle = queue === undefined || (queue.running === null && queue.next === null && queue.retry === null);
    if (signal?.aborted === true || Date.now() >= deadline || idle) return false;
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  return true;
}
