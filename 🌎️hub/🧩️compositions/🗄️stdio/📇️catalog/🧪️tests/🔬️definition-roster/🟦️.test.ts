import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const stdio = join(import.meta.dir, "../../..");
const read = (path: string) => readFileSync(join(stdio, path), "utf8");
const fixture = JSON.parse(read("📇️catalog/🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json")) as { identities: string[]; definitionOnly: string[] };
const roster = (source: string) => source.slice(source.indexOf("vec![") + 5, source.indexOf("]", source.indexOf("vec![")));
const crates = (source: string) => [...source.matchAll(/(semio_s_artifact_stdio_\w+)::contribution\(\)|\b(ifc|semio)_contribution\(\)/gu)].map((match) => match[1] ?? `semio_s_artifact_stdio_${match[2]}`);

test("the definition roster names the full roster's artifacts in the same order and substitutes only the definition-only ones", () => {
  const full = crates(roster(read("🤖️generated/🧩️composition/🦀️.rs"))).map((name) => name.replace(/_catalog$/u, ""));
  const light = crates(roster(read("🔌️plugin/📇️catalog/📜️definition/🦀️.rs")));
  expect(full).toEqual(light);
  expect(full.map((name) => name.replace("semio_s_artifact_stdio_", ""))).toEqual(fixture.identities);
  expect(fixture.definitionOnly.every((identity) => fixture.identities.includes(identity))).toBe(true);
});

test("only definition-only artifacts are contributed without their runtime crate", () => {
  const source = roster(read("🔌️plugin/📇️catalog/📜️definition/🦀️.rs"));
  const runtime = [...source.matchAll(/(semio_s_artifact_stdio_\w+)::contribution\(\)/gu)].map((match) => match[1]!.replace("semio_s_artifact_stdio_", ""));
  expect([...runtime, ...fixture.definitionOnly].sort()).toEqual([...fixture.identities].sort());
});
