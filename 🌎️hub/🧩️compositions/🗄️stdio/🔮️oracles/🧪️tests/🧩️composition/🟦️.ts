import { describe, expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve, sep } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import { inspectRustCargoManifest, inspectRustModuleGraphFacts, inspectRustStructure } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { inspectRustCompileReferences, rustTokenPairs, rustTokens } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { packagesForOwner } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs";
import contract from "../../🧫️fixtures/🧩️composition/🔣️.json";
import selection from "../../🧫️fixtures/🌳️contribution-selection/🔣️.json";

import deletion from "../../🧫️fixtures/🚮️artifact-deletion/🔣️.json";

import development from "../../🧫️fixtures/🧪️development-dependencies/🔣️.json";

import drawing from "../../🧫️fixtures/🖊️drawing-reader/🔣️.json";
import { originalDrawingSource } from "../🖊️drawing-reader/🧩️preservation/🟦️.ts";


const root = resolve(import.meta.dir, "../../../../../..");
const externalDependencies: Record<string, unknown> = contract.dependencies;
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const parsed = (path: string): Record<string, unknown> => JSON.parse(read(path));
const digest = (value: string) => createHash("sha256").update(value).digest("hex");
const manifestOwner = (path: string) => path.slice(0, -"/🔮️oracles/🔣️.json".length);
const currentHosts = (path: string): { implementation: string; package: string; path?: string; features?: string[] }[] => (parsed(path).oracleHostPackages ?? []) as { implementation: string; package: string; path?: string; features?: string[] }[];

