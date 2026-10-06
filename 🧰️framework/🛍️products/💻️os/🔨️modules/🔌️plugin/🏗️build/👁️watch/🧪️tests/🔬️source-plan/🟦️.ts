import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { pathToFileURL } from "node:url";

import ts from "typescript";
import { nativeSourceWatchSelectedV1, parseNativeSourceWatchPlanV1, startNativeSourceWatchV1 } from "../../📋️plan/🟦️.ts";
import { minimatch } from "minimatch";

/** 👁️ Proves the real Cargo/Nx source authority, dependency fixtures and live cancellable watch. */
export async function testNativeSourceWatchPlanV1(workspace: string, artifacts: string): Promise<void> {
  const root = resolve(import.meta.dir, "../..");
  const source = join(root, "📋️plan/🟦️.ts");
  const program = ts.createProgram([source], { target: ts.ScriptTarget.ESNext, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, allowImportingTsExtensions: true, noEmit: true, skipLibCheck: true, types: ["node", "bun"] });
  const unit = program.getSourceFile(source)!;
  assert.deepEqual([...program.getSyntacticDiagnostics(unit), ...program.getSemanticDiagnostics(unit)].map(issue => ts.flattenDiagnosticMessageText(issue.messageText, "\n")), []);
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/🔣️.json"), "utf8"));
  mkdirSync(artifacts, { recursive: true });
  const temporary = mkdtempSync(join(artifacts, "source-watch-"));
  try {
    for (const [path, bytes] of Object.entries(fixture.files)) { mkdirSync(dirname(join(temporary, path)), { recursive: true }); writeFileSync(join(temporary, path), bytes as string); }
    const { nativeSourceWatchPlanV1 } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
    const plan = await nativeSourceWatchPlanV1("owners/assembly/package", temporary);
    const node = "const {nativeSourceWatchPlanV1:f}=await import(process.argv[1]);process.stdout.write(JSON.stringify(await f('owners/assembly/package',process.argv[2])));";
    const nodePlan = JSON.parse(execFileSync("node", ["--input-type=module", "-e", node, pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href, temporary], { encoding: "utf8" }));
    assert.deepEqual(nodePlan, plan);
    const cargo = JSON.parse(execFileSync("cargo", ["metadata", "--no-deps", "--format-version", "1", "--manifest-path", join(temporary, "Cargo.toml")], { cwd: temporary, encoding: "utf8" }));
    for (const owner of cargo.packages) for (const target of owner.targets) assert.ok(plan.files.includes(relative(temporary, resolve(target.src_path)).replaceAll("\\", "/")));
    const selected = (path: string) => plan.files.includes(path) || plan.includes.some((glob: string) => minimatch(path, glob, { dot: true })) && !plan.excludes.some((glob: string) => minimatch(path, glob, { dot: true }));
    for (const path of fixture.required) assert.ok(selected(path), path);
    for (const event of fixture.events) {
      assert.equal(selected(event.path), event.selected, event.path);
      assert.equal(nativeSourceWatchSelectedV1(parseNativeSourceWatchPlanV1(plan), event.path), selected(event.path));
    }
    const events: string[] = [], controller = new AbortController();
    const observer = await startNativeSourceWatchV1(temporary, plan, path => events.push(path), { signal: controller.signal });
    try {
      const wait = async (predicate: () => boolean): Promise<void> => {
        const deadline = Date.now() + 4000;
        while (!predicate()) { if (Date.now() > deadline) throw Error("Source watcher did not settle"); await new Promise(resolve => setTimeout(resolve, 20)); }
      };
      const path = fixture.required.find((p: string) => p.endsWith("bounds.json"));
      writeFileSync(join(temporary, path), '{"version":2}\n');
      await wait(() => events.includes(path));
      await new Promise(resolve => setTimeout(resolve, 50));
      const beforeRoot = events.length;
      writeFileSync(join(temporary, "Cargo.toml"), fixture.files["Cargo.toml"] + "\n");
      await wait(() => events.length > beforeRoot && events.includes("Cargo.toml"));
      const beforeDelete = events.length;
      rmSync(dirname(join(temporary, path)), { recursive: true });
      await wait(() => events.length > beforeDelete);
      mkdirSync(dirname(join(temporary, path)), { recursive: true });
      const beforeRestore = events.length;
      writeFileSync(join(temporary, path), '{"version":2}\n');
      await wait(() => events.length > beforeRestore);
      const count = events.length;
      writeFileSync(join(temporary, path), '{"version":2}\n');
      writeFileSync(join(temporary, "unrelated/entry.rs"), "pub const OTHER: i32 = 8;\n");
      await new Promise(resolve => setTimeout(resolve, 100));
      assert.equal(events.length, count, "identical and unrelated writes do not cause rebuilds");
      controller.abort(); await observer.close();
      writeFileSync(join(temporary, path), '{"version":3}\n');
      await new Promise(resolve => setTimeout(resolve, 50));
      assert.equal(events.length, count, "revoked observer is retired");
    } finally { controller.abort(); await observer.close(); }
    assert.throws(() => parseNativeSourceWatchPlanV1({ ...plan, includes: ["../outside"] }));
    assert.throws(() => parseNativeSourceWatchPlanV1({ ...plan, fallback: true }));
    console.log(`Native watch source plan: ${fixture.required.length} captured owner/dependency files and ${fixture.events.length} independent minimatch vectors passed`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}
