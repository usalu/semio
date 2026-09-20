import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const engineRoot = resolve(suiteRoot, "../..");
const uiRoot = resolve(suiteRoot, "../../../../../../../🔨️modules/🖱️ui");
const fixture = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🖼️raster-residency/🔣️.json"), "utf8")) as {
  readonly capacity: { readonly items: number; readonly bytes: number; readonly retirementUnitsPerStep: number };
  readonly sources: readonly string[];
  readonly scenarios: readonly { readonly name: string }[];
};
const capacityFixture = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🖼️raster-capacity-followup/🔣️.json"), "utf8")) as {
  readonly identity: { readonly pageBytes: number; readonly digestLanes: number; readonly dimensionsIncluded: boolean };
  readonly capacity: { readonly items: number; readonly bytes: number; readonly changedFullPolicy: string };
  readonly scenarios: readonly { readonly name: string; readonly expected: { readonly presenterTargets?: number; readonly tableTargets?: number } }[];
};

class ResidencyOracle {
  readonly live = new Set<string>();
  committed = new Set<string>();
  candidate = new Set<string>();
  previous = new Set<string>();

  publish(keys: readonly string[]): void {
    this.candidate = new Set(keys);
  }

  stage(key: string): void {
    this.live.add(key);
  }

  commit(): void {
    this.previous = this.committed;
    this.committed = this.candidate;
    this.candidate = new Set();
  }

  abort(): void {
    this.candidate = new Set();
  }

  releasePrevious(): void {
    this.previous = new Set();
    const retained = new Set([...this.committed, ...this.candidate, ...this.previous]);
    for (const key of this.live) if (!retained.has(key)) this.live.delete(key);
  }
}

class CapacityOracle {
  readonly live = new Map<string, string>();
  readonly staged = new Map<string, string>();
  presenting = false;

  offer(key: string, content: string, capacity: number): "reuse" | "stage" | "backpressure" {
    const staged = this.staged.get(key);
    if (staged !== undefined) {
      if (staged !== content) throw new Error("conflicting content for one candidate key");
      return "reuse";
    }
    if (this.live.get(key) === content) return "reuse";
    if (this.live.size + this.staged.size >= capacity) return "backpressure";
    this.staged.set(key, content);
    return "stage";
  }

  paintable(key: string): string | undefined {
    return (this.presenting ? this.staged.get(key) : undefined) ?? this.live.get(key);
  }

  beginPresentation(): boolean {
    this.presenting = this.staged.size > 0;
    return this.presenting;
  }

  commit(): void {
    for (const [key, content] of this.staged) this.live.set(key, content);
    this.staged.clear();
    this.presenting = false;
  }

  abort(): void {
    this.staged.clear();
    this.presenting = false;
  }
}

