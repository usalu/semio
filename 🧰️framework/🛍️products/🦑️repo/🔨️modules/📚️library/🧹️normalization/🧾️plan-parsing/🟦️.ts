import { requireRecord, requireString, requireLiteral, type UnknownRecord } from "@semio-tech/schema-validator-ts";
import { normalizeRelative, utf8Compare } from "../🛣️path/🟦️.ts";
import { sha256 } from "../📁️input/🟦️.ts";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { posix } from "node:path";
import { type OpaqueTreeDigest, PLAN_HASH, PLAN_OPERATION_ID, type ReferenceEdit, type TaxonomyEvidenceMember, type TaxonomyGeneratorNodeRecord, type TaxonomyGeneratorPreviewManifest, type TaxonomyGeneratorPreviewNode, type TaxonomyLeafPreimage, type TaxonomyMove, type TaxonomyMoveSourceAuthority, type TaxonomyNoFollowTreeDigest, type TaxonomyPathPreimage, type TaxonomyReferenceAdapter, type TaxonomyRegeneration, type TaxonomyRemovalAuthority, type TaxonomyViolation, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";

export function planRecord(value: unknown, name: string, requiredKeys: readonly string[], optionalKeys: readonly string[] = []): UnknownRecord {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error(`${name} must be an object`);
  const row = value as UnknownRecord;
  const allowed = new Set([...requiredKeys, ...optionalKeys]);
  const keys = Object.keys(row);
  if (requiredKeys.some((key) => !(key in row)) || keys.some((key) => !allowed.has(key))) throw new Error(`${name} has missing or unknown keys`);
  return row;
}

export function planString(value: unknown, name: string, pattern?: RegExp): string {
  if (typeof value !== "string" || (pattern && !pattern.test(value))) throw new Error(`${name} is invalid`);
  return value;
}

export function planPath(value: unknown, name: string): string {
  const path = planString(value, name);
  if (path === "" || path !== normalizeRelative(path) || path !== path.normalize("NFC")) throw new Error(`${name} is not a canonical repository-relative path`);
  return path;
}

export function planInteger(value: unknown, name: string, maximum = Number.MAX_SAFE_INTEGER): number {
  if (!Number.isSafeInteger(value) || (value as number) < 0 || (value as number) > maximum) throw new Error(`${name} is invalid`);
  return value as number;
}

export function planStringArray(value: unknown, name: string, pattern?: RegExp): string[] {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value.map((entry, index) => planString(entry, `${name}[${index}]`, pattern));
}

export function parseLeafPreimage(value: unknown, name: string): TaxonomyLeafPreimage {
  const base = planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"], ["target"]);
  const row = base.nodeKind === "symlink" ? planRecord(value, name, ["nodeKind", "contentHash", "mode", "size", "target"]) : planRecord(value, name, ["nodeKind", "contentHash", "mode", "size"]);
  if (row.nodeKind !== "file" && row.nodeKind !== "symlink") throw new Error(`${name}.nodeKind is invalid`);
  const contentHash = planString(row.contentHash, `${name}.contentHash`, PLAN_HASH);
  const mode = planInteger(row.mode, `${name}.mode`, 0o7777);
  const size = planInteger(row.size, `${name}.size`);
  if (row.nodeKind === "file") return { nodeKind: "file", contentHash, mode, size };
  const target = planString(row.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { nodeKind: "symlink", contentHash, mode, size, target };
}

export function parsePathPreimage(value: unknown, name: string): TaxonomyPathPreimage {
  const base = planRecord(value, name, ["state"], ["contentHash", "mode", "size", "target"]);
  if (!["absent", "directory", "file", "symlink"].includes(String(base.state))) throw new Error(`${name}.state is invalid`);
  if (base.state === "absent" || base.state === "directory") {
    if (Object.keys(base).length !== 1) throw new Error(`${name} absent preimage cannot have payload`);
    return { state: base.state };
  }
  if (typeof base.contentHash !== "string" || !PLAN_HASH.test(base.contentHash) || !Number.isSafeInteger(base.mode) || !Number.isSafeInteger(base.size)) throw new Error(`${name} present preimage requires hash, mode and size`);
  const contentHash = base.contentHash, mode = planInteger(base.mode, `${name}.mode`, 0o7777), size = planInteger(base.size, `${name}.size`);
  if (base.state === "file") {
    if (base.target !== undefined) throw new Error(`${name} file preimage cannot have a symlink target`);
    return { state: "file", contentHash, mode, size };
  }
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { state: "symlink", contentHash, mode, size, target };
}

export function parseNoFollowTreeDigest(value: unknown, name: string): TaxonomyNoFollowTreeDigest {
  const row = planRecord(value, name, ["algorithm", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-no-follow-merkle-v1") throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}

export function dispositionOperationId(domain: string, value: object): string {
  return sha256(`${domain}\u0000${canonicalJson(value)}`).slice(0, 24);
}

function parseEvidenceMember(value: unknown, name: string): TaxonomyEvidenceMember {
  const row = planRecord(value, name, ["sourcePath", "finalPath", "disposition", "preimage"]);
  if (!["remove", "retain", "relocate"].includes(String(row.disposition))) throw new Error(`${name}.disposition is invalid`);
  return { sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), finalPath: planPath(row.finalPath, `${name}.finalPath`), disposition: row.disposition as TaxonomyEvidenceMember["disposition"], preimage: parseLeafPreimage(row.preimage, `${name}.preimage`) };
}

export function parseRemovalAuthority(value: unknown, name: string): TaxonomyRemovalAuthority {
  const candidate = planRecord(value, name, ["kind"], ["evidenceSetDigest", "retainedFinalPath", "members", "catalogPath", "catalogContentHash", "caseId", "sourcePath", "sourcePreimage", "disposition", "contractId", "ownerPath", "manifestPath", "manifestPreimage", "status", "contentState", "serializedInputPath", "expectedViolationCode", "authorityDigest", "generatorContractId", "destinationPath", "outputPreimage", "packageId"]);
  if (candidate.kind === "nested-cargo-generated-source") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "packageId", "generatorContractId", "destinationPath", "sourcePreimage", "authorityDigest"]);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, name + ".sourcePreimage");
    if (sourcePreimage.nodeKind !== "file") throw new Error(name + " requires the exact generated WGPU source file");
    const result = { kind: "nested-cargo-generated-source" as const, catalogPath: planPath(row.catalogPath, name + ".catalogPath"), catalogContentHash: planString(row.catalogContentHash, name + ".catalogContentHash", PLAN_HASH), packageId: requireLiteral(row.packageId, name + ".packageId", ["wgpu-renderer"] as const), generatorContractId: planString(row.generatorContractId, name + ".generatorContractId"), destinationPath: planPath(row.destinationPath, name + ".destinationPath"), sourcePreimage, authorityDigest: planString(row.authorityDigest, name + ".authorityDigest", PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(name + ".authorityDigest does not match nested Cargo generated source authority");
    return result;
  }
  if (candidate.kind === "exact-owner-generated-source") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "generatorContractId", "destinationPath", "outputPreimage", "authorityDigest"]);
    const outputPreimage = parseLeafPreimage(row.outputPreimage, name + ".outputPreimage");
    if (outputPreimage.nodeKind !== "file") throw new Error(name + " generated owner output must be a regular file");
    const result = { kind: "exact-owner-generated-source" as const, catalogPath: planPath(row.catalogPath, name + ".catalogPath"), catalogContentHash: planString(row.catalogContentHash, name + ".catalogContentHash", PLAN_HASH), generatorContractId: planString(row.generatorContractId, name + ".generatorContractId"), destinationPath: planPath(row.destinationPath, name + ".destinationPath"), outputPreimage, authorityDigest: planString(row.authorityDigest, name + ".authorityDigest", PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(name + ".authorityDigest does not match generated source authority");
    return result;
  }
  if (candidate.kind === "byte-and-mode-identical") {
    const row = planRecord(value, name, ["kind", "evidenceSetDigest", "retainedFinalPath", "members"]);
    if (!Array.isArray(row.members) || row.members.length < 2) throw new Error(`${name}.members must contain complete retained evidence`);
    const members = row.members.map((entry, index) => parseEvidenceMember(entry, `${name}.members[${index}]`));
    const keys = members.map((entry) => Buffer.from(entry.sourcePath).toString("hex"));
    if (keys.some((key, index) => index > 0 && requireTaxonomyValue(keys[index - 1],"keys[index - 1]") >= key)) throw new Error(`${name}.members are not unique and bytewise path sorted`);
    const identity = canonicalJson(requireTaxonomyValue(members[0],"members[0]").preimage);
    if (members.some((entry) => canonicalJson(entry.preimage) !== identity)) throw new Error(`${name}.members are not byte, kind, mode and size identical`);
    const retainedFinalPath = planPath(row.retainedFinalPath, `${name}.retainedFinalPath`);
    if (!members.some((entry) => entry.disposition !== "remove" && entry.finalPath === retainedFinalPath)) throw new Error(`${name}.retainedFinalPath has no retained member`);
    const digestible = { algorithm: "sha256-byte-mode-evidence-set-v1", members, retainedFinalPath };
    const evidenceSetDigest = planString(row.evidenceSetDigest, `${name}.evidenceSetDigest`, PLAN_HASH);
    if (evidenceSetDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.evidenceSetDigest does not match its members`);
    return { kind: "byte-and-mode-identical", evidenceSetDigest, retainedFinalPath, members };
  }
  if (candidate.kind === "owner-manifest-status") {
    const row = planRecord(value, name, ["kind", "contractId", "ownerPath", "manifestPath", "manifestPreimage", "status", "contentState", "authorityDigest"]);
    const manifestPreimage = parseLeafPreimage(row.manifestPreimage, `${name}.manifestPreimage`);
    if (manifestPreimage.nodeKind !== "file") throw new Error(`${name} does not use the exact closed-empty ticket authority`);
    const result = { kind: "owner-manifest-status" as const, contractId: requireLiteral(row.contractId, `${name}.contractId`, ["ticket-important-markdown-v1"] as const), ownerPath: planPath(row.ownerPath, `${name}.ownerPath`), manifestPath: planPath(row.manifestPath, `${name}.manifestPath`), manifestPreimage, status: requireLiteral(row.status, `${name}.status`, ["closed"] as const), contentState: requireLiteral(row.contentState, `${name}.contentState`, ["zero-byte"] as const), authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  if (candidate.kind === "exact-path-mutation") {
    const row = planRecord(value, name, ["kind", "catalogPath", "catalogContentHash", "caseId", "sourcePath", "sourcePreimage", "disposition", "authorityDigest"]);
    const sourcePreimage = parseLeafPreimage(row.sourcePreimage, `${name}.sourcePreimage`);
    if (sourcePreimage.nodeKind !== "file" || sourcePreimage.size !== 0) throw new Error(`${name} does not use exact empty-file removal evidence`);
    const result = { kind: "exact-path-mutation" as const, catalogPath: planPath(row.catalogPath, `${name}.catalogPath`), catalogContentHash: planString(row.catalogContentHash, `${name}.catalogContentHash`, PLAN_HASH), caseId: planString(row.caseId, `${name}.caseId`), sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), sourcePreimage, disposition: requireLiteral(row.disposition, `${name}.disposition`, ["remove"] as const), authorityDigest: planString(row.authorityDigest, `${name}.authorityDigest`, PLAN_HASH) };
    const { authorityDigest: _digest, ...digestible } = result;
    if (result.authorityDigest !== sha256(canonicalJson(digestible))) throw new Error(`${name}.authorityDigest does not match its authority`);
    return result;
  }
  
  throw new Error(`${name}.kind is invalid`);
}

export function parseReferenceEdit(value: unknown, name: string): ReferenceEdit {
  const row = planRecord(value, name, ["path", "adapter", "structuredLocation", "oldValue", "newValue", "preimage"]);
  const adapters: readonly TaxonomyReferenceAdapter[] = ["rust", "typescript", "go", "python", "dotnet", "native", "json", "jsonc", "toml", "yaml", "xml", "markdown", "gherkin"];
  if (!adapters.includes(row.adapter as TaxonomyReferenceAdapter)) throw new Error(`${name}.adapter is invalid`);
  const preimage = parseLeafPreimage(row.preimage, `${name}.preimage`);
  if (preimage.nodeKind !== "file") throw new Error(`${name}.preimage must be a regular file`);
  return { path: planPath(row.path, `${name}.path`), adapter: row.adapter as TaxonomyReferenceAdapter, structuredLocation: planString(row.structuredLocation, `${name}.structuredLocation`), oldValue: planString(row.oldValue, `${name}.oldValue`), newValue: planString(row.newValue, `${name}.newValue`), preimage };
}

/** 🪪️ Parses exact role-labelled current-source evidence without authenticating its current bytes. */
function parseMoveSourceAuthority(value: unknown, name: string): TaxonomyMoveSourceAuthority {
  const row = planRecord(value, name, ["kind", "revisionId", "revisionDigest", "inputs"]);
  if (row.kind !== "exact-owner-current-source-approval-v1" || typeof row.revisionId !== "string" || !/^[a-z][a-z0-9-]*$/u.test(row.revisionId)) throw new Error(`${name} has an unsupported current-source revision`);
  if (!Array.isArray(row.inputs) || row.inputs.length !== 3) throw new Error(`${name}.inputs must contain exactly three role-labelled files`);
  const roles = new Set<string>(), paths = new Set<string>();
  const inputs = row.inputs.map((value: unknown, index: number): TaxonomyMoveSourceAuthority["inputs"][number] => {
    const label = `${name}.inputs[${index}]`, input = planRecord(value, label, ["role", "path", "preimage"]);
    if (input.role !== "schema" && input.role !== "catalog" && input.role !== "approval") throw new Error(`${label}.role is invalid`);
    const path = planPath(input.path, `${label}.path`);
    if (/[\\\u0000-\u001f:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path || /^(?:compose|temp\/compose)(?:\/|$)/u.test(path)) throw new Error(`${label}.path is unsafe`);
    if (roles.has(input.role) || paths.has(path)) throw new Error(`${name}.inputs have duplicate roles or paths`);
    roles.add(input.role); paths.add(path);
    const preimage = parseLeafPreimage(input.preimage, `${label}.preimage`);
    if (preimage.nodeKind !== "file") throw new Error(`${label}.preimage must be a regular file`);
    return { role: input.role, path, preimage };
  });
  return { kind: row.kind, revisionId: row.revisionId, revisionDigest: planString(row.revisionDigest, `${name}.revisionDigest`, PLAN_HASH), inputs };
}

export function parseMove(value: unknown, name: string): TaxonomyMove {
  const row = planRecord(value, name, ["operationId", "sourcePath", "destinationPath", "sourcePreimage", "rationaleRule", "ownerId", "referenceEdits"], ["collisionGroup", "sourceAuthority"]);
  if (!Array.isArray(row.referenceEdits)) throw new Error(`${name}.referenceEdits must be an array`);
  const result: TaxonomyMove = { operationId: planString(row.operationId, `${name}.operationId`, PLAN_OPERATION_ID), sourcePath: planPath(row.sourcePath, `${name}.sourcePath`), destinationPath: planPath(row.destinationPath, `${name}.destinationPath`), sourcePreimage: parseLeafPreimage(row.sourcePreimage, `${name}.sourcePreimage`), rationaleRule: planString(row.rationaleRule, `${name}.rationaleRule`), ownerId: planString(row.ownerId, `${name}.ownerId`), collisionGroup: row.collisionGroup === undefined ? undefined : planString(row.collisionGroup, `${name}.collisionGroup`), referenceEdits: row.referenceEdits.map((entry, index) => parseReferenceEdit(entry, `${name}.referenceEdits[${index}]`)), ...(row.sourceAuthority === undefined ? {} : { sourceAuthority: parseMoveSourceAuthority(row.sourceAuthority, `${name}.sourceAuthority`) }) };
  if (result.operationId !== dispositionOperationId("move-v2", { sourcePath: result.sourcePath, destinationPath: result.destinationPath, sourcePreimage: result.sourcePreimage })) throw new Error(`${name}.operationId does not match its fields`);
  return result;
}

function parseGeneratorNodeRecord(value: unknown, name: string): TaxonomyGeneratorNodeRecord {
  const base = planRecord(value, name, ["path", "nodeKind", "contentHash", "mode"], ["size", "target"]);
  const path = planPath(base.path, `${name}.path`), contentHash = planString(base.contentHash, `${name}.contentHash`, PLAN_HASH), mode = planInteger(base.mode, `${name}.mode`, 0o7777);
  if (base.nodeKind === "directory") {
    if (base.size !== undefined || base.target !== undefined) throw new Error(`${name} directory cannot carry leaf evidence`);
    return { path, nodeKind: "directory", contentHash, mode };
  }
  const size = planInteger(base.size, `${name}.size`);
  if (base.nodeKind === "file") {
    if (base.target !== undefined) throw new Error(`${name} file cannot carry a symlink target`);
    return { path, nodeKind: "file", contentHash, mode, size };
  }
  if (base.nodeKind !== "symlink") throw new Error(`${name}.nodeKind is invalid`);
  const target = planString(base.target, `${name}.target`);
  if (sha256(target) !== contentHash || Buffer.byteLength(target) !== size) throw new Error(`${name} symlink target does not match its hash and size`);
  return { path, nodeKind: "symlink", contentHash, mode, size, target };
}

export function parseRegeneration(value: unknown, name: string): TaxonomyRegeneration {
  const row = planRecord(value, name, ["id", "contractId", "cwd", "command", "outputRoots", "inputs", "preOutputs", "outputs", "preview", "previewManifestDigest", "staleRemovals"], ["verifyCommand"]);
  const command = row.command;
  if (!Array.isArray(command) || command.length !== 4 || command[0] !== "bun" || command[1] !== "repo" || command[2] !== "run" || typeof command[3] !== "string") throw new Error(`${name}.command is invalid`);
  const verifyCommand = row.verifyCommand;
  if (verifyCommand !== undefined && (!Array.isArray(verifyCommand) || verifyCommand.length !== 4 || verifyCommand[0] !== "bun" || verifyCommand[1] !== "repo" || verifyCommand[2] !== "run" || typeof verifyCommand[3] !== "string")) throw new Error(`${name}.verifyCommand is invalid`);
  if (!["outputRoots", "inputs", "preOutputs", "outputs", "staleRemovals"].every((key) => Array.isArray(row[key]))) throw new Error(`${name} array fields are invalid`);
  const preview = planRecord(row.preview, `${name}.preview`, ["contractId", "nodes", "schemaVersion", "staleRemovals"]);
  if (preview.schemaVersion !== 1 || preview.contractId !== row.contractId || !Array.isArray(preview.nodes) || !Array.isArray(preview.staleRemovals)) throw new Error(`${name}.preview is invalid`);
  const previewNodes = preview.nodes.map((value, index) => {
    const node = planRecord(value, `${name}.preview.nodes[${index}]`, ["bytesBase64", "mode", "nodeKind", "path"]);
    return { bytesBase64: planString(node.bytesBase64, `${name}.preview.nodes[${index}].bytesBase64`), mode: planInteger(node.mode, `${name}.preview.nodes[${index}].mode`, 0o7777), nodeKind: requireLiteral(node.nodeKind, `${name}.preview.nodes[${index}].nodeKind`, ["directory", "file"] as const), path: planPath(node.path, `${name}.preview.nodes[${index}].path`) };
  });
  const result: TaxonomyRegeneration = { id: planString(row.id, `${name}.id`, PLAN_OPERATION_ID), contractId: planString(row.contractId, `${name}.contractId`), cwd: planPath(row.cwd, `${name}.cwd`), command: command as unknown as TaxonomyRegeneration["command"], verifyCommand: verifyCommand as TaxonomyRegeneration["verifyCommand"], outputRoots: (row.outputRoots as unknown[]).map((entry, index) => planPath(entry, `${name}.outputRoots[${index}]`)), inputs: (row.inputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.inputs[${index}]`)), preOutputs: (row.preOutputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.preOutputs[${index}]`)), outputs: (row.outputs as unknown[]).map((entry, index) => parseGeneratorNodeRecord(entry, `${name}.outputs[${index}]`)), preview: { contractId: preview.contractId as string, nodes: previewNodes, schemaVersion: 1, staleRemovals: (preview.staleRemovals as unknown[]).map((entry, index) => planPath(entry, `${name}.preview.staleRemovals[${index}]`)) }, previewManifestDigest: planString(row.previewManifestDigest, `${name}.previewManifestDigest`, PLAN_HASH), staleRemovals: (row.staleRemovals as unknown[]).map((entry, index) => planPath(entry, `${name}.staleRemovals[${index}]`)) };
  const provisional = { contractId: result.contractId, cwd: result.cwd, command: result.command, verifyCommand: result.verifyCommand, outputRoots: result.outputRoots, inputs: result.inputs, preOutputs: result.preOutputs, outputs: result.outputs, preview: result.preview, previewManifestDigest: result.previewManifestDigest, staleRemovals: result.staleRemovals };
  if (result.id !== sha256(canonicalJson(provisional)).slice(0, 24)) throw new Error(`${name}.id does not match its fields`);
  return result;
}

export function parseOpaqueDigest(value: unknown, name: string): OpaqueTreeDigest {
  const row = planRecord(value, name, ["algorithm", "relativeRoot", "digest", "files", "directories", "symlinks", "others"]);
  if (row.algorithm !== "sha256-merkle-v1") throw new Error(`${name}.algorithm is invalid`);
  return { algorithm: row.algorithm, relativeRoot: planPath(row.relativeRoot, `${name}.relativeRoot`), digest: planString(row.digest, `${name}.digest`, PLAN_HASH), files: planInteger(row.files, `${name}.files`), directories: planInteger(row.directories, `${name}.directories`), symlinks: planInteger(row.symlinks, `${name}.symlinks`), others: planInteger(row.others, `${name}.others`) };
}

export function parsePlanViolation(value: unknown, name: string): TaxonomyViolation {
  const row = planRecord(value, name, ["code", "severity", "path", "message"]);
  if (row.severity !== "warning" && row.severity !== "error") throw new Error(`${name}.severity is invalid`);
  return { code: planString(row.code, `${name}.code`), severity: row.severity, path: planPath(row.path, `${name}.path`), message: planString(row.message, `${name}.message`) };
}

export function generatorPathCompare(left: string, right: string): number {
  return utf8Compare(left, right);
}

export function generatorPreviewJson(manifest: TaxonomyGeneratorPreviewManifest): string {
  return JSON.stringify({
    contractId: manifest.contractId,
    nodes: manifest.nodes.map((node) => ({ bytesBase64: node.bytesBase64, mode: node.mode, nodeKind: node.nodeKind, path: node.path })),
    schemaVersion: manifest.schemaVersion,
    staleRemovals: manifest.staleRemovals,
  });
}

/** 🛰️ Parses one byte-canonical, repository-rooted, JSON-only generator preview manifest. */
export function parseGeneratorPreviewManifest(content: string, expectedContractId: string, outputRoots: readonly string[], excludedRoots: readonly string[] = []): TaxonomyGeneratorPreviewManifest {
  let value: unknown;
  try {
    value = JSON.parse(content);
  } catch {
    throw new Error(`Generator preview stdout is not one canonical JSON document: bytes=${Buffer.byteLength(content)}, sha256=${sha256(content)}`);
  }
  const root = requireRecord(value, "generator preview");
  if (Object.keys(root).join("\u0000") !== "contractId\u0000nodes\u0000schemaVersion\u0000staleRemovals") throw new Error("Generator preview has noncanonical top-level keys or order");
  if (root.schemaVersion !== 1) throw new Error("Generator preview schemaVersion must be 1");
  if (root.contractId !== expectedContractId) throw new Error(`Generator preview contractId does not match ${expectedContractId}`);
  if (!Array.isArray(root.nodes) || !Array.isArray(root.staleRemovals)) throw new Error("Generator preview nodes and staleRemovals must be arrays");
  const roots = [...new Set(outputRoots.map((path) => normalizeRelative(path)))].sort(generatorPathCompare);
  if (roots.length !== outputRoots.length || roots.some((path, index) => path !== outputRoots[index])) throw new Error("Generator preview output roots must be unique, NFC, repository-relative, and byte-sorted");
  const exclusions = excludedRoots.map(normalizeRelative);
  const withinRoot = (path: string): boolean => roots.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const excluded = (path: string): boolean => exclusions.some((candidate) => path === candidate || path.startsWith(`${candidate}/`));
  const nodes: TaxonomyGeneratorPreviewNode[] = root.nodes.map((value, index) => {
    const node = requireRecord(value, `generator preview nodes[${index}]`);
    if (Object.keys(node).join("\u0000") !== "bytesBase64\u0000mode\u0000nodeKind\u0000path") throw new Error(`Generator preview node ${index} has noncanonical keys or order`);
    const path = requireString(node.path, `generator preview nodes[${index}].path`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path)) throw new Error(`Generator preview node path is unsafe or outside registered roots: ${path}`);
    if (node.nodeKind !== "directory" && node.nodeKind !== "file") throw new Error(`Generator preview nodeKind is invalid at ${path}`);
    if (!Number.isSafeInteger(node.mode) || (node.mode as number) < 0 || (node.mode as number) > 0o7777) throw new Error(`Generator preview mode is invalid at ${path}`);
    if (typeof node.bytesBase64 !== "string" || !/^(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?$/u.test(node.bytesBase64)) throw new Error(`Generator preview base64 is invalid at ${path}`);
    const decoded = Buffer.from(node.bytesBase64, "base64");
    if (decoded.toString("base64") !== node.bytesBase64 || (node.nodeKind === "directory" && node.bytesBase64 !== "")) throw new Error(`Generator preview base64 is noncanonical at ${path}`);
    return { bytesBase64: node.bytesBase64, mode: node.mode as number, nodeKind: node.nodeKind, path };
  });
  const nodeByPath = new Map<string, TaxonomyGeneratorPreviewNode>();
  for (let index = 0; index < nodes.length; index++) {
    const node = requireTaxonomyValue(nodes[index],"nodes[index]");

    if (nodeByPath.has(node.path) || (index > 0 && generatorPathCompare(requireTaxonomyValue(nodes[index - 1],"nodes[index - 1]").path, node.path) >= 0)) throw new Error(`Generator preview nodes repeat or are not byte-sorted at ${node.path}`);
    nodeByPath.set(node.path, node);
  }
  for (const registeredRoot of roots) if (!nodeByPath.has(registeredRoot)) throw new Error(`Generator preview omits registered output root ${registeredRoot}`);
  for (const node of nodes) {
    let parent = posix.dirname(node.path);
    const registeredRoot = roots.filter((candidate) => node.path === candidate || node.path.startsWith(`${candidate}/`)).sort((left, right) => right.length - left.length)[0];
    while (registeredRoot && parent !== posix.dirname(registeredRoot)) {
      const parentNode = nodeByPath.get(parent);
      if (!parentNode || parentNode.nodeKind !== "directory") throw new Error(`Generator preview omits directory node ${parent}`);
      if (parent === registeredRoot) break;
      parent = posix.dirname(parent);
    }
    if (node.nodeKind === "file" && nodes.some((candidate) => candidate.path.startsWith(`${node.path}/`))) throw new Error(`Generator preview file has descendants at ${node.path}`);
  }
  const staleRemovals = root.staleRemovals.map((value, index) => {
    const path = requireString(value, `generator preview staleRemovals[${index}]`);
    if (path !== normalizeRelative(path) || path !== path.normalize("NFC") || !withinRoot(path) || excluded(path)) throw new Error(`Generator preview stale removal is unsafe or outside registered roots: ${path}`);
    if (nodeByPath.has(path) || nodes.some((node) => node.path.startsWith(`${path}/`))) throw new Error(`Generator preview stale removal overlaps expected output ${path}`);
    return path;
  });
  for (let index = 0; index < staleRemovals.length; index++) if ((index > 0 && generatorPathCompare(requireTaxonomyValue(staleRemovals[index - 1],"staleRemovals[index - 1]"), requireTaxonomyValue(staleRemovals[index],"stale removal")) >= 0) || staleRemovals.some((path, candidate) => candidate !== index && path.startsWith(`${staleRemovals[index]}/`))) throw new Error(`Generator preview stale removals repeat, overlap, or are not byte-sorted at ${staleRemovals[index]}`);
  const manifest: TaxonomyGeneratorPreviewManifest = { contractId: expectedContractId, nodes, schemaVersion: 1, staleRemovals };
  if (content !== `${generatorPreviewJson(manifest)}\n`) throw new Error("Generator preview stdout is noisy or not byte-canonical JSON");
  return manifest;
}
