import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import type { DiscoveredPackage, RegistryCatalogInputView } from "../../🔍️discovery/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { jsonDocumentDuplicateKeys } from "../../🧬️schema/🔣️json-document/🟦️.ts";
import manifestSchema from "./🧬️schema/🔣️.json";

export interface OwnerCatalogPublicationV1 {
  readonly id: string;
  readonly output: string;
  readonly source: string;
  readonly payloadSchema: string;
  readonly payloadDefinition?: string;
  readonly componentAdmissionPointer?: string;
  readonly integrity: readonly { readonly pointer: string; readonly path: string }[];
}

export interface OwnerCatalogPublicationProjection {
  readonly files: Readonly<Record<string, string>>;
  readonly inputs: readonly string[];
}

export interface PublicationComponentAdmission {
  readonly status: "absent" | "admitted" | "exceeds-limit";
  readonly limitBytes: number;
  readonly byteLength?: number;
  readonly path?: string;
}

const MAX_PUBLICATION_BYTES = 64 * 1024 * 1024;
const MAX_CONTRACT_BYTES = 1024 * 1024;

/** 📦️ Records bounded component admission without reading component bytes into source provenance. */
export function publicationComponentAdmission(repoRoot: string, target: string): PublicationComponentAdmission {
  const workspace = resolve(repoRoot), path = resolve(workspace, target), rel = relative(workspace, path);
  if (rel === ".." || rel.startsWith("..") || rel.startsWith("..\\") || isAbsolute(rel)) throw new Error("Publication component escapes workspace");
  let prefix = workspace;
  for (const part of rel.split(/[\\/]/u)) {
    prefix = join(prefix, part);
    let info;
    try { info = lstatSync(prefix); }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return { status: "absent", limitBytes: MAX_PUBLICATION_BYTES }; throw error; }
    if (info.isSymbolicLink() || (prefix === path ? !info.isFile() : !info.isDirectory())) throw new Error("Publication component requires no-follow regular ancestry");
  }
  const byteLength = lstatSync(path).size;
  return { status: byteLength > MAX_PUBLICATION_BYTES ? "exceeds-limit" : "admitted", limitBytes: MAX_PUBLICATION_BYTES, byteLength, path: rel.replaceAll("\\", "/") };
}

/** 🧭️ Resolves a strict JSON pointer without prototype or escaped-token aliases. */
export function publicationPointer(document: unknown, pointer: string): unknown {
  if (!/^(?:\/(?:[^~]|~[01])*)+$/u.test(pointer)) throw new Error(`Invalid publication pointer: ${pointer}`);
  let value = document;
  for (const token of pointer.slice(1).split("/")) {
    const key = token.replaceAll("~1", "/").replaceAll("~0", "~");
    if (value === null || typeof value !== "object" || !Object.hasOwn(value, key)) throw new Error(`Missing publication pointer: ${pointer}`);
    value = (value as Record<string, unknown>)[key];
  }
  return value;
}

/** 📣️ Admits only the publication subtable explicitly authored by a package. */
export function ownerPublicationManifestPath(text: string): string | undefined {
  const rows = text.split(/\r?\n/u);
  let active = false, seen = false, result: string | undefined;
  for (const source of rows) {
    const line = source.trim();
    if (line.startsWith("[")) {
      active = line === "[package.metadata.semio.publications]";
      if (active && seen) throw new Error("Repeated owner publication subtable");
      if (active) seen = true;
      continue;
    }
    if (!active || !line || line.startsWith("#")) continue;
    const match = /^manifest\s*=\s*"([^"\\]+)"\s*(?:#.*)?$/u.exec(line);
    if (!match || result !== undefined) throw new Error("Owner publication subtable requires exactly one manifest path");
    result = match[1];
  }
  if (seen && !result) throw new Error("Missing owner publication manifest path");
  return result;
}

