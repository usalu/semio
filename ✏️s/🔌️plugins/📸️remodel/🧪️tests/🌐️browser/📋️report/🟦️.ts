import { createHash } from "node:crypto";
import { decodeDocumentArchiveBytes } from "@semio-tech/framework/wire-contracts";
import { resolve, relative, isAbsolute, sep } from "node:path";
import scenarios from "../🧫️fixtures/🔣️.json" with { type: "json" };
import { unwrapBinaryControlled } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🟦️.ts";
import { NativeDecodeControl } from "../../../../../../🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/🛬️decode/🟦️.ts";

/** 📋️ Records only browser-observed evidence, leaving every unperformed scenario explicit. */
export type RemodelBrowserReportV1 = {
  host: string;
  scenarios: { id: string; description: string; status: "unexecuted" | "passed" | "failed" | "unsupported"; evidence: string[] }[];
  faults: string[];
};

/** 🧾️ Starts the full acceptance checklist without treating planned work as successful. */
export function initialRemodelBrowserReportV1(host: string): RemodelBrowserReportV1 {
  return { host, scenarios: scenarios.map(row => ({ ...row, status: "unexecuted", evidence: [] })), faults: [] };
}


/** 🪪️ Finds the editor chip without claiming a relationship to any window body. */
export function remodelEditorPresentV1(observed: readonly { app: string | null; role: string | null }[]): boolean {
  return observed.some(item => item.app === "s.remodel.remodeling@1/*#editor" && item.role === "editor");
}

/** 🪟️ Requires a visible, populated editor body tied to an actual document scope. */
export function remodelBoundEditorV1(rows: readonly { app: string | null; role: string | null; instance: string | null; window: string | null; kind: string | null; space: string | null; document: string | null; visible: boolean; populated: boolean }[]): boolean {
  return rows.some(row => row.app === "s.remodel.remodeling@1/*#editor" && row.role === "editor" && [row.instance, row.window, row.kind, row.space, row.document].every(value => value !== null && value.trim().length > 0) && row.visible && row.populated);
}

/** 📤️ Serializes a complete report as a standalone JSON document. */
export function serializeRemodelBrowserReportV1(report: RemodelBrowserReportV1): string {
  return JSON.stringify(report, null, 2) + "\n";
}

/** 🎛️ Requires one explicit host, Hub, output, language and artifact kind for the shared journey. */
export function validateRemodelTwoHumanArgumentsV1(segments: readonly string[]): void {
  const allowed = new Set(["--hub", "--serve", "--serve-b", "--journey", "--locale", "--kinds", "--space", "--tag", "--out", "--admin-capability", "--users", "--max-create-to-mounted-ms", "--max-open-to-mounted-ms", "--mount-budget-ms"]);
  const seen = new Set<string>();
  for (let index = 0; index < segments.length; index += 2) {
    const flag = segments[index], value = segments[index + 1];
    if (!flag || !allowed.has(flag) || seen.has(flag) || !value || value.startsWith("--")) throw Error("Invalid shared journey arguments");
    seen.add(flag);
  }
  for (const flag of ["--hub", "--serve", "--out", "--locale", "--kinds"]) {
    const positions = segments.flatMap((value, index) => value === flag ? [index] : []);
    if (positions.length !== 1 || !segments[positions[0]! + 1] || segments[positions[0]! + 1]!.startsWith("--")) throw Error("Missing or duplicate " + flag);
  }
  const value = (flag: string) => segments[segments.indexOf(flag) + 1]!;
  for (const flag of ["--hub", "--serve"]) { const url = new URL(value(flag)); if (!["http:", "https:"].includes(url.protocol) || url.username || url.password) throw Error("Invalid " + flag); }
  const otherServe = segments.indexOf("--serve-b");
  if (otherServe !== -1 && (segments.lastIndexOf("--serve-b") !== otherServe || segments[otherServe + 1] !== value("--serve"))) throw Error("One existing host is required");
  const out = resolve(value("--out")), kind = value("--kinds");
  const tagIndex = segments.indexOf("--tag"), tag = tagIndex < 0 ? null : segments[tagIndex + 1];
  if (!out.split(/[\\/]/u).includes("🗑️generated") || out.length + 100 > 256 || !/^[a-zA-Z0-9._-]{1,40}$/u.test(kind) || kind === "." || kind === "..") throw Error("Invalid kind or generated output");
  if (tagIndex >= 0 && (segments.lastIndexOf("--tag") !== tagIndex || !tag || !/^[a-zA-Z0-9_-]{1,32}$/u.test(tag))) throw Error("Invalid output tag");
  if (!["en", "de"].includes(value("--locale"))) throw Error("Invalid locale");
  if (seen.has("--journey") && value("--journey") !== "edit") throw Error("The acceptance phases require the editor journey");
}

