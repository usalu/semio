import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧾️ Verifies service generations and readiness using real loopback responses and schema validation. */
export async function testServiceReadiness(workspace: string, output: string,invocation:ScriptInvocation): Promise<void> {
 const vitePhase=scriptCompilerAdmissionControlV1(invocation,"test:vite-entry",0);
  const directory = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session");
  const api = await import(join(directory, "🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8")), schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new (createRequire(import.meta.url)("ajv").default)(); ajv.addSchema(schema);
  const root = mkdtempSync(join(output, "service-readiness-")), sessionRoot = join(root, "sessions");
  try {
    assert.ok(ajv.validate({ $ref: schema.$id + "#/$defs/MutationLease" }, fixture.mutationLease));
    const { acquireResourceLease } = await import(join(directory, "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"));
    for (const mutation of fixture.mutations) {
      const mutations = join(root, "mutations", mutation), session = await api.openServiceSession(mutations, fixture.owner, fixture.pids[0]);
      const lease = await acquireResourceLease({ directory: join(mutations, fixture.mutationLease.directory), resource: fixture.mutationLease.resource, mode: "exclusive", signal: new AbortController().signal });
      let settled = false;
      const pending = Promise.resolve(mutation === "open" ? api.openServiceSession(mutations, fixture.owner, session.pid) : mutation === "publish" ? api.publishServiceReady(mutations, session, "http://127.0.0.1:8080/") : api.closeServiceSession(mutations, session)).finally(() => { settled = true; });
      try { await Bun.sleep(80); assert.equal(settled, false, `${mutation} must respect the session mutation lease`); assert.deepEqual(api.readServiceSession(mutations, fixture.owner, session.pid), session); }
      finally { lease.release(); await pending; }
      if (mutation === "open") assert.notEqual(api.readServiceSession(mutations, fixture.owner, session.pid).id, session.id);
      else if (mutation === "publish") assert.equal(JSON.parse(readFileSync(join(mutations, String(session.pid), "ready.json"), "utf8")).session.id, session.id);
      else assert.equal(existsSync(join(mutations, String(session.pid), "session.json")), false);
    }
    assert.ok(ajv.validate({ $ref: schema.$id + "#/$defs/ReadinessRace" }, fixture.readinessRace));
    const racingRoot = join(root, "readiness-race"), stale = await api.openServiceSession(racingRoot, fixture.owner, fixture.pids[0]);
    let request!: () => void, respond!: () => void;
    const requested = new Promise<void>(resolve => { request = resolve; }), response = new Promise<void>(resolve => { respond = resolve; });
    const oracle = Bun.serve({ hostname: "127.0.0.1", port: 0, fetch: async () => { request(); await response; return Response.json(stale); } });
    try {
      await api.publishServiceReady(racingRoot, stale, `http://127.0.0.1:${oracle.port}/`);
      const waiting = api.waitForServiceReady(racingRoot, stale, new AbortController().signal, 5000);
      await requested;
      await api.openServiceSession(racingRoot, fixture.owner, stale.pid);
      respond();
      await assert.rejects(() => waiting, /generation/i);
    } finally { respond(); oracle.stop(true); }
    for (const pid of fixture.invalidPids) await assert.rejects(() => api.openServiceSession(sessionRoot, fixture.owner, pid), /pid/i);
    const first = await api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[0]);
    assert.ok(ajv.validate(schema.$id, first));
    const current = await api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[0]);
    assert.notEqual(current.id, first.id);
    await api.closeServiceSession(sessionRoot, first);
    assert.deepEqual(api.readServiceSession(sessionRoot, fixture.owner, current.pid), current);
    assert.throws(() => api.readServiceSession(sessionRoot, "another", current.pid), /owner/i);
    await assert.rejects(() => api.publishServiceReady(sessionRoot, first, "http://127.0.0.1:8080/"), /generation/i);
    for (const url of fixture.invalidUrls) await assert.rejects(() => api.publishServiceReady(sessionRoot, current, url), /url/i);
    const app = join(root, "app"), controller = new AbortController(); mkdirSync(app);
    writeFileSync(join(app, "index.html"), "<!doctype html><title>Service fixture</title>");
    writeFileSync(join(app, "⚙️vite.config.ts"), 'export default { logLevel: "silent" };');
    const { serveVite } = await import(join(directory, "../🟦️.ts"));
    const server = serveVite({control:vitePhase, invocation,root: app, config: join(app, "⚙️vite.config.ts"), host: "127.0.0.1", port: 0, signal: controller.signal, session: current, ready: (url: string) => api.publishServiceReady(sessionRoot, current, url) });
    try {
      const url = await Promise.race([api.waitForServiceReady(sessionRoot, current, new AbortController().signal, 10000), server.then(() => { throw new Error("Service stopped before readiness"); })]);
      const health = await fetch(new URL(fixture.endpoint, url));
      assert.deepEqual(await health.json(), current);
      assert.equal(health.headers.get("cache-control"), "no-store");
      const ready = JSON.parse(readFileSync(join(sessionRoot, String(current.pid), "ready.json"), "utf8"));
      assert.ok(ajv.validate({ $ref: schema.$id + "#/$defs/Ready" }, ready));
      assert.equal(await api.waitForServiceReady(sessionRoot, current, new AbortController().signal, 2000), url);
      const abort = new AbortController(); abort.abort();
      await assert.rejects(() => api.waitForServiceReady(sessionRoot, current, abort.signal, 100), /abort/i);
      const other = await api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[1]);
      await api.publishServiceReady(sessionRoot, other, url);
      await assert.rejects(() => api.waitForServiceReady(sessionRoot, other, new AbortController().signal, 200), /ready|identity|timeout/i);
      await api.closeServiceSession(sessionRoot, other);
    } finally { controller.abort(); await server; }
    writeFileSync(join(sessionRoot, String(current.pid), "unrelated"), "preserve");
    await api.closeServiceSession(sessionRoot, current);
    assert.equal(readFileSync(join(sessionRoot, String(current.pid), "unrelated"), "utf8"), "preserve");
    assert.equal(existsSync(join(sessionRoot, String(current.pid), "session.json")), false);
    const hostile = join(root, "hostile", String(fixture.pids[0])); mkdirSync(hostile, { recursive: true }); writeFileSync(join(hostile, "unowned"), "preserve");
    await assert.rejects(() => api.openServiceSession(join(root, "hostile"), fixture.owner, fixture.pids[0]), /unowned/i);
    const budgetSchema = { $ref: schema.$id + "#/$defs/ReadinessBudget" }, budgets: any[] = [];
    for (const row of fixture.readiness.products) {
      const budget = JSON.parse(readFileSync(join(workspace, row.budget), "utf8")); budgets.push(budget);
      assert.ok(ajv.validate(budgetSchema, budget), `${row.budget}: ${ajv.errorsText()}`);
      assert.deepEqual(api.parseServiceReadinessBudgetV1(budget), budget);
      assert.ok(budget.readyTimeoutMs >= budget.evidence.coldBootMs.maximum * budget.evidence.marginFactor, row.budget);
      assert.ok(budget.readyTimeoutMs >= row.minimumTimeoutMs, row.budget);
    }
    for (const row of fixture.readiness.invalid) {
      const value = structuredClone(budgets[0]); let target = value;
      for (const key of row.path.slice(0, -1)) target = target[key];
      if (row.remove) delete target[row.path.at(-1)]; else target[row.path.at(-1)] = row.value;
      assert.equal(ajv.validate(budgetSchema, value), row.schema, row.reason);
      assert.throws(() => api.parseServiceReadinessBudgetV1(value), /readiness budget/i, row.reason);
    }
    const waiting = join(root, "progress"), idle = await api.openServiceSession(waiting, fixture.owner, fixture.pids[0]), progress = fixture.readiness.progress, events: any[] = [];
    await assert.rejects(() => api.waitForServiceReady(waiting, idle, new AbortController().signal, progress.timeoutMs, { intervalMs: progress.intervalMs, report: (event: any) => events.push(event) }), new RegExp(`within ${progress.timeoutMs} ms`));
    assert.ok(events.length >= progress.minimumEvents, `progress events: ${events.length}`);
    assert.ok(events[0].elapsedMs < progress.intervalMs);
    assert.ok(events.every((event, index) => event.timeoutMs === progress.timeoutMs && event.phase === "awaiting-ready-record" && (index === 0 || event.elapsedMs >= events[index - 1].elapsedMs + progress.intervalMs - 5)));
    await assert.rejects(() => api.waitForServiceReady(waiting, idle, new AbortController().signal, 0), /declared positive timeout/i);
    for (const row of progress.lines) for (const language of ["en", "de"]) assert.equal(api.serviceReadyProgressLine(row.owner, row.progress, language), row[language]);
    for (const row of progress.languages) assert.equal(api.serviceReadyProgressLanguage(row.environment), row.language, JSON.stringify(row.environment));
  } finally { rmSync(root, { recursive: true, force: true }); }
}