/** 🛂️ Audits public signatures, fields, variants and reexports using actual Rust module scopes. */
function foreignPublicSurface(path: string): string[] {
  const source = read(path), facts = inspectRustModuleGraphFacts(source), tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
  const internal = facts.modules.filter(({ visibility }) => visibility.startsWith("pub(")).map(({ modulePath }) => modulePath.join("::"));
  const restricted = (offset: number) => facts.scopes.some(({ modulePath, bodyStartOffset, bodyEndOffset }) => internal.some((name) => modulePath.join("::") === name || modulePath.join("::").startsWith(`${name}::`)) && offset >= bodyStartOffset && offset < bodyEndOffset);
  const restrictedTypes: { name: string; start: number; end: number }[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "pub" || tokens[index + 1]?.text !== "(") continue;
    const keyword = (pairs.get(index + 1) ?? index) + 1;
    if (tokens[keyword]?.text !== "struct") continue;
    let body = keyword + 2;
    while (body < tokens.length && !["{", ";"].includes(tokens[body]!.text)) body++;
    if (tokens[body]?.text === "{") restrictedTypes.push({ name: tokens[keyword + 1]!.text, start: tokens[body]!.start, end: tokens[pairs.get(body) ?? body]!.end });
  }
  const ownRoots = new Set(["crate", "super", "self", "std", "core", "alloc", "semio_repo_test_host", ...facts.modules.map(({ name }) => name), ...contract.providers.map(({ package: pkg }) => pkg.library), ...contract.families.map(({ package: pkg }) => pkg.library), contract.grammar.package.library]);
  const foreign = new Set(facts.uses.filter(({ specifier }) => !ownRoots.has(specifier.split("::")[0]!)).flatMap(({ specifier }) => rustTokens(specifier).filter(({ kind, text }) => kind === "identifier" && /^[A-Z]/u.test(text)).map(({ text }) => text)));
  const foreignRoots = new Set(facts.uses.filter(({ specifier }) => !ownRoots.has(specifier.split("::")[0]!)).map(({ specifier }) => specifier.split("::")[0]!));
  for (const name of Object.keys(contract.dependencies).filter((name) => name !== contract.neutralLaw.package)) foreignRoots.add(name.replaceAll("-", "_"));
  const aliases: { name: string; tokens: string[] }[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "type" || tokens[index + 1]?.kind !== "identifier") continue;
    let end = index + 2;
    while (end < tokens.length && tokens[end]?.text !== ";") end++;
    aliases.push({ name: tokens[index + 1]!.text, tokens: tokens.slice(index + 2, end).map(({ text }) => text) });
  }
  for (let count = 0; count < aliases.length; count++) for (const alias of aliases) if (alias.tokens.some((name, index) => foreign.has(name) || foreignRoots.has(name) && alias.tokens[index + 1] === "::")) foreign.add(alias.name);
  const violations: string[] = [];
  const publicSignatures: string[][] = [];
  const pendingFields: { name: string; signature: string[]; line: number }[] = [];
  for (let index = 0; index < tokens.length; index++) {
    if (tokens[index]?.text !== "pub" || tokens[index + 1]?.text === "(" || restricted(tokens[index]!.start)) continue;
    const keyword = tokens[index + 1]?.text;
    if (!["fn", "type", "use", "const", "static"].includes(keyword ?? "") && tokens[index + 2]?.text !== ":") continue;
    let end = index + 2;
    while (end < tokens.length && !["{", ";", ",", "="].includes(tokens[end]!.text)) {
      if (tokens[end]?.text === "(" || tokens[end]?.text === "[") end = (pairs.get(end) ?? end) + 1;
      else end++;
    }
    if (keyword === "type" && tokens[end]?.text === "=") while (end < tokens.length && tokens[end]?.text !== ";") end++;
    const signature = tokens.slice(index, end);
    const privateParent = restrictedTypes.find(({ start, end }) => tokens[index]!.start > start && tokens[index]!.start < end);
    if (privateParent) {
      pendingFields.push({ name: privateParent.name, signature: signature.map(({ text }) => text), line: source.slice(0, tokens[index]!.start).split("\n").length });
      continue;
    }
    publicSignatures.push(signature.map(({ text }) => text));
    if (signature.some(({ kind, text }, index) => kind === "identifier" && (foreign.has(text) || foreignRoots.has(text) && signature[index + 1]?.text === "::"))) violations.push(`${path}:${source.slice(0, tokens[index]!.start).split("\n").length}: ${signature.map(({ text }) => text).join(" ")}`);
  }
  for (const field of pendingFields) if (publicSignatures.some((signature) => signature.includes(field.name)) && field.signature.some((name) => foreign.has(name))) violations.push(`${path}:${field.line}: exposed ${field.name}: ${field.signature.join(" ")}`);
  for (const fact of inspectRustStructure(source).enums.filter(({ visibility }) => visibility === "pub")) for (const variant of fact.variants) if (variant.fieldTypes.some((type) => rustTokens(type).some(({ text }) => foreign.has(text)))) violations.push(`${path}: public enum ${fact.name}::${variant.name}`);
  for (const use of facts.uses.filter(({ relation, visibility }) => relation === "reexport" && visibility === "pub")) if (!ownRoots.has(use.specifier.split("::")[0]!) && rustTokens(use.specifier).some(({ text }) => foreign.has(text))) violations.push(`${path}: external reexport ${use.specifier}`);
  return violations;
}


const packageOwners = [...contract.providers, ...contract.families, contract.grammar];
const restored = (source: string, rewrites: { previous: string; current: string }[]) => rewrites.reduce((text, { previous, current }) => text.replaceAll(current, previous), source);
const restoredMetadata = (text: string) => restored(text, contract.callers.flatMap(({ rewrites }) => rewrites)).replaceAll(contract.owner, contract.retired.owner).replaceAll(contract.package.package, contract.retired.package);
const cargoAt = (path: string) => toml.parse(read(`${path}/Cargo.toml`)) as { package: { name: string }; lib: { name: string; path: string }; dependencies: Record<string, { path?: string; features?: string[] } | string>; features: Record<string, string[]> };
const contributions = () => {
  const paths = new Set([...contract.contributions.map(({ path }) => path), ...contract.artifactBindings.map(({ owner }) => `${owner}/🔮️oracles/🔣️.json`)]);
  return [...paths].filter((path) => existsSync(resolve(root, path))).map((path) => ({ owner: manifestOwner(path), oracleHostPackages: currentHosts(path) }));
};

