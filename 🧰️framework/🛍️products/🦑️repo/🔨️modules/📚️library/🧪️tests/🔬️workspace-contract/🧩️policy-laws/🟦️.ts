import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { policyCanonicalArtifactKindBreaches, policyCanonicalChildKindBreaches, policyDissolvedKindRedefinitionBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🧊️dissolved-kernels/🟦️.ts";
import { policyEmojiPrefixBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts";
import { policyModeCompletenessBreaches, policyWindowCompletenessBreaches } from "../../../⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { loadTaxonomy } from "../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

function initializeEmojiPolicyFixture(root: string): void {
  const result = spawnSync("git", ["init", "--quiet"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() });
  if (result.status !== 0) throw new Error(result.stderr || "failed to initialize emoji policy fixture repository");
}

describe("emoji-prefix policy", () => {
  test("requires prefixes on renamable entries and infers exact ecosystem manifests from adjacent manifests", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "plain-dir"), { recursive: true });
      writeFileSync(join(owner, "plain-dir", ".gitkeep"), "");
      writeFileSync(join(owner, "plain.ts"), "");
      writeFileSync(join(owner, "Cargo.toml"), "");
      const rustPackage = join(owner, "📦️packages", "🦀️rust");
      mkdirSync(rustPackage, { recursive: true });
      writeFileSync(join(rustPackage, "Cargo.toml"), "");
      initializeEmojiPolicyFixture(root);
      const scopes = policyEmojiPrefixBreaches(root).map((breach) => breach.scope);
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain-dir");
      expect(scopes).toContain("✏️s/🔌️plugins/🧪️probe/plain.ts");
      expect(scopes).not.toContain("✏️s/🔌️plugins/🧪️probe/Cargo.toml");
      expect(scopes).not.toContain("✏️s/🔌️plugins/🧪️probe/📦️packages/🦀️rust/Cargo.toml");
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate sibling emoji identities after VS16 normalization", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(owner, { recursive: true });
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      mkdirSync(join(owner, "🧭️second"), { recursive: true });
      writeFileSync(join(owner, "🧭️first", ".gitkeep"), "");
      writeFileSync(join(owner, "🧭️second", ".gitkeep"), "");
      initializeEmojiPolicyFixture(root);
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind.endsWith("/emoji-not-unique"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("rejects duplicate identities shared by sibling files and directories", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "📄️folder"), { recursive: true });
      writeFileSync(join(owner, "📄️folder", ".gitkeep"), "");
      writeFileSync(join(owner, "📄️file.ts"), "");
      initializeEmojiPolicyFixture(root);
      expect(policyEmojiPrefixBreaches(root).some((breach) => breach.kind.endsWith("/emoji-not-unique"))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("accepts distinct VS16 emoji identities", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-emoji-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe");
    try {
      mkdirSync(join(owner, "🧭️first"), { recursive: true });
      writeFileSync(join(owner, "🧭️first", ".gitkeep"), "");
      writeFileSync(join(owner, "🧾️second.ts"), "");
      initializeEmojiPolicyFixture(root);
      expect(policyEmojiPrefixBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("window completeness policy", () => {
  test("requires item-only capability facets and their language mirrors", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-window-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Windows now live under a subset's surface (👁️viewer/✏️editor), not 🎛️apps (W3 dissolution,
    // ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) — mirrors the on-disk shape every other
    // fixture in this file already uses for 🏅️standards/🪆️subsets.
    const window = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode", "🪟️windows", "🧪️window");
    const crate = { dir: "", libRelPath: "", role: "", shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(window, { recursive: true });
      // 🔣️ The required set is taxonomy-driven, never a literal here — adding a lane to
      // windowRequiredChildDirs must not require editing this assertion.
      const required = loadTaxonomy().windowRequiredChildDirs;
      const missingFacets = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingFacets.filter((breach) => breach.kind === "taxonomy/window-completeness")).toHaveLength(required.length);
      expect(required).toContain("🎚️config");
      expect(required).toContain("🫧️transient");
      for (const facet of required) {
        const dir = join(window, facet);
        mkdirSync(dir, { recursive: true });
        writeFileSync(join(dir, "📝️.md"), "");
      }
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      const action = join(window, "🎬️actions", "🧪️action");
      mkdirSync(action);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-component")).toHaveLength(2);
      expect(policyWindowCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/window-empty-facet")).toHaveLength(1);
      rmSync(join(window, "🎬️actions", "📝️.md"));
      writeFileSync(join(action, "🦀️.rs"), "");
      writeFileSync(join(action, "🟦️.ts"), "");
      expect(policyWindowCompletenessBreaches(root, [crate])).toEqual([]);
      writeFileSync(join(window, "🎬️actions", "🦀️.rs"), "");
      expect(policyWindowCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/window-facet-component"]);
      rmSync(join(window, "🎬️actions", "🦀️.rs"));
      rmSync(join(action, "🟦️.ts"));
      const missingMirror = policyWindowCompletenessBreaches(root, [crate]);
      expect(missingMirror.map((breach) => breach.kind)).toEqual(["taxonomy/window-component"]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("requires every mode to declare its windows collection and its three state lanes", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-mode-policy-"));
    const ownerRel = "🧪️owner";
    // 👁️✏️ Modes now live under a subset's surface, not 🎛️apps (W3 dissolution) — see the sibling
    // window-completeness test above for the same shape rationale.
    const mode = join(root, ownerRel, "🗿️artifacts", "🧪️artifact", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "👁️viewer", "🎭️modes", "🧪️mode");
    const crate = { dir: "", libRelPath: "", role: "", shape: "taxonomy", ownerRel, pluginId: "fixture" } as const;
    try {
      mkdirSync(mode, { recursive: true });
      const required = loadTaxonomy().modeRequiredChildDirs;
      expect(required).toEqual(["🪟️windows", "🎮️commands", "🎚️config", "👥️presence", "🫧️transient"]);
      expect(policyModeCompletenessBreaches(root, [crate]).filter((breach) => breach.kind === "taxonomy/mode-completeness")).toHaveLength(required.length);
      for (const child of required) {
        mkdirSync(join(mode, child), { recursive: true });
      }
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(required.map(() => "taxonomy/mode-empty-child"));
      for (const child of required) {
        writeFileSync(join(mode, child, "📝️.md"), "");
      }
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
      mkdirSync(join(mode, "🪟️windows", "🧪️window"), { recursive: true });
      expect(policyModeCompletenessBreaches(root, [crate]).map((breach) => breach.kind)).toEqual(["taxonomy/mode-empty-child"]);
      rmSync(join(mode, "🪟️windows", "📝️.md"));
      expect(policyModeCompletenessBreaches(root, [crate])).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});

describe("composition policy (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W6)", () => {
  test("canonical-artifact-kind rejects legacy ArtifactKindSpec.id grammar but accepts s.<plugin>.<artifact>", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-kind-policy-"));
    const artifact = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    try {
      mkdirSync(artifact, { recursive: true });
      writeFileSync(
        join(artifact, "🦀️.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "3d.mesh".into(),', "    }", "}"].join("\n"),
      );
      const legacy = policyCanonicalArtifactKindBreaches(root);
      expect(legacy.map((breach) => breach.kind)).toEqual(["composition/canonical-artifact-kind"]);
      expect(legacy[0]?.priority).toBe("medium");

      writeFileSync(
        join(artifact, "🦀️.rs"),
        ['pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {', '    semio_framework_plugin::ArtifactKindSpec {', '        id: "s.probe.probe".into(),', "    }", "}"].join("\n"),
      );
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);

      // 🩹 Doc-comment prose quoting the pattern must not trip the rule (the exact false-positive
      // class flagged in the UCAS ticket's own 📌️important.md sweep-the-pattern lesson).
      writeFileSync(join(artifact, "🦀️.rs"), '/// see `ArtifactKindSpec { id: "3d.mesh" }` for the old shape\n');
      expect(policyCanonicalArtifactKindBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("child-kind metadata validates syntax without inferring instance ownership from kinds", () => {
    const fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🧩️child-kind-metadata/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(contractRoot, "../../🧬️schema/🧩️child-kind-metadata/🔣️.json"), "utf8"));
    const oracle = new Ajv({ strict: true });
    
    const validateKind = oracle.compile(schema.$defs.artifactKind);
    const render: Record<string, (kind: string) => string> = {
      rs: kind => '#[child(kind = "' + kind + '")] pub content: store::os_store::artifact_child::ArtifactChild<Child>,\n',
      ts: kind => '/** @state artifact @child kind=' + kind + ' */ content: ArtifactChild;\n',
      json: kind => JSON.stringify({ properties: { content: { "x-semio-child": kind } } }),
      graphql: kind => 'content: ArtifactChild! @child(kind: "' + kind + '")\n',
      proto: kind => '// @state artifact @child kind=' + kind + '\nArtifactChild content = 1;\n',
    };
    for (const row of fixture.cases) {
      const root = mkdtempSync(join(tmpdir(), "semio-child-kind-policy-"));
      try {
        for (const declaration of row.declarations) {
          const owner = join(root, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio", "🏅️standards", "🔖️1", "🪆️subsets", "🧪️" + declaration.owner, "🧬️schema");
          mkdirSync(owner, { recursive: true });
          writeFileSync(join(owner, "🧪️." + declaration.format), render[declaration.format]!(declaration.kind));
        }
        const expected = row.declarations.filter((declaration: { kind: string }) => !validateKind(declaration.kind)).map((declaration: { kind: string }) => declaration.kind).sort();
        expect(expected).toEqual([...row.rejectedKinds].sort());
        const actual = policyCanonicalChildKindBreaches(root);
        expect(actual.map(breach => breach.summary.match(/declares child kind "([^"]+)"/)?.[1]).sort()).toEqual(expected);
        expect(actual.every(breach => breach.kind === "composition/canonical-child-kind" && breach.priority === "high")).toBe(true);
      } finally {
        rmSync(root, { recursive: true, force: true });
      }
    }
  });

  test("child-kind metadata ignores Rust comments and strings but reads inline attributes", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-child-kind-source-policy-"));
    const owner = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe", "🧬️schema");
    try {
      mkdirSync(owner, { recursive: true });
      writeFileSync(join(owner, "🦀️.rs"), [
        '/* nested /* comment */',
        '#[child(kind = "s.ignored.comment.extra")]',
        '*/',
        'const EXAMPLE: &str = r##"',
        '#[child(kind = "s.ignored.string.extra")]',
        '"##;',
        '// #[child(kind = "s.ignored.line.extra")]',
        'pub struct Parent { #[child(kind = "s.probe.child.extra")] pub child: Child }',
      ].join("\n"));
      const breaches = policyCanonicalChildKindBreaches(root);
      expect(breaches).toHaveLength(1);
      expect(breaches[0]?.line).toBe(8);
      expect(breaches[0]?.summary).toContain('declares child kind "s.probe.child.extra"');
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("dissolved-kind-redefinition bans redeclaring a frozen 🧿️semio subset type outside 🗄️stdio", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-composition-dissolved-policy-"));
    const plugin = join(root, "✏️s", "🔌️plugins", "🧪️probe", "🗿️artifacts", "🧪️probe");
    const stdio = join(root, "✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "🧿️semio");
    try {
      mkdirSync(plugin, { recursive: true });
      mkdirSync(stdio, { recursive: true });
      writeFileSync(join(plugin, "🦀️.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      writeFileSync(join(stdio, "🦀️.rs"), "pub struct SemioMeshSnapshot { pub vertices: Vec<f64> }\n");
      const breaches = policyDissolvedKindRedefinitionBreaches(root);
      expect(breaches).toHaveLength(1);
      expect(breaches[0]?.kind).toBe("composition/dissolved-kind-redefinition");
      expect(breaches[0]?.scope).toBe("✏️s/🔌️plugins/🧪️probe/🗿️artifacts/🧪️probe/🦀️.rs");

      rmSync(join(plugin, "🦀️.rs"));
      writeFileSync(join(plugin, "🦀️.rs"), "pub struct ProbeOwnType { pub vertices: Vec<f64> }\n");
      expect(policyDissolvedKindRedefinitionBreaches(root)).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
