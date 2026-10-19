import { test, expect } from "bun:test";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { resolve, join, relative, dirname } from "node:path";
import * as toml from "@iarna/toml";
import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { proveSchemaValidatorOwnershipV1 } from "../📏️ownership/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const framework = "🧰️framework", workspaceRoot = join(root, framework);
const packagePath = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust";

test("actual structural validator compiles and runs with every product physically absent", async () => {
  proveSchemaValidatorOwnershipV1(root);
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name the ticket artifact directory");
  mkdirSync(output, { recursive: true });
  const copy = mkdtempSync(join(output, "neutral-schema-validator-"));
  const source = toml.parse(readFileSync(join(workspaceRoot, "Cargo.toml"), "utf8")) as any;
  const members = new Set<string>();
  const dependencies: Record<string, any> = {};
  const visit = (path: string): void => {
    const member = relative(workspaceRoot, path).replaceAll("\\", "/");
    if (members.has(member)) return;
    expect(member.startsWith("🔨️modules/")).toBe(true);
    members.add(member);
    const manifest = toml.parse(readFileSync(join(path, "Cargo.toml"), "utf8")) as any;
    const sections = [manifest, ...Object.values(manifest.target ?? {})] as any[];
    for (const section of sections) for (const kind of ["dependencies", "dev-dependencies", "build-dependencies"]) {
      for (const [name, declaration] of Object.entries(section[kind] ?? {}) as [string, any][]) {
        const inherited = declaration.workspace ? source.workspace.dependencies[name] : undefined;
        if (inherited) dependencies[name] = inherited;
        const dependency = inherited ?? declaration;
        if (dependency.path) visit(resolve(inherited ? workspaceRoot : path, dependency.path));
      }
    }
  };
  visit(join(root, packagePath));
  const owners = [...members].map(member => framework + "/" + member.slice(0, member.indexOf("/📦️packages/")));
  const visitedSources = new Set<string>();
  const visitSource = (path: string): void => {
    if (visitedSources.has(path) || !existsSync(path)) return;
    visitedSources.add(path);
    const owned = relative(root, path).replaceAll("\\", "/");
    if (owned.includes("/🧪️tests/") && !owned.startsWith("🧰️framework/🔨️modules/🧬️schema/✅️validator/")) return;
    if (!owned.startsWith("🧰️framework/🔨️modules/")) throw Error("unexpected source ownership: " + owned);
    if (!owners.some(owner => owned.startsWith(owner + "/"))) owners.push(dirname(owned).replaceAll("\\", "/"));
    if (!path.endsWith(".rs")) return;
    const body = readFileSync(path, "utf8");
    for (const match of body.matchAll(/(?:#\[\s*path\s*=\s*|include(?:_str|_bytes)?!\s*\(\s*)"([^"\n]+)"/gu)) visitSource(resolve(dirname(path), match[1]!));
  };
  for (const member of members) {
    const path = join(workspaceRoot, member);
    const manifest = toml.parse(readFileSync(join(path, "Cargo.toml"), "utf8")) as any;
    visitSource(resolve(path, manifest.lib?.path ?? "src/lib.rs"));
  }
  for (const owner of owners.filter(owner => !owners.some(other => owner !== other && owner.startsWith(other + "/")))) {
    mkdirSync(dirname(join(copy, owner)), { recursive: true });
    cpSync(join(root, owner), join(copy, owner), { recursive: true, filter: path => !/(?:^|[\\/])(?:dist|target|node_modules|🗑️generated)(?:[\\/]|$)/u.test(path) });
  }
  const manifest = join(copy, framework, "Cargo.toml");
  mkdirSync(join(copy, framework), { recursive: true });
  writeFileSync(manifest, toml.stringify({ workspace: { resolver: "2", members: [...members], package: source.workspace.package, lints: source.workspace.lints, dependencies } }));
  cpSync(join(workspaceRoot, "Cargo.lock"), join(copy, framework, "Cargo.lock"));
  expect(existsSync(join(copy, "🧰️framework/🛍️products"))).toBe(false);
  expect(existsSync(join(copy, "✏️s"))).toBe(false);
  expect(existsSync(join(copy, "🧰️framework/🔨️modules/🧬️schema/⚛️component"))).toBe(false);
  const cargo = { ...process.env, CARGO_TARGET_DIR: join(copy, "target"), CARGO_BUILD_BUILD_DIR: join(copy, "compiler") };
  await runOwnedCommand("cargo", ["test", "--offline", "--manifest-path", manifest, "-p", "semio-framework-schema-validator", "--lib", "--", "--nocapture"], copy, "schema-validator:products-absent", 120000, { env: cargo });
}, 120000);
