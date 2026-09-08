import { join } from "node:path";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { WORKSPACE_ROOT, toolJobRustBlock } from "../../../../../../../../📜️script.ts";

/** 🧪️ Executes canonical error progress policy assertions. */
export function canonicalErrorProgressSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit");
  const fixture = JSON.parse(readFileSync(join(base, "🧫️fixtures/🚧️canonical-error-progress.json"), "utf8"));
  const schema = JSON.parse(readFileSync(join(base, "🧬️schema/🔣️.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/CanonicalErrorProgress" });
  if (!validate(fixture)) throw new Error("canonical error-progress fixture violates strict schema");
  const prefix = Buffer.from(JSON.stringify([fixture.text, null]).slice(0, -5));
  if (!prefix.equals(Buffer.from(fixture.expectedPrefix)) || prefix.length !== fixture.expectedBytes || Buffer.byteLength(fixture.text) !== fixture.expectedSnapshotBytes) throw new Error("canonical error-progress independent JSON/UTF-8 oracle mismatch");
  const hostiles = [{ ...fixture, extra: true }, { ...fixture, expectedBytes: 7 }, { ...fixture, expectedComplete: true }, { ...fixture, grants: [4097] }, { ...fixture, expectedRootRetirements: 0 }];
  for (const hostile of hostiles) if (validate(hostile)) throw new Error("canonical error-progress schema admitted forged credit or completion");
  let checks = 1 + hostiles.length;
  for (const mode of fixture.modes) for (const maximum of fixture.grants) {
    const grant = Math.min(maximum, 256);
    let written = 0;
    if (grant > 0) while (written < prefix.length) { const count = Math.min(grant, prefix.length - written); const output = Buffer.alloc(512, fixture.sentinel); prefix.copy(output, 0, written, written + count); if (!output.subarray(count).every(byte => byte === fixture.sentinel)) throw new Error(`canonical error-progress ${mode} touched uninitialized suffix`); written += count; }
    if (written !== (grant === 0 ? 0 : fixture.expectedBytes)) throw new Error("canonical error-progress grant oracle changed initialized credit");
    checks += 1;
  }
  const parent = readFileSync(join(base, "🦀️.rs"), "utf8");
  const borrowed = readFileSync(join(base, "🧵️borrowed/🦀️.rs"), "utf8");
  const reader = readFileSync(join(base, "📖️reader/🦀️.rs"), "utf8");
  const method = (text: string, pattern: RegExp) => { const start = text.search(pattern); return start < 0 ? "" : toolJobRustBlock(text, text.indexOf("{", start))?.body ?? ""; };
  const exact = (parent: string, borrowed: string, reader: string) => {
    const indexed = method(parent, /pub fn encode_chunk\(/);
    const borrowing = method(borrowed, /fn encode_chunk</);
    const reading = method(reader, /fn encode_chunk\(/);
    const sealing = method(parent, /pub fn advance\(/);
    return parent.includes("pub struct ArtifactCanonicalJsonEncodeError") && parent.includes("pub written_bytes: usize") && parent.includes("pub reason: String")
      && indexed.includes("ArtifactCanonicalJsonEncodeError { written_bytes: written, reason }") && borrowing.includes("ArtifactCanonicalJsonEncodeError { written_bytes: written, reason }")
      && borrowing.includes("ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason }")
      && reading.includes("Err(error) => { self.failed = true; error.written_bytes }") && reading.includes("self.completed_bytes.checked_add(count as u64)")
      && reading.indexOf("self.completed_bytes = completed;") >= 0 && reading.indexOf("self.completed_bytes = completed;") < reading.lastIndexOf("result")
      && sealing.includes("self.cancelled = true; encoding_error = Some(error.reason); error.written_bytes")
      && sealing.indexOf("self.transcript.update(&self.last_chunk[..self.last_length]);") >= 0 && sealing.indexOf("self.transcript.update(&self.last_chunk[..self.last_length]);") < sealing.indexOf("if let Some(error) = encoding_error { return Err(error); }")
      && sealing.indexOf("self.completed_bytes =") >= 0 && sealing.indexOf("self.completed_bytes =") < sealing.indexOf("if let Some(error) = encoding_error { return Err(error); }")
      && sealing.includes("!grant.permits_one() || self.cancelled || self.closing")
      && !/From<ArtifactCanonicalJsonEncodeError> for String/.test(parent + borrowed + reader);
  };
  if (!exact(parent, borrowed, reader)) throw new Error("canonical error-progress live encoder/reader/sealer linkage is missing");
  const mutations: [string, string, string][] = [
    [parent.replace("written_bytes: written, reason", "written_bytes: 0, reason"), borrowed, reader],
    [parent, borrowed.replace("written_bytes: written, reason", "written_bytes: 0, reason"), reader],
    [parent, borrowed, reader.replace("self.failed = true; error.written_bytes", "self.failed = true; 0")],
    [parent, borrowed, reader.replace("self.completed_bytes = completed;", "return result;")],
    [parent.replace("self.cancelled = true; encoding_error", "encoding_error"), borrowed, reader],
    [parent.replace("self.transcript.update(&self.last_chunk[..self.last_length]);", "self.transcript.update(&[]);"), borrowed, reader],
    [parent + "\nimpl From<ArtifactCanonicalJsonEncodeError> for String {}", borrowed, reader],
  ];
  for (const mutation of mutations) if (exact(...mutation)) throw new Error("canonical error-progress admitted lost initialized bytes or resumed failed authority");
  checks += 1 + mutations.length;
  return checks;
}
