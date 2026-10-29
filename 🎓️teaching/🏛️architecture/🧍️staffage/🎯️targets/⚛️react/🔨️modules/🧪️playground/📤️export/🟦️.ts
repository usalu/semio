/** 📤️ Exports the playground as one SVG: draws every placement through the shared evaluator, composes them at true paper offsets and hands the file to the browser.
 *
 * Progress is reported per placement and the loop yields to the browser between placements, so the page stays responsive and `signal` stops the job at the next placement.
 *
 * @see ../🧱️compose/🟦️.ts — the pure composition
 * @see ../../🧮️pipeline/🟦️.ts — `figureFrame`
 */
import { writeSvg } from "@teaching/staffage";
import type { Asset, JobControl, Language, Text, View } from "@teaching/staffage";
import { figureFrame } from "../../🧮️pipeline/🟦️.ts";
import type { Background, Placement } from "../../💾️persistence/🟦️.ts";
import { placementQuery } from "../../📍️placed/🟦️.tsx";
import { composeSvg, type ComposedPart } from "../🧱️compose/🟦️.ts";

/** 🧾️ What an export needs. */
export type CompositionExport = { readonly placements: readonly Placement[]; readonly assets: ReadonlyMap<string, Asset>; readonly scale: number; readonly view: View; readonly background: Background; readonly lengthM: number; readonly language: Language; readonly timeS: number; readonly title: string };

const yieldToBrowser = (): Promise<void> => new Promise((resolve) => setTimeout(resolve, 0));
const label: Text = { en: "Drawing placements", de: "Platzierungen werden gezeichnet" };

/** 🖼️ Draws and composes the playground; rejects with an `AbortError` when cancelled. */
export async function exportComposition(job: CompositionExport, control: JobControl = {}): Promise<string> {
  const parts: ComposedPart[] = [];
  const drawable = job.placements.filter((placement) => job.assets.has(placement.asset));
  for (const [index, placement] of drawable.entries()) {
    control.signal?.throwIfAborted();
    await yieldToBrowser();
    control.signal?.throwIfAborted();
    const asset = job.assets.get(placement.asset)!;
    const query = placementQuery(asset, placement.pick, job.timeS, placement.offsetS);
    const { scene } = figureFrame(asset, query, { view: job.view, scale: { denominator: job.scale }, language: job.language, transparent: true, annotations: { dimensions: false, scaleBar: false, legend: false, labels: false, ground: false } });
    parts.push({ id: placement.id, asset: asset.id, title: asset.title[job.language], xM: placement.xM, mirrored: placement.mirrored, markup: writeSvg(scene), widthMm: scene.widthMm, heightMm: scene.heightMm, originXMm: scene.metadata.origin.xMm, originYMm: scene.metadata.origin.yMm });
    control.progress?.(index + 1, drawable.length, label);
  }
  return composeSvg(parts, { scale: job.scale, view: job.view, lengthM: job.lengthM, background: job.background, language: job.language, title: job.title });
}

/** 💾️ Offers text as a file download. */
export function downloadText(filename: string, text: string, mime = "image/svg+xml"): void {
  const url = URL.createObjectURL(new Blob([text], { type: mime }));
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
