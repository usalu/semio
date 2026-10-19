import { describe, expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { parse } from "jsonc-parser";
import environmentSchema from "@semio-tech/process/build/environment/schema.json";
import { crateLaunchRouteProblems, crateLaunchRoutes, nextLaunchRouteOrders, type CrateLaunchSubject } from "../../../🏗️authoring/📦️crate-tree/🚀️launch-routes/🟦️.ts";
import schema from "../../../🏗️authoring/📦️crate-tree/🚀️launch-routes/🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const;
const repoRoot = process.cwd();
const ajv = new Ajv({ strict: true, allowUnionTypes: true });
const validateEnvironment = ajv.compile(environmentSchema.definitions.Environment);
const validateRoute = ajv.compile(schema.definitions.Route);
const validateSubject = ajv.compile(schema.definitions.Subject);
const catalog = (file: string) => JSON.parse(JSON.stringify(parse(readFileSync(join(repoRoot, file), "utf8")))) as { configurations: Record<string, any>[] };
const nameOf = /^🦀️ (\S+) (🏗️build|🔎️check|🧪️test) (🛠️prototype|🚀️release)$/u;
const commandOf = /^bun repo run (\S+):(build|check|test) --configuration=(prototype|release) --parallel=1 -- (.*)$/u;

const sample: CrateLaunchSubject[] = [{ name: "semio-demo-core", shape: "lib" }, { name: "semio-demo-core-tests", shape: "tests" }, { name: "semio-demo-laws", project: "@semio-tech/demo-laws-rs", shape: "lib" }];

describe("crate launch routes", () => {
  test("every generated route carries the build environment of its schema and validates against Ajv and a SQLite oracle", () => {
    const routes = crateLaunchRoutes(sample, nextLaunchRouteOrders([]));
    expect(routes.length).toBe(sample.length * protocol.targets.length * protocol.configurations.length);
    const database = new Database(":memory:");
    try {
      database.exec("create table routes(name text, environment text)");
      const insert = database.query("insert into routes values(?, ?)");
      for (const route of routes) {
        expect(validateRoute(route), JSON.stringify(validateRoute.errors)).toBe(true);
        expect(validateEnvironment(route.env), JSON.stringify(validateEnvironment.errors)).toBe(true);
        expect(route.env).toEqual(environmentSchema.definitions.Protocol.const.environment);
        insert.run(route.name, JSON.stringify(route.env));
      }
      const invalid = database.query("select name from routes where coalesce(json_extract(environment,'$.BUN_RUNTIME_TRANSPILER_CACHE_PATH')='0' and json_extract(environment,'$.NODE_DISABLE_COMPILE_CACHE')='1',0)=0").all();
      expect(invalid).toEqual([]);
    } finally { database.close(); }
    expect(crateLaunchRouteProblems(routes)).toEqual([]);
  });

  test("subjects that are no crates are refused", () => {
    for (const subject of [{ name: "Bad Name", shape: "lib" }, { name: "demo", shape: "bin" }, { name: "demo" }, { name: "demo", shape: "lib", extra: true }, { name: "demo", shape: "lib", targets: [] }, { name: "demo", shape: "lib", targets: ["bench"] }, { name: "demo", shape: "lib", arguments: { bench: [] } }]) {
      expect(validateSubject(subject)).toBe(false);
      expect(() => crateLaunchRoutes([subject as CrateLaunchSubject], {})).toThrow();
    }
  });

  test("the protocol regenerates the standard routes of the shipped catalog", () => {
    const launch = catalog(".vscode/launch.json");
    const byCrate = new Map<string, Record<string, any>[]>();
    for (const entry of launch.configurations) {
      const name = nameOf.exec(entry.name ?? "");
      if (name) byCrate.set(name[1]!, [...(byCrate.get(name[1]!) ?? []), entry]);
    }
    let compared = 0;
    for (const [crate, entries] of byCrate) {
      const commands = entries.map((entry) => commandOf.exec(entry.command ?? ""));
      if (commands.some((match) => !match) || new Set(commands.map((match) => match![1])).size !== 1) continue;
      const project = commands[0]![1]!;
      const shape = (Object.keys(protocol.shapes) as (keyof typeof protocol.shapes)[]).find((candidate) => entries.length === 6 && commands.every((match) => match![4] === protocol.shapes[candidate][match![2] as "build"].join(" ")));
      if (!shape) continue;
      const regenerated = crateLaunchRoutes([{ name: crate, ...(project === crate ? {} : { project }), shape }], nextLaunchRouteOrders([]));
      const strip = (route: Record<string, any>): Record<string, any> => ({ ...route, presentation: { group: route.presentation.group } });
      const normal = (list: Record<string, any>[]) => list.map(strip).sort((a, b) => a.name.localeCompare(b.name));
      expect(normal(entries), crate).toEqual(normal(regenerated as unknown as Record<string, any>[]));
      compared++;
    }
    expect(compared).toBeGreaterThan(500);
  });

  test("every route of the crate family in the expanded catalog carries the build environment", () => {
    expect(crateLaunchRouteProblems(catalog(".vscode/launch.json").configurations).slice(0, 5)).toEqual([]);
  });

  test("a route without the build environment is reported", () => {
    const [route] = crateLaunchRoutes([sample[0]!], nextLaunchRouteOrders([]));
    const { env: _env, ...bare } = route!;
    expect(crateLaunchRouteProblems([{ ...bare, env: {} }]).some((problem) => problem.includes("BUN_RUNTIME_TRANSPILER_CACHE_PATH"))).toBe(true);
    expect(crateLaunchRouteProblems([{ ...route!, env: { ...route!.env, NODE_DISABLE_COMPILE_CACHE: "0" } }]).length).toBeGreaterThan(0);
    expect(crateLaunchRouteProblems([{ ...route!, presentation: { group: "4_test", order: 1 } }]).length).toBeGreaterThan(0);
  });

  test("the orders continue above every group and the names are unique", () => {
    const orders = nextLaunchRouteOrders([{ presentation: { group: "4_build", order: 1013.001 } }, { presentation: { group: "4_gate", order: 5 } }, { presentation: { group: "elsewhere", order: 99999 } }]);
    expect(orders).toEqual({ "4_build": 1014, "4_gate": 922, "4_test": 903 });
    const routes = crateLaunchRoutes(sample, orders);
    expect(new Set(routes.map((route) => route.name)).size).toBe(routes.length);
    for (const group of Object.keys(orders)) {
      const mine = routes.filter((route) => route.presentation.group === group).map((route) => route.presentation.order);
      expect(mine).toEqual(Array.from({ length: mine.length }, (_, at) => orders[group]! + at));
    }
  });

  test("a subject limits its targets and replaces the arguments of a target its shape does not fit", () => {
    const routes = crateLaunchRoutes([{ name: "semio-demo-wide", shape: "package", targets: ["test"], arguments: { test: ["--test", "wide", "--offline"] } }, { name: "semio-demo-package", shape: "package" }], nextLaunchRouteOrders([]));
    expect(routes.map((route) => route.name)).toEqual(["🦀️ semio-demo-wide 🧪️test 🛠️prototype", "🦀️ semio-demo-wide 🧪️test 🚀️release", ...["🏗️build", "🔎️check", "🧪️test"].flatMap((target) => [`🦀️ semio-demo-package ${target} 🛠️prototype`, `🦀️ semio-demo-package ${target} 🚀️release`])]);
    expect(routes[0]!.command).toBe("bun repo run semio-demo-wide:test --configuration=prototype --parallel=1 -- --test wide --offline");
    expect(routes.filter((route) => route.command.startsWith("bun repo run semio-demo-package:")).map((route) => route.command.split(" -- ")[1])).toEqual(["--lib --offline", "--lib --offline", "--lib --offline", "--lib --offline", "--offline", "--offline"]);
    expect(crateLaunchRouteProblems(routes)).toEqual([]);
  });
});
