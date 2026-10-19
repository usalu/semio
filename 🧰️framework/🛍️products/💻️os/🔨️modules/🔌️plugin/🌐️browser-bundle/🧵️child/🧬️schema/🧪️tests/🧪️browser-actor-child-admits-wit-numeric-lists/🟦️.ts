/** 🔢️ The child boundary admits exactly the typed arrays jco lifts WIT numeric lists into — a `set-children` patch op's
 * `list<node-id>` arrives as a `BigUint64Array` — each as one exclusive transferable buffer, and refuses every other view
 * by name. Neutral rows: the containment corpus' `valueAdmission`. */
import * as ModuleUnderTest from "@semio-tech/framework/browser-bundle/child/schema";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

type TestSource = { readonly directory: string; readonly url: string };
type Row = { readonly name: string; readonly lift: string; readonly length: number; readonly offsetElements?: number; readonly repeat?: true; readonly expected: string };

export async function registerTests2(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("@semio-tech/framework/browser-bundle/child/schema"), "measureChildValue">, source: TestSource): Promise<void> {
  const { measureChildValue } = dependencies;
  const { expect, it } = vitest;
  it("browser actor child admits WIT numeric lists as exclusive buffers and refuses other views by name", async () => {
    const { readFileSync } = await import("node:fs");
    const fixture = JSON.parse(readFileSync(new URL("../🧪️tests/🧫️fixtures/🔣️.json", source.url), "utf8"));
    const lifts = { Uint8Array, Int8Array, Uint16Array, Int16Array, Uint32Array, Int32Array, BigUint64Array, BigInt64Array, Float32Array, Float64Array, DataView, Uint8ClampedArray } as unknown as Record<string, new (buffer: ArrayBuffer, offset?: number, count?: number) => ArrayBufferView>;
    for (const row of fixture.valueAdmission as readonly Row[]) {
      const Lift = lifts[row.lift]!;
      const width = "BYTES_PER_ELEMENT" in Lift ? (Lift as unknown as { BYTES_PER_ELEMENT: number }).BYTES_PER_ELEMENT : 1;
      const buffer = new ArrayBuffer((row.length + (row.offsetElements ?? 0)) * width);
      const view = row.offsetElements === undefined ? new Lift(buffer) : new Lift(buffer, row.offsetElements * width, row.length);
      const value = [{ tag: "set-children", val: { node: 7n, children: view } }, ...(row.repeat ? [{ tag: "set-children", val: { node: 8n, children: new Lift(buffer) } }] : [])];
      const outcome = (() => {
        try {
          const measured = measureChildValue(value, 1_048_576);
          expect(measured.transfers, row.name).toEqual([buffer]);
          return "admitted";
        } catch (error) {
          return (error as Error).message;
        }
      })();
      expect(outcome, row.name).toBe(row.expected);
    }
  });
}

const moduleUnderTestUrl = pathToFileURL(resolve(import.meta.dirname, "../../🟦️.ts")).href;
const vitestApi = await import("vitest");
await registerTests2(vitestApi, { measureChildValue: ModuleUnderTest.measureChildValue }, { directory: (await import("node:url")).fileURLToPath(new URL(".", moduleUnderTestUrl)), url: moduleUnderTestUrl });
