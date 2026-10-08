import { createHash } from "node:crypto";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import TOML from "@iarna/toml";
import { visit } from "jsonc-parser";
import { afterEach, describe, expect, it } from "vitest";
import type { DiscoveredPackage } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { renderOwnerPublications } from "../../../../../../🦑️repo/🔨️modules/📚️library/📇️catalog/📣️publication/🟦️.ts";
import { APP_CHANNEL_VERSION } from "../../../../../🟦️.ts";
import { readDescriptorJson } from "../../🔎️discovery/🟦️.ts";
import { declaredComponentKind } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const manifestSchema = JSON.parse(readFileSync(join(import.meta.dirname, "../../../../../../🦑️repo/🔨️modules/📚️library/📇️catalog/📣️publication/🧬️schema/🔣️.json"), "utf8"));
const fixture = JSON.parse(readFileSync(join(import.meta.dirname, "../../🧫️fixtures/🧬️catalog-publication/🔣️.json"), "utf8")) as { executionProtocol: { appChannelVersion: number }; vectors: { id: string; change: string; valid: boolean }[]; componentKinds: { id: string; manifest: string; kind?: "plugin" | "extension" | null; invalid?: boolean }[]; descriptorInputs: { id: string; change: string; appChannelVersion?: number; valid: boolean }[]; stringLengths: { id: string; value: string; input: { schema: object }; valid: boolean }[] };
const roots: string[] = [];
const digest = (value: string): string => createHash("sha256").update(value).digest("hex");
const decodeOracle = (bytes: Uint8Array): unknown => {
  const source = new TextDecoder("utf-8", { fatal: true }).decode(bytes), stack: Set<string>[] = [];
  let duplicate = false;
  visit(source, { onObjectBegin: () => { stack.push(new Set()); }, onObjectEnd: () => { stack.pop(); }, onObjectProperty: (key) => { if (stack.at(-1)?.has(key)) duplicate = true; stack.at(-1)?.add(key); } }, { disallowComments: true, allowTrailingComma: false });
  if (duplicate) throw new Error("Duplicate JSON member");
  return JSON.parse(source);
};
const put = (root: string, path: string, value: unknown): void => {
  const target = join(root, path);
  mkdirSync(dirname(target), { recursive: true });
  writeFileSync(target, typeof value === "string" ? value : `${JSON.stringify(value)}\n`);
};
const root = (): string => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the caller-owned test output root");
  mkdirSync(output, { recursive: true });
  const value = mkdtempSync(join(output, "catalog-publication-"));
  roots.push(value);
  return value;
};
afterEach(() => { while (roots.length) rmSync(roots.pop()!, { recursive: true, force: true }); });

