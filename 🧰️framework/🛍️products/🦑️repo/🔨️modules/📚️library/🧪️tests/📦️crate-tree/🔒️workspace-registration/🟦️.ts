import { describe, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { addWorkspaceMembers, hidingExcludePattern, insertBuildOwnerRows, syncBuildOwnerRows, patchCargoLock, patchRouterPackages, workspaceAdmissionProblems } from "../../../🏗️authoring/📦️crate-tree/📝️render/🟦️.ts";
import { rewriteCrateAliases, siblingPathCrates } from "../../../🏗️authoring/📦️crate-tree/🧫️sandbox/🟦️.ts";
import { sharedMountFiles } from "../../../🏗️authoring/📦️crate-tree/🔬️verify/🟦️.ts";

describe("artifact crate carve", () => {
  test("registers carved crates in workspace members, router packages and the lockfile", () => {
    expect(addWorkspaceMembers('[workspace]\nmembers = [ "b", "a" ]\n', ["c", "a"])).toBe('[workspace]\nmembers = [\n  "a",\n  "b",\n  "c",\n]\n');
    expect(patchRouterPackages('await runArtifactRustPackageMain(import.meta.dir, "demo",{snapshotSqliteTests:[]});', "demo", ["demo-x"])).toContain('packages: ["demo", "demo-x"]');
    expect(patchRouterPackages('await runRepositoryCargoTests(["demo"], this.repoRoot, this.invocation, rest);', "demo", ["demo-mesh", "demo-tests"])).toContain('runRepositoryCargoTests(["demo", "demo-mesh", "demo-tests"], this.repoRoot');
    const lock = patchCargoLock('version = 4\n\n[[package]]\nname = "demo"\nversion = "0.1.0"\ndependencies = [\n "serde",\n]\n', "demo", [{ name: "demo-x", dependsOn: [] }]);
    expect(lock).toContain('name = "demo-x"');
    expect(lock).toMatch(/name = "demo"[\s\S]*"demo-x"/u);
  });

  test("keeps every version of a multi-version package when it patches the lockfile", () => {
    const text = 'version = 4\n\n[[package]]\nname = "bit-vec"\nversion = "0.6.3"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "aa"\n\n[[package]]\nname = "bit-vec"\nversion = "0.8.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "bb"\n\n[[package]]\nname = "demo"\nversion = "0.1.0"\ndependencies = [\n "bit-vec 0.8.0",\n]\n\n[[package]]\nname = "zeta"\nversion = "1.0.0"\n';
    const lock = patchCargoLock(text, "demo", [{ name: "demo-x", dependsOn: [] }, { name: "demo-y", dependsOn: ["demo-x"] }]);
    expect(lock.match(/name = "bit-vec"/gu)?.length).toBe(2);
    expect(lock).toContain('checksum = "aa"');
    expect(lock).toContain('checksum = "bb"');
    expect([...lock.matchAll(/name = "([^"]+)"/gu)].map((entry) => entry[1])).toEqual(["bit-vec", "bit-vec", "demo", "demo-x", "demo-y", "zeta"]);
    expect(lock).toContain('name = "demo-y"\nversion = "0.1.0"\ndependencies = [\n "bit-vec 0.8.0",\n "demo-x",\n]');
    expect(patchCargoLock(lock, "demo", [{ name: "demo-x", dependsOn: [] }, { name: "demo-y", dependsOn: ["demo-x"] }])).toBe(lock);
  });

  test("admits carved members that sit under a blanket standards exclusion so Cargo and the launcher agree", () => {
    const workspace = [
      "[workspace]",
      'members = [ "📦️packages/🦀️rust" ]',
      "exclude = [",
      '    "**/🏅️standards/**",',
      '    "**/🏭️bridge/**",',
      '    "🏅️standards/🔖️1/🏭️generator/📦️packages/🦀️rust",',
      "]",
      "",
      "[workspace.metadata.semio.repository]",
      "schema-version = 1",
      "exclude-patterns = [",
      '  "**/🏅️standards/**",',
      '  "**/🏭️bridge/**"',
      "]",
      'member-manifests = [ "**/📦️packages/🦀️rust/Cargo.toml" ]',
      "",
    ].join("\n");
    const member = "🏅️standards/🔖️1/🧬️schema/📦️packages/🦀️rust";
    expect(hidingExcludePattern(member, ["**/🏅️standards/**"])).toBe("**/🏅️standards/**");
    expect(hidingExcludePattern(member, ["**/🏭️bridge/**"])).toBeUndefined();
    expect(workspaceAdmissionProblems(workspace)).toEqual([]);
    expect(workspaceAdmissionProblems(workspace.replace('members = [ "📦️packages/🦀️rust" ]', `members = [ "📦️packages/🦀️rust", "${member}" ]`))).toEqual([`member ${member} is hidden from repository discovery by exclude pattern **/🏅️standards/**`]);
    const admitted = addWorkspaceMembers(workspace, [member]);
    expect(workspaceAdmissionProblems(admitted)).toEqual([]);
    const parsed = Bun.TOML.parse(admitted) as { workspace: { members: string[]; exclude: string[]; metadata: { semio: { repository: { "exclude-patterns": string[]; "member-manifests": string[] } } } } };
    expect(parsed.workspace.members).toEqual([member, "📦️packages/🦀️rust"]);
    expect(parsed.workspace.exclude).toEqual(["**/🏭️bridge/**", "🏅️standards/🔖️1/🏭️generator/📦️packages/🦀️rust"]);
    expect(parsed.workspace.metadata.semio.repository["exclude-patterns"]).toEqual(["**/🏭️bridge/**"]);
    expect(parsed.workspace.metadata.semio.repository["member-manifests"]).toEqual(["**/📦️packages/🦀️rust/Cargo.toml"]);
    expect(addWorkspaceMembers(admitted, [member])).toBe(admitted);
  });

  test("reports a source file that two crates mount and none once the second crate depends on the owner", () => {
    const root = mkdtempSync(join(tmpdir(), "shared-mounts-"));
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
      const manifest = (name: string, lib: string, extra = ""): string => `[package]\nname = "${name}"\nversion = "0.1.0"\n\n[lib]\npath = "${lib}"\n${extra}`;
      write("shared/🦀️.rs", "pub fn shared() {}\n");
      write("owner/Cargo.toml", manifest("owner", "../shared/🦀️.rs"));
      write("left/🦀️.rs", '#[path = "../shared/🦀️.rs"]\nmod shared;\n');
      write("left/Cargo.toml", manifest("left", "🦀️.rs"));
      write("right/🦀️.rs", '#[path = "../shared/🦀️.rs"]\nmod shared;\n#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n');
      write("right/🧪️tests/🦀️.rs", "fn helper() {}\n");
      write("right/Cargo.toml", manifest("right", "🦀️.rs"));
      const manifests = ["owner/Cargo.toml", "left/Cargo.toml", "right/Cargo.toml"];
      expect(sharedMountFiles(root, manifests)).toEqual([{ file: "shared/🦀️.rs", manifests: ["left/Cargo.toml", "owner/Cargo.toml", "right/Cargo.toml"] }]);
      expect(sharedMountFiles(root, manifests, (path) => path.startsWith("owner/"))).toHaveLength(1);
      expect(sharedMountFiles(root, ["left/Cargo.toml", "right/Cargo.toml"], (path) => path.startsWith("nothing/"))).toEqual([]);
      write("left/🦀️.rs", "pub use owner as shared;\n");
      write("right/🦀️.rs", '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\npub use owner as shared;\n');
      expect(sharedMountFiles(root, manifests)).toEqual([]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test("finds the plugin-local path crates an artifact crate depends on and rewrites crate aliases for the combined layout", () => {
    const root = mkdtempSync(join(tmpdir(), "sibling-crates-"));
    try {
      const write = (path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
      write("p/a/Cargo.toml", '[workspace]\nmembers = ["artifact"]\n[workspace.dependencies.demo-shared]\npath = "../shared"\n');
      write("p/shared/Cargo.toml", '[package]\nname = "demo-shared"\nversion = "0.1.0"\n');
      write("p/a/artifact/Cargo.toml", '[package]\nname = "demo-x"\nversion = "0.1.0"\n[lib]\npath = "🦀️.rs"\n[dependencies]\nsibling_alias = { path = "../../ext", package = "demo-ext" }\noutside = { path = "../../../q/other" }\ndemo-shared = { workspace = true }\n[dev-dependencies]\ndemo-inside = { path = "../inside" }\n');
      write("p/a/inside/Cargo.toml", '[package]\nname = "demo-inside"\nversion = "0.1.0"\n');
      write("p/ext/Cargo.toml", '[package]\nname = "demo-ext"\nversion = "0.1.0"\n[lib]\npath = "🦀️.rs"\n[dependencies]\ndeep = { path = "../deep" }\n');
      write("p/deep/Cargo.toml", '[package]\nname = "deep"\nversion = "0.1.0"\n');
      write("q/other/Cargo.toml", '[package]\nname = "other"\nversion = "0.1.0"\n');
      const found = siblingPathCrates(root, "p", "p/a", ["p/a/artifact/Cargo.toml"]);
      expect(found.map((entry) => entry.name).sort()).toEqual(["deep", "demo-ext", "demo-shared", "other"]);
      expect(found.find((entry) => entry.name === "other")!.dir).toBe("q/other");
      expect(found.find((entry) => entry.name === "demo-ext")!.idents.slice().sort()).toEqual(["demo_ext", "sibling_alias"]);
    } finally { rmSync(root, { recursive: true, force: true }); }
    const modules = new Map([["demo_ext", "__c_demo_ext"]]);
    expect(rewriteCrateAliases("pub use demo_ext as control;\nuse crate as native;\nuse demo_ext::x;", "__c_self", modules)).toBe("pub use crate::__c_demo_ext as control;\nuse crate::__c_self as native;\nuse demo_ext::x;");
  });

  test("registers carved crates as build-owner rows at their byte-order position and never twice", () => {
    const raw = JSON.stringify({ version: 1, owners: [{ name: "@a/x", root: "x", manifest: "x/p.json", packages: [] }, { name: "demo-a", root: "a/📦️", manifest: "a/📦️/Cargo.toml", packages: [] }, { name: "demo-z", root: "z/📦️", manifest: "z/📦️/Cargo.toml", packages: [] }], catalogProviders: [] }, null, 2) + "\n";
    const first = insertBuildOwnerRows(raw, [{ name: "demo-m", root: "m/📦️" }, { name: "demo-a", root: "a/📦️" }]);
    expect(first.added).toEqual(["demo-m"]);
    expect(JSON.parse(first.text).owners.map((owner: { name: string }) => owner.name)).toEqual(["@a/x", "demo-a", "demo-m", "demo-z"]);
    expect(JSON.parse(first.text).owners[2].manifest).toBe("m/📦️/Cargo.toml");
    expect(insertBuildOwnerRows(first.text, [{ name: "demo-m", root: "m/📦️" }]).added).toEqual([]);
    expect(insertBuildOwnerRows(raw, []).text).toBe(raw);
  });

  test("renames the owner row of a crate that an earlier carve registered under another name and drops the rows of crates that are gone", () => {
    const row = (name: string, root: string) => ({ name, root, manifest: `${root}/Cargo.toml`, packages: [] });
    const raw = JSON.stringify({ version: 1, owners: [row("art-a", "p/a/x"), row("art-old-name", "p/a/b"), row("art-gone", "p/a/gone"), row("art-hand", "p/a/hand"), row("other-gone", "p/q/gone"), row("art-clash", "p/a/clash")] }, null, 2) + "\n";
    const exists = (root: string): boolean => root === "p/a/hand";
    const synced = syncBuildOwnerRows(raw, [{ name: "art-new-name", root: "p/a/b" }, { name: "art-c", root: "p/a/c" }, { name: "art-a", root: "p/a/clash" }], { artifactCrate: "art", artifactDir: "p/a", exists });
    expect(synced.removed).toEqual([{ name: "art-a", root: "p/a/x" }, { name: "art-gone", root: "p/a/gone" }]);
    expect(synced.renamed).toEqual(["art-new-name", "art-a"]);
    expect(synced.added).toEqual(["art-c"]);
    expect(JSON.parse(synced.text).owners.map((owner: { name: string; root: string }) => `${owner.name}@${owner.root}`)).toEqual(["art-c@p/a/c", "art-new-name@p/a/b", "art-hand@p/a/hand", "other-gone@p/q/gone", "art-a@p/a/clash"]);
    expect(syncBuildOwnerRows(raw, [{ name: "art-new-name", root: "p/a/b" }], undefined, false).renamed).toEqual([]);
  });

  test("drops the lock entries of crates an earlier carve produced and no workspace member has any more", () => {
    const lock = 'version = 4\n\n[[package]]\nname = "art"\nversion = "0.1.0"\ndependencies = [\n "art-stale",\n "serde",\n]\n\n[[package]]\nname = "art-stale"\nversion = "0.1.0"\n\n[[package]]\nname = "art-hand"\nversion = "0.1.0"\ndependencies = [\n "art-stale",\n]\n\n[[package]]\nname = "serde"\nversion = "1.0.0"\nsource = "registry+x"\n';
    const patched = patchCargoLock(lock, "art", [{ name: "art-new", dependsOn: [] }], new Set(["art", "art-hand"]));
    expect(patched).not.toContain('name = "art-stale"');
    expect(patched).toContain('name = "art-new"');
    expect(patched).toContain('name = "art-hand"\nversion = "0.1.0"\n\n');
    expect(patched).toContain('dependencies = [\n "art-new",\n "serde",\n]');
    expect(patchCargoLock(lock, "art", [{ name: "art-new", dependsOn: [] }])).toContain('name = "art-stale"');
  });
});
