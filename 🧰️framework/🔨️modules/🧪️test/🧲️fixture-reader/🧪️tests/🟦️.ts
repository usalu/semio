/** 🧪️ Replays the language-neutral vectors of the Rust fixture reader (`semio-framework-test-fixtures`) with `node:fs` and `node:crypto`: the bytes, digests, UTF-8 verdicts and refusals the Rust tests assert are what an independent reader sees. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const packageDirectory = resolve(import.meta.dir, "../📦️packages/🦀️rust");
const libraryText = readFileSync(resolve(import.meta.dir, "../🦀️.rs"), "utf8");
const [, ...lines] = readFileSync(resolve(import.meta.dir, "🧫️fixtures/📊️.csv"), "utf8").trimEnd().split("\n");
const vectors = lines.map(line => {
  const [kind, id, relative, bytesHex, sha256, messageContains] = line.split(",");
  return { kind: kind!, id: id!, relative: relative!, bytesHex: bytesHex!, sha256: sha256!, messageContains: messageContains!.split("|").filter(Boolean) };
});

test("the vectors cover text, binary and refusals", () => {
  expect(vectors.map(vector => vector.kind)).toEqual(["text", "binary", "refuse"]);
});

test("every readable vector has the recorded bytes and digest", () => {
  for (const vector of vectors.filter(row => row.kind !== "refuse")) {
    const bytes = readFileSync(resolve(packageDirectory, vector.relative));
    expect(bytes.toString("hex"), vector.id).toBe(vector.bytesHex);
    expect(createHash("sha256").update(bytes).digest("hex"), vector.id).toBe(vector.sha256);
  }
});

test("text vectors decode as UTF-8 and binary vectors do not", () => {
  const strict = new TextDecoder("utf-8", { fatal: true });
  for (const vector of vectors.filter(row => row.kind === "text")) expect(() => strict.decode(readFileSync(resolve(packageDirectory, vector.relative))), vector.id).not.toThrow();
  for (const vector of vectors.filter(row => row.kind === "binary")) expect(() => strict.decode(readFileSync(resolve(packageDirectory, vector.relative))), vector.id).toThrow();
});

test("refusal vectors name files that do not exist and tokens the Rust reader reports", () => {
  for (const vector of vectors.filter(row => row.kind === "refuse")) {
    expect(() => readFileSync(resolve(packageDirectory, vector.relative)), vector.id).toThrow();
    for (const needle of vector.messageContains.filter(token => token !== "missing.txt")) expect(libraryText, needle).toContain(needle);
  }
});

test("the Rust reader resolves below the manifest directory of the caller", () => {
  expect(libraryText).toContain("env!(\"CARGO_MANIFEST_DIR\")");
  for (const name of ["fixture_text", "fixture_bytes", "fixture_path", "manifest_dir", "repo_root", "repo_text", "repo_bytes"]) expect(libraryText).toContain("macro_rules! " + name);
});
