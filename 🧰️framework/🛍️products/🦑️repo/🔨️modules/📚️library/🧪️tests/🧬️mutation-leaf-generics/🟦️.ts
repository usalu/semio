import { describe, expect, test } from "bun:test";
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { planArtifactCrates } from "../../🏗️authoring/📦️crate-tree/🟦️.ts";
import { isMutationCrateLabel, mutationCrateLabel, mutationFamilyOf } from "../../🏗️authoring/📦️crate-tree/🗺️plan/🏷️naming/🟦️.ts";
import { inherentSelf } from "../../🏗️authoring/📦️crate-tree/🧬️mutation-families/🧰️support-hoist/🟦️.ts";
import { aggregateScope, convertMutationLeaves, decoupleFromAggregate, findMutationAggregates } from "../../🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
import { hoistAggregateSupport } from "../../🏗️authoring/📦️crate-tree/🧬️mutation-families/🧰️support-hoist/🟦️.ts";
import { pruneUseLeaves } from "../../🏗️authoring/📦️crate-tree/🧬️mutation-families/🌳️use-tree/🟦️.ts";
import { collectDecls } from "../../🏗️authoring/📦️crate-tree/🧱️decl-tree/🟦️.ts";

const base = "🧬️schema/🧬️mutations";

const leaf = (name: string, struct: string, extra: string, inverseBody: string): Record<string, string> => ({
  [`${base}/${name}/🦀️.rs`]: [
    `//! ${name} leaf`,
    "use crate::schema::mutations::DemoMutation;",
    "use crate::schema::{Diff, Snap};",
    "use protocol::{MutationKind, SemanticDescriptor};",
    "",
    "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]",
    "#[mutation_leaf(contract = ::protocol)]",
    `pub struct ${struct} { pub id: String }`,
    "",
    `pub fn ${name}(id: String) -> DemoMutation {`,
    `    DemoMutation::${struct}(${struct} { id })`,
    "}",
    "",
    `impl MutationKind<Snap, DemoMutation> for ${struct} {`,
    "    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: \"x\", entity: \"y\", kind: \"z\", record: \"R\" };",
    "    fn diff(&self, base: &Snap) -> protocol::MutationOutcome<Diff> {",
    "        super::diff::diff(self, base)",
    "    }",
    "    fn inverse(&self, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {",
    "        Ok(super::inverse::inverse(self, base)?)",
    "    }",
    "}",
    extra,
    "",
  ].join("\n"),
  [`${base}/${name}/🔺️diff/🦀️.rs`]: `use super::${struct};\nuse crate::schema::{Diff, Snap};\npub fn diff(_payload: &${struct}, _base: &Snap) -> protocol::MutationOutcome<Diff> { protocol::MutationOutcome::empty() }\n`,
  [`${base}/${name}/↩️inverse/🦀️.rs`]: [
    `use super::${struct};`,
    "use crate::schema::mutations::DemoMutation;",
    "use crate::schema::Snap;",
    `pub fn inverse(payload: &${struct}, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {`,
    inverseBody,
    "}",
    "",
  ].join("\n"),
});

const mount = (name: string, tests: boolean): string => [
  "        #[path = \".\"]",
  `        pub mod ${name} {`,
  `            #[path = "${base}/${name}/🦀️.rs"]`,
  "            mod component;",
  `            #[path = "${base}/${name}/🔺️diff/🦀️.rs"]`,
  "            pub mod diff;",
  `            #[path = "${base}/${name}/↩️inverse/🦀️.rs"]`,
  "            pub mod inverse;",
  "            pub use component::*;",
  ...(tests ? ["            #[cfg(test)]", `            #[path = "${base}/${name}/🧪️tests/🦀️.rs"]`, `            mod tests_${name};`] : []),
  "        }",
].join("\n");

