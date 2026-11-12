/** 🖊️ The TypeScript wrapper of the DWG export: the genuine AC1024 of the dependency-free Rust writer, reached through its wasm module (built on demand by `repo ./📜️script.ts wasm`), read back by ezdwg, an independent DWG reader. The Rust side of the same files is read back by acadrust and ezdwg-core in `🦀️.rs` and `../🖊️dwg-format/🦀️.rs`.
 * @see ./🥒️.feature — the scenarios */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Text } from "../../🧬️schema/🟦️.ts";
import type { ModelScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { configureDwg, DWG_WASM_URL, DwgUnavailableError, dwgAvailability, writeDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { ensureDwgWasm, modelScene } from "../🧰️support/🧰️.ts";
import { expectSameDrawing, readDwgBytes } from "../🧰️support/🖊️.ts";

const sheetScene = (): ModelScene => JSON.parse(readFileSync(new URL("../../🧫️fixtures/📑️sheet-model-scene/🔣️.json", import.meta.url), "utf8")) as ModelScene;
const loaded = async (): Promise<void> => configureDwg({ wasm: readFileSync(ensureDwgWasm()) });

describe("🖊️ writeDwg", () => {
  it("reports exactly which artifact is missing, in both languages, and never fakes a DWG", async () => {
    configureDwg({ wasm: new URL("file:///nonexistent/staffage_dwg.wasm") });
    const availability = await dwgAvailability();
    expect(availability.available).toBe(false);
    if (availability.available) return;
    expect(availability.missing).toContain("staffage_dwg.wasm");
    expect(availability.reason.en).toContain("could not be loaded");
    expect(availability.reason.en).toContain("repo ./📜️script.ts wasm");
    expect(availability.reason.de).toContain("konnte nicht geladen werden");
    expect(availability.reason.de).toContain("repo ./📜️script.ts wasm");
    const failure = await writeDwg(modelScene()).catch((error: unknown) => error);
    expect(failure).toBeInstanceOf(DwgUnavailableError);
    expect((failure as DwgUnavailableError).missing).toBe(availability.missing);
    expect(((failure as DwgUnavailableError).reason as Text).de).toBe(availability.reason.de);
    configureDwg({});
  });

  it("builds the module through its own target and encodes a genuine AC1024 that ezdwg reads back as the scene", async () => {
    expect(ensureDwgWasm()).toEqual(DWG_WASM_URL);
    await loaded();
    expect(await dwgAvailability()).toEqual({ available: true, via: "wasm" });
    const bytes = await writeDwg(modelScene());
    expect(new TextDecoder().decode(bytes.subarray(0, 6))).toBe("AC1024");
    expectSameDrawing(await readDwgBytes(bytes), modelScene());
    configureDwg({});
  }, 120_000);

  it("writes the multiview sheet scene with its view in the layer names and its note, and the same bytes on a rerun", async () => {
    await loaded();
    const scene = sheetScene();
    const bytes = await writeDwg(scene);
    expect(bytes).toEqual(await writeDwg(scene));
    const report = await readDwgBytes(bytes);
    expectSameDrawing(report, scene);
    for (const name of ["PLAN-STAFFAGE-BODY", "FRONT-STAFFAGE-BODY", "STAFFAGE-NOTE"]) expect(report.layers[name], name).toBeDefined();
    configureDwg({});
  }, 120_000);

  it("writes an empty scene, a scene of thousands of polylines, and refuses a scene that is not in metres", async () => {
    await loaded();
    expectSameDrawing(await readDwgBytes(await writeDwg({ ...modelScene(), layers: [] })), { ...modelScene(), layers: [] });
    const polylines = Array.from({ length: 2000 }, (_, index) => ({ closed: index % 2 === 0, points: Array.from({ length: 12 }, (__, step) => ({ x: index * 0.013 + step * 0.0071, y: step * step * 0.0003 - index * 0.002 })) }));
    const big: ModelScene = { ...modelScene(), layers: [{ name: "BIG", aci: 3, lineweight: 25, polylines, texts: [] }] };
    const bytes = await writeDwg(big);
    expect(bytes.length).toBeGreaterThan(0x7400 * 2);
    expectSameDrawing(await readDwgBytes(bytes), big);
    await expect(writeDwg({ ...modelScene(), units: "mm" } as unknown as ModelScene)).rejects.toThrow(/units must be "m"/u);
    await expect(writeDwg({ ...modelScene(), layers: [{ name: "A", aci: 7, lineweight: 25, polylines: [{ closed: false, points: [{ x: 0, y: 0 }] }], texts: [] }] })).rejects.toThrow(/at least two points/u);
    configureDwg({});
  }, 120_000);

  it("reports progress, honours cancellation before and during the encoding, and reaches a configured CLI runner", async () => {
    await loaded();
    const steps: { done: number; total: number; label: Text }[] = [];
    await writeDwg(modelScene(), { progress: (done, total, label) => steps.push({ done, total, label }) });
    expect(steps.map((step) => [step.done, step.total])).toEqual([[0, 1], [1, 1]]);
    expect(steps[0]!.label).toEqual({ en: "Encoding DWG", de: "DWG wird kodiert" });
    const before = new AbortController();
    before.abort();
    await expect(writeDwg(modelScene(), { signal: before.signal })).rejects.toThrow();
    const during = new AbortController();
    await expect(writeDwg(modelScene(), { signal: during.signal, progress: (done) => done === 0 && during.abort() })).rejects.toThrow();
    configureDwg({ cli: async () => new TextEncoder().encode("AC1024-from-cli") });
    expect((await dwgAvailability()).available).toBe(true);
    expect(new TextDecoder().decode(await writeDwg(modelScene()))).toBe("AC1024-from-cli");
    configureDwg({});
  }, 60_000);
});
