import { expect, test } from "bun:test";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { inflateRawSync } from "node:zlib";

const stdio = resolve(import.meta.dir, "../../..");
const repository = resolve(stdio, "../../..");
const artifacts = join(repository, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
const catalog = join(stdio, "🔌️plugin/📇️catalog");
const definitionRoot = join(catalog, "📜️definition/🦀️.rs");
const read = (root: string, path: string) => readFileSync(join(root, path), "utf8");
const full = [...read(stdio, "🤖️generated/🧩️composition/🦀️.rs").matchAll(/semio_s_artifact_stdio_(\w+?)((?:_catalog)?)::contribution\(\)/gu)].map(match => match[1]);

test("the consumer projection uses the full selection and the exact artifact-owned definitions", () => {
  const stored = readFileSync(join(stdio, "🤖️generated/📜️catalog/🗜️.bin"));
  const decoded = inflateRawSync(stored, { maxOutputLength: 1024 * 1024 });
  const projected = JSON.parse(decoded.toString("utf8"));
  expect(projected.schema).toBe("semio.stdio.catalog-input/v1");
  expect(stored.length).toBeLessThanOrEqual(128 * 1024);
  expect(projected.definitions.map((row: { artifact: string }) => row.artifact)).toEqual(full);
  expect(new Set(full).size).toBe(full.length);
  const authored = new Map<string, string>();
  for (const entry of readdirSync(artifacts, { withFileTypes: true })) {
    const path = join(artifacts, entry.name, "📜️artifact-definition.json");
    if (!entry.isDirectory() || !existsSync(path)) continue;
    const source = readFileSync(path, "utf8");
    const { artifact } = JSON.parse(source);
    expect(authored.has(artifact)).toBe(false);
    authored.set(artifact, source);
  }
  const definitions = projected.definitions.map(({ artifact, definition }: { artifact: string; definition: string }) => {
    expect(definition).toBe(authored.get(artifact)!);
    const parsed = JSON.parse(definition);
    expect(parsed.artifact).toBe(artifact);
    expect(parsed.id).toBe(`s.stdio.${artifact}`);
    return parsed;
  });
  const provider = projected.provider;
  expect(provider).toEqual(JSON.parse(read(catalog, "📜️native-codec-factories.json")));
  const registrations = definitions.flatMap(definition => definition.codecs.flatMap((codec: any) => codec.native_factory
    ? [{ ...codec.native_factory, artifact: definition.artifact, descriptor_codec_id: codec.id }]
    : []));
  const receiptFields = ({ definition_path, protocol_path, protocol_source_sha256, ...receipt }: any) => receipt;
  const byFactoryId = (a: any, b: any) => a.factory_id.localeCompare(b.factory_id);
  expect(registrations.sort(byFactoryId)).toEqual(provider.receipts.map(receiptFields).sort(byFactoryId));
});

test("the consumer compiles the bundle within its source boundary without Stdio artifact implementations", () => {
  const source = readFileSync(definitionRoot);
  const includes = [...source.toString("utf8").matchAll(/include(?:_bytes|_str)?!\("([^"]+)"\)/gu)].map(match => resolve(dirname(definitionRoot), match[1]!));
  expect(includes).toEqual([join(stdio, "🤖️generated/📜️catalog/🗜️.bin")]);
  const policy = JSON.parse(read(repository, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"));
  const { maxSourceBytes, maxSourceFiles } = policy.storage.sourceBoundary;
  expect(includes.length + 1).toBeLessThanOrEqual(maxSourceFiles);
  expect(includes.reduce((bytes, path) => bytes + readFileSync(path).length, source.length)).toBeLessThanOrEqual(maxSourceBytes);
  const manifest = read(catalog, "📜️definition/📦️packages/🦀️rust/Cargo.toml");
  expect(Object.keys(Bun.TOML.parse(manifest).dependencies).sort()).toEqual(["semio-framework", "semio-hub-stdio-catalog-projection", "semio-plugin"]);
});