const files: Record<string, string> = {
  "Cargo.toml": '[workspace]\nresolver = "2"\nmembers = [ "📦️packages/🦀️rust" ]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\nrust-version = "1.95"\n',
  "Cargo.lock": 'version = 4\n\n[[package]]\nname = "demo-mut"\nversion = "0.1.0"\n',
  "📦️packages/🦀️rust/Cargo.toml": '[package]\nworkspace = "../.."\nname = "demo-mut"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n',
  "📦️packages/🦀️rust/📜️script.ts": 'await runArtifactRustPackageMain(import.meta.dir, "demo-mut",{snapshotSqliteTests:[]});\n',
  "🦀️.rs": [
    "//! 🧪️ Demo mutation artifact root.",
    "extern crate semio_framework_os_kernel as protocol;",
    "#[path = \".\"]",
    "pub mod schema {",
    "    #[path = \"🧬️schema/🦀️.rs\"]",
    "    mod component;",
    "    pub use component::*;",
    "    #[path = \".\"]",
    "    pub mod mutations {",
    `        #[path = "${base}/🦀️.rs"]`,
    "        mod component;",
    "        pub use component::*;",
    mount("alpha", true),
    mount("beta", true),
    mount("gamma", false),
    "    }",
    "}",
    "",
  ].join("\n"),
  "🧬️schema/🦀️.rs": "//! schema\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Snap { pub id: String }\n#[derive(Clone, Debug, Default, PartialEq)]\npub struct Diff { pub id: String }\n",
  [`${base}/🦀️.rs`]: [
    "//! aggregate",
    "use crate::schema::{Diff, Snap};",
    "",
    "#[derive(Clone, Debug, PartialEq, dsl::Mutations)]",
    "#[mutations(snapshot = Snap, diff = Diff, schema = \"demo.demo\")]",
    "pub enum DemoMutation {",
    "    Alpha(Alpha),",
    "    Beta(Beta),",
    "    Gamma(Gamma),",
    "}",
    "",
    "pub use crate::schema::mutations::alpha::{alpha, Alpha};",
    "pub use crate::schema::mutations::beta::{beta, Beta};",
    "pub use crate::schema::mutations::gamma::{gamma, Gamma};",
    "",
  ].join("\n"),
  ...leaf("alpha", "Alpha", "", "    Ok(vec![DemoMutation::Alpha(Alpha { id: format!(\"{}-{}\", payload.id, base.id) })])"),
  ...leaf("beta", "Beta", "", "    Ok(vec![DemoMutation::Beta(Beta { id: payload.id.clone() }), crate::schema::mutations::alpha::alpha(base.id.clone())])"),
  ...leaf("gamma", "Gamma", "\n#[cfg(test)]\nmod inline_tests {\n    use super::*;\n    #[test]\n    fn builds() { let _ = gamma(String::new()); let _: DemoMutation = gamma(String::new()); }\n}", "    Ok(vec![DemoMutation::Gamma(Gamma { id: payload.id.clone() })])"),
  [`${base}/beta/🧪️tests/🦀️.rs`]: "use super::*;\nuse crate::schema::mutations::DemoMutation;\n#[test]\nfn builds() { let _: DemoMutation = beta(String::new()); }\n",
  [`${base}/alpha/🧪️tests/🦀️.rs`]: "use crate::schema::mutations::{alpha, DemoMutation};\n#[test]\nfn builds() { let _: DemoMutation = alpha(String::new()); }\n",
};

function tree(extra: Record<string, string> = {}): string {
  const root = mkdtempSync(join(tmpdir(), "semio-mutation-leaf-"));
  for (const [path, content] of Object.entries({ ...files, ...extra })) {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  }
  return root;
}

const read = (path: string): string | null => (existsSync(path) ? readFileSync(path, "utf8") : null);
const rustfmt = spawnSync("rustfmt", ["--version"]).status === 0;

function parses(text: string): boolean {
  const result = spawnSync("rustfmt", ["--edition", "2021", "--emit", "stdout"], { input: text, encoding: "utf8" });
  return result.status === 0;
}

