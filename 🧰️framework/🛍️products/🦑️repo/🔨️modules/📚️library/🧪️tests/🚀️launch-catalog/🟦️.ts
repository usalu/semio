import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { execFileSync } from "node:child_process";
import { createRequire } from "node:module";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { parse } from "jsonc-parser";
import environmentSchema from "@semio-tech/process/build/environment/schema.json";
import { checkLaunchCatalog, expandLaunchCatalog, launchCatalogProblems, parsePlaygroundRows, readLaunchSeed, readPlaygroundRegistry, registerCrateLaunchRoutes, renderLaunchDocument, withCrateLaunchSubjects, writeLaunchCatalog, type LaunchSeed, type PlaygroundRow } from "../../🏗️authoring/🚀️launch-catalog/🟦️.ts";
import cratesSchema from "../../🏗️authoring/📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json" with { type: "json" };
import schema from "../../🏗️authoring/🚀️launch-catalog/🧬️schema/🔣️.json" with { type: "json" };
import fixture from "../🧫️fixtures/🚀️launch-catalog/🔣️.json" with { type: "json" };

const require = createRequire(import.meta.url);
const lodash = require("lodash");
const iarna = require("@iarna/toml");
const repoRoot = process.cwd();
const protocol = schema.definitions.Protocol.const;
const crates = cratesSchema.definitions.Protocol.const;
const environment = environmentSchema.definitions.Protocol.const.environment;
const ajv = new Ajv({ strict: false, allowUnionTypes: true });
ajv.addSchema(cratesSchema);
ajv.addSchema(schema);
const validateSeed = ajv.compile({ $ref: `${schema.$id}#/definitions/Seed` });

const seedOf = (value: unknown) => value as LaunchSeed;
const registryOf = (value: unknown) => value as PlaygroundRow[];

/** 🔭️ A second implementation of the dev launchers and crate routes through lodash string templates and plain maps. */
function oracle(seed: any, registry: any[]): any[] {
  const rows = lodash.keyBy(registry, "variant");
  const nameOf = lodash.template("${prefix}${marker}${icon}", { interpolate: /\$\{([\s\S]+?)\}/g });
  const commandOf = lodash.template("bun repo run @semio-tech/framework-os-dev:dev-${variant}-${renderer}-prototype --skip-cache --configuration=prototype", { interpolate: /\$\{([\s\S]+?)\}/g });
  const dev = (row: any, renderer: string, icon: string, port: number, order: number, name: string, extra: object) => ({
    name,
    type: "node-terminal",
    request: "launch",
    command: commandOf({ variant: row.variant, renderer }),
    cwd: "${workspaceFolder}",
    env: { S_OS_PORT: `${port}`, SEMIO_PLUGIN: row.variant, SEMIO_RENDERER: renderer, ...(row.app ? { SEMIO_APP: row.app } : {}), ...extra, ...environment },
    presentation: { group: "3_dev", order },
    serverReadyAction: { action: "openExternally", pattern: `(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):${port})`, uriFormat: "%s" },
  });
  const renderers = [{ id: "react", icon: "⚛️react", order: "order" }, { id: "wgpu", icon: "🧊️wgpu🌐️wasm", order: "wgpuOrder" }];
  const placeholderEntries = (placeholder: string): any[] => {
    const [, variant, kind] = placeholder.split(":") as [string, string, string];
    const row = rows[variant], launcher = seed.devLaunchers[variant];
    if (kind === "users") {
      return renderers.filter((renderer) => launcher[renderer.order] !== undefined).flatMap((renderer) =>
        lodash.map(row.userPorts[renderer.id], (port: number, index: number) => dev(row, renderer.id, renderer.icon, port, lodash.round(launcher[renderer.order] + (index + 1) * 0.01, 2), nameOf({ prefix: "🛠️dev", marker: launcher.users.namePrefixPattern.replace("{N}", `${index + 1}`), icon: renderer.icon }), lodash.mapValues(launcher.users.env, (value: string) => value.replaceAll("{N}", `${index + 1}`)))));
    }
    const renderer = lodash.find(renderers, { id: kind });
    return [dev(row, kind, renderer.icon, row.ports[kind], launcher[renderer.order], nameOf({ prefix: "🛠️dev", marker: launcher.namePrefix, icon: renderer.icon }), launcher.env ?? {})];
  };
  const slots = seed.configurations.map((item: any) => (item === "@generated:crate-routes" ? item : typeof item === "string" ? placeholderEntries(item) : [item]));
  const others = lodash.flatten(slots.filter((slot: any) => slot !== "@generated:crate-routes"));
  const base = lodash.fromPairs(crates.targets.map((target: any) => [target.group, target.orderBase]));
  for (const entry of others) {
    const group = entry.presentation?.group;
    if (group in base) base[group] = Math.max(base[group], Math.floor(entry.presentation.order) + 1);
  }
  const routes = lodash.flatMap(seed.crateRoutes.subjects, (subject: any) =>
    lodash.flatMap(crates.targets.filter((target: any) => !subject.targets || subject.targets.includes(target.id)), (target: any) =>
      crates.configurations.map((configuration: any) => ({
        name: `🦀️ ${subject.name} ${target.icon}${target.id} ${configuration.icon}${configuration.id}`,
        type: "node-terminal",
        request: "launch",
        command: `bun repo run ${subject.project ?? subject.name}:${target.id} --configuration=${configuration.id} --parallel=1 -- ${(subject.arguments?.[target.id] ?? (crates.shapes as Record<string, Record<string, string[]>>)[subject.shape]![target.id]!).join(" ")}`,
        cwd: "${workspaceFolder}",
        env: { ...environment },
        presentation: { group: target.group, order: 0 },
      }))));
  for (const route of routes) {
    route.presentation.order = base[route.presentation.group]++;
  }
  return lodash.flatMap(slots, (slot: any) => (slot === "@generated:crate-routes" ? routes : slot));
}