/** ✅️ Accepts only the complete authored checklist with nonempty observed evidence and no faults. */
export function remodelAcceptanceSatisfiedV1(report: RemodelBrowserReportV1): boolean {
  const expected = new Set(scenarios.map(row => row.id));
  return report.faults.length === 0 && report.scenarios.length === expected.size &&
    report.scenarios.every(row => expected.delete(row.id) && row.status === "passed" && row.evidence.some(item => item.trim().length > 0)) && expected.size === 0;
}


/** 🖼️ Resolves the authored image inputs in place under the same repository. */
export function remodelInputsV1(repoRoot: string, id: string): string[] {
  const root = resolve(repoRoot), row = scenarios.find(item => item.id === id);
  if (!row || !("assets" in row) || !row.assets || row.assets.length === 0) throw Error("Authored media inputs are required");
  return row.assets.map(asset => {
    const path = resolve(root, asset), inside = relative(root, path);
    if (!inside || inside === ".." || inside.startsWith(".." + sep) || isAbsolute(inside) || path.length > 256) throw Error("Image input must stay inside the repository");
    return path;
  });
}


/** 🚦️ Admits UI interactions only after the host publishes its ready beacon. */
export function remodelHostReadyV1(beacon: string | null): boolean {
  return beacon !== null && beacon.startsWith("ready:");
}

/** 🖼️ Requires the new frame count and asset delta to agree with the other editor while retaining prior streams. */
export function mediaImportObservedV1(before: readonly number[], after: readonly number[], peer: readonly number[], frames: number, prior: readonly string[], streams: readonly { id: string; text: string }[]): boolean {
  if (before.length !== 2 || after.length !== 2 || peer.length !== 2 || !Number.isSafeInteger(frames) || frames <= 0) return false;
  if (after[0] !== before[0]! + 1 || after[1] !== before[1]! + frames || peer[0] !== after[0] || peer[1] !== after[1]) return false;
  const ids = new Set(streams.map(stream => stream.id));
  return ids.size === streams.length && streams.every(stream => stream.id.trim().length > 0) && new Set(prior).size === prior.length && prior.every(id => id.trim().length > 0 && ids.has(id)) && streams.some(stream => !prior.includes(stream.id) && new RegExp("(?:^|\\D)" + frames + "\\s+(?:frames|bilder)(?:\\D|$)", "iu").test(stream.text));
}

/** 🏁 Requires an unchanged pre-finalize document and a shared non-placeholder reconstructed mesh afterwards. */
export function finalizationObservedV1(baseline: readonly string[] | null, before: readonly string[] | null, peerBefore: readonly string[] | null, after: readonly string[] | null, peerAfter: readonly string[] | null, finalized: boolean): boolean {
  return finalized && baseline !== null && before !== null && after !== null && baseline.length === 5 && before.length === 5 && after.length === 5 && JSON.stringify(baseline) === JSON.stringify(before) && JSON.stringify(before) === JSON.stringify(peerBefore) && JSON.stringify(before) !== JSON.stringify(after) && JSON.stringify(after) === JSON.stringify(peerAfter) && /reconstructed/iu.test(after[0] ?? "") && /(?:^|\D)[1-9]\d*\s+(?:vertices|eckpunkte)/iu.test(after[0] ?? "");
}

