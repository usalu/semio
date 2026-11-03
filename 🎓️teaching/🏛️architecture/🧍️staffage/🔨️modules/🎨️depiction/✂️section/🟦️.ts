/** ✂️ Section views: the caption of a view ("Section A-A" / "Schnitt A-A" instead of "Front view" / "Vorderansicht") and the cut-line marker the side view shows at the plane of a sectioned front or plan view.
 *
 * The marker follows the drafting convention (ISO 128-40, DIN 6-1): a thin chain line across the object with thick ends, an arrow at each end pointing in the viewing direction, and the section letter beside each arrow.
 * It is drawn in elevations at 1:100 and larger only, where an arrow and a letter still fit beside the drawing. A front section is cut by a vertical plane across the forward axis and viewed from the front, so the line is vertical at `plane`
 * and the arrows point backwards (-x); a plan section is cut by a horizontal plane and viewed from above, so the line is horizontal at `plane` and the arrows point down.
 *
 * @see ../../../🧬️schema/🟦️.ts — {@link ViewSection}
 * @see https://www.iso.org/standard/3925.html — ISO 128-40 sections and cut lines
 */
import type { AuthoredRepresentation, Language, Text, ViewSection, Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { ANNOTATION_TEXT_MM } from "../../📐️units/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { arrowOutline, commandsOf, type Vec } from "../📐️geometry/🟦️.ts";
import { tx } from "../🖍️palette/🟦️.ts";

/** 📏️ The largest scale denominator at which the side view shows the cut-line marker (1:100 and 1:50). */
export const SECTION_MARKER_MAX_DENOMINATOR = 100;
/** 📏️ How far the cut line reaches beyond the object, in paper millimetres. */
export const SECTION_EXTENSION_MM = 6;
/** 📏️ The length of the thick end segments of the cut line, in paper millimetres. */
export const SECTION_END_MM = 4;
/** 📏️ The length of the viewing arrows, in paper millimetres. */
export const SECTION_ARROW_MM = 5;
const SECTION_ARROW_HEAD = { length: 2, width: 1.4 } as const;
const THIN_MM = 0.18;
const THICK_MM = 0.5;

/** 🏷️ The one name of each viewpoint in captions, sheets, viewer and validator messages (terminology R1): Side elevation / Seitenansicht, Front elevation / Vorderansicht, Plan (top view) / Draufsicht. */
export const VIEWPOINT_CAPTIONS: Readonly<Record<Viewpoint, Text>> = { elevation: tx("Side elevation", "Seitenansicht"), front: tx("Front elevation", "Vorderansicht"), plan: tx("Plan (top view)", "Draufsicht") };

/** 🏷️ The one name of a cut: Section / Schnitt for the front elevation, Plan section / Grundriss-Schnitt for the plan. */
export const SECTION_CAPTIONS: Readonly<Record<"front" | "plan", Text>> = { front: tx("Section", "Schnitt"), plan: tx("Plan section", "Grundriss-Schnitt") };

/** 🏷️ The caption of a viewpoint: "Section A-A" / "Schnitt A-A" for a front cut and "Plan section B-B" / "Grundriss-Schnitt B-B" for a plan cut, else "Side elevation", "Front elevation" or "Plan (top view)" (Seitenansicht, Vorderansicht, Draufsicht). */
export function viewpointCaption(viewpoint: Viewpoint, section: ViewSection | undefined, language: Language): string {
  if (section === undefined) return VIEWPOINT_CAPTIONS[viewpoint][language];
  return `${SECTION_CAPTIONS[viewpoint === "plan" ? "plan" : "front"][language]} ${section.label[language]}`;
}

/** ✂️ The section a representation declares for a viewpoint (`undefined` for the side view, which is the rig plane, and for plain views). */
export function sectionOf(representation: AuthoredRepresentation | undefined, viewpoint: Viewpoint): ViewSection | undefined {
  return viewpoint === "elevation" ? undefined : representation?.sections?.[viewpoint];
}

/** 🔠️ The letters at the two ends of a section label: "A-A" gives A and A, a single "B" gives B at both ends. */
export function sectionLetters(label: string): readonly [string, string] {
  const parts = label.split(/[-–]/u).filter((part) => part !== "");
  return [parts[0] ?? label, parts[1] ?? parts[0] ?? label];
}

/** 📐️ The cut-line marker in paper millimetres relative to the model origin, y down: the thin line, the thick end segments, the arrow polygons and the letter positions. */
export type SectionMarker = { readonly line: readonly [Vec, Vec]; readonly ends: readonly [readonly [Vec, Vec], readonly [Vec, Vec]]; readonly arrows: readonly [readonly [Vec, Vec], readonly [Vec, Vec]]; readonly letters: readonly [{ readonly at: Vec; readonly anchor: "start" | "middle" | "end" }, { readonly at: Vec; readonly anchor: "start" | "middle" | "end" }] };

/** 📐️ Places the cut-line marker of a section of `viewpoint` ("front" or "plan") for the elevation extent `bounds` (metres) at `k` paper millimetres per metre. */
export function sectionMarker(viewpoint: "front" | "plan", plane: number, k: number, bounds: { readonly minX: number; readonly maxX: number; readonly minY: number; readonly maxY: number }): SectionMarker {
  if (viewpoint === "front") {
    const x = plane * k;
    const top = -bounds.maxY * k - SECTION_EXTENSION_MM;
    const bottom = -bounds.minY * k + SECTION_EXTENSION_MM;
    return {
      line: [{ x, y: top }, { x, y: bottom }],
      ends: [[{ x, y: top }, { x, y: top + SECTION_END_MM }], [{ x, y: bottom - SECTION_END_MM }, { x, y: bottom }]],
      arrows: [[{ x, y: top }, { x: x - SECTION_ARROW_MM, y: top }], [{ x, y: bottom }, { x: x - SECTION_ARROW_MM, y: bottom }]],
      letters: [{ at: { x, y: top - 1.2 }, anchor: "middle" }, { at: { x, y: bottom + ANNOTATION_TEXT_MM + 0.7 }, anchor: "middle" }],
    };
  }
  const y = -plane * k;
  const left = bounds.minX * k - SECTION_EXTENSION_MM;
  const right = bounds.maxX * k + SECTION_EXTENSION_MM;
  return {
    line: [{ x: left, y }, { x: right, y }],
    ends: [[{ x: left, y }, { x: left + SECTION_END_MM, y }], [{ x: right - SECTION_END_MM, y }, { x: right, y }]],
    arrows: [[{ x: left, y }, { x: left, y: y + SECTION_ARROW_MM }], [{ x: right, y }, { x: right, y: y + SECTION_ARROW_MM }]],
    letters: [{ at: { x: left - 1.5, y: y + ANNOTATION_TEXT_MM * 0.35 }, anchor: "end" }, { at: { x: right + 1.5, y: y + ANNOTATION_TEXT_MM * 0.35 }, anchor: "start" }],
  };
}

/** ✂️ Draws the cut-line markers of the sections a representation declares into the annotations layer of an elevation. */
export function drawSectionMarkers(canvas: Canvas, ctx: Ctx, sections: NonNullable<AuthoredRepresentation["sections"]>, bounds: Parameters<typeof sectionMarker>[3]): void {
  const ink = ctx.colours.ink;
  for (const viewpoint of ["front", "plan"] as const) {
    const section = sections[viewpoint];
    if (section === undefined) continue;
    const marker = sectionMarker(viewpoint, section.plane, ctx.k, bounds);
    const id = `section:${viewpoint}`;
    canvas.path("annotations", `${id}:line`, commandsOf([...marker.line], false), { stroke: { color: ink, widthMm: THIN_MM, cap: "butt" } });
    marker.ends.forEach((end, i) => canvas.path("annotations", `${id}:end-${i}`, commandsOf([...end], false), { stroke: { color: ink, widthMm: THICK_MM, cap: "butt" } }));
    marker.arrows.forEach(([tail, head], i) => canvas.path("annotations", `${id}:arrow-${i}`, commandsOf(arrowOutline(tail, head, 0.3, SECTION_ARROW_HEAD.length, SECTION_ARROW_HEAD.width), true), { stroke: { color: ink, widthMm: 0.05, join: "miter" }, fill: { color: ink } }));
    const letters = sectionLetters(section.label[ctx.language]);
    marker.letters.forEach((letter, i) => canvas.text("annotations", `${id}:letter-${i}`, letter.at, letters[i]!, ANNOTATION_TEXT_MM, ink, { anchor: letter.anchor, weight: "bold" }));
  }
}
