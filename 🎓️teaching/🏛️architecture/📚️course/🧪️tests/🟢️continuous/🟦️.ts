/** 🟢️ The continuous course targets (`dev`, `dev-site`, and the proctor's `dev` behind the dashboard compound) are ready for every build tool at one moment and in one declaration: the neutral readiness `metadata.semio.dashboard.ready` that the dashboard, the Nx wrapper and the Bazel adapter all read. The stack prints its ready line only after the address answered (an independent `node:net` connection and `fetch` agree), the readiness resolves to the port the stack listens on for every parameter choice (the choices are plain environment, so any tool and the dashboard express them alike), and malformed ports are refused. Ajv is the schema oracle.
 * @see ../../🧫️fixtures/🟢️continuous/🔣️.json — language-neutral vectors
 * @see ../../🧱️stack/🟦️.ts — the stack under test
 * @see ../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🟦️.ts — the neutral readiness */
import Ajv from "ajv";
import { createServer } from "node:http";
import { connect, createServer as createNetServer, type AddressInfo } from "node:net";
import { afterEach, describe, expect, it } from "vitest";
import contract from "../../🧫️fixtures/🟢️continuous/🔣️.json" with { type: "json" };
import contractSchema from "../../🧫️fixtures/🟢️continuous/🧬️schema/🔣️.json" with { type: "json" };
import projectSchema from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json" with { type: "json" };
import { ReadinessMatcherV1, readReadinessDeclarationV1, resolveReadinessV1 } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟢️readiness/🟦️.ts";
import { awaitAnnounced, httpAnswers, siteOrigin, stackFollowsSources } from "../../🧱️stack/🟦️.ts";
import { proctorOrigin } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

const site = { portEnv: "TEACHING_ARCHITECTURE_COURSE_PORT", defaultPort: "6061" };
const closing: (() => Promise<unknown>)[] = [];
afterEach(async () => { for (const close of closing.splice(0)) await close(); });

const connects = (port: number): Promise<boolean> => new Promise(accept => {
  const socket = connect({ host: "127.0.0.1", port });
  socket.once("connect", () => { socket.destroy(); accept(true); });
  socket.once("error", () => accept(false));
});

const freePort = async (): Promise<number> => {
  const probe = createNetServer();
  await new Promise<void>(accept => probe.listen(0, "127.0.0.1", accept));
  const port = (probe.address() as AddressInfo).port;
  await new Promise<void>(accept => probe.close(() => accept()));
  return port;
};

it("keeps the fixture and the neutral readiness declaration valid through Ajv", () => {
  const fixture = new Ajv({ strict: false }).compile(contractSchema);
  expect(fixture(contract), JSON.stringify(fixture.errors)).toBe(true);
  const declaration = new Ajv({ strict: false }).compile({ $defs: projectSchema.$defs, $ref: "#/$defs/ReadyDeclaration" });
  for (const [target, row] of Object.entries(contract.targets)) {
    expect(declaration(row.ready), `${target}: ${JSON.stringify(declaration.errors)}`).toBe(true);
    expect(readReadinessDeclarationV1({ semio: { dashboard: { ready: row.ready } } }, target)).toEqual(row.ready);
  }
  for (const row of contract.refused.filter(refused => "port" in refused.ready)) expect(declaration(row.ready), row.id).toBe(true);
});

describe("resolves the declared readiness to the port the stack listens on", () => {
  for (const row of contract.cases) {
    it(row.id, () => {
      for (const [target, port] of Object.entries(row.ports)) {
        const declared = contract.targets[target as keyof typeof contract.targets]!;
        expect(resolveReadinessV1(declared.ready, row.environment, target).port, target).toBe(port);
        if (declared.stack) expect(Number(new URL(siteOrigin(row.environment, site)).port), target).toBe(port);
      }
      expect(proctorOrigin(row.environment)).toBe(row.proctor);
      expect(stackFollowsSources(row.environment)).toBe(row.follows);
    });
  }
});

describe("refuses a port the neutral oracle refuses", () => {
  for (const row of contract.refused) {
    it(row.id, () => {
      const stated = (row.ready as { portEnv: string }).portEnv === undefined ? undefined : (row.environment as Record<string, string>)[(row.ready as { portEnv: string }).portEnv];
      const oracle = stated !== undefined && /^[0-9]{1,5}$/u.test(stated) && Number(stated) >= 1 && Number(stated) <= 65535;
      expect(oracle).toBe(false);
      expect(() => resolveReadinessV1(row.ready, row.environment, row.id)).toThrow();
    });
  }
});

describe("says the ready line only after the address answers", () => {
  for (const delay of contract.announcement.listenAfterMilliseconds) {
    it(`listening after ${delay} ms`, async () => {
      const port = await freePort();
      const origin = `http://127.0.0.1:${port}`;
      const server = createServer((_, answer) => answer.end("ok"));
      closing.push(() => new Promise<void>(accept => { server.close(() => accept()); server.closeAllConnections(); }));
      let listenedAt = Number.POSITIVE_INFINITY;
      setTimeout(() => server.listen(port, "127.0.0.1", () => { listenedAt = performance.now(); }), delay);
      const said: { line: string; at: number }[] = [];
      await awaitAnnounced({ label: `the site at ${origin}`, probe: () => httpAnswers(origin), pollMs: 20, progressMs: 60, say: line => said.push({ line, at: performance.now() }) });
      const ready = said.filter(row => row.line.includes(contract.announcement.line));
      expect(ready).toHaveLength(1);
      expect(said.at(-1)).toBe(ready[0]);
      expect(ready[0]!.at).toBeGreaterThanOrEqual(listenedAt);
      expect(await connects(port)).toBe(true);
      expect((await fetch(origin)).status).toBe(200);
      const matcher = new ReadinessMatcherV1({ port, path: "" });
      matcher.push(`${ready[0]!.line}\n`);
      expect(matcher.take()).toBe(origin);
      for (const row of said.filter(row => row !== ready[0])) expect(row.line).not.toContain(contract.announcement.line);
    });
  }

  it("never says it when the wait is cancelled", async () => {
    const port = await freePort();
    const origin = `http://127.0.0.1:${port}`;
    const control = new AbortController();
    const said: string[] = [];
    setTimeout(() => control.abort(), 60);
    await expect(awaitAnnounced({ label: `the site at ${origin}`, probe: () => httpAnswers(origin, 100), pollMs: 20, signal: control.signal, say: line => said.push(line) })).rejects.toThrow(/cancelled/u);
    expect(said.filter(line => line.includes(contract.announcement.line))).toEqual([]);
  });
});