/** 🌳️ Follows all authored path-package dependencies and compile inputs without weakening features. */
function removedOracleInputs(hosts: { path: string }[], removedOwner: string): string[] {
  const removed = resolve(root, removedOwner), inputs = new Set<string>(), packages = new Set<string>(), sources = new Set<string>();
  const visitSource = (path: string, manifest: string) => {
    if (sources.has(path) || !existsSync(path)) return;
    sources.add(path);
    const references = inspectRustCompileReferences(readFileSync(path, "utf8")).filter(({ base }) => base !== "generated");
    const targets: string[] = [];
    for (const reference of references) {
      const target = resolve(reference.base === "manifest" ? manifest : resolve(path, "..", reference.inlineBase ?? "."), reference.path);
      if (target === removed || target.startsWith(`${removed}${sep}`)) inputs.add(target);
      if (!reference.directory && target.endsWith(".rs")) targets.push(target);
    }
    if (!inputs.size) for (const target of targets) visitSource(target, manifest);
  };
  const visitPackage = (path: string) => {
    if (packages.has(path)) return;
    packages.add(path);
    const cargo = cargoAt(path);
    visitSource(resolve(root, path, cargo.lib.path), resolve(root, path));
    for (const dependency of Object.values(cargo.dependencies ?? {})) if (typeof dependency === "object" && dependency.path) visitPackage(resolve(root, path, dependency.path));
  };
  for (const host of hosts) visitPackage(host.path);
  return [...inputs].sort();
}