describe("owner catalog publication", () => {
  for (const vector of fixture.vectors) it(vector.id, () => {
    const workspace = root();
    const owner = "product/composition";
    const packageRel = `${owner}/packages/rust`;
    const manifestPath = `${packageRel}/Cargo.toml`;
    const payloadSchema = { $schema: "http://json-schema.org/draft-07/schema#", type: "object", additionalProperties: false, required: ["kind", "digest"], properties: { kind: { const: "alpha" }, digest: { type: "string", pattern: "^[0-9a-f]{64}$" } } };
    const payload = { kind: "alpha", digest: digest("protocol-alpha") };
    const publication = { id: "alpha", output: "alpha.json", source: "payload.json", payloadSchema: "schema.json", integrity: [{ pointer: "/digest", path: "protocol.bin" }] };
    const contribution = { schema: "semio.os.catalog-publication/v1", publications: [publication] };
    put(workspace, manifestPath, '[package]\nname="alpha"\n[package.metadata.semio.publications]\nmanifest="../../publication/manifest.json"\n');
    put(workspace, `${owner}/publication/schema.json`, payloadSchema);
    put(workspace, `${owner}/publication/protocol.bin`, "protocol-alpha");
    put(workspace, `${owner}/publication/payload.json`, payload);
    const pkg: DiscoveredPackage = { ownerRel: owner, lang: "🦀️rust", packageRel, manifestPath, role: "hub", id: "alpha", area: "", maturity: "clean" };
    let packages = [pkg];
    switch (vector.change) {
      case "empty": packages = []; break;
      case "missing-source": rmSync(join(workspace, owner, "publication/payload.json")); break;
      case "digest": put(workspace, `${owner}/publication/protocol.bin`, "modified"); break;
      case "pointer": publication.integrity[0]!.pointer = "/absent"; break;
      case "foreign-source": publication.source = "../../../foreign.json"; put(workspace, "foreign.json", payload); break;
      case "foreign-schema": publication.payloadSchema = "../../../foreign.json"; put(workspace, "foreign.json", payloadSchema); break;
      case "escaping-integrity": publication.integrity[0]!.path = "../../../../outside.bin"; break;
      case "duplicate-id": contribution.publications.push({ ...publication, output: "beta.json" }); break;
      case "duplicate-output": contribution.publications.push({ ...publication, id: "beta" }); break;
      case "invalid-payload": put(workspace, `${owner}/publication/payload.json`, { ...payload, kind: "beta" }); break;
      case "unsafe-output": publication.output = "../alpha.json"; break;
      case "overlong-id": publication.id = "a".repeat(257); break;
      case "binary-protocol": {
        const bytes = new Uint8Array([0, 255, 128, 13, 10]);
        payload.digest = createHash("sha256").update(bytes).digest("hex");
        writeFileSync(join(workspace, owner, "publication/protocol.bin"), bytes);
        put(workspace, `${owner}/publication/payload.json`, payload);
        break;
      }
      case "duplicate-payload-key": put(workspace, `${owner}/publication/payload.json`, `{"kind":"alpha","kind":"alpha","digest":"${payload.digest}"}`); break;
      case "escaped-duplicate-key": put(workspace, `${owner}/publication/payload.json`, `{"kind":"alpha","\\u006bind":"alpha","digest":"${payload.digest}"}`); break;
      case "invalid-utf8": writeFileSync(join(workspace, owner, "publication/payload.json"), new Uint8Array([0xff])); break;
      case "symlink-source": {
        const source = join(workspace, owner, "publication/payload.json"), target = join(workspace, owner, "publication/real-payload.json");
        put(workspace, `${owner}/publication/real-payload.json`, payload);
        rmSync(source); symlinkSync(target, source, "file"); break;
      }
    }
    put(workspace, `${owner}/publication/manifest.json`, contribution);
    const ajv = new Ajv({ strict: true, allErrors: true });
    const structural = ajv.compile(manifestSchema)(contribution);
    const oracle = (() => {
      if (!packages.length) return true;
      if (!structural) return false;
      const inside = (base: string, target: string): boolean => { const path = relative(base, target); return path !== ".." && !path.startsWith("../") && !path.startsWith("..\\") && !isAbsolute(path); };
      const directory = join(workspace, owner, "publication"), ids = new Set(), outputs = new Set();
      try {
        for (const item of contribution.publications) {
          if (ids.has(item.id) || outputs.has(item.output)) return false;
          ids.add(item.id); outputs.add(item.output);
          const source = resolve(directory, item.source), schemaPath = resolve(directory, item.payloadSchema);
          if (!inside(join(workspace, owner), source) || !inside(join(workspace, owner), schemaPath)) return false;
          if (lstatSync(source).isSymbolicLink() || lstatSync(schemaPath).isSymbolicLink()) return false;
          const actualPayload = decodeOracle(readFileSync(source)) as any;
          const actualSchema = decodeOracle(readFileSync(schemaPath)) as object;
          if (!ajv.compile(actualSchema)(actualPayload)) return false;
          for (const binding of item.integrity) {
            const target = resolve(directory, binding.path);
            if (!inside(workspace, target)) return false;
            const expected = binding.pointer.slice(1).split("/").reduce((value, key) => value?.[key.replaceAll("~1", "/").replaceAll("~0", "~")], actualPayload);
            if (expected !== createHash("sha256").update(readFileSync(target)).digest("hex")) return false;
          }
        }
        return true;
      } catch { return false; }
    })();
    expect(Boolean(oracle)).toBe(vector.valid);
    if (!vector.valid) expect(() => renderOwnerPublications(workspace, packages)).toThrow();
    else {
      const projection = renderOwnerPublications(workspace, packages);
      expect(Object.keys(projection.files)).toEqual(vector.change === "empty" ? [] : ["alpha.json"]);
      if (vector.change !== "empty") {
        expect(JSON.parse(projection.files["alpha.json"]!)).toEqual(payload);
        expect(projection.inputs).toContain(`${owner}/publication/protocol.bin`);
        expect(existsSync(join(workspace, relative(workspace, join(workspace, owner))))).toBe(true);
      }
    }
  });
  for (const vector of fixture.componentKinds) it(vector.id, () => {
    const oracle = (() => {
      try {
        const value = (TOML.parse(vector.manifest) as any).package?.metadata?.semio?.["component-kind"];
        if (value !== undefined && value !== "plugin" && value !== "extension") return { valid: false, kind: null };
        return { valid: true, kind: value ?? null };
      } catch { return { valid: false, kind: null }; }
    })();
    expect(oracle).toEqual({ valid: !vector.invalid, kind: vector.kind ?? null });
    if (vector.invalid) expect(() => declaredComponentKind(vector.manifest)).toThrow();
    else expect(declaredComponentKind(vector.manifest) ?? null).toBe(vector.kind);
  });
  for (const vector of fixture.stringLengths) it(vector.id, () => {
    const oracle = new Ajv({ strict: true }).compile(vector.input.schema)(vector.value);
    expect(Boolean(oracle)).toBe(vector.valid);
    expect(validateJsonSchemaSubset(vector.input.schema, vector.value).length === 0).toBe(vector.valid);
  });
});


