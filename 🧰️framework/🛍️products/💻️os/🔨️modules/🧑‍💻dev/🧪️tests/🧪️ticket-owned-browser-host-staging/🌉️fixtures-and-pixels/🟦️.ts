/** 🌉️ The scale fixture artifacts, the linked session engines and the owned parity pixel comparison of the ticket-owned browser host staging suite. */
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerFixtureAndPixelTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { PLAYWRIGHT_MODULE_SPECIFIER, checkScaleFixtureArtifacts, compareOwnedParityPixels, cropOwnedParityRgba, decodeParityScreenshot, dirname, encodeParityDiff, ensureParityPlaywrightBrowsersPath, fileURLToPath, join, linkedSessionEngines, readFileSync, renderScaleFixtureArtifacts, repoRoot, scaleFixtureGeneratedDir } = dependencies;
  const { describe, expect, it } = vitest;
  const { devContractModule, itLong } = createTestKit(vitest, dependencies, source);

  type OwnedParityImage = import("../../../⚖️parity/🖼️pixels/🟦️.ts").OwnedParityImage;

  //#region 🌉️LinkedSessionEnginesTests
  describe("scaleFixtureArtifacts", () => {
    itLong("compares both committed semantic outputs without rewriting them", async () => {
      const { execFileSync } = await import("node:child_process");
      const directory = scaleFixtureGeneratedDir(repoRoot);
      const registryJson = readFileSync(join(directory, "📇️registry/🔣️.json"), "utf8");
      const catalogJson = readFileSync(join(directory, "🗂️catalog/🔣️.json"), "utf8");
      const registry = JSON.parse(registryJson);
      const rendered = renderScaleFixtureArtifacts(registry.pluginCount, registry.extensionsPerPlugin, registry.seed);
      expect(rendered.registryJson).toBe(registryJson);
      expect(rendered.catalogJson).toBe(catalogJson);
      expect(checkScaleFixtureArtifacts(repoRoot)).toBe(true);
      const oracle = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", 'import { readFileSync } from "node:fs"; const { registry, catalog } = JSON.parse(readFileSync(0, "utf8")); console.log(JSON.stringify({ count: registry.records.length, plugins: registry.records.filter(row => row.kind === "plugin").map(row => row.id), catalogPlugins: catalog.plugins.map(row => row.pluginId) }));'], { input: JSON.stringify({ registry, catalog: JSON.parse(catalogJson) }), encoding: "utf8" }));
      expect(oracle.count).toBe(2550);
      expect(oracle.plugins).toEqual(oracle.catalogPlugins);
      expect(oracle.plugins).toHaveLength(50);
    });
  });

  describe("linkedSessionEngines", () => {
    itLong("matches strict schema validation and deduplicates only exact owner engine paths", async () => {
      const { default: Ajv } = await import("ajv");
      const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../🧪️tests/🧫️fixtures/🔗️linked-session-engines.json"), "utf8"));
      const validate = await devContractModule("LinkedSessionEnginesV1");
      for (const vector of fixture.valid) {
        expect(validate(vector.declarations)).toBe(true);
        expect(linkedSessionEngines(vector.declarations)).toEqual(vector.engines);
      }
      for (const declarations of fixture.invalid) {
        expect(validate(declarations)).toBe(false);
        expect(() => linkedSessionEngines(declarations)).toThrow();
      }
    });

  });

  //#endregion 🌉️LinkedSessionEnginesTests
  //#region 🔖️PixelCompare-tests
  describe("compareOwnedParityPixels", () => {
    const options = { ignoreAntialiasing: true, threshold: 0.1 } as const;
    const edge = (middle: number): Uint8Array => {
      const pixels = new Uint8Array(3 * 3 * 4);
      for (let y = 0; y < 3; y++) {
        for (let x = 0; x < 3; x++) {
          const value = x === 0 ? 0 : x === 1 ? middle : 255;
          const offset = (y * 3 + x) * 4;
          pixels[offset] = value;
          pixels[offset + 1] = value;
          pixels[offset + 2] = value;
          pixels[offset + 3] = 255;
        }
      }
      return pixels;
    };

    it("writes a muted row-major identity diff", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 64, 64, 64, 255, 255, 255, 255, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, reference.slice(), diff, 3, 1, options)).toBe(0);
      expect([...diff]).toEqual([191, 191, 191, 255, 207, 207, 207, 255, 255, 255, 255, 255]);
    });

    it("keeps a grayscale delta of 25 below threshold and counts 26 above it", () => {
      const reference = Uint8Array.from([0, 0, 0, 255, 0, 0, 0, 255]);
      const candidate = Uint8Array.from([25, 25, 25, 255, 26, 26, 26, 255]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([191, 191, 191, 255, 255, 32, 64, 255]);
    });

    it("composites alpha over white and ignores invisible RGB", () => {
      const reference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128]);
      const candidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0]);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 2, 1, options)).toBe(1);
      expect([...diff]).toEqual([255, 255, 255, 255, 255, 32, 64, 255]);
    });

    it("marks shared-edge coverage differences as antialiasing", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(2);
      expect([...diff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
      expect(compareOwnedParityPixels(reference, candidate, new Uint8Array(reference.length), 3, 3, { ...options, ignoreAntialiasing: false })).toBe(3);
    });

    it("counts a real high-contrast edge displacement", () => {
      const reference = edge(0);
      const candidate = edge(255);
      const diff = new Uint8Array(reference.length);
      expect(compareOwnedParityPixels(reference, candidate, diff, 3, 3, options)).toBe(3);
      expect([...diff.slice(16, 20)]).toEqual([255, 32, 64, 255]);
    });

    it("rejects writable typed-array overlap without corrupting either read input", () => {
      const reference = edge(96);
      const candidate = edge(128);
      const byteLength = reference.byteLength;
      const exactAliasBefore = reference.slice();
      expect(() => compareOwnedParityPixels(reference, candidate, reference, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(reference).toEqual(exactAliasBefore);

      const forwardStorage = new Uint8Array(byteLength + 4);
      const forwardReference = forwardStorage.subarray(0, byteLength);
      const forwardDiff = forwardStorage.subarray(4, byteLength + 4);
      forwardReference.set(reference);
      const forwardBefore = forwardStorage.slice();
      expect(() => compareOwnedParityPixels(forwardReference, candidate, forwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(forwardStorage).toEqual(forwardBefore);

      const backwardStorage = new Uint8Array(byteLength + 4);
      const backwardDiff = backwardStorage.subarray(0, byteLength);
      const backwardCandidate = backwardStorage.subarray(4, byteLength + 4);
      backwardCandidate.set(candidate);
      const backwardBefore = backwardStorage.slice();
      expect(() => compareOwnedParityPixels(reference, backwardCandidate, backwardDiff, 3, 3, options)).toThrow("diff buffer must not overlap");
      expect(backwardStorage).toEqual(backwardBefore);

      const disjointStorage = new Uint8Array(byteLength * 2);
      const disjointReference = disjointStorage.subarray(0, byteLength);
      const disjointDiff = disjointStorage.subarray(byteLength);
      disjointReference.set(reference);
      expect(compareOwnedParityPixels(disjointReference, candidate, disjointDiff, 3, 3, options)).toBe(2);
      expect([...disjointDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);

      const readOnlyAliasDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, reference, readOnlyAliasDiff, 3, 3, options)).toBe(0);
      const empty = new Uint8Array(0);
      expect(compareOwnedParityPixels(empty, empty, empty, 0, 3, options)).toBe(0);

      const retainedDiff = new Uint8Array(byteLength);
      expect(compareOwnedParityPixels(reference, candidate, retainedDiff, 3, 3, options)).toBe(2);
      expect([...retainedDiff.slice(16, 20)]).toEqual([255, 192, 0, 255]);
    });

    it("rejects malformed lengths, dimensions, and thresholds exactly", () => {
      const pixel = new Uint8Array(4);
      expect(() => compareOwnedParityPixels(new Uint8Array(3), pixel, pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, new Uint8Array(5), pixel, 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, new Uint8Array(0), 1, 1, options)).toThrow("exactly 4 RGBA bytes");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, -1, 1, options)).toThrow("non-negative safe integers");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, Number.MAX_SAFE_INTEGER, 2, options)).toThrow("safe byte range");
      expect(() => compareOwnedParityPixels(pixel, pixel, pixel, 1, 1, { ...options, threshold: Number.NaN })).toThrow("finite and between zero and one");
    });

    it("compares a large identical crop within a bounded tooling smoke budget", () => {
      const width = 640;
      const height = 360;
      const reference = new Uint8Array(width * height * 4);
      const candidate = new Uint8Array(reference);
      const diff = new Uint8Array(reference.length);
      const started = performance.now();
      expect(compareOwnedParityPixels(reference, candidate, diff, width, height, options)).toBe(0);
      expect(performance.now() - started).toBeLessThan(2_000);
      expect(diff[0]).toBe(255);
      expect(diff[diff.length - 1]).toBe(255);
    });

    it("retains representative opaque, transparent, text-edge, and scene-gradient counts", () => {
      const opaqueReference = Uint8Array.from([0, 0, 0, 255, 40, 40, 40, 255, 120, 90, 60, 255, 255, 255, 255, 255]);
      const opaqueCandidate = Uint8Array.from([0, 0, 0, 255, 60, 60, 60, 255, 170, 90, 60, 255, 230, 230, 230, 255]);
      const transparentReference = Uint8Array.from([255, 0, 0, 0, 0, 0, 0, 128, 20, 80, 160, 255]);
      const transparentCandidate = Uint8Array.from([0, 255, 0, 0, 0, 0, 0, 0, 20, 80, 160, 192]);
      const edgeReference = edge(96);
      const edgeCandidate = edge(128);
      const gradientReference = new Uint8Array(16 * 4 * 4);
      const gradientCandidate = new Uint8Array(gradientReference.length);
      for (let index = 0; index < 64; index++) {
        const referenceValue = Math.round((index % 16) * 17);
        const candidateValue = Math.min(255, referenceValue + 12);
        const offset = index * 4;
        gradientReference.set([referenceValue, Math.max(0, referenceValue - 20), 255 - referenceValue, 255], offset);
        gradientCandidate.set([candidateValue, Math.max(0, candidateValue - 20), 255 - candidateValue, 255], offset);
      }
      const fixtures = [
        { antialiasMarkers: 0, candidate: opaqueCandidate, height: 1, mismatches: 1, name: "opaque", reference: opaqueReference, width: 4 },
        { antialiasMarkers: 0, candidate: transparentCandidate, height: 1, mismatches: 1, name: "transparent", reference: transparentReference, width: 3 },
        { antialiasMarkers: 1, candidate: edgeCandidate, height: 3, mismatches: 2, name: "text-edge", reference: edgeReference, width: 3 },
        { antialiasMarkers: 0, candidate: gradientCandidate, height: 4, mismatches: 0, name: "scene-gradient", reference: gradientReference, width: 16 },
      ];
      for (const fixture of fixtures) {
        const ownedDiff = new Uint8Array(fixture.reference.length);
        const ownedCount = compareOwnedParityPixels(fixture.reference, fixture.candidate, ownedDiff, fixture.width, fixture.height, options);
        let ownedAntialiasMarkers = 0;
        let ownedMismatchMarkers = 0;
        for (let offset = 0; offset < ownedDiff.length; offset += 4) {
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 192 && ownedDiff[offset + 2] === 0) ownedAntialiasMarkers += 1;
          if (ownedDiff[offset] === 255 && ownedDiff[offset + 1] === 32 && ownedDiff[offset + 2] === 64) ownedMismatchMarkers += 1;
        }
        expect({ fixture: fixture.name, ownedAntialiasMarkers, ownedCount, ownedMismatchMarkers }).toEqual({
          fixture: fixture.name,
          ownedAntialiasMarkers: fixture.antialiasMarkers,
          ownedCount: fixture.mismatches,
          ownedMismatchMarkers: fixture.mismatches,
        });
      }
    });

    it("crops complete RGBA rows with strict bounds and independent storage", () => {
      const image: OwnedParityImage = { width: 4, height: 3, data: Uint8Array.from({ length: 48 }, (_, index) => index) };
      const before = image.data.slice();
      const crop = cropOwnedParityRgba(image, 1, 1, 2, 2);
      expect([...crop]).toEqual([...image.data.slice(20, 28), ...image.data.slice(36, 44)]);
      crop.fill(255);
      expect(image.data).toEqual(before);
      expect([...cropOwnedParityRgba(image, 4, 3, 0, 0)]).toEqual([]);
      expect(() => cropOwnedParityRgba(image, -1, 0, 1, 1)).toThrow("non-negative safe integers");
      expect(() => cropOwnedParityRgba(image, 3, 2, 2, 1)).toThrow("within image bounds");
      expect(() => cropOwnedParityRgba({ ...image, data: image.data.subarray(0, 47) }, 0, 0, 1, 1)).toThrow("exactly 48 RGBA bytes");
    });

    itLong("preserves fixed CSS color, alpha, and diagnostic marker pixels through Canvas PNGs", async () => {
      ensureParityPlaywrightBrowsersPath();
      const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
      const browser = await chromium.launch({ headless: true });
      try {
        const page = await browser.newPage({ viewport: { width: 4, height: 3 } });
        await page.setContent(
          '<style>*{margin:0}body{background:transparent}#opaque{position:absolute;width:1px;height:1px;background:rgb(12 34 56)}#alpha{position:absolute;left:1px;width:1px;height:1px;background:rgb(20 40 60 / 50%)}</style><div id="opaque"></div><div id="alpha"></div>',
        );
        const screenshot = await page.screenshot({ omitBackground: true });
        const decoded = await decodeParityScreenshot(page, screenshot);
        expect({ width: decoded.width, height: decoded.height, firstRow: [...decoded.data.slice(0, 16)] }).toEqual({
          width: 4,
          height: 3,
          firstRow: [12, 34, 56, 255, 20, 40, 60, 128, 0, 0, 0, 0, 0, 0, 0, 0],
        });
        expect([...cropOwnedParityRgba(decoded, 0, 0, 2, 1)]).toEqual([12, 34, 56, 255, 20, 40, 60, 128]);
        const diagnostic = Uint8Array.from([12, 34, 56, 255, 20, 40, 60, 128, 255, 32, 64, 255, 255, 192, 0, 255]);
        const roundTrip = await decodeParityScreenshot(page, await encodeParityDiff(page, diagnostic, 2, 2));
        expect({ width: roundTrip.width, height: roundTrip.height, data: [...roundTrip.data] }).toEqual({ width: 2, height: 2, data: [...diagnostic] });
      } finally {
        await browser.close();
      }
    }, 30_000);
  });
  //#endregion 🔖️PixelCompare-tests
}