/** 🔏️ Projects caller-selected first-party publications with schema and protocol commitments. */
export function renderOwnerPublications(repoRoot: string, packages: readonly DiscoveredPackage[], view?: RegistryCatalogInputView, components: Readonly<Record<string, PublicationComponentAdmission>> = {}): OwnerCatalogPublicationProjection {
  const workspace = resolve(repoRoot), files: Record<string, string> = {}, inputs = new Set<string>(), ids = new Set<string>();
  const inside = (root: string, path: string): boolean => { const rel = relative(root, path); return rel !== ".." && !rel.startsWith("..") && !rel.startsWith("..\\") && !isAbsolute(rel); };
  const admitted = (base: string, name: string, authority: string, limit: number): { path: string; bytes: Uint8Array } => {
    if (isAbsolute(name) || name.includes("\\") || name.includes("\0")) throw new Error(`Invalid publication input path: ${name}`);
    const path = resolve(base, name);
    if (!inside(workspace, path) || !inside(authority, path)) throw new Error(`Publication input escapes its authority: ${name}`);
    const rel = relative(workspace, path).replaceAll("\\", "/");
    let prefix = "";
    for (const part of rel.split("/")) {
      prefix = prefix ? `${prefix}/${part}` : part;
      const kind = view ? view.kind(prefix) : (() => { const info = lstatSync(join(workspace, prefix)); return info.isSymbolicLink() ? "symlink" : info.isFile() ? "file" : info.isDirectory() ? "directory" : "other"; })();
      if (kind === "symlink" || (prefix === rel ? kind !== "file" : kind !== "directory")) throw new Error(`Publication input must be a no-follow regular file: ${prefix}`);
    }
    if (!view && lstatSync(path).size > limit) throw new Error(`Publication input exceeds its byte limit: ${rel}`);
    const bytes = view ? view.readBytes(rel) : readFileSync(path);
    if (bytes.byteLength > limit) throw new Error(`Publication input exceeds its byte limit: ${rel}`);
    inputs.add(rel);
    return { path, bytes };
  };
  const decode = (path: string, bytes: Uint8Array): unknown => {
    try {
      const source = new TextDecoder("utf-8", { fatal: true }).decode(bytes), document: unknown = JSON.parse(source);
      if (jsonDocumentDuplicateKeys(source).length) throw new Error("Duplicate publication input members");
      return document;
    } catch { throw new Error(`Publication input is invalid JSON: ${path}`); }
  };
  for (const pkg of packages.filter((value) => value.lang === "🦀️rust").sort((a, b) => a.manifestPath.localeCompare(b.manifestPath))) {
    const owner = resolve(workspace, pkg.ownerRel);
    const cargo = admitted(workspace, pkg.manifestPath, owner, MAX_CONTRACT_BYTES);
    const declaration = ownerPublicationManifestPath(new TextDecoder("utf-8", { fatal: true }).decode(cargo.bytes));
    if (!declaration) continue;
    const source = admitted(dirname(cargo.path), declaration, owner, MAX_CONTRACT_BYTES);
    const manifest = decode(source.path, source.bytes);
    const errors = validateJsonSchemaSubset(manifestSchema, manifest);
    if (errors.length) throw new Error(`Invalid owner publication manifest: ${errors.join("; ")}`);
    const publications = (manifest as { publications: OwnerCatalogPublicationV1[] }).publications;
    for (const publication of publications) {
      if (ids.has(publication.id) || Object.hasOwn(files, publication.output)) throw new Error(`Duplicate owner publication: ${publication.id}/${publication.output}`);
      ids.add(publication.id);
      const data = admitted(dirname(source.path), publication.source, owner, MAX_PUBLICATION_BYTES);
      const contract = admitted(dirname(source.path), publication.payloadSchema, owner, MAX_CONTRACT_BYTES);
      const payload = decode(data.path, data.bytes), schema = decode(contract.path, contract.bytes);
      const definition = publication.payloadDefinition ? publicationPointer(schema, publication.payloadDefinition) : schema;
      const payloadErrors = validateJsonSchemaSubset(definition, payload, schema);
      if (payloadErrors.length) throw new Error(`Invalid owner publication payload ${publication.id}: ${payloadErrors.join("; ")}`);
      const bindings = new Set<string>();
      for (const binding of publication.integrity) {
        if (bindings.has(binding.pointer)) throw new Error(`Duplicate publication digest binding: ${binding.pointer}`);
        bindings.add(binding.pointer);
        const expected = publicationPointer(payload, binding.pointer);
        if (typeof expected !== "string" || !/^[0-9a-f]{64}$/u.test(expected) || /^0{64}$/u.test(expected)) throw new Error(`Invalid publication digest: ${binding.pointer}`);
        const input = admitted(dirname(source.path), binding.path, workspace, MAX_PUBLICATION_BYTES);
        const actual = createHash("sha256").update(input.bytes).digest("hex");
        if (expected !== actual) throw new Error(`Publication protocol digest mismatch: ${binding.pointer}`);
      }
      if (publication.componentAdmissionPointer) {
        const receipt = publicationPointer(payload, publication.componentAdmissionPointer);
        if (receipt === null || typeof receipt !== "object" || Array.isArray(receipt)) throw new Error("Publication component admission must name an object");
        const row = receipt as Record<string, unknown>;
        for (const key of Object.keys(row)) delete row[key];
        Object.assign(row, components[pkg.manifestPath] ?? { status: "absent", limitBytes: MAX_PUBLICATION_BYTES });
      }
      const finalErrors = validateJsonSchemaSubset(definition, payload, schema);
      if (finalErrors.length) throw new Error(`Invalid admitted publication ${publication.id}: ${finalErrors.join("; ")}`);
      files[publication.output] = `${JSON.stringify(payload, null, 2)}\n`;
    }
  }
  return { files, inputs: [...inputs].sort() };
}
