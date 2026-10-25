import { expect, spyOn, test } from "bun:test";
import { Database } from "bun:sqlite";
import contract from "../../🧬️schema/📈️progress.json";
import { OriginalAdmissionClosedV1, createScriptProcessEnvelope, firstFailureLanguagesV1, renderFirstFailureV1, withIssuedResourceControlV1, withScriptProcessEnvelope } from "../../🟦️.ts";
import fixture from "./🔣️.json";

test("the first failure is rendered in the selected languages and agrees with an independent SQLite substitution", () => {
  const database = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      const reason = row.reason ?? row.first, labels = contract.const.firstFailure.labels as Record<string, string>;
      const expected = row.languages.map(language => (database.query("select replace(replace(?1,'{first}',?2),'{masked}',?3) text").get(labels[language]!, reason, row.masked) as { text: string }).text).join("\n") + "\n";
      expect<unknown>([...firstFailureLanguagesV1(row.environment)]).toEqual(row.languages);
      expect(renderFirstFailureV1(new Error(row.first), new OriginalAdmissionClosedV1(row.masked), row.environment)).toBe(expected);
    }
  } finally { database.close(); }
});

test("a later admission refusal never masks the first failure of its closed owner", async () => {
  const first = "Fixture first failure", writes: string[] = [], spy = spyOn(process.stderr, "write").mockImplementation(((text: unknown) => { writes.push(String(text)); return true; }) as typeof process.stderr.write);
  try {
    const run = withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "first-failure-fixture", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
      await withIssuedResourceControlV1(invocation, "fixture:first", 0, async () => { throw new Error(first); }).catch(() => undefined);
      await withIssuedResourceControlV1(invocation, "fixture:second", 0, async () => undefined);
    });
    await expect(run).rejects.toThrow(first);
  } finally { spy.mockRestore(); }
  expect(writes.join("")).toContain(renderFirstFailureV1(new Error(first), new OriginalAdmissionClosedV1("Original process admission closed - first failure: " + first), process.env));
});

test("an accepted task that reports a retry as an outcome keeps its owner open, while a caught failure closes it for good", async () => {
  const envelope = () => createScriptProcessEnvelope({ version: 1, owner: "first-failure-fixture", maximumElapsedMilliseconds: 0 }, {}, Date.now());
  const outcome = await withScriptProcessEnvelope(envelope(), async invocation => {
    const pass = await withIssuedResourceControlV1(invocation, "fixture:pass", 0, async () => ({ retry: true as const }));
    return [pass, await withIssuedResourceControlV1(invocation, "fixture:retry", 0, async () => ({ value: 1 as const }))];
  });
  expect(outcome).toEqual([{ retry: true }, { value: 1 }]);
  const refused = spyOn(process.stderr, "write").mockImplementation((() => true) as typeof process.stderr.write);
  try {
    await expect(withScriptProcessEnvelope(envelope(), async invocation => {
      await withIssuedResourceControlV1(invocation, "fixture:pass", 0, async () => { throw new Error("Fixture retry as failure"); }).catch(() => undefined);
      return withIssuedResourceControlV1(invocation, "fixture:retry", 0, async () => 1);
    })).rejects.toThrow("Fixture retry as failure");
  } finally { refused.mockRestore(); }
});

test("a refusal by a closed owner names the failure that closed it and keeps it as its cause", async () => {
  const first = "Fixture closing failure", refused = spyOn(process.stderr, "write").mockImplementation((() => true) as typeof process.stderr.write);
  try {
    let seen: unknown;
    await expect(withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "first-failure-fixture", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
      await withIssuedResourceControlV1(invocation, "fixture:first", 0, async () => { throw new Error(first); }).catch(() => undefined);
      try { await withIssuedResourceControlV1(invocation, "fixture:second", 0, async () => undefined); } catch (error) { seen = error; throw error; }
    })).rejects.toThrow(first);
    expect(seen).toBeInstanceOf(OriginalAdmissionClosedV1);
    expect((seen as Error).message).toBe("Original process admission closed - first failure: " + first);
    expect(((seen as Error).cause as Error).message).toBe(first);
  } finally { refused.mockRestore(); }
});
