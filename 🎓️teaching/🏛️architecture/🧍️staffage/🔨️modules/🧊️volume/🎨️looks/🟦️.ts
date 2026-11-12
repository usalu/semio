/** 🎨️ The looks of the seven views as materials of one geometry.
 *
 * The geometry of a model is shared by every view: each solid has a base material (its {@link Surface}: bone, role, tone and thermal regions) and every view is a variant that rebinds the base materials to the materials of its look, exactly as the 2D depiction paints the same shapes in seven ways.
 * The base materials carry the `light-solid` look. `dark-solid` and `light-solid` paint the tone ramp of the theme (`toneColour`), glazing translucent; `dark-line` and `light-line` an unlit paper-coloured surface (glazing transparent) under ink feature edges;
 * the analytic looks (`thermal`, `forces`, `moments`) live in ../🌡️thermal and ../🏗️structure and return the same {@link LookSpec}.
 *
 * Colours are sRGB `#rrggbb` hex strings as everywhere in the depiction; a material holds them linear, as glTF and FBX expect.
 *
 * @see ../../🎨️depiction/🖍️palette/🟦️.ts — `THEMES`, `toneColour`
 */
import type { Slug, Text, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import type { Material, Variant } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { View } from "../../🔗️contracts/🟦️.ts";
import { type Theme, THEMES, toneColour, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { regionsOfSolid } from "../🧊️solids/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🔢️ The sRGB `#rrggbb` colour as linear `[r, g, b]`. */
export function linearOf(hex: string): [number, number, number] {
  const v = Number.parseInt(hex.slice(1), 16);
  const lin = (c: number): number => ((c /= 255) <= 0.04045 ? c / 12.92 : portable.pow(((c + 0.055) / 1.055), 2.4));
  return [lin((v >> 16) & 255), lin((v >> 8) & 255), lin(v & 255)];
}

/** 📤️ The linear `[r, g, b]` as sRGB `#rrggbb`. */
export function hexOf(linear: readonly number[]): string {
  const enc = (c: number): string => {
    const x = Math.max(0, Math.min(1, c));
    const s = x <= 0.0031308 ? x * 12.92 : 1.055 * portable.pow(x, (1 / 2.4)) - 0.055;
    return Math.round(s * 255).toString(16).padStart(2, "0");
  };
  return `#${enc(linear[0]!)}${enc(linear[1]!)}${enc(linear[2]!)}`;
}

/** 🖌️ A painted surface: sRGB colour and opacity. */
export type Paint = { readonly hex: string; readonly alpha: number };

/** 🧱️ The material of a paint: lit (rough, non-metallic) or unlit, opaque or blended by its alpha. */
export function materialOf(id: Slug, title: Text, paint: Paint, unlit = false): Material {
  const [r, g, b] = linearOf(paint.hex);
  return { id, title, baseColor: [r, g, b, paint.alpha], metallic: 0, roughness: 0.85, emissive: [0, 0, 0], alphaMode: paint.alpha < 1 ? "blend" : "opaque", doubleSided: paint.alpha < 1, unlit };
}

/** 🧩️ A surface: all solids that share bone, role, tone and thermal regions, so one base material paints them. */
export type Surface = { readonly id: Slug; readonly bone: Slug; readonly role: VolumeSolid["role"]; readonly tone: number; readonly regions: readonly Slug[]; readonly solids: readonly Slug[] };

/** 🧊️ The surfaces of a set of solids in order of first use, `surface-1`, `surface-2`, …, and the surface of every solid id. */
export function surfacesOf(solids: readonly VolumeSolid[]): { readonly surfaces: readonly Surface[]; readonly of: ReadonlyMap<Slug, Surface> } {
  const byKey = new Map<string, { id: Slug; bone: Slug; role: VolumeSolid["role"]; tone: number; regions: readonly Slug[]; solids: Slug[] }>();
  const of = new Map<Slug, Surface>();
  for (const solid of solids) {
    const regions = regionsOfSolid(solid);
    const key = `${solid.bone}|${solid.role}|${solid.tone}|${regions.join("+")}`;
    let surface = byKey.get(key);
    if (surface === undefined) {
      surface = { id: `surface-${byKey.size + 1}`, bone: solid.bone, role: solid.role, tone: solid.tone, regions, solids: [] };
      byKey.set(key, surface);
    }
    surface.solids.push(solid.id);
    of.set(solid.id, surface);
  }
  return { surfaces: [...byKey.values()], of };
}

/** 🎭️ One look of one view: the time-independent key of the material a surface gets, its title, whether it is unlit, and the paint of every key at one instant. Surfaces with equal keys share one material; `paint` of a static look ignores its argument. */
export type LookSpec<Frame = undefined> = {
  readonly view: View;
  readonly unlit: boolean;
  readonly keyOf: (surface: Surface) => string;
  readonly titleOf: (key: string, surface: Surface) => Text;
  readonly paint: (frame: Frame) => ReadonlyMap<string, Paint>;
};

const glazing = (surface: Surface): boolean => surface.role === "glazing";
const toneKey = (surface: Surface): string => `${Math.round(surface.tone * 1000)}${glazing(surface) ? "-glazing" : ""}`;

/** 🌗️ The tone-ramp look of the `dark-solid` and `light-solid` views: `toneColour` of the surface tone, glazing at 45 % opacity. */
export function solidLook(theme: Theme, view: "dark-solid" | "light-solid", surfaces: readonly Surface[]): LookSpec {
  const paints = new Map<string, Paint>();
  for (const s of surfaces) paints.set(toneKey(s), { hex: toneColour(theme, s.tone), alpha: glazing(s) ? 0.45 : 1 });
  return { view, unlit: false, keyOf: toneKey, titleOf: (key, s) => tx(`Tone ${Math.round(s.tone * 100)} %${glazing(s) ? ", glazing" : ""}`, `Tonwert ${Math.round(s.tone * 100)} %${glazing(s) ? ", Verglasung" : ""}`), paint: () => paints };
}

/** ✏️ The paper-surface look of the `dark-line` and `light-line` views: an unlit surface in the theme's paper colour that hides what lies behind it (glazing is not filled and stays transparent), drawn under the feature edges. */
export function lineLook(theme: Theme, view: "dark-line" | "light-line"): LookSpec {
  const paints = new Map<string, Paint>([["paper", { hex: THEMES[theme].background, alpha: 1 }], ["glazing", { hex: THEMES[theme].background, alpha: 0 }]]);
  return { view, unlit: true, keyOf: (s) => (glazing(s) ? "glazing" : "paper"), titleOf: (key) => (key === "glazing" ? tx("Transparent glazing", "Transparente Verglasung") : tx("Paper", "Papier")), paint: () => paints };
}

/** 🧵️ The base material of a surface in the `light-solid` look. */
export function baseMaterial(surface: Surface, title: Text): Material {
  return materialOf(surface.id, title, { hex: toneColour("light", surface.tone), alpha: glazing(surface) ? 0.45 : 1 });
}

/** 🏷️ The id of the material a look gives a key: `<view>-<n>`, numbered by first use. */
export function materialIds(view: View, surfaces: readonly Surface[], look: Pick<LookSpec<never>, "keyOf">): ReadonlyMap<string, Slug> {
  const ids = new Map<string, Slug>();
  for (const surface of surfaces) {
    const key = look.keyOf(surface);
    if (!ids.has(key)) ids.set(key, `${view}-${ids.size + 1}`);
  }
  return ids;
}

/** 🔭️ A variant of the document: the view's id and title, binding every base material id to the material of its look. */
export function variantOf(view: View, title: Text, bindings: Readonly<Record<Slug, Slug>>): Variant {
  return { id: view, title, bindings };
}
