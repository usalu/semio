import { describe, expect, test } from "bun:test";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { assessMutationCrateBudget, newScaffoldMutationTree } from "../../🏗️authoring/🧬️mutation-tree/🟦️.ts";
import { mutationFixtureRoot } from "../🧬️mutation-authority/🧫️fixture/🟦️.ts";

const mutationsRel = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";
const artifactRel = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model";

function seed(root: string): void {
  const files: Record<string, string> = {
    [`${artifactRel}/Cargo.toml`]: '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n',
    [`${artifactRel}/Cargo.lock`]: 'version = 4\n\n[[package]]\nname = "demo-energy-model"\nversion = "0.1.0"\n',
    [`${artifactRel}/📦️packages/🦀️rust/Cargo.toml`]: '[package]\nworkspace = "../.."\nname = "demo-energy-model"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
    [`${artifactRel}/📦️packages/🦀️rust/📜️script.ts`]: 'await runArtifactRustPackageMain(import.meta.dir, "demo-energy-model",{snapshotSqliteTests:[]});\n',
    [`${artifactRel}/🦀️.rs`]: "//! 🔋️ demo\npub mod mutations {\n    #[path = \"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs\"]\n    pub mod root;\n}\n",
    [`${mutationsRel}/🦀️.rs`]: "#[derive(Clone, Debug)]\n#[serde(tag = \"kind\")]\npub enum EnergyModelMutation {}\n",
  };
  for (const [path, content] of Object.entries(files)) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
}

describe("mutation families", () => {
  test("a family leaf mounts in its own crate and the aggregate only names the family", () => {
    const root = mutationFixtureRoot("semio-mutation-family-");
    try {
      seed(root);
      const result = newScaffoldMutationTree(root, mutationsRel, "🏡️zone/🏘️create", { familyCrate: true }, false);
      const familyRoot = readFileSync(join(root, mutationsRel, "🏡️zone/🦀️.rs"), "utf8");
      expect(familyRoot).toContain('#[path = "🏘️create/🦀️.rs"]\npub mod create_zone;');
      const aggregate = readFileSync(join(root, mutationsRel, "🦀️.rs"), "utf8");
      expect(aggregate).toContain("extern crate demo_energy_model_mut_zone as zone_mutations;");
      expect(aggregate).toContain("CreateZone(zone_mutations::create_zone::Mutation)");
      expect(aggregate).not.toContain("#[path");
      const manifest = readFileSync(join(root, mutationsRel, "🏡️zone/📦️packages/🦀️rust/Cargo.toml"), "utf8");
      expect(manifest).toContain('name = "demo-energy-model-mut-zone"');
      expect(manifest).toContain('path = "../../🦀️.rs"');
      expect(readFileSync(join(root, artifactRel, "Cargo.toml"), "utf8")).toContain("🧬️mutations/🏡️zone/📦️packages/🦀️rust");
      expect(readFileSync(join(root, artifactRel, "📦️packages/🦀️rust/Cargo.toml"), "utf8")).toContain("demo-energy-model-mut-zone = { path =");
      expect(readFileSync(join(root, artifactRel, "📦️packages/🦀️rust/📜️script.ts"), "utf8")).toContain("demo-energy-model-mut-zone");
      expect(readFileSync(join(root, artifactRel, "Cargo.lock"), "utf8")).toContain('name = "demo-energy-model-mut-zone"');
      expect(result.created).toContain(`${mutationsRel}/🏡️zone/🏘️create/🦀️.rs`);
      const second = newScaffoldMutationTree(root, mutationsRel, "🏡️zone/🏚️delete", { familyCrate: true }, false);
      expect(second.updated).not.toContain("member demo-energy-model-mut-zone");
      expect(readFileSync(join(root, mutationsRel, "🏡️zone/🦀️.rs"), "utf8")).toContain("pub mod delete_zone;");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("a family leaf needs an operation name and an owning crate", () => {
    const root = mutationFixtureRoot("semio-mutation-family-refusal-");
    try {
      seed(root);
      expect(() => newScaffoldMutationTree(root, mutationsRel, "🏡️zone", { familyCrate: true }, false)).toThrow(/domain|registration/iu);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("the budget assessment reports no crate where Cargo does not own the root", async () => {
    const root = mutationFixtureRoot("semio-mutation-family-budget-");
    try {
      mkdirSync(join(root, mutationsRel), { recursive: true });
      expect(await assessMutationCrateBudget(root, mutationsRel)).toBeNull();
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