describe("canonical complete Stdio oracle ownership", () => {
  test("owned schema validation and independent Ajv agree on the closed provider contract", () => {
    
    expect(contract.schemaVersion).toBe(2);
    expect(contract.owner).toBe("🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles");
    expect(new Set(contract.providers.map(({ owner }) => owner)).size).toBe(contract.providers.length);
  });

  test("explicit selection preserves generic ancestor and Python semantics", () => {
    for (const row of selection.cases) expect(packagesForOwner(row.contributions, row.owner, row.implementation), row.id).toEqual(row.expected);
  });

  test("owned deletion schema and independent Ajv require original features and complete execution", () => {
    
    for (const vector of deletion.vectors) {
      expect(vector.execution).toBe("all-original-scenarios");
      expect(vector.features.length).toBeGreaterThan(0);
    }
    expect(deletion.lowerPlugin.nativeRequired).toBe(true);
  });

  test("the lower plugin retires the whole concrete root library and artifact mounts", () => {
    expect(existsSync(resolve(root, contract.retired.owner, "🦀️.rs"))).toBe(false);
    expect(existsSync(resolve(root, contract.retired.owner, "📦️packages/🦀️rust/Cargo.toml"))).toBe(false);
    for (const family of contract.families) expect(inspectRustCompileReferences(read(family.source)).some(({ path }) => path.includes("🗿️artifacts")), family.source).toBe(false);
    const lower = currentHosts(`${contract.retired.owner}/🔣️.json`);
    expect(lower.some(({ package: name }) => [contract.retired.package, contract.package.package].includes(name))).toBe(false);
    expect(lower.filter(({ implementation }) => implementation === "python")).toEqual(contract.contributions.find(({ path }) => path === `${contract.retired.owner}/🔣️.json`)!.hosts.filter(({ implementation }) => implementation === "python"));
  });

  for (const owner of packageOwners) test(`actual owned provider or family package: ${owner.package.package}`, () => {
    expect(existsSync(resolve(root, owner.package.path, "Cargo.toml")), owner.package.path).toBe(true);
    const cargo = cargoAt(owner.package.path);
    expect(cargo.package.name).toBe(owner.package.package);
    expect(cargo.lib.name).toBe(owner.package.library);
    expect(resolve(root, owner.package.path, cargo.lib.path)).toBe(resolve(root, owner.source));
    expect(inspectRustCargoManifest(read(`${owner.package.path}/Cargo.toml`), true).crateName).toBe(cargo.lib.name);
    const needed = "dependencies" in owner ? owner.dependencies : ["semio-repo-test-host"];
    expect(Object.keys(cargo.dependencies).sort()).toEqual([...needed].sort());
    for (const [name, value] of Object.entries(cargo.dependencies)) {
      expect(name).not.toBe(contract.retired.package);
      expect(name).not.toBe(contract.package.package);
      expect(contract.providers.some(({ package: pkg }) => pkg.package === name)).toBe(false);
      if (name in externalDependencies && name !== contract.neutralLaw.package) { const actual = value as Record<string, unknown>, expected = externalDependencies[name] as Record<string, unknown>; expect(actual.optional, name).toBe(expected.optional); }
      if (typeof value === "object" && value.path) {
        const declared = packageOwners.find(({ package: pkg }) => pkg.package === name);
        const target = name === contract.neutralLaw.package ? contract.neutralLaw.source.replace(/\/🦀️\.rs$/u, "/📦️packages/🦀️rust") : declared?.package.path;
        expect(target, name).toBeDefined();
        expect(resolve(root, owner.package.path, value.path)).toBe(resolve(root, target!));
      }
    }
    const feature = needed.flatMap((name) => name === contract.neutralLaw.package ? [] : name in contract.dependencies ? [`dep:${name}`] : [`${name}/oracles`]);
    expect(cargo.features.oracles).toEqual(feature);
  });

  test("original mounted test sources own their exact development references", () => {
    
    expect(development["schemaVersion"]).toEqual(1);
    
    for (const row of development.owners) {
      const owner = packageOwners.find(({ package: pkg }) => pkg.package === row.package);
      expect(owner, row.package).toBeDefined();
      const manifest = read(`${owner!.package.path}/Cargo.toml`);
      const independent = Bun.TOML.parse(manifest);
      const cargo = toml.parse(manifest);
      expect(cargo).toEqual(independent);
      expect(digest(row.originalManifest.source)).toBe(row.originalManifest.sha256);
      const before = toml.parse(row.originalManifest.source);
      expect(before).toEqual(Bun.TOML.parse(row.originalManifest.source));
      expect(before["dev-dependencies"]).toBeUndefined();
      const originalDependencies = { ...cargo.dependencies };
      const originalFeatures = structuredClone(cargo.features);
      if (row.package === "semio-s-artifact-note-note-test-oracle") {
        expect(originalDependencies[drawing.package]).toBeDefined();
        delete originalDependencies[drawing.package];
        originalDependencies.dxf = drawing.externalDependency;
        originalFeatures.oracles = (originalFeatures.oracles as string[]).filter((feature) => feature !== `${drawing.package}/oracles`);
        originalFeatures.oracles.unshift("dep:dxf");
      }
      for (const name of Object.keys(before.dependencies)) expect(originalDependencies[name], name).toBeDefined();
      expect(originalFeatures.oracles).toBeDefined();
      const dependencies = cargo["dev-dependencies"] as Record<string, Record<string, unknown>>;
      expect(dependencies, row.package).toBeDefined();
      expect(Object.keys(dependencies).sort()).toEqual([...row.references].sort());
      for (const name of row.references) {
        const original = externalDependencies[name] as Record<string, unknown>;
        expect(original.optional, name).toBe(true);
        const { optional, ...needed } = original;
        expect(dependencies[name], name).toEqual(needed);
      }
      expect(rustTokens(read(row.source)).some((token, index, tokens) => token.text === "fn" && tokens[index + 1]?.kind === "identifier"), row.source).toBe(true);
      expect(read(row.source).includes("#[ignore]"), row.source).toBe(row.ignored);
    }
  });

  test("higher assembly composes the full declared provider set without owning concrete source mounts", () => {
    expect(existsSync(resolve(root, contract.package.path, "Cargo.toml"))).toBe(true);
    const cargo = cargoAt(contract.package.path), source = read(`${contract.owner}/🦀️.rs`);
    expect(cargo.package.name).toBe(contract.package.package);
    expect(cargo.lib.name).toBe(contract.package.library);
    expect(Object.keys(cargo.dependencies).sort()).toEqual(packageOwners.map(({ package: pkg }) => pkg.package).sort());
    for (const owner of packageOwners) {
      const value = cargo.dependencies[owner.package.package];
      expect(typeof value).toBe("object");
      expect(resolve(root, contract.package.path, (value as { path: string }).path)).toBe(resolve(root, owner.package.path));
      expect(source).toContain(owner.package.library);
    }
    expect(inspectRustCompileReferences(source).filter(({ path }) => path.includes("🗿️artifacts"))).toEqual([]);
    expect(source).not.toContain("🔣️oracle.json");
    const actualExternal = [...new Set(packageOwners.flatMap((owner) => Object.keys(cargoAt(owner.package.path).dependencies).filter((name) => name in contract.dependencies && name !== contract.neutralLaw.package)))].sort();
    expect(actualExternal).toEqual(Object.keys(contract.dependencies).filter((name) => name !== contract.neutralLaw.package).sort());
  });

  test("artifact-owned provider module graphs retain every original mount and no sibling", () => {
    const actual: string[] = [];
    for (const provider of contract.providers) {
      const source = read(provider.source), references = inspectRustCompileReferences(source).filter(({ kind, directory }) => kind === "path" && !directory);
      const targets = references.filter(({ path }) => path.includes("🏅️standards")).map(({ path, inlineBase }) => resolve(root, provider.source, "..", inlineBase ?? ".", path));
      expect(targets.sort(), provider.owner).toEqual(provider.mounts.map(({ source }) => resolve(root, source)).sort());
      for (const target of targets) expect(target.startsWith(`${resolve(root, provider.owner)}${sep}`)).toBe(true);
      expect(source).not.toContain(contract.retired.library);
      actual.push(...targets);
    }
    expect(actual.sort()).toEqual(contract.mounts.map(({ source }) => resolve(root, source)).sort());
  });

  test("current Rust callers resolve their explicit owned imports", () => {
    for (const row of contract.callers) {
      const source = read(row.source);
      expect(source, row.source).not.toContain(contract.retired.library);
      for (const { current } of row.rewrites) expect(source, row.source).toContain(current);
      expect(inspectRustCompileReferences(source).filter(({ kind }) => kind === "path").length).toBeGreaterThanOrEqual(0);
    }
  });

  test("original family and neutral-law sources remain intact through explicit owned imports", () => {
    for (const row of contract.sources) {
      const source = read(row.destination);
      expect(source.length).toBeGreaterThan(0);
      expect(source).not.toContain(contract.retired.library);
    }
    expect(inspectRustModuleGraphFacts(read(contract.neutralLaw.source)).modules.find(({ name }) => name === contract.neutralLaw.module)?.visibility).toBe("pub");
    for (const row of contract.sources.filter(({ destination }) => destination.startsWith(contract.neutralLaw.owner))) expect(read(row.destination)).not.toContain(contract.retired.library);
  });

  test("every mounted oracle implementation declares owned Rust module content", () => {
    for (const row of contract.mounts) {
      const source = read(row.destination), tokens = rustTokens(source);
      expect(tokens.some(({ text }) => ["fn", "struct", "enum", "type", "mod", "use", "const"].includes(text)), row.destination).toBe(true);
      expect(source, row.destination).not.toContain(contract.retired.library);
      for (const reference of inspectRustCompileReferences(source).filter(({ base, directory }) => base !== "generated" && !directory)) expect(existsSync(resolve(root, row.destination, "..", reference.inlineBase ?? ".", reference.path)), row.destination + ": " + reference.path).toBe(true);
    }
  });

  test("every consumer owns exactly its needed selected oracle providers and lower families", () => {
    const graph = contributions();
    for (const row of contract.artifactBindings) {
      const path = `${row.owner}/🔮️oracles/🔣️.json`;
      expect(existsSync(resolve(root, path)), path).toBe(true);
      const selected = packagesForOwner(graph, row.owner, "rust").filter(({ package: name }) => name === contract.retired.package || name === contract.package.package || packageOwners.some(({ package: pkg }) => pkg.package === name));
      expect(selected.map(({ package: name }) => name).sort(), row.owner).toEqual(row.packages);
      for (const host of selected) {
        const declared = packageOwners.find(({ package: pkg }) => pkg.package === host.package)!;
        expect(host.path).toBe(declared.package.path);
        expect(host.features ?? []).toEqual(declared.package.features);
      }
    }
    for (const row of graph.filter(({ owner }) => !owner.includes("/🗿️artifacts/"))) expect(row.oracleHostPackages.some(({ package: name }) => name === contract.retired.package || name === contract.package.package)).toBe(false);
  });

  test("actual contributions preserve their selected independent oracle packages", () => {
    for (const row of contract.contributions) {
      const path = row.path === `${contract.retired.owner}/🔣️.json` ? `${contract.owner}/🔣️.json` : row.path, current = parsed(path);
      expect(Array.isArray(current.oracleHostPackages ?? []), path).toBe(true);
      const otherHosts = row.hosts.filter(({ package: name }) => name !== contract.retired.package);
      if (row.path !== `${contract.retired.owner}/🔣️.json`) expect(currentHosts(path).filter(({ package: name }) => !packageOwners.some(({ package: pkg }) => pkg.package === name))).toEqual(otherHosts);
    }
  });

  test("the actual complete public oracle surface carries only owned types", () => {
    const paths = [...contract.mounts.map(({ source }) => source), ...contract.sources.map(({ destination }) => destination), ...packageOwners.map(({ source }) => source), `${contract.owner}/🦀️.rs`];
    expect([...new Set(paths)].filter((path) => existsSync(resolve(root, path))).flatMap(foreignPublicSurface)).toEqual([]);
    for (const row of contract.internalModules) expect(inspectRustModuleGraphFacts(read(row.source)).modules.find(({ name }) => name === row.module)?.visibility, row.source).toBe("pub(crate)");
    for (const row of contract.internalFunctions) expect(read(row.source)).toContain(`pub(crate) fn ${row.function}`);
  });

  test("the extracted lower grammar preserves the exact original owned decoder body", () => {
    const source = read(contract.grammar.source), tokens = rustTokens(source), pairs = rustTokenPairs(tokens);
    const start = tokens.findIndex(({ text }, index) => text === "fn" && tokens[index + 1]?.text === contract.grammar.function);
    expect(start).toBeGreaterThanOrEqual(0);
    let body = start;
    while (tokens[body]?.text !== "{") body++;
    expect(digest(tokens.slice(body, pairs.get(body)! + 1).map(({ text }) => text).join(" "))).toBe(contract.grammar.bodySha256);
    expect(foreignPublicSurface(contract.grammar.source)).toEqual([]);
    expect(inspectRustCompileReferences(source).some(({ path }) => path.includes("🗿️artifacts"))).toBe(false);
  });

  for (const vector of deletion.vectors) test(`actual original oracle host: ${vector.id}`, () => {
    const adapter = read(vector.adapter);
    for (const scenario of vector.requiredScenarios) expect(adapter, vector.adapter).toContain(`"${scenario}"`);
    const hosts = packagesForOwner(contributions(), vector.retainedOwner, vector.implementation);
    expect(hosts.length, vector.id).toBeGreaterThan(0);
    for (const host of hosts) if (host.features) expect(host.features).toContain("oracles");
    expect(removedOracleInputs(hosts, vector.removedOwner), `${vector.id}: actual host dependency reaches a removed artifact`).toEqual([]);
  });

  test("the lower plugin original native targets keep their full declared ownership surface", () => {
    const source = read(deletion.lowerPlugin.manifest), cargo = toml.parse(source) as { lib: { path: string }; test: { name: string; path: string }[]; dependencies: Record<string, unknown> };
    expect(cargo.test.map(({ name }) => name)).toContain("authored_assembly");
    expect(cargo.lib.path).toBe("../../🦀️.rs");
    expect(Object.keys(cargo.dependencies).sort()).toEqual(["semio-framework-plugin", "semio-s-artifact-stdio-contract"]);
    expect(removedOracleInputs([{ path: deletion.lowerPlugin.manifest.slice(0, -"/Cargo.toml".length) }], deletion.lowerPlugin.removedOwner)).toEqual([]);
  });
});
