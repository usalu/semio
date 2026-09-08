import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { WORKSPACE_ROOT, toolJobRustBlock } from "../../../../../../../../📜️script.ts";
import { canonicalErrorProgressSelfTests } from "../🔬️canonical-error-progress/🟦️.ts";

/** 🧪️ Validates language-neutral canonical bytes and private Store sealer source boundaries. */
export function storeCanonicalEditSealerSelfTests(): { grants: number; schemaHostiles: number; sourceHostiles: number; digestOracles: number; mapGrants: number; mapSchemaHostiles: number; mapSourceHostiles: number; mapDigestOracles: number; readerChecks: number } {
  const storePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
  const base = join(WORKSPACE_ROOT, storePath, "🧵️canonical-edit");
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/🔏️canonical-edit-sealer.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/CanonicalEditSealer" });
  if (!validate(fixture)) throw new Error(`canonical edit fixture schema: ${JSON.stringify(validate.errors)}`);
  const schemaHostiles = [
    { ...fixture, extra: true },
    { ...fixture, edit: { ...fixture.edit, extra: true } },
    { ...fixture, edit: { ...fixture.edit, forwards: [{ Unknown: {} }] } },
    { ...fixture, edit: { ...fixture.edit, forwards: [{ Replace: { ...fixture.edit.forwards[0].Replace, text: 1 } }] } },
    { ...fixture, edit: { ...fixture.edit, mutationMeta: [{ ...fixture.edit.mutationMeta[0], timestamp: { actor: 1, physical_ms: 42, logical: 2, extra: true } }] } },
    { ...fixture, edit: { ...fixture.edit, mutationMeta: [{ ...fixture.edit.mutationMeta[0], undo_policy: "Unknown" }] } },
    { ...fixture, grants: [0, -1, 2, 7, 256, 4096] },
    { ...fixture, expectedDigest: "forged" },
    { ...fixture, hostile: [...fixture.hostile, "forged-arbitrary-authority"] },
  ];
  for (const hostile of schemaHostiles) if (validate(hostile)) throw new Error("strict canonical sealer schema accepted hostile input");
  const utf8 = new TextEncoder();
  function* canonicalBytes(value: unknown): Generator<number> {
    if (typeof value === "string") {
      yield 34;
      for (const character of value) {
        const scalar = character.codePointAt(0)!;
        const escaped = character === '"' ? '\\"' : character === "\\" ? "\\\\" : scalar === 8 ? "\\b" : scalar === 9 ? "\\t" : scalar === 10 ? "\\n" : scalar === 12 ? "\\f" : scalar === 13 ? "\\r" : scalar < 32 ? `\\u${scalar.toString(16).padStart(4, "0")}` : character;
        yield* utf8.encode(escaped);
      }
      yield 34;
    } else if (Array.isArray(value)) {
      yield 91;
      for (let index = 0; index < value.length; index++) { if (index) yield 44; yield* canonicalBytes(value[index]); }
      yield 93;
    } else if (value !== null && typeof value === "object") {
      yield 123;
      let index = 0;
      for (const key of Object.keys(value)) { if (index++) yield 44; yield* canonicalBytes(key); yield 58; yield* canonicalBytes((value as Record<string, unknown>)[key]); }
      yield 125;
    } else {
      yield* utf8.encode(JSON.stringify(value));
    }
  }
  const expected = utf8.encode(fixture.expectedJson);
  if (fixture.expectedJson !== JSON.stringify(fixture.edit) || expected.length <= 4096) throw new Error("canonical edit JSON oracle mismatch");
  for (const maximum of fixture.grants.filter((value: number) => value > 0)) {
    const iterator = canonicalBytes(fixture.edit);
    const actual: number[] = [];
    let complete = false;
    while (!complete) {
      const before = actual.length;
      for (let count = 0; count < Math.min(maximum, 256); count++) {
        const step = iterator.next();
        if (step.done) { complete = true; break; }
        actual.push(step.value);
      }
      if (actual.length - before > maximum) throw new Error("canonical edit byte grant exceeded");
    }
    if (!Buffer.from(actual).equals(expected)) throw new Error(`canonical edit byte oracle mismatch for grant ${maximum}`);
  }
  const hash = createHash("sha256");
  const integer = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
  hash.update("semio.artifact.cursor.v2");
  for (const part of [utf8.encode("edit"), utf8.encode(fixture.edit.id), expected]) { hash.update(integer(part.length)); hash.update(part); }
  if (hash.digest("hex") !== fixture.expectedDigest) throw new Error("canonical edit third-party digest oracle mismatch");
  const store = readFileSync(join(WORKSPACE_ROOT, storePath, "🦀️.rs"), "utf8");
  const source = readFileSync(join(base, "🦀️.rs"), "utf8");
  const method = (text: string, pattern: RegExp) => { const start = text.search(pattern); return start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body ?? ""; };
  const exact = (storeText: string, sealerText: string) => {
    const validation = method(storeText, /fn validate_prepared</);
    const mint = method(storeText, /fn seal_prepared_owned</);
    const advance = method(sealerText, /pub fn advance\(&mut self, grant: ArtifactStoreOneItemGrant/);
    const commitStart = storeText.indexOf("ArtifactStoreOneItemPublicationPhase::PreflightingCommit =>", storeText.indexOf("pub fn advance_apply_one("));
    const commit = commitStart < 0 ? "" : toolJobRustBlock(storeText, storeText.indexOf("{", commitStart))?.body ?? "";
    return validation.includes("Arc::ptr_eq(self, &prepared.seal.authority)")
      && validation.includes("prepared.seal.edit_address != prepared.edit.as_ref() as *const")
      && validation.includes("prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot)")
      && validation.includes("prepared.seal.digest != prepared.edit_digest")
      && mint.includes("edit_address: edit.as_ref() as *const") && mint.includes("digest: edit_digest")
      && !/pub\s+fn\s+seal_prepared/.test(storeText)
      && advance.includes("self.encoder.encode_chunk(edit.as_ref(), &mut self.last_chunk[..maximum])")
      && advance.includes("self.hash.update(&self.last_chunk[..self.last_length])")
      && advance.includes("authority.seal_prepared_owned(edit, post, self.hash.clone().finalize(), identities)")
      && !/serde_json::to_(?:vec|value|string)|prepared_edit_digest\(/.test(advance)
      && commit.includes("authority.validate_prepared(prepared)") && !commit.includes("prepared_edit_digest(");
  };
  if (!exact(store, source)) throw new Error("live Store canonical sealer authority/byte source linkage missing");
  const sourceHostiles = [
    [store.replace("Arc::ptr_eq(self, &prepared.seal.authority)", "true"), source],
    [store.replace("prepared.seal.edit_address != prepared.edit.as_ref() as *const", "prepared.seal.edit_address != forged_edit as *const"), source],
    [store.replace("prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot)", "prepared.seal.post_address != 0"), source],
    [store.replace("authority.validate_prepared(prepared)", "authority.prepared_edit_digest(&prepared.edit)"), source],
    [store.replace("fn seal_prepared_owned<", "pub fn seal_prepared_owned<"), source],
    [store, source.replace("self.encoder.encode_chunk(edit.as_ref(), &mut self.last_chunk[..maximum])", "serde_json::to_vec(edit.as_ref())")],
  ];
  for (const [candidateStore, candidateSealer] of sourceHostiles) if (exact(candidateStore, candidateSealer)) throw new Error("canonical Store sealer accepted hostile authority/serialization source");
  const mapFixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/🗺️canonical-borrowed-map.json"), "utf8"));
  const validateMap = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/CanonicalBorrowedMap" });
  if (!validateMap(mapFixture)) throw new Error(`borrowed map fixture schema: ${JSON.stringify(validateMap.errors)}`);
  const mapSchemaHostiles = [{ ...mapFixture, extra: true }, { ...mapFixture, longKeyBytes: 4096 }, { ...mapFixture, lifetime: { ...mapFixture.lifetime, iteratorDropsBeforeRoot: false } }, { ...mapFixture, hostile: ["unchecked-pointer"] }];
  for (const hostile of mapSchemaHostiles) if (validateMap(hostile)) throw new Error("borrowed map schema accepted hostile lifetime shape");
  const mapExpected = utf8.encode(mapFixture.expectedJson);
  if (mapFixture.expectedJson !== JSON.stringify(mapFixture.edit)) throw new Error("borrowed map canonical JSON oracle mismatch");
  const keys = Object.keys(mapFixture.edit.forwards[0].ReplaceMap.map);
  if (Math.max(...keys.map((key) => utf8.encode(key).length)) !== mapFixture.longKeyBytes) throw new Error("borrowed map long key is not the advertised UTF-8 length");
  for (const grant of mapFixture.grants.filter((value: number) => value > 0)) {
    const stream = canonicalBytes(mapFixture.edit);
    const actual: number[] = [];
    let done = false;
    while (!done) for (let count = 0; count < Math.min(grant, 256); count++) {
      const next = stream.next();
      if (next.done) { done = true; break; }
      actual.push(next.value);
    }
    if (!Buffer.from(actual).equals(mapExpected)) throw new Error("borrowed map byte oracle mismatch");
  }
  const mapHash = createHash("sha256");
  mapHash.update("semio.artifact.cursor.v2");
  for (const part of [utf8.encode("edit"), utf8.encode(mapFixture.edit.id), mapExpected]) { mapHash.update(integer(part.length)); mapHash.update(part); }
  if (mapHash.digest("hex") !== mapFixture.expectedDigest) throw new Error("borrowed map Node crypto digest oracle mismatch");
  const borrowed = readFileSync(join(base, "🧵️borrowed/🦀️.rs"), "utf8");
  const borrowedExact = (parent: string, child: string) => {
    const close = method(parent, /pub fn close_step\(&mut self, grant: ArtifactStoreOneItemGrant/);
    const bind = method(child, /fn bind</);
    return parent.includes("pub trait ArtifactCanonicalJson: Sync")
      && parent.indexOf("encoder: ArtifactCanonicalEditEncoder") < parent.indexOf("edit: Option<Box<Edit<M>>>")
      && close.indexOf("self.encoder.close_step()") >= 0 && close.indexOf("self.encoder.close_step()") < close.indexOf("self.edit.take()")
      && bind.includes("self.root_address != address") && bind.includes("root.canonical_json_borrowed_root()")
      && child.includes("pub(super) struct ArtifactCanonicalEditEncoder")
      && child.includes("Iterator<Item = (&'a str, ArtifactCanonicalJsonValue<'a>)> + Send + 'a")
      && child.includes("values.values.next()") && child.includes("self.frames[self.depth] = None")
      && parent.includes("self.depth >= self.maximum_depth") && child.includes("maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top")
      && !/\.nth\(|\.range\(|serde_json::to_(?:vec|value|string)/.test(child);
  };
  if (!borrowedExact(source, borrowed)) throw new Error("borrowed map private root lifetime/source linkage missing");
  const mapSourceHostiles = [
    [source.replace("pub trait ArtifactCanonicalJson: Sync", "pub trait ArtifactCanonicalJson"), borrowed],
    [source.replace("self.encoder.close_step()", "self.edit.take()"), borrowed],
    [source, borrowed.replace("self.root_address != address", "false")],
    [source, borrowed.replace("values.values.next()", "values.values.nth(0)")],
    [source, borrowed.replace("pub(super) struct ArtifactCanonicalEditEncoder", "pub struct ArtifactCanonicalEditEncoder")],
    [source, borrowed.replace("maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top", "maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH")],
  ];
  for (const [parent, child] of mapSourceHostiles) if (borrowedExact(parent, child)) throw new Error("borrowed map accepted hostile lifetime/source substitution");
  const readerFixture = JSON.parse(readFileSync(join(base, "🧪️fixtures/📖️canonical-reader.json"), "utf8"));
  const readerValidate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/CanonicalReader" });
  if (!readerValidate(readerFixture) || readerFixture.expectedByteLength !== mapExpected.length || createHash("sha256").update(mapExpected).digest("hex") !== readerFixture.expectedJsonSha256) throw new Error("typed canonical reader schema/Node byte oracle mismatch");
  const readerSchemaHostiles = [{ ...readerFixture, extra: true }, { ...readerFixture, sourceFixture: "unbound-root" }, { ...readerFixture, grants: [0, 1, 7, 4097] }];
  for (const hostile of readerSchemaHostiles) if (readerValidate(hostile)) throw new Error("strict canonical reader schema accepted hostile input");
  for (const maximum of readerFixture.grants.filter((grant: number) => grant > 0)) {
    const iterator = canonicalBytes(mapFixture.edit);
    const actual: number[] = [];
    let done = false;
    while (!done) for (let index = 0; index < Math.min(maximum, 256); index += 1) { const next = iterator.next(); if (next.done) { done = true; break; } actual.push(next.value); }
    if (!Buffer.from(actual).equals(mapExpected)) throw new Error("canonical reader bounded byte oracle mismatch");
  }
  const reader = readFileSync(join(base, "📖️reader/🦀️.rs"), "utf8");
  const readerExact = (text: string) => {
    const close = method(text, /fn close_step\(&mut self, grant: ArtifactStoreOneItemGrant/);
    const transfer = method(text, /fn take_root\(&mut self\)/);
    const encode = method(text, /fn encode_chunk\(&mut self, grant: ArtifactStoreOneItemGrant/);
    return text.indexOf("encoder: ArtifactCanonicalEditEncoder") < text.indexOf("root: Option<Arc<T>>")
      && close.indexOf("self.encoder.close_step()") >= 0 && close.indexOf("self.encoder.close_step()") < close.indexOf("self.root.take()")
      && transfer.includes("if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() }") && transfer.includes("self.encoder.reset()")
      && text.includes("owned: ManuallyDrop<ReaderState<T>>") && text.includes("!self.cancelled && !self.failed && !self.closing && self.encoder.is_complete()")
      && text.includes("!std::thread::panicking()") && text.includes("ManuallyDrop::drop(&mut self.owned)")
      && encode.includes("self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum])") && encode.includes("grant.maximum_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES)")
      && !/seal_prepared|serde_json::to_(?:vec|value|string)|unsafe impl/.test(text);
  };
  if (!readerExact(reader)) throw new Error("canonical reader exact retained root wiring missing");
  const readerSourceHostiles = [
    reader.replace("self.encoder.close_step()", "self.root.take()"),
    reader.replace("if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() }", "self.encoder.is_complete() || self.closing && self.encoder.terminal_is_empty()"),
    reader.replace("self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum])", "serde_json::to_vec(root)"),
    reader.replace("grant.maximum_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES)", "output.len()"),
    reader.replace("owned: ManuallyDrop<ReaderState<T>>", "owned: ReaderState<T>"),
    reader.replace("!std::thread::panicking()", "true"),
  ];
  for (const hostile of readerSourceHostiles) if (readerExact(hostile)) throw new Error("canonical reader accepted hostile ownership/grant substitution");
  return { grants: fixture.grants.length - 1, schemaHostiles: schemaHostiles.length, sourceHostiles: sourceHostiles.length, digestOracles: 1, mapGrants: mapFixture.grants.length - 1, mapSchemaHostiles: mapSchemaHostiles.length, mapSourceHostiles: mapSourceHostiles.length, mapDigestOracles: 1, readerChecks: 1 + readerSchemaHostiles.length + readerFixture.grants.length - 1 + readerSourceHostiles.length + canonicalErrorProgressSelfTests() };
}