describe("mutation leaves generic over their aggregate", () => {
  test("finds the transparent aggregate and its variants", () => {
    const root = tree();
    try {
      const rootFile = join(root, "🦀️.rs");
      const decls = collectDecls(rootFile, files["🦀️.rs"]!, read);
      const aggregates = findMutationAggregates(decls.decls.values(), read);
      expect(aggregates.map((aggregate) => [aggregate.name, aggregate.snapshot, aggregate.diff, aggregate.variants.map((entry) => entry.payload)])).toEqual([["DemoMutation", "Snap", "Diff", ["Alpha", "Beta", "Gamma"]]]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rewrites builder, impl and inverse to be generic, keeps a concrete builder on the aggregate and leaves unsupported leaves concrete", () => {
    const root = tree();
    try {
      const result = convertMutationLeaves(join(root, "🦀️.rs"), files["🦀️.rs"]!, read);
      expect(result.converted).toEqual(["Alpha", "Beta"]);
      expect(result.hand.map((entry) => entry.payload)).toEqual(["Gamma"]);
      const alpha = result.edits.get(join(root, base, "alpha/🦀️.rs"))!;
      expect(alpha).not.toContain("use crate::schema::mutations::DemoMutation;");
      expect(alpha).toContain("pub fn alpha<Op>(id: String) -> Op");
      expect(alpha).toContain("Op::from(Alpha { id })");
      expect(alpha).toMatch(/impl<Op> MutationKind<Snap, Op> for Alpha\s+where\s+Op: protocol::Mutation<Snap, Diff = Diff> \+ From<Alpha>/u);
      expect(alpha).toContain("Result<Vec<Op>, semio_framework_value::ValueError>");
      const betaInverse = result.edits.get(join(root, base, "beta/↩️inverse/🦀️.rs"))!;
      expect(betaInverse).toContain("From<crate::schema::mutations::beta::Beta>");
      expect(betaInverse).toContain("From<crate::schema::mutations::alpha::Alpha>");
      expect(betaInverse).toContain("Op::from(Beta { id: payload.id.clone() })");
      expect(betaInverse).toContain("crate::schema::mutations::alpha::alpha(base.id.clone())");
      const beta = result.edits.get(join(root, base, "beta/🦀️.rs"))!;
      expect(beta).toMatch(/Op: protocol::Mutation<Snap, Diff = Diff> \+ From<Beta> \+ From<crate::schema::mutations::alpha::Alpha>/u);
      const aggregate = result.edits.get(join(root, base, "🦀️.rs"))!;
      expect(aggregate).toContain("pub use crate::schema::mutations::alpha::Alpha;");
      expect(aggregate).toContain("pub use crate::schema::mutations::gamma::{gamma, Gamma};");
      expect(aggregate).toContain("pub fn alpha(id: String) -> DemoMutation {\n    crate::schema::mutations::alpha::alpha::<DemoMutation>(id)\n}");
      expect(aggregate).not.toContain("pub fn gamma");
      expect(result.edits.has(join(root, base, "gamma/🦀️.rs"))).toBe(false);
      expect(result.notes.some((note) => note.includes("Gamma stays concrete"))).toBe(true);
      expect(result.edits.get(join(root, base, "beta/🧪️tests/🦀️.rs"))).toContain("use crate::schema::mutations::beta::*;");
      expect(result.edits.has(join(root, base, "alpha/🧪️tests/🦀️.rs"))).toBe(false);
      expect(convertMutationLeaves(join(root, "🦀️.rs"), files["🦀️.rs"]!, read).edits).toEqual(result.edits);
      if (rustfmt) for (const [file, text] of result.edits) expect([file, parses(text)]).toEqual([file, true]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("keeps a companion that is already generic over its aggregate and carries its where-bounds onto the leaf impl", () => {
    const delta = leaf("delta", "Delta", "", "");
    const root = tree({
      "🦀️.rs": files["🦀️.rs"]!.replace(mount("gamma", false), `${mount("gamma", false)}\n${mount("delta", false)}`),
      [`${base}/🦀️.rs`]: files[`${base}/🦀️.rs`]!.replace("    Gamma(Gamma),", "    Gamma(Gamma),\n    Delta(Delta),").replace("pub use crate::schema::mutations::gamma::{gamma, Gamma};", "pub use crate::schema::mutations::gamma::{gamma, Gamma};\npub use crate::schema::mutations::delta::{delta, Delta};"),
      ...delta,
      [`${base}/delta/↩️inverse/🦀️.rs`]: "use super::Delta;\nuse crate::schema::Snap;\npub fn inverse<Op>(payload: &Delta, base: &Snap) -> Result<Vec<Op>, semio_framework_value::ValueError>\nwhere\n    Op: crate::schema::Restoring + From<crate::schema::mutations::alpha::Alpha>,\n{\n    Ok(Op::restoring(&payload.id, base))\n}\n",
    });
    try {
      const rootText = readFileSync(join(root, "🦀️.rs"), "utf8");
      const result = convertMutationLeaves(join(root, "🦀️.rs"), rootText, read);
      expect(result.converted).toContain("Delta");
      expect(result.edits.has(join(root, base, "delta/↩️inverse/🦀️.rs"))).toBe(false);
      const impl = result.edits.get(join(root, base, "delta/🦀️.rs"))!;
      expect(impl).toMatch(/Op: protocol::Mutation<Snap, Diff = Diff> \+ From<Delta>/u);
      expect(impl).toContain("crate::schema::Restoring");
      expect(impl).toContain("From<crate::schema::mutations::alpha::Alpha>");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("leaves an aggregate concrete when its module is private, because no absolute path can name its leaves", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-mutation-private-"));
    try {
      const layout: Record<string, string> = {
        "🦀️.rs": '#[path = "agg/🦀️.rs"]\nmod mutations;\n',
        "agg/🦀️.rs": 'use crate::{Diff, Snap};\n#[path = "leaf/🦀️.rs"]\nmod leaf;\n#[derive(Clone, Debug, PartialEq, dsl::Mutations)]\n#[mutations(snapshot = Snap, diff = Diff, schema = "demo.demo")]\npub enum DemoMutation {\n    Alpha(leaf::Alpha),\n}\n',
        "agg/leaf/🦀️.rs": 'use super::*;\n#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]\n#[mutation_leaf(contract = ::protocol)]\npub struct Alpha { pub id: String }\n',
      };
      for (const [path, content] of Object.entries(layout)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), content); }
      const result = convertMutationLeaves(join(root, "🦀️.rs"), layout["🦀️.rs"]!, read);
      expect(result.converted).toEqual([]);
      expect(result.notes.some((note) => note.includes("DemoMutation stays concrete: its module is private"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("imports the builder of another leaf from that leaf instead of from the aggregate", () => {
    const inverse = [
      "use super::Beta;",
      "use crate::schema::mutations::{alpha, DemoMutation};",
      "use crate::schema::Snap;",
      "pub fn inverse(payload: &Beta, base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {",
      "    let _ = alpha::Alpha { id: String::new() };",
      "    Ok(vec![DemoMutation::Beta(Beta { id: payload.id.clone() }), alpha(base.id.clone())])",
      "}",
      "",
    ].join("\n");
    const root = tree({ [`${base}/beta/↩️inverse/🦀️.rs`]: inverse });
    try {
      const result = convertMutationLeaves(join(root, "🦀️.rs"), files["🦀️.rs"]!, read);
      const betaInverse = result.edits.get(join(root, base, "beta/↩️inverse/🦀️.rs"))!;
      expect(betaInverse).not.toContain("use crate::schema::mutations::{alpha");
      expect(betaInverse).toContain("use crate::schema::mutations::alpha::{self, alpha};");
      expect(betaInverse).toContain("Op::from(Beta { id: payload.id.clone() })");
      if (rustfmt) expect(parses(betaInverse)).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("writes the sibling leaf payload that an inverse builds as an absolute path, not through super::super", () => {
    const inverse = [
      "use super::Beta;",
      "use crate::schema::mutations::DemoMutation;",
      "use crate::schema::Snap;",
      "pub fn inverse(payload: &Beta, _base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {",
      "    Ok(vec![DemoMutation::Alpha(super::super::alpha::Alpha { id: payload.id.clone() })])",
      "}",
      "",
    ].join("\n");
    const root = tree({ [`${base}/beta/↩️inverse/🦀️.rs`]: inverse });
    try {
      const result = convertMutationLeaves(join(root, "🦀️.rs"), files["🦀️.rs"]!, read);
      const betaInverse = result.edits.get(join(root, base, "beta/↩️inverse/🦀️.rs"))!;
      expect(betaInverse).not.toContain("super::super");
      expect(betaInverse).toContain("Op::from(crate::schema::mutations::alpha::Alpha { id: payload.id.clone() })");
      if (rustfmt) expect(parses(betaInverse)).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("names a leaf-family crate after its family and never after its first leaf", () => {
    const leafEff = (leaf: string): string[] => ["schema", "mutations", leaf];
    expect(mutationFamilyOf(leafEff("insert_element"))).toBe("insert_element");
    expect(mutationFamilyOf(["schema", "mutations"])).toBeNull();
    expect(mutationFamilyOf(["schema", "snapshot"])).toBeNull();
    expect(mutationFamilyOf(["standards", "v1", "subsets", "any", "schema", "mutations", "alpha"])).toBe("any");
    expect(mutationCrateLabel(["alpha"])).toBe("mut-alpha");
    expect(mutationCrateLabel(["insert_element", "set_name", "remove_element"])).toBe("mut-family");
    expect(mutationCrateLabel(["insert_element", "set_name", null])).toBe("mut-family");
    expect(mutationCrateLabel([...Array.from({ length: 26 }, (_, i) => `leaf_${i}`), null])).toBe("mut-family");
    expect(mutationCrateLabel(["any", "any", null])).toBe("mut-any");
    expect(mutationCrateLabel([null, "alpha"])).toBeNull();
    expect(mutationCrateLabel([null, null, "alpha"])).toBeNull();
    expect(mutationCrateLabel([null])).toBeNull();
    expect(isMutationCrateLabel("mut-family")).toBe(true);
    expect(isMutationCrateLabel("insert-element")).toBe(false);
  });

  test("labels every family crate of a planned artifact with the mut- family label", async () => {
    const root = tree();
    try {
      const small = { target: { files: 4, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };
      const family = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...small, mutationFamilies: "on" });
      const leafCrates = family.crates.filter((crate) => crate.slices.every((slice) => slice.attach.every((decl) => decl.eff.includes("mutations") && decl.eff.at(-1) !== "mutations")));
      expect(leafCrates.length).toBeGreaterThan(0);
      for (const crate of leafCrates) expect(crate.name).toMatch(/^demo-mut-mut-[a-z0-9-]+$/u);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("prunes use statements of any nesting like a second parser would", () => {
    expect(pruneUseLeaves("use crate::{PngSnapshot,schema::{snapshot::PngImage,diff::PngDiff,mutations::PngMutation}};", new Set(["PngMutation"]))).toBe("use crate::{PngSnapshot, schema::{snapshot::PngImage, diff::PngDiff}};");
    expect(pruneUseLeaves("use crate::schema::mutations::NoteMutation;", new Set(["NoteMutation"]))).toBeNull();
    expect(pruneUseLeaves("pub use crate::a::b::{move_block, MoveBlock};", new Set(["move_block"]))).toBe("pub use crate::a::b::MoveBlock;");
    expect(pruneUseLeaves("use super::*;", new Set(["X"]))).toBe("use super::*;");
    if (rustfmt) {
      const pruned = pruneUseLeaves("use crate::{A, b::{c, D as E, self}, f::*};", new Set(["c"]))!;
      expect(pruned).toBe("use crate::{A, b::{D as E, self}, f::*};");
      expect(parses(pruned)).toBe(true);
    }
  });

  test("places converted leaves in family crates and keeps unconverted ones with the aggregate", async () => {
    const root = tree();
    try {
      const small = { target: { files: 4, bytes: 230000, lines: 3500 }, hard: { files: 32, bytes: 262144, lines: 4000 } };
      const plain = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...small, mutationFamilies: "off" });
      const family = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...small, mutationFamilies: "on" });
      const holder = (plan: typeof family, suffix: string) => plan.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::").endsWith(suffix)));
      expect(holder(plain, "mutations::alpha")).toBe(holder(plain, "mutations"));
      expect(holder(family, "mutations::alpha")).toBeDefined();
      expect(holder(family, "mutations::alpha")).not.toBe(holder(family, "mutations::beta"));
      expect(holder(family, "mutations::alpha")).not.toBe(holder(family, "mutations::gamma"));
      const betaTests = family.crates.find((crate) => crate.attach.some((decl) => decl.eff.join("::").endsWith("mutations::beta::tests_beta")));
      expect(betaTests).toBeDefined();
      expect(betaTests).not.toBe(holder(family, "schema::mutations"));
      const aggregateCrate = holder(family, "schema::mutations") ?? family.crates.find((crate) => crate.slices.some((slice) => slice.attach.some((decl) => decl.eff.join("::") === "schema::mutations")));
      expect(aggregateCrate).toBeDefined();
      expect(family.overlay.notes.some((note) => note.startsWith("mutation aggregate DemoMutation: 2/3 leaves converted"))).toBe(true);
      const again = await planArtifactCrates(root, "📦️packages/🦀️rust", { ...small, mutationFamilies: "on" });
      expect(again.crates.map((crate) => [crate.name, crate.packageDir])).toEqual(family.crates.map((crate) => [crate.name, crate.packageDir]));
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("accepts associated diff types through the aggregate, glob imports of its variants and relative re-export paths", () => {
    const delta = leaf("delta", "Delta", "", "    use DemoMutation::*;\n    Ok(vec![Delta(Delta { id: payload.id.clone() })])");
    const component = delta[`${base}/delta/🦀️.rs`]!.replace("protocol::MutationOutcome<Diff>", "protocol::MutationOutcome<<DemoMutation as Mutation<Snap>>::Diff>").replace("use protocol::{MutationKind, SemanticDescriptor};", "use protocol::{Mutation, MutationKind, SemanticDescriptor};");
    const root = tree({
      ...delta,
      [`${base}/delta/🦀️.rs`]: component,
      "🦀️.rs": files["🦀️.rs"]!.replace(mount("gamma", false), `${mount("gamma", false)}\n${mount("delta", false)}`),
      [`${base}/🦀️.rs`]: files[`${base}/🦀️.rs`]!.replace("    Gamma(Gamma),", "    Gamma(Gamma),\n    Delta(delta::Delta),").replace("pub use crate::schema::mutations::gamma::{gamma, Gamma};", "pub use crate::schema::mutations::gamma::{gamma, Gamma};\npub use delta::{delta, Delta};"),
    });
    try {
      const rootText = read(join(root, "🦀️.rs"))!;
      const result = convertMutationLeaves(join(root, "🦀️.rs"), rootText, read);
      expect(result.converted).toContain("Delta");
      const converted = result.edits.get(join(root, base, "delta/🦀️.rs"))!;
      expect(converted).toContain("<Op as Mutation<Snap>>::Diff");
      expect(converted).toMatch(/Diff = crate::schema::Diff/u);
      const inverse = result.edits.get(join(root, base, "delta/↩️inverse/🦀️.rs"))!;
      expect(inverse).not.toContain("use DemoMutation::*;");
      expect(inverse).toContain("Op::from(Delta { id: payload.id.clone() })");
      expect(result.edits.get(join(root, base, "🦀️.rs"))).toContain("pub fn delta(id: String) -> DemoMutation");
      if (rustfmt) for (const [file, text] of result.edits) expect([file, parses(text)]).toEqual([file, true]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("turns the sibling imports of a converted leaf into absolute paths so the leaf no longer reaches its aggregate through super", () => {
    const direct = (name: string, struct: string, other: string, otherStruct: string): Record<string, string> => ({
      [`${base}/${name}/🦀️.rs`]: [
        `//! ${name} leaf`,
        `use super::${other}::${otherStruct};`,
        "use crate::schema::mutations::DemoMutation;",
        "use crate::schema::{Diff, Snap};",
        "use protocol::{MutationKind, SemanticDescriptor};",
        "",
        "#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf)]",
        "#[mutation_leaf(contract = ::protocol)]",
        `pub struct ${struct} { pub id: String }`,
        "",
        `impl MutationKind<Snap, DemoMutation> for ${struct} {`,
        "    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: \"x\", entity: \"y\", kind: \"z\", record: \"R\" };",
        "    fn diff(&self, _base: &Snap) -> protocol::MutationOutcome<Diff> { protocol::MutationOutcome::empty() }",
        "    fn inverse(&self, _base: &Snap) -> Result<Vec<DemoMutation>, semio_framework_value::ValueError> {",
        `        Ok(vec![DemoMutation::${otherStruct}(${otherStruct} { id: self.id.clone() })])`,
        "    }",
        "}",
        "",
      ].join("\n"),
    });
    const aggregate = [
      "//! aggregate",
      "use crate::schema::{Diff, Snap};",
      "",
      `#[path = "delta/🦀️.rs"]`,
      "pub mod delta;",
      `#[path = "epsilon/🦀️.rs"]`,
      "pub mod epsilon;",
      "pub use delta::Delta;",
      "pub use epsilon::Epsilon;",
      "",
      "#[derive(Clone, Debug, PartialEq, dsl::Mutations)]",
      "#[mutations(snapshot = Snap, diff = Diff, schema = \"demo.demo\")]",
      "pub enum DemoMutation {",
      "    Delta(Delta),",
      "    Epsilon(Epsilon),",
      "}",
      "",
    ].join("\n");
    const root = tree({
      ...direct("delta", "Delta", "epsilon", "Epsilon"),
      ...direct("epsilon", "Epsilon", "delta", "Delta"),
      [`${base}/🦀️.rs`]: aggregate,
      "🦀️.rs": files["🦀️.rs"]!.replace(/\n        #\[path = "\."\]\n        pub mod (alpha|beta|gamma) \{[\s\S]*?\n        \}/gu, ""),
    });
    try {
      const rootText = read(join(root, "🦀️.rs"))!;
      const result = convertMutationLeaves(join(root, "🦀️.rs"), rootText, read);
      expect(result.converted).toEqual(expect.arrayContaining(["Delta", "Epsilon"]));
      const delta = result.edits.get(join(root, base, "delta/🦀️.rs"))!;
      expect(delta).not.toMatch(/\bsuper::/u);
      expect(delta).toContain("use crate::schema::mutations::epsilon::Epsilon;");
      expect(delta).not.toContain("component");
      expect(delta).toContain("From<crate::schema::mutations::epsilon::Epsilon>");
      if (rustfmt) expect(parses(delta)).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
  test("hosts the helpers its leaves call in a support module and keeps the items that mention the aggregate or a child module", () => {
    const aggregate = [
      "use crate::schema::{Diff, Snap};",
      "use protocol::Mutation;",
      "",
      "#[path = \"🎬alpha/🦀️.rs\"]",
      "pub mod alpha;",
      "pub enum DemoMutation { Alpha(alpha::Alpha) }",
      "",
      "//#region 🔖️Helpers",
      "/// 🧱️ Wraps a diff.",
      "// 🚫️async: pure",
      "fn wrap(diff: Diff) -> Diff { diff }",
      "pub const LIMIT: usize = 4;",
      "fn at(base: &Snap, index: usize) -> Option<usize> { if index < LIMIT { Some(index + wrap_len(base)) } else { None } }",
      "fn wrap_len(_base: &Snap) -> usize { 1 }",
      "fn builds_alpha() -> alpha::Alpha { alpha::Alpha }",
      "fn mentions(mutation: &DemoMutation) -> bool { matches!(mutation, DemoMutation::Alpha(_)) }",
      "//#endregion 🔖️Helpers",
      "",
    ].join("\n");
    const leaf = "use super::*;\npub fn diff(base: &Snap) -> Option<usize> { at(base, 1) }\n";
    const scope = aggregateScope(aggregate, ["schema", "mutations"], false);
    const result = hoistAggregateSupport({ aggregateFile: join(tmpdir(), "r4-hoist-fixture", "🦀️.rs"), aggregateText: aggregate, aggregateName: "DemoMutation", leafTexts: [leaf], scope })!;
    expect(result.moved).toEqual(["LIMIT", "at", "wrap_len"]);
    expect(result.aggregateText).toContain("pub mod support;");
    expect(result.aggregateText).toMatch(/pub use support::\{LIMIT\};/u);
    expect(result.aggregateText).toMatch(/\nuse support::\{at, wrap_len\};/u);
    expect(result.aggregateText).toContain("fn builds_alpha()");
    expect(result.aggregateText).toContain("fn mentions(");
    expect(result.aggregateText).toContain("fn wrap(");
    expect(result.aggregateText).not.toContain("fn at(");
    expect(result.supportText).toContain("pub fn at(base: &Snap, index: usize)");
    expect(result.supportText).toContain("pub fn wrap_len(");
    expect(result.supportText).toContain("use crate::schema::Snap;");
    expect(result.supportText).not.toContain("builds_alpha");
    expect(result.supportFile.endsWith("/🦀️.rs")).toBe(true);
    if (rustfmt) for (const text of [result.aggregateText, result.supportText]) expect(parses(text)).toBe(true);
  });
});

describe("leaf decoupling and inherent impl hosting", () => {
  const aggregate = ["use crate::schema::{Diff, Snap};", "use protocol::Mutation;", "use semio_base::IndexAdded as Added;", "pub mod alpha;", "pub const KINDS: &[&str] = &[];", "mod hidden;", ""].join("\n");
  const scope = aggregateScope(aggregate, ["schema", "mutations"], true);

  test("replaces the super glob by the absolute imports of the names the file uses", () => {
    const leaf = "use super::*;\n\npub fn diff(base: &Snap) -> Diff { alpha::run(base); Added::default() }\n";
    const result = decoupleFromAggregate(leaf, 1, scope)!;
    expect(result.coupled).toEqual([]);
    expect(result.text).not.toContain("use super::*");
    expect(result.text).toContain("use crate::schema::Snap;");
    expect(result.text).toContain("use crate::schema::Diff;");
    expect(result.text).toContain("use crate::schema::mutations::alpha;");
    expect(result.text).toContain("use semio_base::IndexAdded as Added;");
    expect(result.text).not.toContain("Mutation;");
  });

  test("keeps the glob and names the items a leaf takes from the aggregate itself", () => {
    const leaf = "use super::*;\n\npub fn all() -> usize { KINDS.len() }\n";
    const result = decoupleFromAggregate(leaf, 1, scope)!;
    expect(result.coupled).toEqual(["KINDS"]);
    expect(result.text).toBe(leaf);
  });

  test("ignores files whose glob does not reach the aggregate", () => {
    expect(decoupleFromAggregate("use super::*;\n", 2, scope)).toBeNull();
  });

  test("reads the type of an inherent impl header and ignores trait impls", () => {
    expect(inherentSelf("impl DocBlockPath ")).toBe("DocBlockPath");
    expect(inherentSelf("impl<T: Clone> Wrapper<T> where T: Debug ")).toBe("Wrapper");
    expect(inherentSelf("impl Default for DocBlockPath ")).toBeNull();
  });
});
