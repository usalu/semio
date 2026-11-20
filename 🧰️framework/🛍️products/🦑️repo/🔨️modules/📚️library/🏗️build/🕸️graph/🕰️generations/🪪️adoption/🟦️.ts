import { randomUUID } from "node:crypto";
import { mkdir, open, rename, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { signFleetDocumentV1, verifyFleetDocumentV1, type FleetRegistryV1 } from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type { LocalWholeGenerationAdoptionV1, RestoreExpectedWholeGenerationOutcomeV1 } from "../🔌️load/🟨️.mjs";
import type { GenerationHeadCandidateV1, GenerationHeadEvidenceV1 } from "../🌐️remote-exchange/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = schema.definitions.Policy.const;
const receiptRole = policy.role as "cache-read";

export type WholeGenerationAdoptionReceiptV1 = Readonly<{
  version: 1;
  destination: string;
  head: GenerationHeadCandidateV1;
  localGeneration: number;
  localManifest: string;
  completedMs: number;
}>;
type Intent = Omit<WholeGenerationAdoptionReceiptV1, "completedMs">;
type Envelope<Proof = WholeGenerationAdoptionReceiptV1> = Readonly<{ receipt: Proof; signature: string }>;
type Location = Readonly<{ root: string; store: string; origin: string; localDirectory: string; signal?: AbortSignal }>;

const sameHead = (a: GenerationHeadEvidenceV1, b: GenerationHeadEvidenceV1) =>
  a.origin === b.origin && a.generation === b.generation && a.manifest === b.manifest && a.scope === b.scope;

const sameCandidate = (a: GenerationHeadCandidateV1, b: GenerationHeadCandidateV1) =>
  sameHead(a, b) && a.identity === b.identity && a.implementation === b.implementation && a.platform === b.platform;

const validReceipt = (value: unknown): value is WholeGenerationAdoptionReceiptV1 =>
  validateJsonSchemaSubset(schema.definitions.Receipt, value, schema).length === 0;

const validEnvelope = (value: unknown): value is Envelope =>
  validateJsonSchemaSubset(schema.definitions.Envelope, value, schema).length === 0;

function signedText(receipt: Intent | WholeGenerationAdoptionReceiptV1): string {
  const { version, destination, head, localGeneration, localManifest } = receipt;
  return JSON.stringify([version, destination, head.origin, head.generation, head.manifest, head.scope, head.identity,
    head.implementation, head.platform, localGeneration, localManifest, ...("completedMs" in receipt ? [receipt.completedMs] : [])]);
}

/** 📍 Locates the signed receipt in the destination's existing replicated origin directory. */
export function wholeGenerationAdoptionReceiptPathV1(store: string, origin: string): string {
  if (!new RegExp(schema.definitions.Origin.pattern, "u").test(origin)) throw Error("Invalid graph adoption destination");
  return join(store, "v1", "origins", origin, policy.filename);
}

/** 🔏 Verifies the enrolled destination's signature and the exact whole head the observer expects. */
export async function verifyWholeGenerationAdoptionReceiptV1(
  registry: FleetRegistryV1, expected: Readonly<{ origin: string; head: GenerationHeadCandidateV1 }>, value: unknown,
): Promise<WholeGenerationAdoptionReceiptV1 | undefined> {
  if (!validEnvelope(value)) return;
  const { receipt, signature } = value;
  if (receipt.destination !== expected.origin || receipt.destination === receipt.head.origin || !sameCandidate(receipt.head, expected.head)) return;
  if (await verifyFleetDocumentV1(registry, receipt.destination, receiptRole, policy.label, signedText(receipt), signature)) return receipt;
}

async function readBounded(path: string): Promise<string | undefined> {
  const file = await open(path, "r").catch((error: NodeJS.ErrnoException) => {
    if (error.code !== "ENOENT") throw error;
  });
  if (!file) return;
  try {
    const info = await file.stat();
    if (!info.isFile() || info.size > policy.maximumBytes) return;
    const bytes = Buffer.alloc(info.size + 1);
    let length = 0;
    while (length < bytes.length) {
      const { bytesRead } = await file.read(bytes, length, bytes.length - length, length);
      if (!bytesRead) break;
      length += bytesRead;
    }
    if (length === info.size) return new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(0, length));
  } finally {
    await file.close();
  }
}

async function replaceReceipt(path: string, text: string, mode: number, signal?: AbortSignal): Promise<void> {
  signal?.throwIfAborted();
  const directory = dirname(path), temporary = join(directory, ".ga-" + randomUUID());
  await mkdir(directory, { recursive: true, mode: 0o700 });
  const file = await open(temporary, "wx", mode);
  try {
    try {
      await file.writeFile(text);
      await file.sync();
    } finally {
      await file.close();
    }
    signal?.throwIfAborted();
    await rename(temporary, path);
  } catch (error) {
    await rm(temporary, { force: true }).catch(() => undefined);
    throw error;
  }
}

function ownEnvelope<Proof extends Intent>(location: Location, text: string, definition: "Envelope" | "IntentEnvelope", label: string): Envelope<Proof> | undefined {
  let value: unknown;
  try { value = JSON.parse(text); } catch { return; }
  if (validateJsonSchemaSubset(schema.definitions[definition], value, schema).length) return;
  const envelope = value as Envelope<Proof>, receipt = envelope.receipt;
  if (receipt.destination !== location.origin || receipt.head.origin === location.origin) return;
  const proof = signFleetDocumentV1(location.root, receiptRole, label, signedText(receipt));
  if (proof?.origin === location.origin && proof.signature === envelope.signature) return envelope;
}