/** 🛑 Requires real pause, step, resume and abort observations with identical shared products. */
export function abortObservedV1(stepped: boolean, resumed: boolean, aborted: boolean, before: readonly string[] | null, after: readonly string[] | null, peer: readonly string[] | null): boolean {
  return stepped && resumed && aborted && before !== null && before.length === 5 && JSON.stringify(before) === JSON.stringify(after) && JSON.stringify(after) === JSON.stringify(peer);
}

/** ↩️ Requires one undo and redo to change and restore the same shared products. */
export function undoRedoObservedV1(finalized: boolean, before: readonly string[] | null, undo: readonly string[] | null, peerUndo: readonly string[] | null, redo: readonly string[] | null, peerRedo: readonly string[] | null): boolean {
  return finalized && before !== null && undo !== null && before.length === 5 && undo.length === 5 && JSON.stringify(before) !== JSON.stringify(undo) && JSON.stringify(undo) === JSON.stringify(peerUndo) && JSON.stringify(before) === JSON.stringify(redo) && JSON.stringify(redo) === JSON.stringify(peerRedo);
}

/** 🗄️ Validates actual public archive bytes and retains a bounded comparison witness. */
export function archiveWitnessV1(name: string, bytes: Uint8Array, mime: string | null) {
  if (!name.endsWith(".semio-archive")) throw Error("Unexpected archive filename");
  const decoded = decodeDocumentArchiveBytes(bytes);
  return { sha256: createHash("sha256").update(bytes).digest("hex"), bytes: bytes.length, parentBytes: decoded.parent_pack.length, members: decoded.members.length, mime, raw: bytes };
}

/** ⚖️ Compares the complete downloaded archives byte for byte. */
export function archiveBytesEqualV1(left: Uint8Array, right: Uint8Array): boolean {
  return left.length === right.length && Buffer.from(left.buffer, left.byteOffset, left.byteLength).equals(Buffer.from(right.buffer, right.byteOffset, right.byteLength));
}

/** ❄️ Requires equal nonempty-member archives for the cold-reopen byte witness. */
export function coldArchiveStatusV1(left: Uint8Array, right: Uint8Array, members: number): "passed" | "failed" | "unexecuted" {
  return left.length === 0 || right.length === 0 ? "unexecuted" : !archiveBytesEqualV1(left, right) ? "failed" : Number.isSafeInteger(members) && members > 0 ? "passed" : "unexecuted";
}

/** 📸 Validates the declared snapshot download and its native binary envelope framing. */
export async function snapshotWitnessV1(name: string, bytes: Uint8Array, mime: string | null, signal: AbortSignal) {
  signal.throwIfAborted();
  if (name !== "remodeling-snapshot.pack" || mime !== "application/vnd.semio.pack" || bytes.length > 32 * 1024 * 1024) throw Error("Invalid snapshot download");
  const body = await unwrapBinaryControlled(bytes, "remodeling.remodeling", "pack", 1, new NativeDecodeControl(0, () => !signal.aborted, signal));
  if (body.length === 0) throw Error("Empty snapshot payload");
  return { bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex"), raw: bytes };
}

/** ↩️ Requires finalization to change the snapshot and single undo/redo to restore its exact heads. */
export function snapshotUndoRedoObservedV1(before: Uint8Array, final: Uint8Array, undo: Uint8Array, redo: Uint8Array): boolean {
  return before.length > 0 && final.length > 0 && !archiveBytesEqualV1(before, final) && archiveBytesEqualV1(before, undo) && archiveBytesEqualV1(final, redo);
}

/** ⛔ Requires an observed corrupt-image refusal and unchanged exact archive and peer counts. */
export function corruptImportObservedV1(alert: string, sameArchive: boolean, before: readonly number[], after: readonly number[], peer: readonly number[]): boolean {
  return /png|image|bild/iu.test(alert) && /truncated|invalid|corrupt|ungültig|beschädigt/iu.test(alert) && sameArchive && before.length === 2 && JSON.stringify(before) === JSON.stringify(after) && JSON.stringify(after) === JSON.stringify(peer);
}
