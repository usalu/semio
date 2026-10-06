import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

type ArtifactAdmissionFixtureV1 = Readonly<{
  schema: string;
  cases: readonly Readonly<{ id: string; plugin: string; package: string; channel: "artifact" | "definition" | "tree"; kind: string; code: "accepted" | "plugin-assembly.artifact-kind" | "plugin-assembly.artifact-owner" }>[];
  firstParty: readonly Readonly<{ plugin: string; kind: string; definition: string; root: string }>[];
}>;

type CompletionRejectionLawV1 = Readonly<{
  schema: string;
  noImplicitRetry: boolean;
  cases: readonly Readonly<{ id: string; cell: "empty" | "existing"; busy: boolean; outcome: "accepted" | "rejected"; finalCell: "empty" | "existing" | "submitted"; submittedOwnerReturned: boolean }>[];
  callers: readonly Readonly<{ family: string; source: string; terminalGuard: string; incrementalClose: string; retainedPhysicalOwners: readonly string[]; legacyLossToken: string }>[];
  reservedCallers: readonly Readonly<{ family: string; source: string; sourceStart: string; sourceEnd: string; terminalGuard: string; retainedOwner: string; incrementalClose: string; legacyLossToken: string }>[];
}>;

type AwaitedCompletionFixtureV1 = Readonly<{ version: 1; ownerReturningRejection: CompletionRejectionLawV1 }>;

/** 🪪️ Independent admission oracle: structural AJV validation and separately decoded owner grammar. */
export function artifactAdmissionOracle(repoRoot?: string): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🏗️builder/🧫️fixtures/🪪️artifact-admission/🔣️.json", import.meta.url), "utf8")) as ArtifactAdmissionFixtureV1;
  const schema = JSON.parse(readFileSync(new URL("../../🏗️builder/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  
  
  const canonical = ajv.compile({ type: "string", pattern: "^s\\.[a-z0-9]+(?:-[a-z0-9]+)*\\.[a-z0-9]+(?:-[a-z0-9]+)*$" });
  const segment = (value: string) => value.length > 0 && !value.startsWith("-") && !value.endsWith("-") && !value.includes("--") && [...value].every((char) => "abcdefghijklmnopqrstuvwxyz0123456789-".includes(char));
  assert.equal(new Set(fixture.cases.map((row) => row.id)).size, fixture.cases.length);
  for (const row of fixture.cases) {
    const parts = row.kind.split(".");
    const valid = parts.length === 3 && parts[0] === "s" && parts.slice(1).every(segment);
    assert.equal(canonical(row.kind), valid, row.id);
    assert.equal(row.package, `semio:${row.plugin}`, row.id);
    const code = !valid ? "plugin-assembly.artifact-kind" : parts[1] !== row.plugin ? "plugin-assembly.artifact-owner" : "accepted";
    assert.equal(code, row.code, row.id);
  }
  if (repoRoot) {
    assert.equal(new Set(fixture.firstParty.map((row) => row.kind)).size, 39);
    for (const row of fixture.firstParty) {
      assert.equal(row.kind.split(".")[1], row.plugin);
      const definition: string = readFileSync(resolve(repoRoot, row.definition), "utf8");
      const root: string = readFileSync(resolve(repoRoot, row.root), "utf8");
      assert(definition.includes(`ArtifactDefinition::new(ArtifactIdentity::parse("${row.kind}")`), row.definition);
      assert(root.includes(`.package_id("semio:${row.plugin}")`), row.root);
      const roots: string[] = [...definition.matchAll(/ArtifactDefinition::new\(ArtifactIdentity::parse\("([^"]+)"/g)].map((match) => match[1]!);
      assert.deepEqual(roots, [row.kind]);
      const capabilityIds = [...definition.matchAll(/^\s*\("(s\.[^"]+)",\s*"(?:standard|profile|schema|inference|grammar|codec|localization|resource|representation|mutation)"/gm)].map((match) => match[1]!);
      assert(
        capabilityIds.every((kind) => kind.startsWith(`${row.kind}.`)),
        row.definition,
      );
    }
  }
  return fixture.cases.length;
}

/** ♻️ Independently models completion admission and pins every migrated caller to its retained close owner. */
export function completionRejectionOracle(repoRoot?: string): number {
  const fixture: unknown = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏳️completion/🔣️.json", import.meta.url), "utf8"));
  const law = fixture.ownerReturningRejection;
  for (const replacement of [{ incrementalClose: "emit.close_child_one(maximum_items, maximum_bytes)" }, { retainedPhysicalOwners: [] }]) {
    const callers = law.callers.map((row) => row.family === "writer" ? { ...row, ...replacement } : row);
  }
  assert.equal(new Set(law.cases.map((row) => row.id)).size, law.cases.length);
  for (const row of law.cases) {
    const rejected = row.busy || row.cell !== "empty";
    const outcome = rejected ? "rejected" : "accepted";
    const finalCell = rejected ? row.cell : "submitted";
    assert.equal(outcome, row.outcome, row.id);
    assert.equal(finalCell, row.finalCell, row.id);
    assert.equal(rejected, row.submittedOwnerReturned, row.id);
  }
  if (repoRoot) {
    assert.equal(new Set(law.callers.map((row) => row.family)).size, law.callers.length);
    for (const row of law.callers) {
      const source: string = readFileSync(resolve(repoRoot, row.source), "utf8");
      const retain = source.indexOf("self.pending_completion_rejection = Some(rejected)");
      const guard = source.indexOf(row.terminalGuard);
      const close = source.indexOf(row.incrementalClose);
      assert(retain >= 0, `${row.family} loses the returned completion owner`);
      assert(guard >= 0, `${row.family} can replay after terminal completion rejection`);
      assert(close > retain, `${row.family} lacks its declared child-first rejection close operation`);
      for (const owner of row.retainedPhysicalOwners) assert(source.includes(owner), `${row.family} lost a retained physical parent terminal guard: ${owner}`);
      assert.equal(source.includes(row.legacyLossToken), false, `${row.family} retained the lossy is_err handoff`);
    }
    assert.equal(new Set(law.reservedCallers.map((row) => row.family)).size, law.reservedCallers.length);
    for (const row of law.reservedCallers) {
      const source: string = readFileSync(resolve(repoRoot, row.source), "utf8");
      const region: string | undefined = source.split(row.sourceStart, 2)[1]?.split(row.sourceEnd, 1)[0];
      assert(region !== undefined, `${row.family} source region is absent`);
      const guard = region.indexOf(row.terminalGuard);
      const prepare = region.indexOf("commit.prepare");
      const retain = region.indexOf(row.retainedOwner);
      const close = region.indexOf(row.incrementalClose);
      assert(guard >= 0 && guard < prepare, `${row.family} can replay after a rejected completion`);
      assert(prepare >= 0 && prepare < retain, `${row.family} does not retain the exact returned owner`);
      assert(close > retain, `${row.family} does not retire the returned owner incrementally`);
      assert.equal(region.includes(row.legacyLossToken), false, `${row.family} retained the lossy completion handoff`);
    }
  }
  return law.cases.length + law.callers.length + law.reservedCallers.length;
}
