/** 🧊️ The 3D downloads of the archive: GLB per pose with all seven views, FBX per clip in the current representation, the model JSON and the ZIP bundle, with the Khronos validator and three's loaders as test-only oracles, the file names, progress and cancellation, and the export panel that offers what the asset's `exports` lists.
 * @see ./🥒️.feature — the scenarios */
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { unzipSync } from "fflate";
import { afterEach, describe, expect, it, vi } from "vitest";
import type { Asset } from "@teaching/staffage";
import { exportCapabilities, exportFigure, exportFilename } from "@teaching/staffage-react";
import type { ExportResult } from "@teaching/staffage-react";
import { validateModel } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { loadFbx, loadGlb, validateGlb } from "../../../../🧪️tests/🧰️support/🧊️.ts";
import { ArchiveProvider } from "../../🔨️modules/🧭️context/🟦️.tsx";
import { translator } from "../../🔨️modules/🌐️i18n/🟦️.ts";
import { ENGLISH } from "../../🔨️modules/🌐️i18n/🔠️english/🟦️.ts";
import { ExportPanel } from "../../🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx";
import { CatalogStore } from "../../🔨️modules/📚️catalog/🟦️.ts";
import { ArchiveStore, memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { sampleFigure, walker } from "../🧰️support/🧰️.ts";

afterEach(cleanup);

const request = { view: "light-solid", viewpoint: "elevation", scale: { denominator: 100 }, language: "en" } as const;
const stand = { kind: "pose", pose: "stand" } as const;
const walk = { kind: "clip", clip: "walk", timeS: 0 } as const;
const done = (result: ExportResult): Extract<ExportResult, { status: "done" }> => {
  if (result.status !== "done") throw new Error(`export ${result.status}`);
  return result;
};

describe("🧊️ GLB per pose", () => {
  it("is a valid glTF with the seven views as material variants, named like the drawings", async () => {
    const result = done(await exportFigure(sampleFigure, stand, request, "glb"));
    expect(result.filename).toBe("sample-figure_stand_model_all_1-100.glb");
    expect(result.mime).toBe("model/gltf-binary");
    expect(String.fromCharCode(...result.bytes.subarray(0, 4))).toBe("glTF");
    const report = await validateGlb(result.bytes);
    expect(report.errors).toBe(0);
    expect(report.warnings).toBe(0);
    const loaded = await loadGlb(result.bytes);
    const variants = (loaded.parser.json.extensions?.KHR_materials_variants?.variants ?? []).map((variant: { name: string }) => variant.name).sort();
    expect(variants).toEqual(["dark-line", "dark-solid", "forces", "light-line", "light-solid", "moments", "thermal"]);
    expect(loaded.animations).toHaveLength(0);
  });

  it("is the same bytes for the same request and takes the pose the request names, not the current one", async () => {
    const a = done(await exportFigure(sampleFigure, stand, { ...request, pose: "carry-bag" }, "glb"));
    const b = done(await exportFigure(sampleFigure, walk, { ...request, pose: "carry-bag" }, "glb"));
    expect(a.filename).toBe("sample-figure_carry-bag_model_all_1-100.glb");
    expect(Buffer.from(a.bytes).equals(Buffer.from(b.bytes))).toBe(true);
  });
});

describe("🎬️ FBX per clip", () => {
  it("loads in three's FBXLoader with the clip's animation, in the representation of the request, named with it", async () => {
    const result = done(await exportFigure(sampleFigure, walk, { ...request, view: "thermal" }, "fbx"));
    expect(result.filename).toBe("sample-figure_walk_model_thermal_1-100.fbx");
    expect(String.fromCharCode(...result.bytes.subarray(0, 18))).toBe("Kaydara FBX Binary");
    const loaded = loadFbx(result.bytes);
    expect(loaded.animations.length).toBeGreaterThanOrEqual(1);
    expect(loaded.animations[0].duration).toBeCloseTo(sampleFigure.clips.find((clip) => clip.id === "walk")!.durationS, 1);
    expect(result.details.notes.length).toBeLessThanOrEqual(8);
  });

  it("makes the default clip when the current state is a pose", async () => {
    const result = done(await exportFigure(sampleFigure, stand, request, "fbx"));
    expect(result.filename).toBe(`sample-figure_${sampleFigure.presentation.defaultClip}_model_light-solid_1-100.fbx`);
  });
});

describe("🗿️ model JSON and the ZIP bundle", () => {
  it("is a valid semio.model/v1 document of the pose or the clip", async () => {
    for (const [options, name] of [[{ pose: "stand" }, "sample-figure_stand_model_all_1-100.json"], [{ clip: "walk" }, "sample-figure_walk_model_all_1-100.json"]] as const) {
      const result = done(await exportFigure(sampleFigure, stand, { ...request, ...options }, "model"));
      expect(result.filename).toBe(name);
      const document = JSON.parse(new TextDecoder().decode(result.bytes));
      expect(document.version).toBe("semio.model/v1");
      expect(validateModel(document).filter((issue) => issue.severity === "error")).toEqual([]);
      expect(document.variants).toHaveLength(7);
    }
  });

  it("holds every pose as GLB and JSON and every clip as FBX and JSON, read back by an independent unzip", async () => {
    const result = done(await exportFigure(sampleFigure, stand, request, "model-zip"));
    expect(result.filename).toBe("sample-figure_bundle_model_all_1-100.zip");
    const names = Object.keys(unzipSync(result.bytes)).sort();
    const poses = sampleFigure.poses.filter((pose) => pose.key !== true).length;
    expect(names.filter((name) => name.startsWith("glb/"))).toHaveLength(poses);
    expect(names.filter((name) => name.startsWith("fbx/"))).toHaveLength(sampleFigure.clips.length);
    expect(names.filter((name) => name.startsWith("model/"))).toHaveLength(poses + sampleFigure.clips.length);
  });

  it("names the filename of a 3D format in exportFilename like the downloads do", () => {
    expect(exportFilename(sampleFigure, stand, request, "glb")).toBe("sample-figure_stand_model_all_1-100.glb");
    expect(exportFilename(sampleFigure, walk, { ...request, view: "forces" }, "fbx")).toBe("sample-figure_walk_model_forces_1-100.fbx");
  });
});

describe("📶️ progress and cancellation", () => {
  it("reports whole percent from 0 to 100 and never goes backwards", async () => {
    const seen: number[] = [];
    done(await exportFigure(sampleFigure, stand, { ...request, pose: "stand" }, "glb", { progress: (d, t) => seen.push(Math.round((d / t) * 100)) }));
    expect(seen.length).toBeGreaterThan(3);
    expect(Math.max(...seen)).toBe(100);
    expect(seen).toEqual([...seen].sort((a, b) => a - b));
  });

  it("is cancelled before and during the work and returns no file", async () => {
    const before = new AbortController();
    before.abort();
    expect((await exportFigure(sampleFigure, stand, request, "glb", { signal: before.signal })).status).toBe("cancelled");
    const during = new AbortController();
    const outcome = await exportFigure(sampleFigure, walk, request, "fbx", { signal: during.signal, progress: (d, t) => d / t > 0.1 && during.abort() });
    expect(outcome.status).toBe("cancelled");
  });

  it("says in both languages why an asset without volume has no 3D download", async () => {
    const outcome = await exportFigure({ ...sampleFigure, volume: undefined } as unknown as Asset, stand, request, "glb");
    expect(outcome.status).toBe("unavailable");
    if (outcome.status === "unavailable") {
      expect(outcome.reason.en).toMatch(/no authored volume/u);
      expect(outcome.reason.de).toMatch(/kein erstelltes Volumen/u);
    }
  });

  it("reports 3D formats as available and drawings of the axonometric view as drawings", async () => {
    const capabilities = await exportCapabilities();
    for (const format of ["glb", "fbx", "model", "model-zip"] as const) expect(capabilities[format].available).toBe(true);
    const svg = done(await exportFigure(sampleFigure, stand, { ...request, viewpoint: "axonometric" }, "svg"));
    expect(svg.filename).toBe("sample-figure_stand_axonometric_light-solid_1-100.svg");
    expect(new TextDecoder().decode(svg.bytes)).toContain("axonometric");
  });
});

describe("📤️ export panel", () => {
  const catalog = new CatalogStore({ manifest: async () => { throw new Error("unused"); }, category: async () => null });
  const wrap = (node: React.ReactElement, language: "en" | "de" = "en") => <ArchiveProvider language={language} t={translator(language)} catalog={catalog} store={new ArchiveStore(memoryStorage(), ["dark-line"])}>{node}</ArchiveProvider>;
  const capabilities = { svg: { available: true }, pdf: { available: true }, dxf: { available: true }, dwg: { available: false, reason: { en: "no", de: "nein" } }, "png-sequence": { available: true }, mp4: { available: true }, webm: { available: true }, glb: { available: true }, fbx: { available: true }, model: { available: true }, "model-zip": { available: true } } as const;
  const formats = () => [...document.querySelectorAll("[data-format]")].map((card) => card.getAttribute("data-format"));

  it("offers exactly the 3D formats the asset's exports list names and nothing else", () => {
    const only = (exports: Asset["exports"]) => ({ ...sampleFigure, exports }) as Asset;
    render(wrap(<ExportPanel asset={only(["svg", "glb"])} query={stand} request={request} language="en" capabilities={capabilities} />));
    expect(formats()).toEqual(["svg", "glb"]);
    cleanup();
    render(wrap(<ExportPanel asset={only(["fbx", "model"])} query={stand} request={request} language="en" capabilities={capabilities} />));
    expect(formats()).toEqual(["fbx", "model", "model-zip"]);
    cleanup();
    const flat = { ...walker, exports: walker.exports.filter((format) => !["glb", "fbx", "model", "model-zip"].includes(format)) } as Asset;
    render(wrap(<ExportPanel asset={flat} query={stand} request={request} language="en" capabilities={capabilities} />));
    expect(formats().filter((format) => ["glb", "fbx", "model", "model-zip"].includes(format!))).toEqual([]);
  });

  it("marks a 3D format whose asset has no volume as unsupported with the reason, and lets the pose, the clip or either be chosen", () => {
    const noVolume = { ...sampleFigure, volume: undefined } as unknown as Asset;
    render(wrap(<ExportPanel asset={noVolume} query={stand} request={request} language="de" capabilities={capabilities} />, "de"));
    const card = document.querySelector("[data-format='glb']") as HTMLElement;
    expect(card.getAttribute("data-state")).toBe("unsupported");
    expect(card.textContent).toMatch(/kein erstelltes Volumen/u);
    cleanup();
    render(wrap(<ExportPanel asset={sampleFigure} query={walk} request={request} language="en" capabilities={capabilities} />));
    const glb = within(document.querySelector("[data-format='glb']") as HTMLElement);
    const fbx = within(document.querySelector("[data-format='fbx']") as HTMLElement);
    const model = within(document.querySelector("[data-format='model']") as HTMLElement);
    expect((glb.getByLabelText(ENGLISH["model.exports.pose"]) as HTMLSelectElement).value).toBe("stand");
    expect((fbx.getByLabelText(ENGLISH["model.exports.clip"]) as HTMLSelectElement).value).toBe("walk");
    expect((model.getByLabelText(ENGLISH["model.exports.source"]) as HTMLSelectElement).value).toBe("clip:walk");
    fireEvent.change(glb.getByLabelText(ENGLISH["model.exports.pose"]), { target: { value: "carry-bag" } });
    expect((glb.getByLabelText(ENGLISH["model.exports.pose"]) as HTMLSelectElement).value).toBe("carry-bag");
  });

  it("runs a 3D export with the chosen pose, shows progress with a cancel control, saves the file and reports its name and size", async () => {
    const save = vi.fn();
    let seen: unknown = null;
    const exporter = vi.fn(async (_asset, _query, options, format, control): Promise<ExportResult> => {
      seen = options;
      control?.progress?.(40, 100, { en: "Building the model", de: "Modell wird aufgebaut" });
      const bytes = new Uint8Array(4096);
      return { status: "done", format, filename: "x.glb", mime: "model/gltf-binary", bytes, blob: new Blob([bytes]), details: { metadataEmbedded: true, notes: [{ en: "A note.", de: "Eine Notiz." }] } };
    });
    render(wrap(<ExportPanel asset={sampleFigure} query={stand} request={request} language="en" capabilities={capabilities} exporter={exporter as never} save={save} />));
    const card = within(document.querySelector("[data-format='glb']") as HTMLElement);
    fireEvent.change(card.getByLabelText(ENGLISH["model.exports.pose"]), { target: { value: "carry-bag" } });
    fireEvent.click(card.getByRole("button", { name: /Export GLB/u }));
    await waitFor(() => expect(save).toHaveBeenCalledTimes(1));
    expect(seen).toMatchObject({ pose: "carry-bag", view: "light-solid" });
    expect(document.querySelector("[data-result]")!.textContent).toContain("x.glb");
    expect(document.querySelector("[data-result]")!.textContent).toContain("4.0 kB");
    expect(document.querySelector("[data-result]")!.textContent).toContain("A note.");
  });

  it("cancels a running 3D export through the signal and blocks a second run meanwhile", async () => {
    const exporter = vi.fn((_a, _q, _o, format, control) => new Promise<ExportResult>((resolve) => control.signal.addEventListener("abort", () => resolve({ status: "cancelled", format }))));
    render(wrap(<ExportPanel asset={sampleFigure} query={stand} request={request} language="en" capabilities={capabilities} exporter={exporter as never} />));
    fireEvent.click(within(document.querySelector("[data-format='model-zip']") as HTMLElement).getByRole("button", { name: /Export 3D models/u }));
    expect((within(document.querySelector("[data-format='fbx']") as HTMLElement).getByRole("button", { name: /Export FBX/u }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(await screen.findByRole("button", { name: ENGLISH["action.cancel"] }));
    await waitFor(() => expect(document.querySelector("[data-result]")!.textContent).toContain(ENGLISH["detail.export.cancelled"]));
  });

  it("blocks the drawing formats under the 3D model viewpoint with the reason and keeps the 3D formats open", () => {
    render(wrap(<ExportPanel asset={sampleFigure} query={stand} request={request} viewpoint="model" language="en" capabilities={capabilities} />));
    const state = (format: string) => document.querySelector(`[data-format='${format}']`)!.getAttribute("data-state");
    expect([state("svg"), state("pdf"), state("png-sequence"), state("glb"), state("fbx"), state("model"), state("model-zip")]).toEqual(["needs-drawing", "needs-drawing", "needs-drawing", "available", "available", "available", "available"]);
    expect((within(document.querySelector("[data-format='svg']") as HTMLElement).getByRole("button") as HTMLButtonElement).disabled).toBe(true);
    expect(document.querySelector("[data-format='svg']")!.textContent).toContain(ENGLISH["model.exports.drawingOnly"]);
    expect(document.querySelector("[data-format='component']")!.textContent).toContain('viewpoint="model"');
  });
});
