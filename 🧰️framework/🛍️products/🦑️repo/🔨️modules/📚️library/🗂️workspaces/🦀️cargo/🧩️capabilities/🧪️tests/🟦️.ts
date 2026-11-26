import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import { selectCargoCapabilityTargetsV1 } from "../🟦️.ts";
import { createRequire } from "node:module";

/** 🧪️ Proves present-target selection against an independent schema implementation. */
export function runCargoCapabilityContributionChecks(): number {
  const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
  const corpus = read("./🧫️fixtures/🔣️.json"), schema = read("../🧬️schema/🔣️.json");
  const valid = new Ajv({ strict: true }).compile(schema);
  assert(valid(corpus.links));
  for (const vector of corpus.cases) {
    const selected = selectCargoCapabilityTargetsV1(corpus.links, new Set<string>(vector.present));
    const oracle = corpus.links.groups.flatMap((group: { targets: { package: string; feature: string }[] }) => group.targets.filter((target) => vector.present.includes(target.package))).map((target: { feature: string }) => target.feature);
    assert.deepEqual(selected.map((target) => target.feature), oracle);
    assert.deepEqual(oracle, vector.expected);
  }
  assert(!valid({ ...corpus.links, unknown: true }));
  assert.throws(() => selectCargoCapabilityTargetsV1({ ...corpus.links, unknown: true }, new Set()));
  return corpus.cases.length + 2;
}

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { prepareCargoCapabilityLinksV1, admitCargoCapabilityLinksV1, writeCargoManifestAtomicV1 } from "../🟦️.ts";

/** 🧱️ Proves physical target removal, partial-owner refusal and source-preserving publication. */
export function runCargoCapabilityPhysicalChecks(): number {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifacts, { recursive: true });
  const root = mkdtempSync(join(artifacts, "cargo-capability-")), owner = join(root, "owner");
  mkdirSync(owner);
  const corpus = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  writeFileSync(join(owner, "links.json"), JSON.stringify(corpus.links));
  const source = '[package]\nname = "neutral-parent"\nversion = "0.0.0"\n[features]\n# 🧩️ Capability Features\n# /🧩️ Capability Features\n[dependencies]\n# 🧩️ Capability Dependencies\n# /🧩️ Capability Dependencies\n';
  writeFileSync(join(owner, "Cargo.toml"), source);
  for (const name of ["alpha", "bravo"]) { mkdirSync(join(root, name)); writeFileSync(join(root, name, "Cargo.toml"), `[package]\nname = "neutral-${name}"\n`); }
  assert.equal(prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"), 2);
  const before = readFileSync(join(owner, "Cargo.toml"), "utf8");
  rmSync(join(root, "alpha"), { recursive: true });
  assert.equal(prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"), 1);
  const after = readFileSync(join(owner, "Cargo.toml"), "utf8");
  assert(!after.includes("neutral-alpha")); assert(after.includes("neutral-bravo"));
  const document = Bun.TOML.parse(after) as any;
  assert.deepEqual(document.features["neutral-capabilities"], ["bravo-link"]);
  assert.deepEqual(document.dependencies["neutral-bravo"], { workspace: true, optional: true });
  mkdirSync(join(root, "alpha"));
  assert.throws(() => prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"));
  assert.equal(readFileSync(join(owner, "Cargo.toml"), "utf8"), after);
  rmSync(join(root, "alpha"), { recursive: true }); rmSync(join(root, "bravo"), { recursive: true });
  assert.equal(prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"), 0);
  assert.deepEqual((Bun.TOML.parse(readFileSync(join(owner, "Cargo.toml"), "utf8")) as any).features["neutral-capabilities"], []);
  const hostile = structuredClone(corpus.links); hostile.groups[0].targets[0].owner = "../../escape";
  writeFileSync(join(owner, "links.json"), JSON.stringify(hostile));
  assert.throws(() => prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"));
  const cyclic = structuredClone(corpus.links); cyclic.groups[0].requires = [cyclic.groups[0].feature];
  assert.throws(() => admitCargoCapabilityLinksV1(cyclic));
  assert(before.includes('name = "neutral-parent"'));
  writeFileSync(join(owner, "links.json"), JSON.stringify(corpus.links));
  mkdirSync(join(root, "bravo"));
  writeFileSync(join(root, "bravo", "Cargo.toml"), '[package]\nname = "neutral-bravo"\n');
  const outside = source + 'neutral-bravo = { workspace = true, optional = true }\n';
  writeFileSync(join(owner, "Cargo.toml"), outside);
  assert.equal(prepareCargoCapabilityLinksV1(root, owner, "Cargo.toml", "links.json"), 1);
  const merged = readFileSync(join(owner, "Cargo.toml"), "utf8");
  assert.equal(merged.split("neutral-bravo = {").length, 2);
  assert.deepEqual((Bun.TOML.parse(merged) as any).dependencies["neutral-bravo"], { workspace: true, optional: true });
  assert.throws(() => writeCargoManifestAtomicV1(join(owner, "Cargo.toml"), outside + outside));
  assert.equal(readFileSync(join(owner, "Cargo.toml"), "utf8"), merged);
  return 9;
}

/** 🚧️ Proves that every tracked or present Cargo manifest parses with the real TOML parser and repeats no key. */
export function runCargoManifestParseLawV1(root: string): number {
  const listed = spawnSync("git", ["-c", "core.quotePath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "Cargo.toml", "**/Cargo.toml"], { cwd: root, encoding: "utf8", maxBuffer: 1 << 28 });
  if (listed.status !== 0) throw new Error("Cargo manifest listing failed");
  const failures: string[] = [], manifests = listed.stdout.split("\0").filter(Boolean);
  for (const manifest of manifests) {
    try { Bun.TOML.parse(readFileSync(join(root, manifest), "utf8")); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") failures.push(`${manifest}: ${String(error).slice(0, 160)}`); }
  }
  assert.deepEqual(failures, []);
  assert(manifests.length > 0);
  return manifests.length;
}

/** 🧱️ Proves that every crate manifest of the tree has a build owner row, so tooling that writes crate manifests never leaves a native dependency the graph cannot resolve; the message names the repair command the operations catalogue declares. Returns the number of crate manifests without a row, which is 0 when it passes. */
export function runCargoManifestOwnerRowLawV1(root: string): number {
  const lazy = createRequire(import.meta.url);
  const { ownerlessCrateManifests } = lazy("../../../../🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts");
  const { buildRepairCommandV1 } = lazy("../../../../🏗️build/⚙️operations/🔤️options/🟦️.ts");
  const missing = ownerlessCrateManifests(root);
  assert.deepEqual(missing, [], `crate manifests without a build owner row; run ${buildRepairCommandV1("build-owners-sync")}`);
  return missing.length;
}