describe("owner descriptor admission", () => {
  for (const vector of fixture.descriptorInputs) it(vector.id, () => {
    const workspace = root(), path = join(workspace, "owner/🔣️.json");
    const descriptor: Record<string, unknown> = { descriptorVersion: 1, packageId: "semio:alpha", role: "plugin", manifest: { pluginId: "alpha", label: "Alpha", version: "1.0.0", apps: [], examples: [] }, execution: "isolated", executionProtocol: { appChannelVersion: fixture.executionProtocol.appChannelVersion }, quotas: {}, contributions: {}, hashes: { wasmSha256: "1".repeat(64), coreWasmSha256: "2".repeat(64), descriptorSha256: "3".repeat(64) } };
    expect(fixture.executionProtocol.appChannelVersion).toBe(APP_CHANNEL_VERSION);
    if (vector.change === "protocol") descriptor.executionProtocol = { appChannelVersion: vector.appChannelVersion };
    if (vector.change === "role-missing") delete descriptor.role;
    if (vector.change === "version") descriptor.descriptorVersion = 2;
    if (vector.change === "role-unknown") descriptor.role = "library";
    if (vector.change === "field") descriptor.unknown = true;
    if (vector.change === "activation") descriptor.activationEvents = [{ onCommand: {} }];
    let source = JSON.stringify(descriptor);
    if (vector.change === "json") source = "{";
    if (vector.change === "duplicate") source = source.replace('"role":"plugin"', '"role":"plugin","role":"plugin"');
    if (vector.change === "escaped-duplicate") source = source.replace('"role":"plugin"', '"role":"plugin","\\u0072ole":"plugin"');
    if (vector.change === "null") source = "null";
    if (vector.change === "array") source = "[]";
    if (vector.change === "scalar") source = '"alpha"';
    if (vector.change === "oversized") source = " ".repeat(4 * 1024 * 1024) + source;
    put(workspace, "owner/🔣️.json", source);
    if (vector.change === "utf8") writeFileSync(path, new Uint8Array([0xff]));
    if (vector.change === "missing") rmSync(path);
    if (vector.change === "symlink") { put(workspace, "owner/actual.json", source); rmSync(path); symlinkSync(join(workspace, "owner/actual.json"), path, "file"); }
    const contract = JSON.parse(readFileSync(join(import.meta.dirname, "../../🛂️descriptor-verification/🧬️schema/🔣️.json"), "utf8"));
    const oracle = (() => {
      try {
        const bytes = readFileSync(path);
        if (lstatSync(path).isSymbolicLink() || bytes.byteLength > 4 * 1024 * 1024) return false;
        const value = decodeOracle(bytes);
        return Boolean(new Ajv({ strict: true }).compile({ ...contract, $ref: "#/$defs/PackageDescriptorV1" })(value));
      } catch { return false; }
    })();
    expect(oracle).toBe(vector.valid);
    if (vector.valid) expect(readDescriptorJson(workspace, "owner/📦️packages/🦀️rust")).toEqual(descriptor);
    else expect(() => readDescriptorJson(workspace, "owner/📦️packages/🦀️rust")).toThrow();
  });
});
