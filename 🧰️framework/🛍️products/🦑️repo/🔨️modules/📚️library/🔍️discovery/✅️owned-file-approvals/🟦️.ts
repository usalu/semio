import { parseGeneratorPreviewLiteralCommandV1 } from "../../🏭️generator/👁️preview/🔤️arguments/🟦️.ts";
import { reservedDocumentationBasename } from "../../../../../../📦️packages/🟦️typescript/🪪️identity/🛣️path/🟦️.ts";
import { ephemeralMap } from "../../../../../../🔨️modules/🫧️transient/🟦️.ts";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, constants, fstatSync, lstatSync, openSync, readFileSync, readSync } from "node:fs";
import { basename, dirname, join, posix } from "node:path";
import type { CurrentSourceApproval, CurrentSourceApprovalResult, CurrentSourceApprovalSnapshot, SemanticExactOwnedFileProjectionContract, SemanticOwnedDocumentCorrection, SemanticOwnedInputFileSnapshot } from "../📜️taxonomy-contracts/🟦️.ts";
import { type GeneratorContract, type OwnerProjectionEntry, type OwnerProjectionPolicy, exactOwnerPath, semanticOwnedSourceBasename } from "../📦️package-projection-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { generatorPreviewScriptArguments } from "../🛠️generator-commands/🟦️.ts";

