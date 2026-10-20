/** 🚦️ The test protocol: the fixture is the contract, a Python implementation and real `bun test --shard`, real vitest and Python's XML parser are the oracles. */
import { test, expect } from "bun:test";
import Ajv from "ajv";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { appendTestReportV1, junitSuitesV1, planTestCommandV1, prepareTestCommandV1, readTestShardV1, renderTestReportV1, syntheticTestSuiteV1, testProtocolMessageV1, testReportPathV1, testShardFilesV1 } from "../🟦️.ts";
import { runBudgetedTestCommand } from "../../🎛️execution/🟦️.ts";
import { captureOwnedCommandV1 } from "../../../🎛️owned-execution/🟦️.ts";
import protocol from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

const ajv = new Ajv({ strict: true, allowUnionTypes: true });
const compile = (definition: string) => ajv.compile({ $schema: schema.$schema, definitions: schema.definitions, $ref: `#/definitions/${definition}` });
const shardOf = (text: string | null) => (text === null ? null : readTestShardV1({ SEMIO_TEST_SHARD: text }));
const summary = (suites: ReturnType<typeof junitSuitesV1>) => suites.map(({ xml: _xml, ...row }) => row);

test("the schema closes the protocol table, and the fixture agrees with it", () => {
  expect(ajv.validateSchema(schema)).toBe(true);
  expect(compile("Protocol")(protocol), JSON.stringify(compile("Protocol").errors)).toBe(true);
  const shard = compile("Shard");
  for (const row of fixture.shards) if (row.shard !== null) expect(shard(row.shard), row.name).toBe(true);
  for (const text of ["0/2", "1/1", "a/b", "2/3 ", "1/2/3", "-1/2", "01/2"]) expect(shard(text), text).toBe(false);
  const suite = compile("Suite");
  for (const row of fixture.junit) for (const expected of row.expected) expect(suite(expected), row.name).toBe(true);
  expect(compile("Protocol")({ ...protocol, tools: [] })).toBe(false);
  expect(compile("Protocol")({ ...protocol, extra: 1 })).toBe(false);
});

test("every fixture row has exactly its authored answer", () => {
  for (const row of fixture.shards) expect(testShardFilesV1(row.files, shardOf(row.shard)), row.name).toEqual(row.expected);
  for (const row of fixture.reads) expect(readTestShardV1(row.environment), row.name).toEqual(row.expected);
  for (const text of fixture.refusedShards) expect(() => readTestShardV1({ SEMIO_TEST_SHARD: text }), text).toThrow("Closed test shard required");
  for (const row of fixture.plans) {
    const plan = planTestCommandV1(row.command, row.args, shardOf(row.shard), row.junit);
    expect({ tool: plan.tool, args: plan.args, sharded: plan.sharded, junit: plan.junit }, row.name).toEqual(row.expected);
  }
  for (const row of fixture.junit) expect(summary(junitSuitesV1(row.xml)), row.name).toEqual(row.expected);
  for (const row of fixture.paths) expect(testReportPathV1(row.environment), row.name).toEqual(row.expected);
  for (const environment of fixture.refusedPaths) expect(() => testReportPathV1(environment)).toThrow("Absolute test result file required");
  for (const row of fixture.reports) {
    const xml = renderTestReportV1(row.suites.map((suite) => ({ ...suite, xml: `<testsuite name="${suite.name}"/>` })));
    const root = /<testsuites ([^>]*)>/u.exec(xml)![1]!;
    for (const [key, value] of Object.entries(row.expected)) expect(new RegExp(`${key}="${key === "time" ? value.toFixed(3) : value}"`, "u").test(root), `${row.name} ${key}`).toBe(true);
  }
});

