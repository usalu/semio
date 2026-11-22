import { expect, test } from "bun:test";
import { createConnection } from "node:net";
import { admissionWouldWaitV1, startAdmissionBrokerV1 } from "../../🟦️.ts";
import vectors from "./🔣️.json" with { type: "json" };
import requests from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import Ajv from "ajv";

test("the probe decision follows the language-neutral vectors", () => {
  for (const row of vectors.cases) expect(admissionWouldWaitV1(row.state, row.waiting, row.candidate), row.name).toBe(row.wait);
});

test("the headroom reply is schema-first", () => {
  const check = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/definitions/HeadroomV1`)!;
  expect(check({ v: 1, op: "headroom", wait: true })).toBe(true);
  expect(check({ v: 1, op: "headroom", wait: "yes" })).toBe(false);
});

const exchange = (endpoint: string, message: object, keepOpen = false): Promise<{ row: Record<string, unknown>; close: () => void }> => new Promise((done, fail) => {
  const socket = createConnection(endpoint), timer = setTimeout(() => { socket.destroy(); fail(Error("broker silent")); }, 5000);
  let text = "";
  socket.on("error", fail);
  socket.on("connect", () => socket.write(JSON.stringify(message) + "\n"));
  socket.on("data", (chunk) => { text += chunk; const at = text.indexOf("\n"); if (at < 0) return; clearTimeout(timer); done({ row: JSON.parse(text.slice(0, at)), close: () => socket.destroy() }); if (!keepOpen) socket.destroy(); });
});

test("a live broker answers the probe without queueing the asker", async () => {
  const broker = await startAdmissionBrokerV1({ physicalBytes: 15 * 1024 ** 3, history: new Map() });
  try {
    broker.registerPass({ id: "wasm-release-00", profile: "release", target: "wasm32-wasip2" });
    const facts = { ...requests.requests.valid[0]!, op: "probe", crateTypes: ["cdylib"], target: "wasm32-wasip2" };
    expect((await exchange(broker.endpoint, facts)).row).toEqual({ v: 1, op: "headroom", wait: false });
    expect(broker.stats()).toMatchObject({ running: 0, waiting: 0 });
    const held = await exchange(broker.endpoint, { ...facts, op: "admit" }, true);
    expect(held.row).toEqual({ v: 1, op: "verdict", admitted: true });
    expect(broker.stats().running).toBe(1);
    const second = await exchange(broker.endpoint, { ...facts, crate: "semio_hub_other_component", op: "probe" });
    expect(second.row).toEqual({ v: 1, op: "headroom", wait: true });
    expect(broker.stats()).toMatchObject({ running: 1, waiting: 0 });
    held.close();
  } finally { await broker.close(); }
});
