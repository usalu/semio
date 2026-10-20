import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { readStoreSourceText } from "../../../../../../../📦️packages/🟦️typescript/🧪️test/📖️source-text/🟦️.ts";
import { WORKSPACE_ROOT } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { toolJobRustBlock } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { canonicalErrorProgressSelfTests, canonicalRustMethodBody } from "../🔬️canonical-error-progress/🟦️.ts";

/** 🔢️ An edit's revision value: its members in declaration order without `sequenceNumber` (design §22.28) — where an edit sits in its history is not part of what it is. */
const revisionValue = ({ sequenceNumber: _sequenceNumber, ...revision }: Record<string, unknown>): Record<string, unknown> => revision;

/** 🧪️ Validates language-neutral canonical bytes and private Store sealer source boundaries. */
export function storeCanonicalEditSealerSelfTests(): { grants: number; editHostiles: number; sourceHostiles: number; digestOracles: number; mapGrants: number; mapEditHostiles: number; mapSourceHostiles: number; mapDigestOracles: number; readerChecks: number } {
  const storePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store";
  const base = join(WORKSPACE_ROOT, storePath, "🧵️canonical-edit");
  const contract = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧪️testing/🧬️schema/🔣️.json"), "utf8"));
  const fixture = JSON.parse(readFileSync(join(base, "🧪️tests/🧫️fixtures/🔏️canonical-edit-sealer.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).addSchema(contract).compile({ ...schema, $ref: "#/$defs/CanonicalEditSealerEdit" });
  if (!validate(fixture.edit)) throw new Error(`canonical edit payload schema: ${JSON.stringify(validate.errors)}`);
  const editHostiles = [
    { ...fixture.edit, extra: true },
    { ...fixture.edit, forwards: [{ Unknown: {} }] },
    { ...fixture.edit, forwards: [{ Replace: { ...fixture.edit.forwards[0].Replace, text: 1 } }] },
    { ...fixture.edit, mutationMeta: [{ ...fixture.edit.mutationMeta[0], timestamp: { actor: 1, physical_ms: 42, logical: 2, extra: true } }] },
    { ...fixture.edit, mutationMeta: [{ ...fixture.edit.mutationMeta[0], undo_policy: "Unknown" }] },
  ];
  for (const hostile of editHostiles) if (validate(hostile)) throw new Error("canonical edit payload schema accepted hostile input");
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
  if (fixture.expectedJson !== JSON.stringify(revisionValue(fixture.edit)) || expected.length <= 4096) throw new Error("canonical edit JSON oracle mismatch");
  for (const maximum of fixture.grants.filter((value: number) => value > 0)) {
    const iterator = canonicalBytes(revisionValue(fixture.edit));
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
  const store = readFileSync(join(WORKSPACE_ROOT, storePath, "🎮️one-item/📬️publication/🦀️.rs"), "utf8") + "\n" + readStoreSourceText();
  const sealer = readFileSync(join(WORKSPACE_ROOT, storePath, "🎮️one-item/📬️publication/📥️lifted/🧵️canonical-edit/🦀️.rs"), "utf8");
  const source = readFileSync(join(base, "\u{1f980}\uFE0F.rs"), "utf8");
  const native = readFileSync(join(base, "\u{1f9f3}\uFE0Fsource/\u{1f980}\uFE0F.rs"), "utf8");
  const protocolSeal = readFileSync(join(WORKSPACE_ROOT, "\u{1f9f0}\uFE0Fframework/\u{1f528}\uFE0Fmodules/\u{1f4e1}\uFE0Freplication/\u{1f6aa}\uFE0Fio/\u{1f4dd}\uFE0Ftext/\u{1f9f5}\uFE0Fcanonical/\u{1f50f}\uFE0Fseal/\u{1f980}\uFE0F.rs"), "utf8");
  const compact = (text: string) => text.replaceAll(/\s+/g, "");
  const method = canonicalRustMethodBody;
  const replaceClause = (text: string, clause: string, replacement: string) => {
    const pattern = new RegExp(clause.replaceAll(/[.*+?^$()|[\]\\]/g, "\\$&").replaceAll(/\s+/g, "\\s*"));
    const changed = text.replace(pattern, replacement);
    if (changed === text) throw new Error("canonical hostile clause did not alter its current owner: " + clause);
    return changed;
  };
  const exact = (storeText: string, sealerText: string, nativeText: string, sealText: string) => {
    const validation = method(storeText, /fn validate_prepared</);
    const mint = method(storeText, /fn seal_prepared_owned</);
    const advance = compact(method(sealerText, /pub fn advance\(&mut self, grant: ArtifactStoreOneItemGrant/));
    const original = compact(method(nativeText, /fn advance_original\(/));
    const hashing = compact(method(sealText, /pub fn advance\(/));
    const commit = [...storeText.matchAll(/fn fold_batch_item\(/g)].map(match => toolJobRustBlock(storeText, storeText.indexOf("{", match.index))?.body ?? "").reduce((longest, body) => body.length > longest.length ? body : longest, "");
    return validation.includes("Arc::ptr_eq(self, &prepared.seal.authority)")
      && validation.includes("prepared.seal.edit_address != prepared.edit.as_ref() as *const")
      && validation.includes("prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot)")
      && validation.includes("prepared.seal.digest != prepared.edit_digest")
      && mint.includes("edit_address: edit.as_ref() as *const") && mint.includes("digest: edit_digest")
      && !/pub\s+fn\s+seal_prepared/.test(storeText)
      && advance.includes("owner.take(child)?") && advance.includes("self.prepared_digest=Some(digest)")
      && advance.includes("owner.advance(child)?.progress()") && advance.includes("authority.seal_prepared_owned(edit,post,digest,identities)")
      && original.includes("ArtifactCanonicalEditAuthorityCursor::admit(authority,edit,child(g))")
      && original.includes("ArtifactCanonicalEditIdentityCursor::admit(authority,edit,child(g))")
      && original.includes("ArtifactCanonicalEditSealCursor::admit_source(&mutself.edit_source,child(g))")
      && original.includes("o.take_edit(child(g))?") && original.includes("self.digest=Some(digest)")
      && hashing.includes("if!admitted(demand,grant){returnOk(RetainedCloneStep::Progress(Default::default()));}")
      && hashing.includes("self.hash.as_mut().unwrap().update(&self.page[..self.page_length])")
      && hashing.includes("self.digest=Some(self.hash.take().unwrap().finalize())")
      && !/serde_json::to_(?:vec|value|string)|prepared_edit_digest\(/.test(advance + original + hashing)
      && commit.includes("authority.validate_prepared(candidate)") && !commit.includes("prepared_edit_digest(");
  };
  if (!exact(store, sealer, native, protocolSeal)) throw new Error("live Store canonical original-owner/sealer linkage missing");
  const sourceHostiles: [string, string, string, string][] = [
    [replaceClause(store, "Arc::ptr_eq(self, &prepared.seal.authority)", "true"), sealer, native, protocolSeal],
    [replaceClause(store, "prepared.seal.edit_address != prepared.edit.as_ref() as *const", "prepared.seal.edit_address != forged_edit as *const"), sealer, native, protocolSeal],
    [replaceClause(store, "prepared.seal.post_address != Arc::as_ptr(&prepared.post_snapshot)", "prepared.seal.post_address != 0"), sealer, native, protocolSeal],
    [replaceClause(store, "authority.validate_prepared(candidate)", "authority.prepared_edit_digest(&candidate.edit)"), sealer, native, protocolSeal],
    [replaceClause(store, "pub(crate) fn seal_prepared_owned<", "pub fn seal_prepared_owned<"), sealer, native, protocolSeal],
    [store, replaceClause(sealer, "owner.advance(child)?.progress()", "serde_json::to_vec(edit.as_ref())"), native, protocolSeal],
    [store, replaceClause(sealer, "self.prepared_digest=Some(digest)", "self.prepared_digest=Some([0;32])"), native, protocolSeal],
    [store, sealer, replaceClause(native, "ArtifactCanonicalEditAuthorityCursor::admit(authority,edit,child(g))", "accept_unchecked_edit()"), protocolSeal],
    [store, sealer, replaceClause(native, "ArtifactCanonicalEditIdentityCursor::admit(authority,edit,child(g))", "accept_unchecked_identity()"), protocolSeal],
    [store, sealer, native, replaceClause(protocolSeal, "self.hash.as_mut().unwrap().update(&self.page[..self.page_length])", "self.hash.as_mut().unwrap().update(&[])" )],
    [store, sealer, native, replaceClause(protocolSeal, "let demand=self.next_demand()?;if !admitted(demand,grant)", "let demand=self.next_demand()?;if false")],
  ] as const;
  for (const [index, candidate] of sourceHostiles.entries()) if (exact(...candidate)) throw new Error(`canonical Store sealer accepted hostile authority/serialization sealer ${index}`);
  const mapDescriptor = JSON.parse(readFileSync(join(base, "🧫️fixtures/🗺️canonical-borrowed-map.json"), "utf8"));
  const validateMap = new Ajv({ strict: true, allErrors: true }).addSchema(contract).compile({ ...schema, $ref: "#/$defs/CanonicalBorrowedMapEdit" });
  if (!validateMap(mapDescriptor.edit)) throw new Error(`borrowed map edit schema: ${JSON.stringify(validateMap.errors)}`);
  const mapEditHostiles = [{ ...mapDescriptor.edit, extra: true }, { ...mapDescriptor.edit, forwards: [{ Replace: {} }] }];
  for (const hostile of mapEditHostiles) if (validateMap(hostile)) throw new Error("borrowed map edit schema accepted hostile mutation");
  const mapExpected = utf8.encode(mapDescriptor.expectedJson);
  if (mapDescriptor.expectedJson !== JSON.stringify(revisionValue(mapDescriptor.edit))) throw new Error("borrowed map canonical JSON oracle mismatch");
  const keys = Object.keys(mapDescriptor.edit.forwards[0].ReplaceMap.map);
  if (Math.max(...keys.map((key) => utf8.encode(key).length)) !== mapDescriptor.longKeyBytes) throw new Error("borrowed map long key is not the advertised UTF-8 length");
  for (const grant of mapDescriptor.grants.filter((value: number) => value > 0)) {
    const stream = canonicalBytes(revisionValue(mapDescriptor.edit));
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
  for (const part of [utf8.encode("edit"), utf8.encode(mapDescriptor.edit.id), mapExpected]) { mapHash.update(integer(part.length)); mapHash.update(part); }
  if (mapHash.digest("hex") !== mapDescriptor.expectedDigest) throw new Error("borrowed map Node crypto digest oracle mismatch");
  const borrowed = readFileSync(join(base, "🧵️borrowed/🦀️.rs"), "utf8");
  const borrowedReader = readFileSync(join(base, "\u{1f4d6}\uFE0Freader/\u{1f980}\uFE0F.rs"), "utf8");
  const borrowedExact = (parent: string, child: string, readerText: string) => {
    const close = compact(method(readerText, /fn close_step\(&mut self, grant: RetainedCloneGrant/));
    const bind = method(child, /fn bind</);
    const encoder = readerText.indexOf("encoder: ArtifactCanonicalEditEncoder"), root = readerText.indexOf("root: Option<Arc<T>>");
    return parent.includes("pub trait ArtifactCanonicalJson: Sync")
      && encoder >= 0 && root > encoder
      && close.indexOf("self.encoder.close_step(grant)?") >= 0 && close.indexOf("self.encoder.close_step(grant)?") < close.indexOf("self.root.take()")
      && bind.includes("self.root_address != address") && bind.includes("root.canonical_json_borrowed_root()")
      && child.includes("pub(super) struct ArtifactCanonicalEditEncoder")
      && child.includes("Iterator<Item = (ArtifactCanonicalJsonText<'a>, ArtifactCanonicalJsonValue<'a>)> + Send + 'a")
      && child.includes("values.values.next()") && child.includes("self.frames[self.depth].take()")
      && parent.includes("demand.depth > grant.maximum_depth || demand.depth > self.maximum_depth") && child.includes("maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top")
      && !/\.nth\(|\.range\(|serde_json::to_(?:vec|value|string)/.test(child);
  };
  if (!borrowedExact(source, borrowed, borrowedReader)) throw new Error("borrowed map private root lifetime/source linkage missing");
  const mapSourceHostiles: [string, string, string][] = [
    [replaceClause(source, "pub trait ArtifactCanonicalJson: Sync", "pub trait ArtifactCanonicalJson"), borrowed, borrowedReader],
    [source, borrowed, replaceClause(borrowedReader, "self.encoder.close_step(grant)?", "self.root.take()")],
    [source, replaceClause(borrowed, "self.root_address != address", "false"), borrowedReader],
    [source, replaceClause(borrowed, "values.values.next()", "values.values.nth(0)"), borrowedReader],
    [source, replaceClause(borrowed, "pub(super) struct ArtifactCanonicalEditEncoder", "pub struct ArtifactCanonicalEditEncoder"), borrowedReader],
    [source, replaceClause(borrowed, "maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH - top", "maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH"), borrowedReader],
    [replaceClause(source, "demand.depth > grant.maximum_depth || demand.depth > self.maximum_depth", "false"), borrowed, borrowedReader],
  ] as const;
  for (const candidate of mapSourceHostiles) if (borrowedExact(...candidate)) throw new Error("borrowed map accepted hostile lifetime/source substitution");
  const readerFixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/📖️canonical-reader.json"), "utf8"));
  if (readerFixture.expectedByteLength !== mapExpected.length || createHash("sha256").update(mapExpected).digest("hex") !== readerFixture.expectedJsonSha256) throw new Error("typed canonical reader Node byte oracle mismatch");
  for (const maximum of readerFixture.grants.filter((grant: number) => grant > 0)) {
    const iterator = canonicalBytes(revisionValue(mapDescriptor.edit));
    const actual: number[] = [];
    let done = false;
    while (!done) for (let index = 0; index < Math.min(maximum, 256); index += 1) { const next = iterator.next(); if (next.done) { done = true; break; } actual.push(next.value); }
    if (!Buffer.from(actual).equals(mapExpected)) throw new Error("canonical reader bounded byte oracle mismatch");
  }
  const reader = readFileSync(join(base, "📖️reader/🦀️.rs"), "utf8");
  const readerExact = (text: string) => {
    const close = method(text, /fn close_step\(&mut self, grant: RetainedCloneGrant/);
    const transfer = method(text, /fn take_root\(&mut self\)/);
    const encode = method(text, /fn encode_chunk\(&mut self, grant: ArtifactStoreOneItemGrant/);
    return text.indexOf("encoder: ArtifactCanonicalEditEncoder") < text.indexOf("root: Option<Arc<T>>")
      && close.indexOf("self.encoder.close_step(grant)?") >= 0 && close.indexOf("self.encoder.close_step(grant)?") < close.indexOf("self.root.take()")
      && transfer.includes("if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() }") && transfer.includes("self.encoder.reset()")
      && text.includes("owned: ManuallyDrop<ReaderState<T>>") && text.includes("!self.cancelled && !self.failed && !self.closing && self.encoder.is_complete()")
      && text.includes("!std::thread::panicking()") && text.includes("ManuallyDrop::drop(&mut self.owned)")
      && encode.includes("self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum],grant.retained_grant())") && encode.includes("grant.maximum_copy_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES)")
      && !/seal_prepared|serde_json::to_(?:vec|value|string)|unsafe impl/.test(text);
  };
  if (!readerExact(reader)) throw new Error("canonical reader exact retained root wiring missing");
  const readerSourceHostiles = [
    replaceClause(reader, "self.encoder.close_step(grant)?", "self.root.take()"),
    reader.replace("if self.closing { self.encoder.terminal_is_empty() } else { self.is_complete() }", "self.encoder.is_complete() || self.closing && self.encoder.terminal_is_empty()"),
    reader.replace("self.encoder.encode_chunk(root.as_ref(), &mut output[..maximum],grant.retained_grant())", "serde_json::to_vec(root)"),
    reader.replace("grant.maximum_copy_bytes.min(output.len()).min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES)", "output.len()"),
    reader.replace("owned: ManuallyDrop<ReaderState<T>>", "owned: ReaderState<T>"),
    reader.replace("!std::thread::panicking()", "true"),
  ];
  for (const hostile of readerSourceHostiles) if (readerExact(hostile)) throw new Error("canonical reader accepted hostile ownership/grant substitution");
  return { grants: fixture.grants.length - 1, editHostiles: editHostiles.length, sourceHostiles: sourceHostiles.length, digestOracles: 1, mapGrants: mapDescriptor.grants.length - 1, mapEditHostiles: mapEditHostiles.length, mapSourceHostiles: mapSourceHostiles.length, mapDigestOracles: 1, readerChecks: 1 + readerFixture.grants.length - 1 + readerSourceHostiles.length + canonicalErrorProgressSelfTests() };
}
