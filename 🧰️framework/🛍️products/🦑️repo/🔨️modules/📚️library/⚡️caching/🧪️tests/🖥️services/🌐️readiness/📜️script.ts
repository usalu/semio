import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧾️ Verifies service generations and readiness using real loopback responses and schema validation. */
export async function testServiceReadiness(workspace: string, output: string): Promise<void> {
  const directory = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🧾️session");
  const api = await import(join(directory, "🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(directory, "🧫️cases.json"), "utf8")), schema = JSON.parse(readFileSync(join(directory, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new (createRequire(import.meta.url)("ajv").default)(); ajv.addSchema(schema);
  const root = mkdtempSync(join(output, "service-readiness-")), sessionRoot = join(root, "sessions");
  try {
    for (const pid of fixture.invalidPids) assert.throws(() => api.openServiceSession(sessionRoot, fixture.owner, pid), /pid/i);
    const first = api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[0]);
    assert.ok(ajv.validate(schema.$id, first));
    const current = api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[0]);
    assert.notEqual(current.id, first.id);
    api.closeServiceSession(sessionRoot, first);
    assert.deepEqual(api.readServiceSession(sessionRoot, fixture.owner, current.pid), current);
    assert.throws(() => api.readServiceSession(sessionRoot, "another", current.pid), /owner/i);
    assert.throws(() => api.publishServiceReady(sessionRoot, first, "http://127.0.0.1:8080/"), /generation/i);
    for (const url of fixture.invalidUrls) assert.throws(() => api.publishServiceReady(sessionRoot, current, url), /url/i);
    const app = join(root, "app"), controller = new AbortController(); mkdirSync(app);
    writeFileSync(join(app, "index.html"), "<!doctype html><title>Service fixture</title>");
    writeFileSync(join(app, "⚙️vite.config.ts"), 'export default { logLevel: "silent" };');
    const { serveVite } = await import(join(directory, "../🟦️.ts"));
    const server = serveVite({ root: app, config: join(app, "⚙️vite.config.ts"), host: "127.0.0.1", port: 0, signal: controller.signal, session: current, ready: url => api.publishServiceReady(sessionRoot, current, url) });
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
      const other = api.openServiceSession(sessionRoot, fixture.owner, fixture.pids[1]);
      api.publishServiceReady(sessionRoot, other, url);
      await assert.rejects(() => api.waitForServiceReady(sessionRoot, other, new AbortController().signal, 200), /ready|identity|timeout/i);
      api.closeServiceSession(sessionRoot, other);
    } finally { controller.abort(); await server; }
    writeFileSync(join(sessionRoot, String(current.pid), "unrelated"), "preserve");
    api.closeServiceSession(sessionRoot, current);
    assert.equal(readFileSync(join(sessionRoot, String(current.pid), "unrelated"), "utf8"), "preserve");
    assert.equal(existsSync(join(sessionRoot, String(current.pid), "session.json")), false);
    const hostile = join(root, "hostile", String(fixture.pids[0])); mkdirSync(hostile, { recursive: true }); writeFileSync(join(hostile, "unowned"), "preserve");
    assert.throws(() => api.openServiceSession(join(root, "hostile"), fixture.owner, fixture.pids[0]), /unowned/i);
    console.log("[DEBUG] Service generation ownership, schema validation, real HTTP identity, cancellation and selective cleanup PASS");
  } finally { rmSync(root, { recursive: true, force: true }); }
}
