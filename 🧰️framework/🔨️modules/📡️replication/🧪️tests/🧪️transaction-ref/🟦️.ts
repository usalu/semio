import Ajv from "ajv";

type TestSource = { readonly directory: string; readonly url: string };

/** 🧾️ Pins the TypeScript `mintTransactionRef` to the language-agnostic mint vectors the Rust mint and the third-party
 * `blake3` crate reproduce (`🎮️mutation/🧫️fixtures/🧫️transaction-ref`), validated by Ajv, and checks that a transaction
 * rides the TypeScript envelope codec. */
export async function registerTransactionRefTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "mintTransactionRef" | "writeVecEnvelope" | "readVecEnvelope">, source: TestSource): Promise<void> {
  const { mintTransactionRef, writeVecEnvelope, readVecEnvelope } = dependencies;
  const { describe, expect, it } = vitest;

  type Json = Readonly<Record<string, any>>;

  async function load(): Promise<Readonly<{ fixture: Json; schema: object }>> {
    const { readFile } = await import("node:fs/promises");
    const { dirname, join } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const root = dirname(fileURLToPath(source.url));
    const [fixture, schema] = await Promise.all([readFile(join(root, "🎮️mutation/🧫️fixtures/🧫️transaction-ref/🔣️.json"), "utf8"), readFile(join(root, "🎮️mutation/🧬️schema/🔣️transaction-ref/🔣️.json"), "utf8")]);
    return { fixture: JSON.parse(fixture) as Json, schema: JSON.parse(schema) as object };
  }

  describe("transaction ref", () => {
    it("mints the language-agnostic vectors", async () => {
      const { fixture, schema } = await load();
      const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
      expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
      for (const row of fixture.cases as Json[]) expect(mintTransactionRef(row.actor, row.hlc, row.tool), row.id).toEqual(row.expect);
    });

    it("rides the envelope codec and refuses an unknown flag", () => {
      const envelope = {
        mutation_id: "op-1",
        document_id: "doc-1",
        actor: "alice",
        dependencies: [],
        observed: null,
        target: ["nodes", "n-1"],
        diff: { schema: "demo/v1", payload: [1, 2] },
        inverse: { schema: "demo/v1", payload: [] },
        timestamp: { actor: 1, physical_ms: 2, logical: 3 },
        transaction: mintTransactionRef("alice", { actor: 1, physical_ms: 2, logical: 3 }, "app#select"),
        verb: null as string | null,
      };
      for (const value of [envelope, { ...envelope, transaction: null }, { ...envelope, verb: "select" }, { ...envelope, transaction: null, verb: "select" }]) {
        const out: number[] = [];
        writeVecEnvelope(out, [value]);
        expect(readVecEnvelope(new Uint8Array(out), [0])).toEqual([value]);
      }
      const out: number[] = [];
      writeVecEnvelope(out, [{ ...envelope, transaction: null }]);
      out[out.length - 1] = 4;
      expect(() => readVecEnvelope(new Uint8Array(out), [0])).toThrow("trailing flags 4");
    });
  });
}