describe("wgpu prepared raster residency", () => {
  it("matches the neutral fixture through an independent Set ownership oracle", () => {
    expect(fixture.capacity).toEqual({ items: 256, bytes: 268435456, retirementUnitsPerStep: 1 });
    expect(fixture.sources).toEqual(["worldReference", "worldPaint", "uiRaster", "overlayRaster"]);
    expect(fixture.scenarios.map(({ name }) => name)).toEqual([
      "three hundred distinct reference replacements",
      "two windows share one raster",
      "final owner closes",
      "aborted replacement preserves committed image",
      "paint and overlay consumers publish ownership",
      "budget admission retires one unowned resource per step",
    ]);
    const oracle = new ResidencyOracle();
    let maximum = 0;
    for (let index = 0; index < 300; index += 1) {
      const key = `reference-${index.toString().padStart(3, "0")}`;
      oracle.publish([key]);
      oracle.stage(key);
      maximum = Math.max(maximum, oracle.live.size);
      oracle.commit();
      oracle.releasePrevious();
    }
    expect(maximum).toBeLessThanOrEqual(2);
    expect([...oracle.live]).toEqual(["reference-299"]);
  });

  it("preserves shared and committed owners across pane close and abort", () => {
    const oracle = new ResidencyOracle();
    oracle.publish(["image-A", "image-A"]);
    oracle.stage("image-A");
    oracle.commit();
    oracle.releasePrevious();
    oracle.publish(["image-A"]);
    oracle.commit();
    oracle.releasePrevious();
    expect([...oracle.live]).toEqual(["image-A"]);
    oracle.publish(["image-B"]);
    oracle.abort();
    oracle.releasePrevious();
    expect([...oracle.live]).toEqual(["image-A"]);
    oracle.publish([]);
    oracle.commit();
    oracle.releasePrevious();
    expect(oracle.live.size).toBe(0);
  });

  it("reuses a repeated full frame and backpressures changed content until peak headroom exists", () => {
    expect(capacityFixture.identity).toEqual({ pageBytes: 16384, digestLanes: 2, dimensionsIncluded: true });
    expect(capacityFixture.capacity).toEqual({ items: 256, bytes: 268435456, changedFullPolicy: "backpressureUntilHeadroom" });
    expect(capacityFixture.scenarios.map(({ name }) => name)).toEqual([
      "repeat unchanged full frame",
      "changed same key stages distinct content",
      "changed full frame applies backpressure",
      "aborted changed replacement keeps committed content",
      "previous frame remains owned until release",
      "engine canvas publishes one accounted target",
    ]);
    const oracle = new CapacityOracle();
    for (let index = 0; index < 256; index += 1) oracle.live.set(`image-${index}`, `content-${index}`);
    for (let index = 0; index < 256; index += 1) expect(oracle.offer(`image-${index}`, `content-${index}`, 256)).toBe("reuse");
    expect(oracle.live.size).toBe(256);
    expect(oracle.staged.size).toBe(0);
    expect(oracle.offer("image-0", "changed", 256)).toBe("backpressure");
    expect(oracle.paintable("image-0")).toBe("content-0");
  });

  it("keeps committed content across a changed same-key abort", () => {
    const oracle = new CapacityOracle();
    oracle.live.set("image", "A");
    expect(oracle.offer("image", "B", 2)).toBe("stage");
    expect(oracle.paintable("image")).toBe("A");
    expect(oracle.beginPresentation()).toBe(true);
    expect(oracle.paintable("image")).toBe("B");
    expect(oracle.live.get("image")).toBe("A");
    oracle.abort();
    expect(oracle.paintable("image")).toBe("A");
    expect(oracle.staged.size).toBe(0);
    expect(oracle.offer("image", "B", 2)).toBe("stage");
    expect(oracle.paintable("image")).toBe("A");
    expect(oracle.beginPresentation()).toBe(true);
    expect(oracle.paintable("image")).toBe("B");
    oracle.commit();
    expect(oracle.paintable("image")).toBe("B");
  });

  it("wires complete packet ownership before every raster allocation", () => {
    const drawTypes = readFileSync(resolve(uiRoot, "🎯️targets/🧊️wgpu/🖍️draw/🏷️types/🦀️.rs"), "utf8");
    const draw = readFileSync(resolve(uiRoot, "🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs"), "utf8");
    const prepared = readFileSync(resolve(uiRoot, "🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs"), "utf8");
    const gpu = readFileSync(resolve(uiRoot, "🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs"), "utf8");
    const renderer = readFileSync(resolve(engineRoot, "🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"), "utf8");
    const canvas = readFileSync(resolve(engineRoot, "🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"), "utf8");
    for (const source of ["pass.textured_draws", "layer.raster_instances", "layer.overlay_raster_instances"]) expect(drawTypes).toContain(source);
    expect(prepared).toContain("self.overlay.as_ref()");
    expect(draw).toContain("committed: RasterKeepSetV1");
    expect(draw).toContain("candidate: RasterKeepSetV1");
    expect(draw).toContain("previous: RasterKeepSetV1");
    expect(renderer).toContain("AppPresentPhase::Ownership =>");
    expect(renderer).toContain("self.gpu.begin_raster_ownership(witness)");
    expect(renderer).toContain("self.gpu.publish_raster_ownership(witness, key)");
    expect(renderer).toContain("self.gpu.seal_raster_ownership(witness)");
    expect(renderer.indexOf("if !previous.retire_step()")).toBeLessThan(renderer.indexOf("gpu.release_previous_raster_ownership()"));
    expect(gpu).toContain("self.raster_store.prepare_admission_step");
    expect(prepared).toContain("pages.content_identity.mix_bytes(start, &self.source[start..end])");
    expect(draw).toContain("raster_content_is_reusable(&self.live, &self.staged");
    expect(draw.indexOf("if raster_content_is_reusable")).toBeLessThan(draw.indexOf("let admitted = raster_admission_fits"));
    expect(canvas.indexOf("gpu.prepare_raster_admission_step")).toBeLessThan(canvas.indexOf("gpu.reserve_engine_texture"));
    expect(canvas).toContain("gpu.raster_content_is_reusable(key, identity, candidate_generation, expected)");
    expect(canvas.match(/create_target_texture\(gpu\.device\(\), build\.width, build\.height\)/g)).toHaveLength(1);
    expect(canvas).not.toContain("replacement_texture");
    expect(capacityFixture.scenarios[5]?.expected).toMatchObject({ presenterTargets: 0, tableTargets: 1 });
  });
});