describe("launch catalog expansion", () => {
  test("the fixture seed is admitted by the neutral schema and expands to the fixture catalog", () => {
    expect(validateSeed(fixture.seed), JSON.stringify(validateSeed.errors)).toBe(true);
    expect(expandLaunchCatalog(seedOf(fixture.seed), registryOf(fixture.registry))).toEqual(fixture.expected as any);
    expect(launchCatalogProblems(seedOf(fixture.seed), registryOf(fixture.registry))).toEqual([]);
  });

  test("a lodash template implementation produces the same configurations", () => {
    const independent = oracle(fixture.seed, fixture.registry);
    expect(independent).toEqual(fixture.expected.configurations as any);
    expect(expandLaunchCatalog(seedOf(fixture.seed), registryOf(fixture.registry)).configurations).toEqual(independent);
  });

  test("every dev launcher carries the build environment, its registry port and a ready action on that port", () => {
    for (const entry of expandLaunchCatalog(seedOf(fixture.seed), registryOf(fixture.registry)).configurations.filter((candidate) => candidate.name.startsWith(protocol.dev.namePrefix))) {
      expect(entry.env).toMatchObject(environment);
      expect(entry.serverReadyAction.pattern).toContain(`:${entry.env.S_OS_PORT})`);
      expect(entry.command).toContain(`dev-${entry.env.SEMIO_PLUGIN}-${entry.env.SEMIO_RENDERER}-prototype`);
    }
  });

  test("the crate routes continue every group above all other routes and keep the order of the subjects", () => {
    const configurations = expandLaunchCatalog(seedOf(fixture.seed), registryOf(fixture.registry)).configurations;
    const routes = configurations.filter((entry) => entry.name.startsWith(crates.namePrefix));
    for (const group of ["4_build", "4_gate", "4_test"]) {
      const others = configurations.filter((entry) => entry.presentation?.group === group && !routes.includes(entry)).map((entry) => entry.presentation.order);
      const mine = routes.filter((entry) => entry.presentation.group === group).map((entry) => entry.presentation.order);
      expect(Math.min(...mine)).toBeGreaterThan(Math.max(...others));
      expect(mine).toEqual([...mine].sort((a, b) => a - b));
    }
  });

  test("a repeated expansion is byte identical and ends with one newline", () => {
    const first = renderLaunchDocument(expandLaunchCatalog(seedOf(fixture.seed), registryOf(fixture.registry)));
    expect(renderLaunchDocument(expandLaunchCatalog(seedOf(structuredClone(fixture.seed)), registryOf(fixture.registry)))).toBe(first);
    expect(first.endsWith("}\n")).toBe(true);
    expect(JSON.parse(first)).toEqual(fixture.expected as any);
  });

  for (const row of fixture.problems) {
    test(`a broken catalog is reported: ${row.name}`, () => {
      const base = { seed: structuredClone(fixture.seed), registry: structuredClone(fixture.registry) };
      const broken = applyPatch(base as any, row.patch as any[], true).newDocument as { seed: LaunchSeed; registry: PlaygroundRow[] };
      const problems = launchCatalogProblems(broken.seed, broken.registry);
      expect(problems.some((problem) => problem.includes(row.problem)), JSON.stringify(problems)).toBe(true);
    });
  }

  test("registering subjects adds the crates that have none and repeating adds nothing", () => {
    const seed = seedOf(fixture.seed);
    const first = withCrateLaunchSubjects(seed, fixture.register.subjects as any);
    expect(first.added.map((subject) => subject.name)).toEqual(fixture.register.addedNames);
    expect(first.seed.crateRoutes.subjects.length).toBe(seed.crateRoutes.subjects.length + fixture.register.addedNames.length);
    const again = withCrateLaunchSubjects(first.seed, fixture.register.subjects as any);
    expect(again.added).toEqual([]);
    expect(again.seed).toBe(first.seed);
    expect(() => withCrateLaunchSubjects(seed, [{ name: "Bad Name", shape: "lib" }] as any)).toThrow();
    expect(launchCatalogProblems(first.seed, registryOf(fixture.registry))).toEqual([]);
  });

  test("writing, checking and registering work on the files of a checkout and never write a broken catalog", () => {
    const root = mkdtempSync(join(tmpdir(), "semio-launch-catalog-"));
    try {
      mkdirSync(join(root, ".vscode"));
      writeFileSync(join(root, protocol.seed), renderLaunchDocument(fixture.seed));
      const registry = registryOf(fixture.registry);
      expect(checkLaunchCatalog(root, registry).some((problem) => problem.includes("is not the expansion"))).toBe(true);
      expect(writeLaunchCatalog(root, registry)).toEqual({ changed: true, problems: [] });
      expect(readFileSync(join(root, protocol.catalog), "utf8")).toBe(renderLaunchDocument(fixture.expected));
      expect(writeLaunchCatalog(root, registry).changed).toBe(false);
      expect(checkLaunchCatalog(root, registry)).toEqual([]);
      writeFileSync(join(root, protocol.catalog), "{}\n");
      expect(checkLaunchCatalog(root, registry)).toHaveLength(1);
      const plan = { crates: [{ name: "demo-new", lib: true }, { name: "demo-new-tests", lib: false }] } as never;
      const backup = join(root, "backup", "carve");
      const names = registerCrateLaunchRoutes(root, plan, backup, registry);
      expect(names).toHaveLength(12);
      expect(JSON.parse(readFileSync(`${backup}-launch-subjects.json`, "utf8"))).toEqual(["demo-new", "demo-new-tests"]);
      expect(checkLaunchCatalog(root, registry)).toEqual([]);
      expect((JSON.parse(readFileSync(join(root, protocol.catalog), "utf8")) as { configurations: { name: string }[] }).configurations.filter((entry) => entry.name.startsWith("🦀️ demo-new ")).map((entry) => entry.name)).toEqual(names.filter((name) => name.startsWith("🦀️ demo-new ")));
      expect(registerCrateLaunchRoutes(root, plan, undefined, registry)).toEqual([]);
      const before = readFileSync(join(root, protocol.catalog), "utf8");
      const broken = { ...readLaunchSeed(root), compounds: [{ name: "broken", configurations: ["missing"] }] };
      writeFileSync(join(root, protocol.seed), renderLaunchDocument(broken));
      const result = writeLaunchCatalog(root, registry);
      expect(result.changed).toBe(false);
      expect(result.problems.some((problem) => problem.includes("missing"))).toBe(true);
      expect(readFileSync(join(root, protocol.catalog), "utf8")).toBe(before);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("playground rows of a manifest agree with an independent TOML parser", () => {
    const rows = parsePlaygroundRows(fixture.manifest.text);
    const independent = iarna.parse(fixture.manifest.text).package.metadata.semio.playground.map((row: any) => ({ variant: row.variant, ...(row.app ? { app: row.app } : {}), ports: { react: row.ports.react, wgpu: row.ports.wgpu }, ...(row.user_ports ? { userPorts: { react: [...row.user_ports.react], wgpu: [...row.user_ports.wgpu] } } : {}) }));
    expect(rows).toEqual(independent);
    expect(parsePlaygroundRows("[package]\nname = \"plain\"\n")).toEqual([]);
    expect(() => parsePlaygroundRows("[[package.metadata.semio.playground]]\nvariant = \"x\"\nports = { react = 1 }\n")).toThrow();
  });
});

describe("the checked in launch catalog", () => {
  const seedText = readFileSync(join(repoRoot, protocol.seed), "utf8");
  const catalogText = readFileSync(join(repoRoot, protocol.catalog), "utf8");

  test("launch.json is the deterministic expansion of the seed and the seed has no problems", () => {
    expect(checkLaunchCatalog(repoRoot)).toEqual([]);
  }, 120_000);

  test("the seed is admitted by the neutral schema and two JSONC parsers read it alike", () => {
    const seed = parse(seedText);
    expect(validateSeed(seed), JSON.stringify(validateSeed.errors?.slice(0, 3))).toBe(true);
    expect(seed).toEqual(readLaunchSeed(repoRoot) as any);
  });

  test("the playground registry of the component crates agrees with an independent TOML parser", () => {
    const registry = readPlaygroundRegistry(repoRoot);
    const listed = execFileSync("git", ["ls-files", "-z", "--", "*Cargo.toml"], { cwd: repoRoot, encoding: "utf8", maxBuffer: 1 << 28 }).split("\0").filter((file) => file && !file.startsWith(protocol.registry.excludedPrefix) && existsSync(join(repoRoot, file)));
    const independent = listed.flatMap((file) => {
      const text = readFileSync(join(repoRoot, file), "utf8");
      if (!text.includes(protocol.registry.marker)) return [];
      return (iarna.parse(text).package?.metadata?.semio?.playground ?? []).map((row: any) => ({ variant: row.variant, ports: { react: row.ports.react, wgpu: row.ports.wgpu } }));
    }).sort((left: any, right: any) => left.variant.localeCompare(right.variant));
    expect(registry.map((row) => ({ variant: row.variant, ports: row.ports }))).toEqual(independent);
    expect(registry.length).toBeGreaterThan(100);
  }, 120_000);

  test("launch.json holds no placeholder, a unique name per route and the build environment on every route", () => {
    const document = JSON.parse(catalogText) as { configurations: any[]; compounds: any[]; inputs: any[] };
    const names = document.configurations.map((entry) => entry.name);
    expect(document.configurations.every((entry) => typeof entry === "object")).toBe(true);
    expect(new Set(names).size).toBe(names.length);
    for (const entry of document.configurations) expect(entry.env).toMatchObject(environment);
    for (const compound of document.compounds) for (const member of compound.configurations) expect(names).toContain(member);
    expect(Object.keys(document).sort()).toEqual(["compounds", "configurations", "inputs", "version"]);
    expect(catalogText.includes("@generated:")).toBe(false);
  });

  test("every playground variant has its dev launchers in launch.json", () => {
    const document = JSON.parse(catalogText) as { configurations: any[] };
    const commands = new Set(document.configurations.map((entry) => entry.command));
    for (const row of readPlaygroundRegistry(repoRoot)) {
      expect(commands.has(protocol.dev.command.replace("{variant}", row.variant).replace("{renderer}", "react")), row.variant).toBe(true);
    }
  }, 120_000);
});
