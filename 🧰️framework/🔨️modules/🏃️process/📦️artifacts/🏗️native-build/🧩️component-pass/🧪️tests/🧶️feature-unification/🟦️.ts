import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { registeredToolExecutableV1 } from "../../../../../🧰️tools/🏃️run/🟦️.ts";
import { nativeBuildUnitGraphArgsV1, NATIVE_BUILD_DRIVER_POLICY_V1, type CargoUnitGraphV1 } from "../../🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./🧬️.json";

test("selected component libraries share one Cargo type identity", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  const root = mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!), "features-"));
  const workspace = resolve(import.meta.dirname, "../../../../../../../..");
  const repo = registeredToolExecutableV1("repo", workspace);
  const cargo = registeredToolExecutableV1("cargo", workspace);
  const environment = {
    ...process.env,
    SEMIO_HOST_CAPACITY_DIRECTORY: join(root, "capacity"),
    CARGO_UNSTABLE_FEATURE_UNIFICATION: "true",
    CARGO_RESOLVER_FEATURE_UNIFICATION: NATIVE_BUILD_DRIVER_POLICY_V1.featureUnification,
  };
  const manifest = join(root, "Cargo.toml");
  const names = fixture.packages.map(row => row.name);
  const packages = ["base", ...names];
  function run(args: string[], unification = NATIVE_BUILD_DRIVER_POLICY_V1.featureUnification) {
    const result = spawnSync(repo, ["cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--", ...args], {
      cwd: workspace, env: { ...environment, CARGO_RESOLVER_FEATURE_UNIFICATION: unification }, encoding: "utf8", timeout: 120_000,
    });
    expect(result.error).toBeUndefined();
    expect(result.status, result.stderr).toBe(0);
    return result.stdout;
  }
  try {
    writeFileSync(manifest, `cargo-features = ["trim-paths"]\n\n[workspace]\nresolver = "2"\nmembers = ${JSON.stringify(packages)}\n\n[profile.prototype]\ninherits = "dev"\n`);
    for (const name of packages) {
      mkdirSync(join(root, name, "src"), { recursive: true });
      const row = fixture.packages.find(row => row.name === name);
      const settings = row
        ? `[lib]\ncrate-type = ["cdylib"]\n\n[dependencies]\nbase = { path = "../base", features = ["${row.feature}"] }\n`
        : "[features]\n" + fixture.sharedFeatures.map(feature => `${feature} = []\n`).join("");
      writeFileSync(join(root, name, "Cargo.toml"), `[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n\n${settings}`);
      writeFileSync(join(root, name, "src", "lib.rs"), row ? "pub fn shared(_: base::Value) {}\n" : "pub struct Value;\n");
    }
    const lock = spawnSync(cargo, ["generate-lockfile", "--offline", "--manifest-path", manifest], {
      cwd: root, env: environment, encoding: "utf8", timeout: 15_000,
    });
    expect(lock.status, lock.stderr).toBe(0);
    const pass = {
      id: "feature-unification", manifest, profile: fixture.profile, target: fixture.target,
      packages: names.map(name => ({ name, manifest: join(root, name, "Cargo.toml") })),
    };
    function plan(unification?: string) {
      const output = run([...nativeBuildUnitGraphArgsV1(pass), "--offline"], unification);
      const graphLine = output.split("\n").find(line => line.startsWith("{\"version\":1,\"units\":"));
      expect(graphLine).toBeDefined();
      return JSON.parse(graphLine!) as CargoUnitGraphV1;
    }
    expect(plan("package").units.filter(unit => unit.target.name === "base")).toHaveLength(fixture.packages.length);
    const graph = plan();
    const base = graph.units.flatMap((unit, index) => unit.target.name === "base" ? [index] : []);
    expect(base).toHaveLength(1);
    expect(graph.units[base[0]!]!.features).toEqual(fixture.sharedFeatures);
    expect(graph.roots).toHaveLength(names.length);
    for (const root of graph.roots) expect(graph.units[root]!.dependencies.map(edge => edge.index)).toEqual(base);
  } finally {
    rmSync(root, { force: true, recursive: true });
  }
}, 250_000);