/** 📐️ Parses stable source identities in a closed current approval registry without reading any path. */
export function parseCurrentSourceApprovals(input: unknown): Readonly<Record<string, CurrentSourceApproval>> {
  const fail = (message: string): never => { throw new Error("current-source-approval-invalid: " + message); };
  const record = (value: unknown, keys: readonly string[], label: string): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join("\0") !== [...keys].sort().join("\0")) return fail(label + " fields");
    return value as Record<string, unknown>;
  };
  if (!input || typeof input !== "object" || Array.isArray(input)) return fail("registry fields");
  const result: Record<string, CurrentSourceApproval> = {}, sources = new Set<string>();
  const tuple = (value: unknown, label: string): CurrentSourceApproval["currentPreimage"] => {
    const selected = record(value, ["sha256", "size", "mode"], label);
    if (typeof selected.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(selected.sha256) || selected.mode !== "0644" || !Number.isSafeInteger(selected.size) || (selected.size as number) < 0) return fail(label + " tuple");
    return { sha256: selected.sha256, size: selected.size as number, mode: selected.mode };
  };
  for (const [id, value] of Object.entries(input)) {
    if (!/^[a-z][a-z0-9-]*$/u.test(id)) return fail("approval identity");
    const row = record(value, ["sourcePath", "baselineCommit", "baselineBlob", "baselinePreimage", "currentPreimage", "approvalPath", "approvalSha256"], id);
    if (typeof row.baselineCommit !== "string" || !/^[0-9a-f]{40}$/u.test(row.baselineCommit) || typeof row.baselineBlob !== "string" || !/^[0-9a-f]{40}$/u.test(row.baselineBlob)) return fail(id + " baseline identity");
    if (!exactOwnerPath(row.sourcePath) || /[:*?"<>|]/u.test(row.sourcePath) || semanticOwnedSourceBasename(row.sourcePath) !== "README.md" || Buffer.from(row.sourcePath).toString("utf8") !== row.sourcePath || sources.has(row.sourcePath)) return fail(id + " duplicate or invalid source identity");
    if (!exactOwnerPath(row.approvalPath) || /[:*?"<>|]/u.test(row.approvalPath) || !row.approvalPath.endsWith(".json") || Buffer.from(row.approvalPath).toString("utf8") !== row.approvalPath || typeof row.approvalSha256 !== "string" || !/^[0-9a-f]{64}$/u.test(row.approvalSha256)) return fail(id + " approval identity");
    const baselinePreimage = tuple(row.baselinePreimage, "baseline"), currentPreimage = tuple(row.currentPreimage, "current");
    if (baselinePreimage.sha256 === currentPreimage.sha256) return fail(id + " requires a distinct current preimage");
    sources.add(row.sourcePath);
    result[id] = { sourcePath: row.sourcePath, baselineCommit: row.baselineCommit, baselineBlob: row.baselineBlob, baselinePreimage, currentPreimage, approvalPath: row.approvalPath, approvalSha256: row.approvalSha256 };
  }
  return result;
}

/** 🧮️ Canonicalizes the closed revision envelope with sorted object keys and retained array order. */
function semanticOwnedCurrentRevisionCanonical(value: unknown): string {
  if (Array.isArray(value)) return "[" + value.map(semanticOwnedCurrentRevisionCanonical).join(",") + "]";
  if (value !== null && typeof value === "object") return "{" + Object.keys(value).sort().map((key) => JSON.stringify(key) + ":" + semanticOwnedCurrentRevisionCanonical((value as Record<string, unknown>)[key])).join(",") + "}";
  return JSON.stringify(value);
}

/** 🧭️ Selects a raw owner's current preimage from exact supplied evidence without filesystem or Git access. */
export function semanticExactOwnedFileCurrentPreimageAuthority(catalog: OwnerProjectionPolicy, contract: Pick<SemanticExactOwnedFileProjectionContract, "authorityCatalogPath" | "authorityCatalogSha256">, input: unknown, facts: Readonly<{ path: string; nodeKind: string; contentHash: string; mode: number; size: number; expectations: readonly CurrentSourceApprovalSnapshot[] }>): CurrentSourceApprovalResult {
  const empty = { sourcePath: null, preimage: null, revisionId: null, revisionDigest: null, problems: [] } as const;
  const fail = (message: string): CurrentSourceApprovalResult => ({ ...empty, disposition: "problem", problems: ["current-source-approval-invalid: " + message] });
  let revisions: Readonly<Record<string, CurrentSourceApproval>>;
  try { revisions = parseCurrentSourceApprovals(input); } catch (error) { return { ...empty, disposition: "problem", problems: [error instanceof Error ? error.message : String(error)] }; }
  if (!exactOwnerPath(contract.authorityCatalogPath) || !/^[0-9a-f]{64}$/u.test(contract.authorityCatalogSha256) || !Array.isArray(catalog.entries) || catalog.schemaVersion !== 1 || catalog.kind !== "owner-projection-policy" || Object.keys(catalog).sort().join("\0") !== "entries\0generatorOwners\0kind\0ownerEvidence\0referenceOwners\0schemaVersion") return fail("policy identity");
  if (!exactOwnerPath(facts.path) || /[:*?"<>|]/u.test(facts.path)) return fail("source coordinate");
  const owners = catalog.entries.filter(entry => entry.sourcePath === facts.path);
  if (owners.length === 0) return Object.values(revisions).some(row => row.sourcePath === facts.path) ? fail("approved source owner is absent") : { ...empty, disposition: "none" };
  if (owners.length !== 1) return fail("duplicate owner identity");
  const owner = owners[0]!, selected = Object.entries(revisions).find(([, row]) => row.sourcePath === owner.sourcePath), preimage = selected ? selected[1].currentPreimage : owner.preimage;
  if (!preimage || !/^[0-9a-f]{64}$/u.test(preimage.sha256) || preimage.mode !== "0644" || !Number.isSafeInteger(preimage.size) || preimage.size < 0) return fail("selected preimage tuple");
  if (facts.nodeKind !== "file" || facts.contentHash !== preimage.sha256 || facts.mode !== Number.parseInt(preimage.mode, 8) || facts.size !== preimage.size) return fail("selected source preimage drifted");
  if (!selected) return { ...empty, disposition: "catalog", sourcePath: owner.sourcePath, preimage: { ...preimage } };
  const [id, row] = selected, ownerEvidence = catalog.ownerEvidence[owner.ownerEvidenceId];
  if (ownerEvidence?.kind !== "ordinary-owner-doc" || !Array.isArray(ownerEvidence.evidencePaths) || ownerEvidence.evidencePaths.some(path => !exactOwnerPath(path)) || owner.disposition !== "owner-documentation-relocate" || owner.fixedContractId !== null || owner.generatorOwnerId !== null || owner.projectionContractId !== "exact-owner-readme-projection" || semanticOwnedSourceBasename(owner.sourcePath) !== "README.md" || owner.destinationPath !== posix.dirname(owner.sourcePath) + "/\u{1f4c3}\uFE0Freadme/\u{1f4dd}\uFE0F.md" || !Array.isArray(owner.referenceOwnerIds) || owner.referenceOwnerIds.some((referenceId: string) => !Object.hasOwn(catalog.referenceOwners, referenceId))) return fail("approval is not ordinary owner documentation");
  const sameTuple = (left: unknown, right: Readonly<{ sha256: string; size: number; mode: string }>): boolean => {
    if (!left || typeof left !== "object" || Array.isArray(left)) return false;
    const value = left as Record<string, unknown>;
    return Object.keys(value).sort().join("\0") === "mode\0sha256\0size" && value.sha256 === right.sha256 && value.size === right.size && value.mode === right.mode;
  };
  if (!sameTuple(owner.preimage, row.baselinePreimage)) return fail("policy baseline tuple drifted");
  if (!Array.isArray(facts.expectations) || facts.expectations.length !== 1) return fail("missing or duplicate approval identity");
  const evidence = facts.expectations[0]!;
  if (evidence.path !== row.approvalPath || evidence.nodeKind !== "file" || evidence.mode !== 0o644 || !Array.isArray(evidence.ancestorNodeKinds) || evidence.ancestorNodeKinds.length !== evidence.path.split("/").length - 1 || evidence.ancestorNodeKinds.some((kind: string) => kind !== "directory") || !(evidence.bytes instanceof Uint8Array)) return fail("approval is not an exact no-follow regular input");
  const bytes = Buffer.from(evidence.bytes), text = bytes.toString("utf8");
  if (!Buffer.from(text).equals(bytes) || createHash("sha256").update(bytes).digest("hex") !== row.approvalSha256) return fail("approval bytes drifted");
  let document: Record<string, unknown>;
  try { document = JSON.parse(text); } catch { return fail("approval JSON is invalid"); }
  const keys = (value: unknown, expected: readonly string[]): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value) && Object.keys(value).sort().join("\0") === [...expected].sort().join("\0");
  if (!keys(document, ["schemaVersion", "kind", "sourcePath", "policy", "baseline", "currentPreimage"]) || !keys(document.policy, ["path", "sha256"]) || !keys(document.baseline, ["commit", "blob", "preimage"]) || document.schemaVersion !== 1 || document.kind !== "current-source-approval" || document.sourcePath !== owner.sourcePath || document.policy.path !== contract.authorityCatalogPath || document.policy.sha256 !== contract.authorityCatalogSha256 || document.baseline.commit !== row.baselineCommit || document.baseline.blob !== row.baselineBlob || !sameTuple(document.baseline.preimage, row.baselinePreimage) || !sameTuple(document.currentPreimage, row.currentPreimage)) return fail("approval policy or source-owner binding drifted");
  const envelope = { kind: "exact-owner-current-source-approval-v1", policyIdentity: { path: contract.authorityCatalogPath, sha256: contract.authorityCatalogSha256 }, revisionId: id, revision: row, owner: { sourcePath: owner.sourcePath, destinationPath: owner.destinationPath, ownerEvidenceId: owner.ownerEvidenceId, ownerEvidence, disposition: owner.disposition, fixedContractId: owner.fixedContractId, projectionContractId: owner.projectionContractId, generatorOwnerId: owner.generatorOwnerId, referenceOwners: owner.referenceOwnerIds.map((referenceId: string) => ({ id: referenceId, ...catalog.referenceOwners[referenceId] })) } };
  return { disposition: "revised", sourcePath: owner.sourcePath, preimage: { ...row.currentPreimage }, revisionId: id, revisionDigest: createHash("sha256").update(semanticOwnedCurrentRevisionCanonical(envelope)).digest("hex"), problems: [] };
}

/** 📏️ Validates the closed language-neutral authored-document correction grammar. */
export function parseSemanticOwnedDocumentCorrections(input: unknown): Readonly<Record<string, SemanticOwnedDocumentCorrection>> {
  const record = (value: unknown, keys: readonly string[], label: string): Record<string, unknown> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join("\0") !== [...keys].sort().join("\0")) throw new Error(`Invalid authored document ${label} fields`);
    return value as Record<string, unknown>;
  };
  const rows = record(input, ["repo-library-script-filename-v1"], "registry");
  for (const [id, value] of Object.entries(rows)) {
    const row = record(value, ["contractKind", "activation", "sourcePath", "destinationPath", "preimage", "postimage", "replacementFixedFilenameContractId", "splices", "rationaleRule"], id);
    if (row.contractKind !== "exact-owner-content-splices" || row.activation !== "owner-leaf-move" || row.replacementFixedFilenameContractId !== "root-script" || row.rationaleRule !== "owner-script-filename-documentation-v1" || !exactOwnerPath(row.sourcePath) || semanticOwnedSourceBasename(row.sourcePath) !== "README.md" || !exactOwnerPath(row.destinationPath) || row.destinationPath !== dirname(row.sourcePath) + "/📃️readme/📝️.md") throw new Error(`Invalid authored document ${id} owner or activation`);
    const preimage = record(row.preimage, ["sha256", "mode", "size"], id + " preimage"), postimage = record(row.postimage, ["sha256", "size"], id + " postimage");
    if (typeof preimage.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(preimage.sha256) || preimage.mode !== "0644" || !Number.isSafeInteger(preimage.size) || (preimage.size as number) < 0 || typeof postimage.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(postimage.sha256) || !Number.isSafeInteger(postimage.size) || (postimage.size as number) < 0) throw new Error(`Invalid authored document ${id} preimage or postimage`);
    if (!Array.isArray(row.splices) || row.splices.length !== 10) throw new Error(`Authored document ${id} requires exactly ten filename splices`);
    let previousEnd = 0, previousLine = 0, delta = 0;
    for (const value of row.splices) {
      const splice = record(value, ["line", "startByte", "endByte", "oldValue", "newValue", "linePreimage"], id + " splice");
      if (!Number.isSafeInteger(splice.line) || (splice.line as number) < 1 || (splice.line as number) < previousLine || !Number.isSafeInteger(splice.startByte) || (splice.startByte as number) < previousEnd || !Number.isSafeInteger(splice.endByte) || (splice.endByte as number) > (preimage.size as number) || splice.oldValue !== "script.ts" || splice.newValue !== "📜️script.ts" || (splice.endByte as number) - (splice.startByte as number) !== Buffer.byteLength(splice.oldValue) || typeof splice.linePreimage !== "string" || splice.linePreimage.includes("\n") || !splice.linePreimage.includes(splice.oldValue)) throw new Error(`Invalid, repeated, or overlapping authored document ${id} splice`);
      previousEnd = splice.endByte as number;
      previousLine = splice.line as number;
      delta += Buffer.byteLength(splice.newValue) - Buffer.byteLength(splice.oldValue);
    }
    if ((postimage.size as number) !== (preimage.size as number) + delta) throw new Error(`Authored document ${id} postimage size does not match its splices`);
  }
  return rows as unknown as Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
}

/** 🧩️ Produces exact UTF-16 edit offsets from byte-bound authored authority without modifying the frozen source. */
export function semanticExactOwnedDocumentCorrectionAuthority(catalog: OwnerProjectionPolicy, contract: SemanticExactOwnedFileProjectionContract, facts: Readonly<{ path: string; finalPath: string; content: string; mode: number; moving: boolean }>): Readonly<{ disposition: "none" | "rewrite" | "problem"; splices: readonly Readonly<{ start: number; end: number; oldValue: string; newValue: string; correctionId: string }>[]; problems: readonly string[] }> {
  const failure = (message: string) => ({ disposition: "problem" as const, splices: [], problems: [message] });
  let corrections: Readonly<Record<string, SemanticOwnedDocumentCorrection>>;
  try { corrections = parseSemanticOwnedDocumentCorrections(contract.authoredDocumentCorrections); } catch (error) { return failure(error instanceof Error ? error.message : String(error)); }
  if (!facts.moving) return { disposition: "none", splices: [], problems: [] };
  const selected = Object.entries(corrections).find(([, row]) => row.sourcePath === facts.path);
  if (!selected) return { disposition: "none", splices: [], problems: [] };
  const [id, row] = selected;
  const owner = catalog.entries.find((entry) => entry.sourcePath === row.sourcePath);
  if (!owner || owner.disposition !== "owner-documentation-relocate" || owner.generatorOwnerId !== null || owner.destinationPath !== row.destinationPath || facts.finalPath !== row.destinationPath || owner.preimage.sha256 !== row.preimage.sha256 || owner.preimage.mode !== row.preimage.mode || owner.preimage.size !== row.preimage.size) return failure(`Authored document ${id} does not match its frozen owner move`);
  const bytes = Buffer.from(facts.content), hash = (value: string | Buffer): string => createHash("sha256").update(value).digest("hex");
  if (hash(bytes) !== row.preimage.sha256 || bytes.byteLength !== row.preimage.size || facts.mode !== Number.parseInt(row.preimage.mode, 8)) return failure(`Authored document ${id} source preimage drifted`);
  const lines = facts.content.split("\n");
  const splices: { start: number; end: number; oldValue: string; newValue: string; correctionId: string }[] = [];
  for (const splice of row.splices) {
    const prefix = bytes.subarray(0, splice.startByte), selected = bytes.subarray(splice.startByte, splice.endByte);
    const prefixText = prefix.toString("utf8"), oldValue = selected.toString("utf8");
    if (!Buffer.from(prefixText).equals(prefix) || !Buffer.from(oldValue).equals(selected) || oldValue !== splice.oldValue || prefixText.split("\n").length !== splice.line || lines[splice.line - 1] !== splice.linePreimage) return failure(`Authored document ${id} exact span or line context drifted`);
    splices.push({ start: prefixText.length, end: prefixText.length + oldValue.length, oldValue, newValue: splice.newValue, correctionId: id });
  }
  const result = [...splices].reverse().reduce((text, splice) => text.slice(0, splice.start) + splice.newValue + text.slice(splice.end), facts.content);
  if (hash(result) !== row.postimage.sha256 || Buffer.byteLength(result) !== row.postimage.size) return failure(`Authored document ${id} resulting content drifted`);
  return { disposition: "rewrite", splices, problems: [] };
}

const gitIndexFileModeCache = ephemeralMap<string, ReadonlyMap<string, number>>("framework.products.repo.modules.lib.discovery.component.ts.gitIndexFileModeCache");

/** 🗝️ Stage-zero blob permission bits recorded in Git's index, keyed by the path Git reports relative to `repoRoot`; empty outside a work tree, where the platform default is the only authority. */
function gitIndexFileModes(repoRoot: string): ReadonlyMap<string, number> {
  const cached = gitIndexFileModeCache.get(repoRoot);
  if (cached) return cached;
  const modes = new Map<string, number>();
  try {
    const stdout = execFileSync("git", ["ls-files", "--stage", "-z"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 256 * 1024 * 1024 });
    for (const row of stdout.split("\0")) {
      if (!row) continue;
      const tab = row.indexOf("\t");
      if (tab < 0) continue;
      const [mode, , stage] = row.slice(0, tab).split(" ");
      if (stage !== "0" || (mode !== "100644" && mode !== "100755")) continue;
      modes.set(row.slice(tab + 1).normalize("NFC"), mode === "100755" ? 0o755 : 0o644);
    }
  } catch {}
  gitIndexFileModeCache.set(repoRoot, modes);
  return modes;
}

/** 🔐️ Platform-correct POSIX permission bits of a regular file: the filesystem's own bits where they exist, Git's index mode (else the `0o644` Git assumes for a new file) on Windows, whose NTFS carries none. */
export function posixRegularFileMode(repoRoot: string, path: string, mode: number): number {
  if (process.platform !== "win32") return mode & 0o7777;
  return gitIndexFileModes(repoRoot).get(path.normalize("NFC")) ?? 0o644;
}

export function exactOwnerRegularFile(repoRoot: string, path: string): "file" | "absent" | "invalid" {
  if (!exactOwnerPath(path)) return "invalid";
  const segments = path.split("/");
  for (let index = 0; index < segments.length; index++) {
    let stat;
    try { stat = lstatSync(join(repoRoot, ...segments.slice(0, index + 1))); } catch (error) { return (error as NodeJS.ErrnoException).code === "ENOENT" ? "absent" : "invalid"; }
    if (stat.isSymbolicLink() || (index < segments.length - 1 ? !stat.isDirectory() : !stat.isFile())) return "invalid";
  }
  return "file";
}

/** 🛡️ Captures a safe regular input once and rejects changed descriptors or ancestry. */
export function semanticOwnedInputFileSnapshot(repoRoot: string, path: string, control: Readonly<{ maximumBytes?: number; checkpoint?: () => void }> = {}): SemanticOwnedInputFileSnapshot | null {
  const maximumBytes = control.maximumBytes ?? Number.MAX_SAFE_INTEGER;
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0) throw new Error("Exact owner input byte boundary is invalid");
  control.checkpoint?.();
  if (!exactOwnerPath(path) || /[:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path) throw new Error("Exact owner input has an unsafe coordinate");
  const parts = path.split("/"), witnesses: { path: string; dev: number; ino: number }[] = [];
  let current = repoRoot;
  const root = lstatSync(current);
  if (!root.isDirectory() || root.isSymbolicLink()) throw new Error("Exact owner input repository root is not a no-follow directory");
  witnesses.push({ path: current, dev: root.dev, ino: root.ino });
  for (const [index, part] of parts.entries()) {
    control.checkpoint?.();
    current = join(current, part);
    let node;
    try { node = lstatSync(current); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
    if (node.isSymbolicLink() || (index === parts.length - 1 ? !node.isFile() : !node.isDirectory())) throw new Error("Exact owner input must be a regular file beneath no-follow directories: " + path);
    if (index < parts.length - 1) witnesses.push({ path: current, dev: node.dev, ino: node.ino });
  }
  const before = lstatSync(current), descriptor = openSync(current, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  let retained: Buffer | undefined, complete = false;
  try {
    const node = fstatSync(descriptor);
    if (!node.isFile() || node.dev !== before.dev || node.ino !== before.ino || node.mode !== before.mode || node.size !== before.size || node.mtimeMs !== before.mtimeMs || node.ctimeMs !== before.ctimeMs) throw new Error("Exact owner input changed during open: " + path);
    if (node.size > maximumBytes) throw new Error("Exact owner input exceeds its byte boundary: " + path);
    retained = Buffer.alloc(node.size);
    for (let offset = 0; offset < retained.byteLength;) {
      control.checkpoint?.();
      const count = readSync(descriptor, retained, offset, Math.min(65536, retained.byteLength - offset), offset);
      if (!count) throw new Error("Exact owner input shortened during read: " + path);
      offset += count;
    }
    control.checkpoint?.();
    const bytes = retained, after = fstatSync(descriptor), named = lstatSync(current);
    if (bytes.byteLength !== node.size || after.dev !== node.dev || after.ino !== node.ino || after.mode !== node.mode || after.size !== node.size || after.mtimeMs !== node.mtimeMs || after.ctimeMs !== node.ctimeMs || named.isSymbolicLink() || !named.isFile() || named.dev !== node.dev || named.ino !== node.ino || named.mode !== node.mode || named.size !== node.size || named.mtimeMs !== node.mtimeMs || named.ctimeMs !== node.ctimeMs) throw new Error("Exact owner input changed during read: " + path);
    for (const witness of witnesses) {
      control.checkpoint?.();
      const ancestor = lstatSync(witness.path);
      if (!ancestor.isDirectory() || ancestor.isSymbolicLink() || ancestor.dev !== witness.dev || ancestor.ino !== witness.ino) throw new Error("Exact owner input ancestry changed: " + path);
    }
    const result: SemanticOwnedInputFileSnapshot = { path, nodeKind: "file", contentHash: createHash("sha256").update(bytes).digest("hex"), mode: posixRegularFileMode(repoRoot, path, node.mode), size: bytes.byteLength, ancestorNodeKinds: parts.slice(1).map(() => "directory"), bytes };
    complete = true;
    return result;
  } finally { if (!complete) retained?.fill(0); closeSync(descriptor); }
}

/** 🔐️ Loads only the schema-registered catalog with exact bytes, paths, counts, and owner classifications. */
export function parseOwnerProjectionPolicy(input: unknown, taxonomy: Taxonomy, contract: SemanticExactOwnedFileProjectionContract): OwnerProjectionPolicy {
  const policyPath = (value: unknown): value is string => exactOwnerPath(value) && !/[:*?"<>|]/u.test(value) && Buffer.from(value, "utf8").toString("utf8") === value;
  const object = (value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, any> => {
    if (!value || typeof value !== "object" || Array.isArray(value) || required.some(key => !Object.hasOwn(value, key)) || Object.keys(value).some(key => !required.includes(key) && !optional.includes(key))) throw new Error("Owner projection policy has invalid closed fields");
    return value as Record<string, any>;
  };
  const value = object(input, ["schemaVersion", "kind", "ownerEvidence", "referenceOwners", "generatorOwners", "entries"]);
  if (value.schemaVersion !== 1 || value.kind !== "owner-projection-policy" || !Array.isArray(value.entries)) throw new Error("Owner projection policy identity is invalid");
  const registry = (input: unknown): Record<string, any> => {
    if (!input || typeof input !== "object" || Array.isArray(input) || Object.keys(input).some(id => !/^[a-z][a-z0-9-]*$/u.test(id))) throw new Error("Owner projection policy registry identity is invalid");
    return input as Record<string, any>;
  };
  const ownerEvidence = registry(value.ownerEvidence), referenceOwners = registry(value.referenceOwners), generatorOwners = registry(value.generatorOwners);
  for (const input of Object.values(ownerEvidence)) {
    const evidence = object(input, ["kind", "evidencePaths"], ["expectedPackageName", "private"]);
    if (!(contract.ownerEvidenceKinds as readonly string[]).includes(evidence.kind) || !Array.isArray(evidence.evidencePaths) || evidence.evidencePaths.some(path => !policyPath(path)) || new Set(evidence.evidencePaths).size !== evidence.evidencePaths.length || Object.hasOwn(evidence, "expectedPackageName") && (typeof evidence.expectedPackageName !== "string" || !evidence.expectedPackageName) || Object.hasOwn(evidence, "private") && typeof evidence.private !== "boolean") throw new Error("Owner projection policy evidence is invalid");
  }
  if (JSON.stringify(Object.keys(referenceOwners).sort()) !== JSON.stringify([...contract.referenceOwnerIds].sort()) || JSON.stringify(Object.keys(generatorOwners).sort()) !== JSON.stringify([...contract.generatorOwnerIds].sort())) throw new Error("Owner projection policy consumer registries drifted");
  for (const [id, input] of Object.entries(referenceOwners)) {
    const consumer = object(input, ["kind", "ownerPath"]);
    if (typeof consumer.kind !== "string" || !consumer.kind || typeof consumer.ownerPath !== "string" || !consumer.ownerPath || ["repo-cli-dev-docs-go", "commonmark-scratch-rust-reader", "vscode-package-ignore"].includes(id) && !policyPath(consumer.ownerPath)) throw new Error("Owner projection policy consumer is invalid: " + id);
  }
  for (const input of Object.values(generatorOwners)) { const generator = object(input, ["ownerPath", "target", "currentOutputPath", "requiredOutputPath"]); if (!policyPath(generator.ownerPath) || typeof generator.target !== "string" || !generator.target || !policyPath(generator.currentOutputPath) || !policyPath(generator.requiredOutputPath)) throw new Error("Owner projection policy generator is invalid"); }
  const sources = new Set<string>(), destinations = new Set<string>();
  const entries = value.entries.map(input => {
    const row = object(input, ["sourcePath", "destinationPath", "ownerEvidenceId", "disposition", "fixedContractId", "projectionContractId", "referenceOwnerIds", "generatorOwnerId", "preimage"]) as unknown as OwnerProjectionEntry;
    const sourceBasename = typeof row.sourcePath === "string" ? semanticOwnedSourceBasename(row.sourcePath) : "";
    if (!policyPath(row.sourcePath) || !policyPath(row.destinationPath) || !(contract.sourceBasenames as readonly string[]).includes(sourceBasename) || !contract.allowedDispositions.includes(row.disposition) || !Object.hasOwn(ownerEvidence, row.ownerEvidenceId) || sources.has(row.sourcePath) || destinations.has(row.destinationPath) || Buffer.byteLength(row.destinationPath) > 240) throw new Error("Owner projection policy has invalid or duplicate source/destination ownership");
    const readme = sourceBasename === "README.md", kind = contract.destinationDirectoryKinds[readme ? "readme" : "license"], evidenceKind = ownerEvidence[row.ownerEvidenceId].kind;
    const expectedDisposition = evidenceKind === "package-publication" ? "fixed" : evidenceKind === "third-party-attribution" ? "attribution-relocate" : evidenceKind === "configurable-owner-license" ? "configurable-owner-license-relocate" : evidenceKind === "ticket-evidence" || evidenceKind === "ticket-scratch" ? "generated-evidence-relocate" : "owner-documentation-relocate";
    const fixed = row.disposition === "fixed";
    if (row.disposition !== expectedDisposition || row.destinationPath !== (fixed ? row.sourcePath : dirname(row.sourcePath) + "/" + kind.directoryName + "/" + kind.filename) || row.fixedContractId !== (fixed ? readme ? "bun-package-readme" : "bun-package-license" : null) || row.projectionContractId !== (fixed ? null : readme ? "exact-owner-readme-projection" : "exact-owner-license-projection")) throw new Error("Owner projection policy classification or destination drifted: " + row.sourcePath);
    const preimage = object(row.preimage, ["sha256", "mode", "size"]);
    if (typeof preimage.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(preimage.sha256) || preimage.mode !== "0644" || !Number.isSafeInteger(preimage.size) || preimage.size < 0 || !Array.isArray(row.referenceOwnerIds) || row.referenceOwnerIds.length === 0 || new Set(row.referenceOwnerIds).size !== row.referenceOwnerIds.length || row.referenceOwnerIds.some(id => !Object.hasOwn(referenceOwners, id))) throw new Error("Owner projection policy leaf/reference evidence is invalid: " + row.sourcePath);
    if (row.generatorOwnerId !== null) {
      const generator = generatorOwners[row.generatorOwnerId], registered = taxonomy.generatorContracts[row.generatorOwnerId], currentOutputPath = reservedDocumentationBasename(basename(row.sourcePath)) ? row.sourcePath : row.destinationPath, retiredOutputPath = currentOutputPath === row.sourcePath ? row.destinationPath : row.sourcePath;
      if (!generator || !registered || generator.currentOutputPath !== row.sourcePath || generator.requiredOutputPath !== row.destinationPath || generator.ownerPath !== registered.ownerPath || generator.target !== registered.target || !registered.outputRoots.some(output => output.path === currentOutputPath) || retiredOutputPath !== currentOutputPath && registered.outputRoots.some(output => output.path === retiredOutputPath)) throw new Error("Owner projection policy generator registration drifted: " + row.sourcePath);
    }
    sources.add(row.sourcePath); destinations.add(row.destinationPath);
    return row;
  });
  return { schemaVersion: 1, kind: "owner-projection-policy", entries, ownerEvidence, referenceOwners, generatorOwners };
}

/** 📊️ Derives an owner policy inventory without prescribing a cohort size. */
export function ownerProjectionPolicyCensus(policy: OwnerProjectionPolicy): Readonly<{ fixed: number; license: number; projected: number; readme: number; referenceBindings: number; total: number }> {
  const counts = { fixed: 0, license: 0, projected: 0, readme: 0, referenceBindings: 0, total: policy.entries.length };
  for (const entry of policy.entries) { counts[semanticOwnedSourceBasename(entry.sourcePath) === "README.md" ? "readme" : "license"]++; counts[entry.disposition === "fixed" ? "fixed" : "projected"]++; counts.referenceBindings += entry.referenceOwnerIds.length; }
  return counts;
}

/** 🔐️ Captures and parses only the declared policy through exact no-follow bytes. */
export function semanticExactOwnedFileCatalog(repoRoot: string, taxonomy: Taxonomy, observe?: (snapshot: SemanticOwnedInputFileSnapshot) => void): OwnerProjectionPolicy | null {
  const contract = taxonomy.semanticOwnedFileProjectionContracts["readme-license-owner-leaves-v1"];
  if (contract?.contractKind !== "exact-owner-path-catalog") return null;
  const path = contract.authorityCatalogPath, state = exactOwnerRegularFile(repoRoot, path);
  if (state === "absent") return null;
  if (state !== "file") throw new Error("Owner projection policy must be a regular file beneath non-symlink parents");
  const snapshot = semanticOwnedInputFileSnapshot(repoRoot, path);
  if (!snapshot || snapshot.mode !== 0o644) throw new Error("Owner projection policy disappeared or changed mode: " + path);
  const bytes = Buffer.from(snapshot.bytes), text = bytes.toString("utf8");
  if (createHash("sha256").update(bytes).digest("hex") !== contract.authorityCatalogSha256 || !Buffer.from(text).equals(bytes)) throw new Error("Owner projection policy digest or UTF-8 drift: " + path);
  const policy = parseOwnerProjectionPolicy(JSON.parse(text), taxonomy, contract);
  observe?.(snapshot);
  return policy;
}

/** 🧭️ Resolves an exact owner leaf from language-neutral facts; raw-source bytes authorize projection once. */
export function semanticExactOwnedFileProjectionAuthority(catalog: OwnerProjectionPolicy, facts: Readonly<{ path: string; nodeKind: string; contentHash: string; mode: number; size: number; sourcePresent: boolean; destinationPresent: boolean; destinationPreimage?: Readonly<{ contentHash: string; mode: number; size: number }>; occupiedPaths: readonly string[] }>, current?: Readonly<{ contract: Pick<SemanticExactOwnedFileProjectionContract, "authorityCatalogPath" | "authorityCatalogSha256">; revisions: unknown; expectations: readonly CurrentSourceApprovalSnapshot[] }>): Readonly<{ disposition: "none" | "fixed" | "project" | "regenerate" | "canonical" | "problem"; entry: OwnerProjectionEntry | null; problems: readonly string[]; currentSource?: CurrentSourceApprovalResult }> {
  const entry = catalog.entries.find((entry) => entry.sourcePath === facts.path || entry.destinationPath === facts.path);
  if (!entry) return { disposition: "none", entry: null, problems: [] };
  const problems: string[] = [];
  const fixed = entry.disposition === "fixed", raw = facts.path === entry.sourcePath;
  let preimage = entry.preimage, currentSource: CurrentSourceApprovalResult | undefined;
  if (current) {
    const selected = semanticExactOwnedFileCurrentPreimageAuthority(catalog, current.contract, current.revisions, { ...facts, expectations: current.expectations });
    if (!raw || fixed || selected.disposition !== "revised" || selected.sourcePath !== entry.sourcePath || !selected.preimage) problems.push(...(selected.problems.length ? selected.problems : ["Current source proof does not select this revised raw owner"]));
    else { preimage = selected.preimage; currentSource = selected; }
  }
  const convergentGenerator = entry.generatorOwnerId !== null && facts.destinationPreimage?.contentHash === entry.preimage.sha256 && facts.destinationPreimage?.mode === Number.parseInt(entry.preimage.mode, 8) && facts.destinationPreimage?.size === entry.preimage.size;
  if (facts.nodeKind !== "file") problems.push("Owner leaf must be a regular file");
  if (!fixed && facts.sourcePresent && facts.destinationPresent && !convergentGenerator) problems.push("Raw and projected owner leaves coexist");
  if (raw && (facts.contentHash !== preimage.sha256 || facts.mode !== Number.parseInt(preimage.mode, 8) || facts.size !== preimage.size)) problems.push("Frozen owner leaf preimage drifted");
  const fold = (path: string): string => path.normalize("NFC").replaceAll("\uFE0F", "").toLocaleLowerCase("und");
  if (!fixed && raw && facts.occupiedPaths.some((path) => path !== entry.sourcePath && !(convergentGenerator && path === entry.destinationPath) && fold(path) === fold(entry.destinationPath))) problems.push("Projected owner destination is occupied or folded-colliding");
  return { disposition: problems.length ? "problem" : fixed ? "fixed" : raw ? entry.generatorOwnerId ? "regenerate" : "project" : "canonical", entry, problems, ...(currentSource ? { currentSource } : {}) };
}

export function exactOwnerGeneratorPrestate(repoRoot: string, outputPath: string, generatorId: string, catalog: OwnerProjectionPolicy | null): boolean {
  const entry = catalog?.entries.find((entry) => entry.generatorOwnerId === generatorId && entry.destinationPath === outputPath);
  if (!entry || exactOwnerRegularFile(repoRoot, entry.sourcePath) !== "file") return false;
  const stat = lstatSync(join(repoRoot, entry.sourcePath)), bytes = readFileSync(join(repoRoot, entry.sourcePath));
  return posixRegularFileMode(repoRoot, entry.sourcePath, stat.mode) === Number.parseInt(entry.preimage.mode, 8) && bytes.byteLength === entry.preimage.size && createHash("sha256").update(bytes).digest("hex") === entry.preimage.sha256;
}

/** 🚦 Resolves an exact preview invocation with its required native owner policy. */
export function generatorPreviewExecution(contract: Pick<GeneratorContract, "ownership" | "ownerPath" | "target" | "previewTarget" | "previewArguments">, target: { executor?: unknown; options?: Record<string, unknown> }): { command: string; args: string[]; cwd: string } {
  const owner = contract.ownerPath, options = target.options;
  if (!owner || target.executor !== "repo:script" || !options) throw new Error("Generator target is not the exact owner JSON preview command");
  const direct = ["./📜️script.ts", ...generatorPreviewScriptArguments(contract)];
  const native = ["./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📜️script.ts", "native", "owner-command", "--manifest", `${owner}/Cargo.toml`, "--cwd", owner, "--", "bun", ...direct];
  const framework = "🧰️framework", frameworkOwner = owner.startsWith(`${framework}/`) && !/[\\\s]/u.test(owner) && owner.split("/").every(part => part && part !== "." && part !== "..") ? owner.slice(framework.length + 1) : null;
  const configured = frameworkOwner ? ["./🔨️modules/🏃️process/📜️script.ts", "command", "--config", "./⚙️configuration/🏃️process/🔣️.json", "--cwd", frameworkOwner, "--manifest", `${frameworkOwner}/Cargo.toml`, "--", "bun", ...direct] : null;
  const args = options.cwd === owner ? direct : options.cwd === "." ? native : options.cwd === framework ? configured : null;
  let declared: readonly string[];
  try { declared = parseGeneratorPreviewLiteralCommandV1(options.command); } catch { throw new Error("Generator target is not the exact owner JSON preview command"); }
  const expected = ["bun", ...(args ?? [])];
  if (!args || declared.length !== expected.length || declared.some((value, index) => value !== expected[index])) throw new Error("Generator target is not the exact owner JSON preview command");
  return { command: "bun", args, cwd: options.cwd as string };
}