test("an independent Python implementation agrees on every section", () => {
  const run = spawnSync("python3", ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py")], { input: JSON.stringify({ protocol, fixture }), encoding: "utf8" });
  expect(run.status, run.stderr).toBe(0);
  const answers = JSON.parse(run.stdout);
  fixture.shards.forEach((row, index) => expect(answers.shards[index], row.name).toEqual(row.expected));
  fixture.reads.forEach((row, index) => expect(answers.reads[index], row.name).toEqual(row.expected));
  answers.refusedShards.forEach((answer: unknown, index: number) => expect(answer, fixture.refusedShards[index]).toBe("refused"));
  fixture.plans.forEach((row, index) => expect(answers.plans[index], row.name).toEqual(row.expected));
  fixture.junit.forEach((row, index) => expect(answers.junit[index], row.name).toEqual(row.expected));
  fixture.paths.forEach((row, index) => expect(answers.paths[index], row.name).toEqual(row.expected));
  answers.refusedPaths.forEach((answer: unknown) => expect(answer).toBe("refused"));
  fixture.reports.forEach((row, index) => expect(answers.reports[index], row.name).toEqual(row.expected));
});

const scratch = (): string => realpathSync(mkdtempSync(join(tmpdir(), "test-protocol-")));
const names = ["alpha", "beta", "gamma", "delta"];
const write = (directory: string): string[] => names.map((name) => {
  const file = join(directory, `${name}.test.ts`);
  writeFileSync(file, `import { test, expect } from "bun:test";\ntest("${name} one", () => expect(1).toBe(1));\ntest("${name} two", () => expect(${name === "beta" ? 2 : 1}).toBe(1));\n`);
  return file;
});
const caseNames = (xml: string): string[] => [...xml.matchAll(/<testcase\b[^>]*?\sname="([^"]*)"/gu)].map((match) => match[1]!).sort();

test("real bun test: the shards split the files without overlap, report into test.xml and advertise support", () => {
  const directory = scratch();
  try {
    const files = write(directory);
    const seen: string[][] = [];
    let failures = 0;
    for (const index of [1, 2]) {
      const xml = join(directory, `shard-${index}.xml`), status = join(directory, `status-${index}`);
      const environment = { SEMIO_TEST_SHARD: `${index}/2`, SEMIO_TEST_XML_OUTPUT_FILE: xml, SEMIO_TEST_SHARD_STATUS_FILE: status };
      const prepared = prepareTestCommandV1(process.execPath, ["test", ...files], environment, directory);
      const started = performance.now();
      const run = spawnSync(process.execPath, prepared.args as string[], { cwd: directory, encoding: "utf8", env: { ...process.env, ...prepared.environment } });
      prepared.finish({ code: run.status, signal: run.signal, durationMs: performance.now() - started });
      expect(existsSync(status), `shard ${index} advertises support`).toBe(true);
      const text = readFileSync(xml, "utf8");
      seen.push(caseNames(text));
      failures += (text.match(/<failure/gu) ?? []).length;
      const python = spawnSync("python3", ["-I", "-c", "import sys,xml.etree.ElementTree as E;r=E.parse(sys.argv[1]).getroot();print(r.get('tests'),len(list(r.iter('testcase'))))", xml], { encoding: "utf8" });
      expect(python.status, python.stderr).toBe(0);
      const [declared, counted] = python.stdout.trim().split(" ");
      expect(declared, "root count equals counted cases").toBe(counted);
      expect(readdirSync(directory).filter((name) => name.startsWith("junit-")), "scratch JUnit removed").toEqual([]);
    }
    expect(seen[0]!.length).toBeGreaterThan(0);
    expect(seen[1]!.length).toBeGreaterThan(0);
    expect([...seen[0]!, ...seen[1]!].sort()).toEqual(names.flatMap((name) => [`${name} one`, `${name} two`]).sort());
    expect(seen[0]!.filter((name) => seen[1]!.includes(name))).toEqual([]);
    expect(failures).toBe(1);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("a recognised tool without a shard still reports, an unknown command stays out of the report, and a crash without any failed case is reported as a failure", () => {
  const directory = scratch();
  try {
    const xml = join(directory, "test.xml");
    const environment = { SEMIO_TEST_XML_OUTPUT_FILE: xml };
    const quiet = prepareTestCommandV1("cargo", ["test", "-p", "quiet"], environment, directory);
    expect(quiet.args).toEqual(["test", "-p", "quiet"]);
    quiet.finish({ code: 0, signal: null, durationMs: 12 });
    const loud = prepareTestCommandV1("cargo", ["test", "-p", "loud"], environment, directory);
    loud.finish({ code: 3, signal: null, durationMs: 40 });
    const timed = prepareTestCommandV1("cargo", ["test", "-p", "slow"], environment, directory);
    timed.finish({ code: null, signal: "SIGTERM", reason: "timeout", durationMs: 9000 });
    const suites = junitSuitesV1(readFileSync(xml, "utf8"));
    expect(suites.map((suite) => [suite.tests, suite.failures])).toEqual([[1, 0], [1, 1], [1, 1]]);
    expect(readFileSync(xml, "utf8")).toContain('message="exit status 3"');
    const unknown = prepareTestCommandV1("sh", ["-c", "exit 0"], environment, directory);
    expect(unknown.args).toEqual(["-c", "exit 0"]);
    expect(unknown.environment).toEqual({});
    unknown.finish({ code: 0, signal: null, durationMs: 1 });
    expect(junitSuitesV1(readFileSync(xml, "utf8")).length).toBe(3);
    expect(readFileSync(xml, "utf8")).toContain('message="timeout"');
    const bun = join(directory, "crash.test.ts");
    writeFileSync(bun, 'import { test } from "bun:test";\ntest("only", () => {});\n');
    const out = join(directory, "crash.xml");
    const crashed = prepareTestCommandV1(process.execPath, ["test", bun], { SEMIO_TEST_XML_OUTPUT_FILE: out }, directory);
    const run = spawnSync(process.execPath, crashed.args as string[], { cwd: directory, encoding: "utf8" });
    crashed.finish({ code: 1, signal: null, durationMs: 1 });
    expect(run.status).toBe(0);
    expect(junitSuitesV1(readFileSync(out, "utf8")).map((suite) => suite.failures)).toEqual([0, 1]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the first spawn of a report owner starts a fresh file, children of an owner append", () => {
  const directory = scratch();
  try {
    const xml = join(directory, "own.xml");
    writeFileSync(xml, renderTestReportV1([syntheticTestSuiteV1({ name: "stale", durationMs: 1 })]));
    const first = prepareTestCommandV1("cargo", ["test", "-p", "first"], { SEMIO_TEST_XML_OUTPUT_FILE: xml }, directory);
    expect(first.environment).toEqual({ SEMIO_TEST_REPORT_OWNER: String(process.pid) });
    first.finish({ code: 0, signal: null, durationMs: 1 });
    expect(junitSuitesV1(readFileSync(xml, "utf8")).map((suite) => suite.name)).toEqual(["cargo test -p first"]);
    const child = prepareTestCommandV1("cargo", ["test", "-p", "second"], { SEMIO_TEST_XML_OUTPUT_FILE: xml, SEMIO_TEST_REPORT_OWNER: "1" }, directory);
    expect(child.environment).toEqual({});
    child.finish({ code: 0, signal: null, durationMs: 1 });
    expect(junitSuitesV1(readFileSync(xml, "utf8")).map((suite) => suite.name)).toEqual(["cargo test -p first", "cargo test -p second"]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("concurrent processes append to one report without losing a suite", async () => {
  const directory = scratch();
  try {
    const xml = join(directory, "many.xml");
    mkdirSync(dirname(xml), { recursive: true });
    const module = join(import.meta.dir, "..", "🟦️.ts");
    const program = `import { appendTestReportV1, syntheticTestSuiteV1 } from ${JSON.stringify(module)};\nfor (let i = 0; i < 4; i++) appendTestReportV1(process.argv[2], [syntheticTestSuiteV1({ name: process.argv[3] + "-" + i, durationMs: 1 })], false);\n`;
    const script = join(directory, "append.ts");
    writeFileSync(script, program);
    await Promise.all(Array.from({ length: 6 }, (_v, index) => new Promise<void>((accept, refuse) => {
      const child = spawn(process.execPath, [script, xml, `p${index}`], { stdio: "inherit" });
      child.once("exit", (code) => (code === 0 ? accept() : refuse(Error(`append exited ${code}`))));
    })));
    expect(junitSuitesV1(readFileSync(xml, "utf8")).length).toBe(24);
    expect(existsSync(xml + ".lock")).toBe(false);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 60000);

const vitestBinary = (() => { try { return join(dirname(Bun.resolveSync("vitest/package.json", import.meta.dir)), "vitest.mjs"); } catch { return undefined; } })();
const node = spawnSync("node", ["--version"], { encoding: "utf8" }).status === 0;
(vitestBinary && node ? test : test.skip)("real vitest: a shard and the JUnit file reach the tool and its cases land in test.xml", () => {
  const directory = scratch();
  try {
    for (const name of names) writeFileSync(join(directory, `${name}.test.js`), `test("${name} one", () => expect(1).toBe(1));\ntest("${name} two", () => expect(1).toBe(1));\n`);
    const seen: string[][] = [];
    for (const index of [1, 2]) {
      const xml = join(directory, `v-${index}.xml`);
      const prepared = prepareTestCommandV1("node", [vitestBinary!, "run", "--root", directory, "--globals"], { SEMIO_TEST_SHARD: `${index}/2`, SEMIO_TEST_XML_OUTPUT_FILE: xml }, directory);
      const run = spawnSync("node", prepared.args as string[], { cwd: directory, encoding: "utf8" });
      prepared.finish({ code: run.status, signal: run.signal, durationMs: 1 });
      expect(run.status, run.stderr + run.stdout).toBe(0);
      seen.push(caseNames(readFileSync(xml, "utf8")));
    }
    expect([...seen[0]!, ...seen[1]!].sort()).toEqual(names.flatMap((name) => [`${name} one`, `${name} two`]).sort());
    expect(seen[0]!.length).toBe(4);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 120000);

test("the spawn points of the framework apply the protocol: shard, result file and shard status reach a real bun test", async () => {
  const directory = scratch();
  try {
    const files = write(directory).filter((file) => !file.includes("beta"));
    const environment = (name: string) => ({ ...process.env, SEMIO_TEST_SHARD: "1/2", SEMIO_TEST_XML_OUTPUT_FILE: join(directory, `${name}.xml`), SEMIO_TEST_SHARD_STATUS_FILE: join(directory, `${name}.status`) });
    await runBudgetedTestCommand(process.execPath, ["test", ...files], { cwd: directory, budgetMs: 60000, env: environment("budgeted"), throwOnFailure: true });
    await captureOwnedCommandV1(process.execPath, ["test", ...files], directory, { env: environment("captured") });
    for (const name of ["budgeted", "captured"]) {
      expect(existsSync(join(directory, `${name}.status`)), name).toBe(true);
      const cases = caseNames(readFileSync(join(directory, `${name}.xml`), "utf8"));
      expect(cases.length, name).toBeGreaterThan(0);
      expect(cases.length, name).toBeLessThan(files.length * 2);
    }
    expect(caseNames(readFileSync(join(directory, "budgeted.xml"), "utf8"))).toEqual(caseNames(readFileSync(join(directory, "captured.xml"), "utf8")));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}, 120000);

test("operator texts exist in both languages and neither is a default", () => {
  const text = testProtocolMessageV1("reportFailed", { reason: "disk full" });
  expect(text).toContain("could not be written: disk full");
  expect(text).toContain("konnte nicht geschrieben werden: disk full");
  expect(Object.keys(protocol.messages.reportFailed).sort()).toEqual(["de", "en"]);
});
