import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, dirname } from "node:path";
import { createNextRebuildAdmissionV1, nextRebuildPluginV1 } from "../🟦️.ts";
import { acquireCompilerCapacityLeaseV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { createIssuedResourceControlV1 } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type { ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { withBuildTestInvocationV1 } from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import vector from "./🧫️fixtures/🔣️.json" with { type: "json" };

const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!artifacts) throw Error("Development rebuild tests require their authored artifact directory");
mkdirSync(artifacts, { recursive: true });
let issued!: ScriptInvocation;
const control = (signal?: AbortSignal) => createIssuedResourceControlV1(issued, "next:rebuild", vector.maximumElapsedMilliseconds, signal);

test("overlapping development compilers share one claim and idle services release it", () => withBuildTestInvocationV1(async (o) => { issued = o;
  expect(new Ajv({ strict: true }).compile(schema)(vector)).toBe(true);
  const directory = mkdtempSync(join(artifacts, "a"));
  let sessions = 0, admitted = false;
  const gate = createNextRebuildAdmissionV1({ directory, environment: {}, run: work => { sessions++; return work(control()); } });
  await Promise.all(vector.participants.map(id => gate.begin(id)));
  const successor = acquireCompilerCapacityLeaseV1({ directory, control: control(), environment: {}, request:{kind:"exact",parallelism:1} }).then(lease => { admitted = true; return lease; });
  await Bun.sleep(30);
  expect(admitted).toBe(false);
  await gate.finish(vector.participants[0]!);
  expect(admitted).toBe(false);
  await gate.finish(vector.participants[1]!);
  const lease = await successor; lease.release();
  await gate.begin(vector.participants[0]!);
  await gate.finish(vector.participants[0]!);
  expect(sessions).toBe(vector.sessions);
}));

test("closing a queued development compiler cancels its own wait and preserves the caller", () => withBuildTestInvocationV1(async (o) => { issued = o;
  const directory = mkdtempSync(join(artifacts, "c")), original = new AbortController();
  const held = await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"}, directory, control: control(), environment: {} });
  const gate = createNextRebuildAdmissionV1({ directory, environment: {}, run: work => work(control(original.signal)) });
  const pending = gate.begin("queued");
  await Bun.sleep(30);
  await gate.finish("queued");
  await expect(pending).rejects.toThrow();
  expect(original.signal.aborted).toBe(false);
  held.release();
  await gate.begin("successor"); await gate.finish("successor");
}));

test("the installed Webpack compiler runs the same grouped lifecycle and releases idle capacity", () => withBuildTestInvocationV1(async (o) => { issued = o;
  const directory = mkdtempSync(join(artifacts, "w")), entry = join(directory, "entry.js");
  writeFileSync(entry, "export const answer = 42;");
  const require = createRequire(new URL("../../../📦️packages/🟦️typescript/package.json", import.meta.url)), sdk = require("next/dist/compiled/webpack/webpack");
  sdk.init();
  let sessions = 0;
  const gate = createNextRebuildAdmissionV1({ directory, environment: {}, run: work => { sessions++; return work(control()); } }), plugin = nextRebuildPluginV1(gate);
  const compiler = sdk.webpack.webpack(vector.participants.map(name => ({ name, mode: "development", entry, output: { path: join(directory, name) }, plugins: [plugin] })));
  try {
    const stats = await new Promise<any>((accept, reject) => compiler.run((error: unknown, result: any) => error ? reject(error) : accept(result)));
    expect(stats.hasErrors()).toBe(false);
    expect(sessions).toBe(1);
    const lease = await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"}, directory, control: control(), environment: {} }); lease.release();
  } finally { await new Promise<void>((accept, reject) => compiler.close((error: unknown) => error ? reject(error) : accept())); }
}), vector.maximumElapsedMilliseconds);
