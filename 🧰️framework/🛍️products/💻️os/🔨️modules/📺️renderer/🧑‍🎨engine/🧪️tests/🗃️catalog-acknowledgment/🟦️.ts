import Ajv from "ajv";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { LocalCatalog, LocalCatalogConfigMutation } from "@semio-tech/framework/config/mutations";
import { awaitLocalCatalogMutationV1, localCatalogMutationReflectedV1, type LocalCatalogAcknowledgmentV1 } from "../../🧱️elements/🏛️ShellHost/🗂️local-catalog/💾️acknowledgment/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

afterEach(() => vi.useRealTimers());
const document = fixture.document as LocalCatalog["documents"][number];
const mutation: LocalCatalogConfigMutation = { mutation: "admitLocalDocument", ...document };

describe("catalog durability", () => {
  it("matches the language-neutral corpus and the independent JSON-schema contains oracle", () => {
    for (const row of fixture.cases) {
      const change: LocalCatalogConfigMutation = row.mutation === "admit" ? mutation : { mutation: "retireLocalDocument", documentId: document.documentId };
      const documents = row.documents === "empty" ? [] : row.documents === "old" ? [{ ...document, name: "Old" }] : row.documents === "extra" ? [{ ...document, documentId: "other" }, document] : [document];
      const contains = { type: "object", required: row.mutation === "admit" ? Object.keys(document) : ["documentId"], properties: Object.fromEntries(Object.entries(row.mutation === "admit" ? document : { documentId: document.documentId }).map(([key, value]) => [key, { const: value }])) };
      const validate = new Ajv().compile({ type: "array", ...(row.mutation === "admit" ? { contains } : { not: { contains } }) });
      expect(validate(documents), row.name).toBe(row.reflected);
      expect(localCatalogMutationReflectedV1({ documents }, change), row.name).toBe(row.reflected);
    }
  });

  it("waits for disk readback after persisted status and ignores an older archive", async () => {
    let listener!: LocalCatalogAcknowledgmentV1;
    const log: string[] = [];
    let done = false;
    const result = awaitLocalCatalogMutationV1({ mutation, signal: new AbortController().signal, current: () => true, timeoutMs: 1000,
      subscribe: (next) => { listener = next; return () => { log.push("unsubscribe"); }; },
      submit: () => { log.push("submit"); }, probe: () => { log.push("probe"); },
    }).then((catalog) => { done = true; return catalog; });
    expect(log).toEqual(["submit", "probe"]);
    listener.status(true);
    listener.archive({ documents: [{ ...document, name: "Old" }] });
    await Promise.resolve();
    expect(done).toBe(false);
    listener.archive({ documents: [document] });
    expect(await result).toEqual({ documents: [document] });
    expect(log).toEqual(["submit", "probe", "probe", "unsubscribe"]);
  });

  for (const reason of ["abort", "retire", "timeout", "submit"] as const) it(`releases the waiter on ${reason}`, async () => {
    vi.useFakeTimers();
    let current = true, listener!: LocalCatalogAcknowledgmentV1;
    const controller = new AbortController(), unsubscribe = vi.fn();
    const result = awaitLocalCatalogMutationV1({ mutation, signal: controller.signal, current: () => current, timeoutMs: 20,
      subscribe: (next) => { listener = next; return unsubscribe; }, probe: () => undefined,
      submit: () => { if (reason === "submit") throw new Error("worker gone"); },
    });
    if (reason === "abort") controller.abort();
    if (reason === "retire") { current = false; listener.status(true); }
    if (reason === "timeout") await vi.advanceTimersByTimeAsync(20);
    expect(await result).toBeNull();
    expect(unsubscribe).toHaveBeenCalledTimes(1);
    expect(vi.getTimerCount()).toBe(0);
  });
});
