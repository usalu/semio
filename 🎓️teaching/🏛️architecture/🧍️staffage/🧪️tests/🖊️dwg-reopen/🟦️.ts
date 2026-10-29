/** 🖊️ The TypeScript wrapper of the DWG export: genuine AC1024 through the encoder when present, an explicit `unavailable` otherwise. */
import { existsSync, readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { configureDwg, DWG_WASM_URL, DwgUnavailableError, dwgAvailability, writeDwg } from "../../🔨️modules/📤️export/🖊️dwg/🟦️.ts";
import { modelScene } from "../🧰️support/🧰️.ts";

describe("🖊️ writeDwg", () => {
  it("reports exactly which artifact is missing and never fakes a DWG", async () => {
    configureDwg({ wasm: new URL("file:///nonexistent/staffage_dwg.wasm") });
    const availability = await dwgAvailability();
    expect(availability.available).toBe(false);
    if (!availability.available) expect(availability.missing).toContain("staffage_dwg.wasm");
    await expect(writeDwg(modelScene())).rejects.toBeInstanceOf(DwgUnavailableError);
  });

  it.runIf(existsSync(new URL(DWG_WASM_URL)))("encodes through the wasm module into AC1024", async () => {
    configureDwg({ wasm: readFileSync(DWG_WASM_URL) });
    const bytes = await writeDwg(modelScene());
    expect(new TextDecoder().decode(bytes.subarray(0, 6))).toBe("AC1024");
    configureDwg({});
  });

  it("uses a CLI runner when one is configured and honours cancellation", async () => {
    configureDwg({ cli: async () => new TextEncoder().encode("AC1024-from-cli") });
    expect((await dwgAvailability()).available).toBe(true);
    expect(new TextDecoder().decode(await writeDwg(modelScene()))).toBe("AC1024-from-cli");
    const controller = new AbortController();
    controller.abort();
    await expect(writeDwg(modelScene(), { signal: controller.signal })).rejects.toThrow();
    configureDwg({});
  });
});
