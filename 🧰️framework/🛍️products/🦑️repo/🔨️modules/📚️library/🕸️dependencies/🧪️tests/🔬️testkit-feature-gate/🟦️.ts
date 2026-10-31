import { describe, expect, test } from "bun:test";
import { spawnSync } from "bun";
import { existsSync, readFileSync } from "node:fs";
import { sep } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { cargoMetadataTestkitVerdict, findTestkitLeaks, scannerRows, testkitLeaksOfRepository, type TestkitLeak } from "../../🚧️testkit-leaks/🟦️.ts";

describe("testkit feature gate", () => {
  test("a testkit feature in a normal dependency table is a leak", () => {
    const source = '[package]\nname = "product"\n\n[dependencies]\nfoo = { workspace = true, features = ["testkit"] }\n\n[dev-dependencies]\nbar = { workspace = true, features = ["testkit"] }\n';
    expect(findTestkitLeaks("product/Cargo.toml", source).map((leak) => leak.line)).toEqual([5]);
  });

  test("dev-dependencies, test crates, testkit crates and an opt-in testkit feature are not leaks", () => {
    const dev = '[package]\nname = "product"\n[features]\ntestkit = ["foo/testkit"]\n[dev-dependencies]\nfoo = { workspace = true, features = ["testkit"] }\n';
    expect(findTestkitLeaks("product/Cargo.toml", dev)).toEqual([]);
    expect(findTestkitLeaks(`unit${sep}🧪️tests${sep}laws${sep}Cargo.toml`, '[dependencies]\nfoo = { features = ["testkit"] }\n')).toEqual([]);
    expect(findTestkitLeaks("kit/Cargo.toml", '[package]\nname = "my-testkit"\n[dependencies]\nfoo = { features = ["testkit"] }\n')).toEqual([]);
  });

  test("a testkit crate dependency, a target table and a default feature are leaks", () => {
    const source = '[package]\nname = "product"\n[dependencies]\nmy-testkit = { workspace = true }\n[target.\'cfg(unix)\'.dependencies]\nfoo = { features = ["testkit"] }\n[features]\ndefault = ["testkit"]\n';
    expect(findTestkitLeaks("product/Cargo.toml", source).map((leak) => leak.line)).toEqual([4, 6, 8]);
  });

  test("an optional testkit dependency is a leak when default or any production feature, transitively, activates it", () => {
    const head = '[package]\nname = "product"\n[dependencies]\nfoo-testkit = { workspace = true, optional = true }\nfoo-testing-caller = { workspace = true, optional = true }\n[features]\n';
    const rows = (features: string) => findTestkitLeaks("product/Cargo.toml", head + features).map((leak) => leak.subject).sort();
    expect(rows('default = ["dep:foo-testkit"]\n')).toEqual(["foo-testkit"]);
    expect(rows('default = ["fast"]\nfast = ["helper"]\nhelper = ["foo-testkit/extra"]\n')).toEqual(["foo-testkit"]);
    expect(rows('extra = [\n  "foo-testing-caller",\n]\n')).toEqual(["foo-testing-caller"]);
    expect(rows('default = ["integration"]\nintegration = ["testkit"]\ntestkit = ["dep:foo-testkit"]\n')).toEqual(["default"]);
  });

  test("an optional testkit dependency reached solely through the testkit feature, a weak feature or a test crate is allowed", () => {
    const head = '[package]\nname = "product"\n[dependencies]\nfoo-testkit = { workspace = true, optional = true }\n[features]\n';
    const rows = (features: string) => findTestkitLeaks("product/Cargo.toml", head + features).map((leak) => leak.subject);
    expect(rows('testkit = ["dep:foo-testkit"]\n')).toEqual([]);
    expect(rows('testkit = ["foo-testkit/extra", "other"]\nother = []\n')).toEqual([]);
    expect(rows('weak = ["foo-testkit?/extra"]\n')).toEqual([]);
    expect(findTestkitLeaks(`unit${sep}🧪️tests${sep}Cargo.toml`, head + 'default = ["dep:foo-testkit"]\n')).toEqual([]);
  });

  test("no production manifest of the repository enables a testkit outside dev-dependencies", () => {
    const root = getWorkspaceRoot();
    const listing = spawnSync(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*Cargo.toml"], { cwd: root, stdout: "pipe" });
    const leaks: TestkitLeak[] = [];
    for (const path of new TextDecoder().decode(listing.stdout).split("\0").filter((entry) => entry.endsWith("Cargo.toml"))) {
      if (!path.startsWith(".") && existsSync(`${root}/${path}`)) leaks.push(...findTestkitLeaks(path, readFileSync(`${root}/${path}`, "utf8")));
    }
    expect(leaks.map((leak) => `${leak.manifest}:${leak.line} [${leak.section}] ${leak.text}`)).toEqual([]);
  }, 120_000);

  test("cargo metadata, the independent implementation, agrees with the scanner on every workspace of the repository", () => {
    const root = getWorkspaceRoot();
    const { leaks, manifests } = testkitLeaksOfRepository(root);
    const roots = manifests.filter((manifest) => /^\[workspace\]/m.test(readFileSync(`${root}/${manifest}`, "utf8")));
    expect(roots.length).toBeGreaterThan(0);
    const covered = new Set<string>();
    const cargoRows: string[] = [];
    for (const workspace of roots) {
      const run = spawnSync(["repo", "cargo", "--store=shared", "--", "metadata", "--format-version", "1", "--no-deps", "--manifest-path", workspace], { cwd: root, stdout: "pipe", stderr: "pipe", env: { ...process.env, SEMIO_LANGUAGE: "en" } });
      const json = new TextDecoder().decode(run.stdout).split("\n").find((line) => line.startsWith("{\"packages\""));
      expect(json, `cargo metadata of ${workspace}: ${new TextDecoder().decode(run.stderr).slice(-400)}`).toBeDefined();
      const metadata = JSON.parse(json!);
      for (const pkg of metadata.packages) covered.add(pkg.manifest_path.slice(root.length + 1));
      cargoRows.push(...cargoMetadataTestkitVerdict(metadata, root));
    }
    expect([...covered].every((manifest) => manifests.includes(manifest) || manifest.startsWith("."))).toBe(true);
    expect([...new Set(cargoRows)].sort()).toEqual([...new Set(scannerRows(leaks, covered))]);
    expect(leaks).toEqual([]);
  }, 3_600_000);
});
