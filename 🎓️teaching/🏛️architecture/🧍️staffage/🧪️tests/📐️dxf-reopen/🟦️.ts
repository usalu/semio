/** 📐️ DXF export reopened and audited with ezdxf. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { snapLineweight, writeDxf } from "../../🔨️modules/📤️export/📐️dxf/🟦️.ts";
import { modelScene, python, STAFFAGE_ROOT, withScratch } from "../🧰️support/🧰️.ts";

type Report = { version: string; insunits: number; measurement: number; extmin: number[]; extmax: number[]; errors: string[]; fixes: string[]; layers: Record<string, { aci: number; lineweight: number }>; lwpolylines: { layer: string; closed: boolean; points: number[][] }[]; texts: { layer: string; text: string; height: number; at: number[] }[]; bbox: number[][] | null; layouts: string[]; handles: boolean };

const reopen = (text: string): Promise<Report> => withScratch((directory) => {
  const file = join(directory, "scene.dxf");
  writeFileSync(file, text);
  return JSON.parse(python(`${STAFFAGE_ROOT}/🧪️tests/📐️dxf-reopen/🐍️.py`, ["ezdxf"], [file])) as Report;
});

describe("📐️ writeDxf", () => {
  it("is read by ezdxf as AC1015 in metres and passes ezdxf.audit() without findings", async () => {
    const report = await reopen(writeDxf(modelScene()));
    expect(report.version).toBe("AC1015");
    expect(report.insunits).toBe(6);
    expect(report.measurement).toBe(1);
    expect(report.errors).toEqual([]);
    expect(report.fixes).toEqual([]);
    expect(report.layouts).toEqual(["Model", "Layout1"]);
  });

  it("keeps extents, layers, polylines and text exact", async () => {
    const scene = modelScene();
    const report = await reopen(writeDxf(scene));
    const points = scene.layers.flatMap((layer) => layer.polylines.flatMap((line) => line.points));
    const xs = points.map((point) => point.x);
    const ys = points.map((point) => point.y);
    const close = (a: readonly number[], b: readonly number[]): void => a.forEach((value, index) => expect(Math.abs(value - b[index]!)).toBeLessThan(1e-6));
    close(report.extmin.slice(0, 2), [Math.min(...xs), Math.min(...ys)]);
    close(report.extmax.slice(0, 2), [Math.max(...xs), Math.max(...ys)]);
    close(report.bbox![0]!, report.extmin);
    close(report.bbox![1]!, report.extmax);
    for (const layer of scene.layers) expect(report.layers[layer.name]).toEqual({ aci: layer.aci, lineweight: snapLineweight(layer.lineweight) });
    const expected = scene.layers.flatMap((layer) => layer.polylines.map((line) => ({ layer: layer.name, closed: line.closed, points: line.points.map((point) => [point.x, point.y]) })));
    expect(report.lwpolylines.length).toBe(expected.length);
    expected.forEach((line, index) => { expect(report.lwpolylines[index]!.layer).toBe(line.layer); expect(report.lwpolylines[index]!.closed).toBe(line.closed); close(report.lwpolylines[index]!.points.flat(), line.points.flat()); });
    expect(report.texts).toHaveLength(1);
    expect(report.texts[0]!.text).toBe("1,80 m · 2,5 m²");
    expect(report.texts[0]!.height).toBeCloseTo(0.1, 9);
  });

  it("writes an empty scene that still audits clean, snaps lineweights and refuses non-metre scenes", async () => {
    const report = await reopen(writeDxf({ units: "m", asset: "probe", timeS: 0, representation: { kind: "none" }, layers: [] }));
    expect(report.errors).toEqual([]);
    expect(snapLineweight(48)).toBe(50);
    expect(snapLineweight(0)).toBe(0);
    expect(() => writeDxf({ ...modelScene(), units: "mm" as "m" })).toThrow(/units/u);
  });
});
