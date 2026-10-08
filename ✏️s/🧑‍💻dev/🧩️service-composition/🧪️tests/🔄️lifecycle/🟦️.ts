import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import Ajv from "ajv";
import { concat, map, reverse, take, takeWhile } from "lodash";

/** 🧪 Matches caller-owned composition lifecycles against portable vectors and Ajv. */
export async function sDevCompositionLawsV1(): Promise<void> {
  assert(!existsSync(new URL("../../🧬️schema/🔣️.json", import.meta.url)), "service-composition examples must not own a corpus schema");
  const require = createRequire(import.meta.url), lock = readFileSync(new URL("../../../../../bun.lock", import.meta.url), "utf8");
  for (const name of ["ajv", "lodash"]) {
    const { version } = JSON.parse(readFileSync(require.resolve(`${name}/package.json`), "utf8"));
    assert(lock.includes(`"${name}": ["${name}@${version}",`), `${name} must match the actual lockfile`);
  }
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../../🚀️entry/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const { mountSDevInventoryV1 } = await import("../../🟦️.ts");
  for (const row of fixture.cases) {
    const events: string[] = [], progress: string[] = [], controller = new AbortController();
    if (row.cancel === "before") controller.abort(new Error("test cancellation"));
    let mounted: Awaited<ReturnType<typeof mountSDevInventoryV1>> | undefined;
    try {
      mounted = await mountSDevInventoryV1(row.installed, async (owner: string) => {
        events.push(`mount:${owner}`);
        if (row.fail === owner) throw new Error("test mount failure");
        if (row.cancel === "mount") controller.abort(new Error("test cancellation"));
        await Promise.resolve();
        return { dispose: () => { events.push(`dispose:${owner}`); } };
      }, { signal: controller.signal, progress: event => progress.push(`${event.stage}:${event.completed}/${event.total}`) });
      mounted.dispose();
      mounted.dispose();
      controller.abort();
      assert.equal(progress[0], `mount:0/${row.installed.length}`);
      assert(progress.includes(`ready:${row.installed.length}/${row.installed.length}`));
    } catch (error) {
      assert(row.cancel !== "none" || row.fail !== null, `${row.name}: unexpected refusal ${String(error)}`);
      assert.equal(mounted, undefined);
    }
    assert.deepEqual(events, row.expected, row.name);
    const candidates = row.cancel === "before" ? [] : row.cancel === "mount" ? take(row.installed, 1) : row.installed;
    const acquired = takeWhile(candidates, owner => owner !== row.fail);
    const attempted = take(candidates, acquired.length + (acquired.length < candidates.length ? 1 : 0));
    const oracle = concat(map(attempted, owner => `mount:${owner}`), map(reverse(acquired), owner => `dispose:${owner}`));
    assert.deepEqual(events, oracle, `${row.name}: independent locked Lodash retirement`);
    console.log(`[DEBUG] s-dev-composition ${row.name}: ${JSON.stringify(events)}`);
  }
  const { bootSDevV1 } = await import("../../../🚀️entry/🟦️.ts");
  const catalogSchema = JSON.parse(readFileSync(new URL("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const admit = new Ajv({ strict: false }).addSchema(catalogSchema).compile(schema);
  for (const row of fixture.admission) {
    assert.equal(admit(row.value), row.accepted, row.name);
    let calls = 0;
    const boot = bootSDevV1(row.value, { mount: async received => { assert.equal(received, row.value); calls++; return { dispose() {} }; } }, { signal: new AbortController().signal, progress: () => {} });
    if (row.accepted) (await boot).dispose();
    else await assert.rejects(boot, /s-dev.missing-installed-inventory/u);
    assert.equal(calls, row.accepted ? 1 : 0);
    console.log(`[DEBUG] s-dev-admission ${row.name}: accepted=${row.accepted} host=${calls}`);
  }
  const empty = { catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] }, plugins: [], documentServices: [], surfaceSessionFactories: [] };
  const browser = new AbortController();
  let received: unknown, retired = 0;
  const mounted = await bootSDevV1(empty, { mount: async inventory => { received = inventory; return { dispose: () => { retired++; } }; } }, { signal: browser.signal, progress: () => {} });
  assert.equal(received, empty);
  browser.abort();
  mounted.dispose();
  assert.equal(retired, 1);
  const { installSDevWorkersV1 } = await import("../../👷️worker/🟦️.ts");
  let installs = 0;
  const workers = await installSDevWorkersV1([], { install: async () => { installs++; return { dispose() {} }; } }, { signal: new AbortController().signal, progress: () => {} });
  workers.dispose();
  assert.equal(installs, 0);
  console.log(`[DEBUG] s-dev-browser empty caller inventory received; abort/dispose retired=${retired}`);
  console.log(`s-dev-composition: ${fixture.cases.length} complete portable lifecycle vectors passed`);
}
