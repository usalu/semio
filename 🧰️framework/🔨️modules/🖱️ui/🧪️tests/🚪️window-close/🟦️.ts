/** 🚪️ Language-neutral admission cases checked through production and an independent schema oracle. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { createWindowCloseAdmissionV1 } from "../../🧱️elements/🎨️Canvas/🚪️close-admission/🟦️.ts";

const root = new URL("../🧫️fixtures/🚪️window-close/", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("🔣️.json", root), "utf8")) as { cases: { name: string; result: string; expected: boolean }[] };
const schema = JSON.parse(readFileSync(new URL("🧬️.json", root), "utf8"));

describe("window close admission", () => {
  test("validates neutral cases through Ajv", () => {
    expect(new Ajv().validate(schema, fixture)).toBe(true);
  });
  for (const row of fixture.cases) test(row.name, async () => {
    const closed: string[] = [], errors: unknown[] = [];
    const admission = createWindowCloseAdmissionV1({
      request: () => {
        if (row.result === "throw") throw new Error("refused");
        if (row.result === "reject") return Promise.reject(new Error("refused"));
        if (row.result === "async-true") return Promise.resolve(true);
        if (row.result === "async-false") return Promise.resolve(false);
        return row.result === "void" ? undefined : row.result === "true";
      },
      commit: id => { closed.push(id); },
      report: error => { errors.push(error); },
    });
    await admission.close("a");
    const oracle = new Ajv().compile({ enum: ["true", "void", "async-true"] });
    expect(oracle(row.result)).toBe(row.expected);
    expect(closed).toEqual(row.expected ? ["a"] : []);
    expect(errors.length).toBe(["throw", "reject"].includes(row.result) ? 1 : 0);
  });
  test("deduplicates one pending window while allowing another and permits retry after keep", async () => {
    let settle!: (value: boolean) => void;
    const closed: string[] = [], requested: string[] = [];
    const admission = createWindowCloseAdmissionV1({
      request: id => { requested.push(id); return id === "a" ? new Promise<boolean>(resolve => { settle = resolve; }) : true; },
      commit: id => { closed.push(id); }, report: () => undefined,
    });
    const first = admission.close("a"), duplicate = admission.close("a");
    expect(duplicate).toBe(first);
    await admission.close("b");
    expect(closed).toEqual(["b"]);
    expect(requested).toEqual(["a", "b"]);
    settle(false);
    await first;
    const retry = admission.close("a");
    await Promise.resolve();
    settle(true);
    await retry;
    expect(closed).toEqual(["b", "a"]);
  });
  test("retirement prevents pending admission from committing", async () => {
    let settle!: (value: boolean) => void;
    const closed: string[] = [];
    const admission = createWindowCloseAdmissionV1({ request: () => new Promise<boolean>(resolve => { settle = resolve; }), commit: id => { closed.push(id); }, report: () => undefined });
    const closing = admission.close("a");
    await Promise.resolve();
    admission.dispose();
    settle(true);
    await closing;
    await admission.close("b");
    expect(closed).toEqual([]);
  });
});