async function mirror(location: Location, text: string, incoming: Envelope): Promise<boolean> {
  const path = wholeGenerationAdoptionReceiptPathV1(location.store, location.origin);
  const timeout = AbortSignal.timeout(policy.mirrorMilliseconds);
  const signal = location.signal ? AbortSignal.any([location.signal, timeout]) : timeout;
  if (signal.aborted) return false;
  let cancelled: (() => void) | undefined;
  try {
    const off = new Promise<false>(resolve => {
      cancelled = () => resolve(false);
      signal.addEventListener("abort", cancelled, { once: true });
    });
    const write = (async () => {
      const current = await readBounded(path);
      if (current === text) return true;
      if (current !== undefined) {
        const existing = ownEnvelope<WholeGenerationAdoptionReceiptV1>(location, current, "Envelope", policy.label);
        if (existing && existing.receipt.completedMs >= incoming.receipt.completedMs) return false;
      }
      await replaceReceipt(path, text, 0o644, signal);
      return true;
    })().catch(() => false);
    return await Promise.race([write, off]);
  } finally {
    if (cancelled) signal.removeEventListener("abort", cancelled);
  }
}

/** 🔁 Retries mirroring a receipt already signed and atomically written in local sidecar state. */
export async function mirrorWholeGenerationAdoptionReceiptV1(location: Location): Promise<boolean> {
  if (location.signal?.aborted) return false;
  try {
    const text = await readBounded(join(location.localDirectory, policy.filename));
    if (text === undefined) return false;
    const value = ownEnvelope<WholeGenerationAdoptionReceiptV1>(location, text, "Envelope", policy.label);
    return value !== undefined && await mirror(location, text, value);
  } catch {
    return false;
  }
}

function intentFor(location: Location, head: GenerationHeadCandidateV1, local: LocalWholeGenerationAdoptionV1) {
  return { version: policy.version as 1, destination: location.origin, head, localGeneration: local.generation, localManifest: local.localManifest };
}

async function writeLocalProof<Proof extends Intent>(location: Location, receipt: Proof, filename: string, label: string, signal?: AbortSignal) {
  const proof = signFleetDocumentV1(location.root, receiptRole, label, signedText(receipt));
  if (proof?.origin !== location.origin) return;
  const envelope: Envelope<Proof> = { receipt, signature: proof.signature };
  const text = JSON.stringify(envelope) + "\n";
  await replaceReceipt(join(location.localDirectory, filename), text, 0o600, signal);
  return { envelope, text };
}

/** 🗂️ Records the exact planned pointer under the graph writer fence before publication. */
export async function stageWholeGenerationAdoptionIntentV1(
  location: Location & Readonly<{ candidate: GenerationHeadCandidateV1; local: LocalWholeGenerationAdoptionV1 }>,
): Promise<void> {
  const intent = intentFor(location, location.candidate, location.local);
  if (intent.head.origin === intent.destination || validateJsonSchemaSubset(schema.definitions.Intent, intent, schema).length ||
      !await writeLocalProof(location, intent, policy.intentFilename, policy.intentLabel, location.signal))
    throw Error("Could not attest the planned whole graph adoption");
}

/** 🩹 Reissues local evidence only when an authenticated intent names the verified current seal. */
export async function recoverWholeGenerationAdoptionReceiptV1(
  location: Location & Readonly<{ local: LocalWholeGenerationAdoptionV1; now?: () => number }>,
): Promise<WholeGenerationAdoptionReceiptV1 | undefined> {
  location.signal?.throwIfAborted();
  const text = await readBounded(join(location.localDirectory, policy.intentFilename));
  if (text === undefined) return;
  const intent = ownEnvelope<Intent>(location, text, "IntentEnvelope", policy.intentLabel)?.receipt;
  if (!intent || intent.localGeneration !== location.local.generation || intent.localManifest !== location.local.localManifest) return;
  const held = await readBounded(join(location.localDirectory, policy.filename));
  const existing = held === undefined ? undefined : ownEnvelope<WholeGenerationAdoptionReceiptV1>(location, held, "Envelope", policy.label)?.receipt;
  if (existing?.localGeneration === intent.localGeneration && existing.localManifest === intent.localManifest && sameCandidate(existing.head, intent.head)) return existing;
  const receipt = { ...intent, completedMs: (location.now ?? Date.now)() };
  if (validReceipt(receipt) && await writeLocalProof(location, receipt, policy.filename, policy.label, location.signal)) return receipt;
}

/** 🧾 Signs the verified restore-only outcome and persists it locally before mirroring it. */
export async function publishWholeGenerationAdoptionReceiptV1(
  location: Location & Readonly<{ candidate: GenerationHeadCandidateV1; outcome: RestoreExpectedWholeGenerationOutcomeV1; now?: () => number }>,
): Promise<WholeGenerationAdoptionReceiptV1 | undefined> {
  const { candidate, outcome } = location;
  if (outcome.status !== "restored" || !sameHead(outcome.head, candidate) || candidate.origin === location.origin) return;
  const receipt = { ...intentFor(location, candidate, outcome), completedMs: (location.now ?? Date.now)() };
  if (!validReceipt(receipt)) return;
  const saved = await writeLocalProof(location, receipt, policy.filename, policy.label);
  if (!saved) return;
  await mirror(location, saved.text, saved.envelope);
  return receipt;
}
