import { test, expect } from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import { readFileSync } from "node:fs";
import vectors from "./🔣️.json" with { type: "json" };
import schema from "./🧬️.json" with { type: "json" };

const extracted = (): ((root: unknown, parts: string[], create: boolean, body: (directory: unknown) => Promise<unknown>) => Promise<unknown>) => {
  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
  const tree = ts.createSourceFile("ancestry.ts", source, ts.ScriptTarget.Latest, true);
  const node = tree.statements.find((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "withJavascriptDependencyProducedDirectoryV1");
  if (!node) throw Error("Actual dependency directory traversal required");
  const policy = JSON.parse(readFileSync(new URL("../../🧬️schema/📥️install.json", import.meta.url), "utf8"));
  return new Function("installSchema", ts.transpileModule(node.getText(tree) + ";return withJavascriptDependencyProducedDirectoryV1;", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText)(policy);
};

const directoryModel = (existing: number, fault: string | null) => {
  let created = 0, work = 0;
  const primary = Object.freeze({ reason: "original cancellation" });
  const records = new Map(Array.from({ length: existing }, (_, index) => [Array.from({ length: index + 1 }, (_, part) => "d" + part).join("/"), { kind: "directory", dev: 1n, ino: BigInt(index + 2) }]));
  const check = () => { if (++work === 3 && fault === "cancel") throw primary; };
  const owner = (parts: string[]): any => ({
    stat: () => { check(); return { dev: 1n }; },
    sync: async () => { check(); },
    entry: async (name: string) => { check(); const path = [...parts, name].join("/"); if (fault === "missing") return null; const row = records.get(path); return row ? (fault === "symlink" ? { ...row, kind: "link" } : fault === "device" ? { ...row, dev: 2n } : row) : null; },
    withDirectoryPath: async (path: readonly { name: string; identity: { ino: bigint } }[], body: (directory: unknown) => Promise<unknown>) => { let current = parts; for (const entry of path) { check(); current = [...current, entry.name]; if (fault === "identity" || records.get(current.join("/"))?.ino !== entry.identity.ino) throw Error("named directory identity changed"); } return body(owner(current)); },
    withNewDirectory: async (name: string, body: (directory: unknown) => Promise<unknown>) => { check(); created++; const path = [...parts, name]; records.set(path.join("/"), { kind: "directory", dev: 1n, ino: BigInt(records.size + 2) }); return body(owner(path)); },
    parts,
  });
  return { root: owner([]), primary, observation: () => ({ created }) };
};

test("dependency produced ancestry follows neutral creation cases", async () => {
  expect(new Ajv({ strict: true }).validate(schema, vectors)).toBe(true);
  const run = extracted();
  for (const row of vectors.cases) {
    const model = directoryModel(row.existing, null);
    const result = await run(model.root, row.parts, row.create, async directory => (directory as { parts: string[] }).parts);
    expect(result, row.id).toEqual(row.parts);
    expect(model.observation(), row.id).toEqual(row.expected);
  }
});

test("dependency immutable ancestry retains hostile identity and cancellation refusal", async () => {
  const run = extracted();
  for (const row of vectors.refusals) {
    const model = directoryModel(4, row.fault);
    let returned = false, primary: unknown;
    try { await run(model.root, ["d0", "d1", "d2", "d3"], row.create, async () => { returned = true; }); } catch (error) { primary = error; }
    expect(returned, row.id).toBe(false);
    expect(primary, row.id).toBeDefined();
    if (row.fault === "cancel") expect(primary).toBe(model.primary);
  }
});

const captureFunctions = async (admissions: Map<string, number>[]) => {
  const api = await import("@semio-tech/framework-filesystem/observation/descriptor");
  const contract = await import("../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts");
  const { createHash } = await import("node:crypto");
  const { join } = await import("node:path");
  const policy = JSON.parse(readFileSync(new URL("../../🧬️schema/📥️install.json", import.meta.url), "utf8"));
  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("capture.ts", source, ts.ScriptTarget.Latest, true);
  const names = ["withJavascriptDependencyProducedDirectoryV1", "withJavascriptDependencyInputParentsV1", "captureJavascriptDependencyDescriptorInputsV1", "verifyJavascriptDependencyDescriptorGroupV1"];
  const nodes = tree.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && names.includes(node.name?.text ?? ""));
  const wrap = (directory: any, prefix: string[], observed: Map<string, number>): any => new Proxy(Object.create(null), { get(_target, key) { const target = directory;
    if (key === "withDirectoryPath") return async (parts: { name: string }[], body: (child: unknown) => Promise<unknown>) => {
      const path = [...prefix, ...parts.map(part => part.name)], name = path.join("/");
      observed.set(name, (observed.get(name) ?? 0) + 1); if (observed.get(name)! > 1) throw Error("Immutable directory parent reacquired: " + name);
      return target.withDirectoryPath(parts, (child: unknown) => body(wrap(child, path, observed)));
    };
    if (key === "withDirectory") return async (name: string, body: (child: unknown) => Promise<unknown>) => { const path = [...prefix, name], nameKey = path.join("/"); observed.set(nameKey, (observed.get(nameKey) ?? 0) + 1); if (observed.get(nameKey)! > 1) throw Error("Immutable directory ancestry reacquired: " + nameKey); return target.withDirectory(name, (child: unknown) => body(wrap(child, path, observed))); };
    const value = Reflect.get(target, key); return typeof value === "function" ? value.bind(target) : value;
  } });
  const ports = { Buffer, JSON, createHash, join, installSchema: policy, readNativeProtectedArtifactInputGroupV1: contract.readNativeProtectedArtifactInputGroupV1, withArtifactRelativeDirectoryV1: (path: string, check: () => void, body: (directory: unknown) => Promise<unknown>) => api.withArtifactRelativeDirectoryV1(path, check, actual => { const observed = new Map<string, number>(); admissions.push(observed); return body(wrap(actual, [], observed)); }) };
  const run = new Function(...Object.keys(ports), ts.transpileModule(nodes.map(node => node.getText(tree)).join("\n") + ";return {capture:captureJavascriptDependencyDescriptorInputsV1,verify:verifyJavascriptDependencyDescriptorGroupV1};", { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText)(...Object.values(ports));
  return { ...run, api, policy };
};

const nativeFixture = async (depth = vectors.aggregate.depth) => {
  const fs = await import("node:fs/promises"), { join } = await import("node:path"), { createHash } = await import("node:crypto");
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("Original ticket or process-test artifact root required");
  const root = await fs.mkdtemp(join(artifacts, "ic-")), config = vectors.aggregate, batches: any[] = [];
  for (let scope = 0; scope < config.roots; scope++) {
    const selected = "j/" + (scope ? "b" : "a").repeat(20), parent = Array.from({ length: depth }, (_, index) => "d" + index).join("/"), files: any[] = [];
    for (let index = 0; index <= config.siblings; index++) {
      const path = parent + "/f" + index + ".json", bytes = Buffer.alloc(index === config.siblings ? 0 : index === 0 ? config.multiChunkBytes : 13, index + 1);
      await fs.mkdir(join(root, selected, parent), { recursive: true }); await fs.writeFile(join(root, selected, path), bytes);
      files.push({ root: 0, path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex"), data: bytes.toString("base64") });
    }
    batches.push({ version: 1, purpose: "stage-fresh-immutable-artifact-inputs", roots: [selected], files });
  }
  return { root, group: { version: 1, purpose: "stage-immutable-artifact-input-group", batches }, fs, join };
};

test("dependency immutable capture and replay admit each unique parent once against SQLite", async () => {
  const { Database } = await import("bun:sqlite"), fixture = await nativeFixture(), admissions: Map<string, number>[] = [], run = await captureFunctions(admissions), baseline = run.api.artifactDescriptorCustodyV1(), db = new Database(":memory:");
  let work = 0;
  try {
    db.exec("create table input(path text primary key)");
    for (const batch of fixture.group.batches) for (const file of batch.files) db.query("insert into input values(?)").run(batch.roots[0] + "/" + file.path);
    const expected = db.query("with recursive p(prefix,rest) as (select char(0),path from input union all select case when prefix=char(0) then substr(rest,1,instr(rest,char(47))-1) else prefix||char(47)||substr(rest,1,instr(rest,char(47))-1) end,substr(rest,instr(rest,char(47))+1) from p where instr(rest,char(47))>0) select distinct prefix path from p where prefix<>char(0) order by prefix").all() as { path: string }[];
    const receipt = await run.capture(fixture.root, fixture.group, { publish: async () => {}, yieldContinuation: async () => {} }, () => { work++; });
    expect([...receipt.directories.keys()].sort()).toEqual(expected.map(row => row.path));
    expect([...receipt.files.keys()].sort()).toEqual((db.query("select path from input order by path").all() as { path: string }[]).map(row => row.path));
    expect(admissions).toHaveLength(2);
    for (const phase of admissions) { expect([...phase.keys()].sort()).toEqual(expected.map(row => row.path)); expect([...phase.values()].every(count => count === 1)).toBe(true); }
    expect(run.policy.$defs.SourceOwnerStoreProtocol.const.maximumWork).toBe(4194304);
    expect(work).toBeLessThan(run.policy.$defs.SourceOwnerStoreProtocol.const.maximumWork * 2);
    expect(run.api.artifactDescriptorCustodyV1()).toEqual(baseline);
  } finally { db.close(); await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }
});

test("dependency immutable capture preserves native hostile Source and cancellation fences", async () => {
  for (const mode of vectors.nativeRefusals) {
    const fixture = await nativeFixture(), run = await captureFunctions([]), baseline = run.api.artifactDescriptorCustodyV1(), first = fixture.group.batches[0]!, path = fixture.join(fixture.root, first.roots[0], first.files[0].path), parent = fixture.join(fixture.root, first.roots[0], "d0/d1/d2"), primary = Object.freeze({ reason: "original capture cancellation" });
    let failure: unknown, changed = false;
    const check = () => { if (changed && mode === "cancel") throw primary; };
    const control = { publish: async () => {}, yieldContinuation: async () => {
      if (changed || mode === "fresh-roster" || mode === "path-escape" || mode === "file-link" || mode === "parent-link") return;
      changed = true;
      if (mode === "file-hash") await fixture.fs.writeFile(path, Buffer.alloc(first.files[0].bytes, 7));
      if (mode === "file-replacement") { await fixture.fs.rename(path, path + ".old"); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); }
      if (mode === "parent-replacement") { await fixture.fs.rename(parent, parent + ".old"); await fixture.fs.mkdir(parent); await fixture.fs.writeFile(path, Buffer.from(first.files[0].data, "base64")); }
    } };
    try {
      if (mode === "path-escape") first.files[0].path = "../outside";
      if (mode === "file-link") { await fixture.fs.link(path, path + ".link"); }
      if (mode === "parent-link") { await fixture.fs.rename(parent, parent + ".old"); await fixture.fs.symlink(parent + ".old", parent, process.platform === "win32" ? "junction" : "dir"); }
      try {
        const receipt = await run.capture(fixture.root, fixture.group, control, check);
        if (mode === "fresh-roster") { await fixture.fs.writeFile(fixture.join(parent, "unexpected"), "refuse"); await run.verify(fixture.root, receipt, control, check, true); }
      } catch (error) { failure = error; }
      expect(failure, mode).toBeDefined();
      if (mode === "cancel") expect(failure).toBe(primary);
      expect(run.api.artifactDescriptorCustodyV1(), mode).toEqual(baseline);
    } finally { await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }
  }
});

test("dependency produced ancestry creates through retained actual parent custody", async () => {
  const fs = await import("node:fs/promises"), { join } = await import("node:path"), api = await import("@semio-tech/framework-filesystem/observation/descriptor"), glob = (await import("fast-glob")).default, artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("Original ticket or process-test artifact root required");
  for (const row of vectors.cases.filter(row => row.parts.length)) {
    const root = await fs.mkdtemp(join(artifacts, "ip-")), parts = row.parts, baseline = api.artifactDescriptorCustodyV1();
    try {
      if (row.existing) await fs.mkdir(join(root, ...parts.slice(0, row.existing)), { recursive: true });
      expect(join(root, ...parts).length).toBeLessThanOrEqual(256);
      const run = extracted();
      await api.withArtifactRelativeDirectoryV1(root, () => {}, directory => run(directory, parts, row.create, async child => { expect((child as { stat(): { dev: bigint } }).stat().dev).toBe(directory.stat().dev); }));
      expect(await glob(parts.join("/"), { cwd: root, onlyDirectories: true, followSymbolicLinks: false })).toEqual([parts.join("/")]);
      expect(api.artifactDescriptorCustodyV1(), row.id).toEqual(baseline);
    } finally { await fs.rm(root, { recursive: true, force: true }); }
  }
}, 30000);

test("dependency immutable non-fresh replay refuses hash and replacement drift after successful capture", async () => {
  for (const mode of ["hash", "replacement"] as const) {
    const fixture = await nativeFixture(), run = await captureFunctions([]), baseline = run.api.artifactDescriptorCustodyV1(), batch = fixture.group.batches[0]!, file = batch.files[0]!, path = fixture.join(fixture.root, batch.roots[0], file.path), control = { publish: async () => {}, yieldContinuation: async () => {} };
    try {
      const receipt = await run.capture(fixture.root, fixture.group, control, () => {});
      if (mode === "replacement") { await fixture.fs.rename(path, path + ".old"); await fixture.fs.writeFile(path, Buffer.from(file.data, "base64")); }
      else await fixture.fs.writeFile(path, Buffer.alloc(file.bytes, 3));
      let failure: unknown;
      try { await run.verify(fixture.root, receipt, control, () => {}); } catch (error) { failure = error; }
      expect(failure, mode).toBeDefined();
      expect(run.api.artifactDescriptorCustodyV1(), mode).toEqual(baseline);
    } finally { await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }
  }
}, 30000);

test("dependency immutable deep handoffs preserve the unchanged descriptor owner bound", async () => {
  const fixture = await nativeFixture(vectors.cases[0]!.parts.length), admissions: Map<string, number>[] = [], run = await captureFunctions(admissions), baseline = run.api.artifactDescriptorCustodyV1();
  let peak = baseline.active;
  try {
    const receipt = await run.capture(fixture.root, fixture.group, { publish: async () => {}, yieldContinuation: async () => {} }, () => { peak = Math.max(peak, run.api.artifactDescriptorCustodyV1().active); });
    expect(receipt.files.size).toBe(vectors.aggregate.roots * (vectors.aggregate.siblings + 1));
    for (const phase of admissions) expect([...phase.values()].every(count => count === 1)).toBe(true);
    expect(peak).toBeLessThanOrEqual(32);
    expect(run.api.artifactDescriptorCustodyV1()).toEqual(baseline);
  } finally { await fixture.fs.rm(fixture.root, { recursive: true, force: true }); }
}, 30000);
