/** 🏗️ The structural looks of the model: one material per bone coloured by the member quantity (forces: axial N, moments: bending moment M) with the very ramp, symmetric range about zero, sign convention (compression warm, tension cool) and member filter of the 2D force and moment views (`memberPaint`, `eligibleBones`),
 * the force and moment glyphs of the same result as arrows and arcs with the paper sizes of the 2D views, and the centre of mass.
 *
 * Bones that are not members (no section and shorter than 5 % of the height), glazing, texture and foliage are neutral tone grey; ghosted (glazing, texture, foliage) at 40 % opacity, exactly as in 2D.
 *
 * @see ../../🎨️depiction/🏗️structural-view/🟦️.ts — `memberPaint`, `forceList`, `momentList`
 */
import type { Asset, Slug, Text } from "../../../🧬️schema/🟦️.ts";
import type { StructuralResult } from "../../🔗️contracts/🟦️.ts";
import { type MemberPaint, memberPaint } from "../../🎨️depiction/🏗️structural-view/🟦️.ts";
import { type Theme, THEMES, overlayColours, toneColour, tx } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import type { DisplayRange } from "../../🔗️contracts/🟦️.ts";
import type { StructuralQuantity } from "../../🏗️structure/🟦️.ts";
import { type LookSpec, type Paint, type Surface } from "../🎨️looks/🟦️.ts";

/** ⚙️ What a member look reads besides the result. */
export type MemberSettings = { readonly view: "forces" | "moments"; readonly quantity: StructuralQuantity; readonly range: DisplayRange; readonly theme: Theme; readonly eligible: (bone: Slug) => boolean };

/** 🖌️ The structural look of a set of surfaces; the paint of a key at an instant is read from the structural result. */
export function memberLook(asset: Asset, settings: MemberSettings, surfaces: readonly Surface[]): LookSpec<StructuralResult> {
  const regionKind = (id: Slug): string | undefined => asset.thermal.regions.find((r) => r.id === id)?.kind;
  const ghost = (s: Surface): boolean => s.role === "glazing" || s.role === "texture" || (s.regions[0] !== undefined && regionKind(s.regions[0]) === "foliage");
  const keyOf = (s: Surface): string => (ghost(s) || !settings.eligible(s.bone) ? `neutral-${Math.round(s.tone * 1000)}${ghost(s) ? "-ghost" : ""}` : `bone-${s.bone}-${Math.round(s.tone * 1000)}`);
  const representative = new Map<string, Surface>();
  for (const s of surfaces) if (!representative.has(keyOf(s))) representative.set(keyOf(s), s);
  const boneTitle = (id: Slug): Text => asset.rig.bones.find((b) => b.id === id)?.title ?? tx(id, id);
  return {
    view: settings.view,
    unlit: false,
    keyOf,
    titleOf: (key, s) => (key.startsWith("neutral") ? tx("Neutral", "Neutral") : boneTitle(s.bone)),
    paint: (result) => {
      const paint: MemberPaint = memberPaint(result, settings.quantity, settings.range, settings.theme, settings.eligible);
      const out = new Map<string, Paint>();
      for (const [key, s] of representative) {
        const neutral = { hex: toneColour(settings.theme, s.tone), alpha: ghost(s) ? 0.4 : 1 };
        if (ghost(s) || !settings.eligible(s.bone)) {
          out.set(key, neutral);
          continue;
        }
        const value = paint.valueOf(s.bone);
        out.set(key, value === null ? neutral : { hex: paint.colourOf(value), alpha: 1 });
      }
      return out;
    },
  };
}

/** 🏹️ The kinds of force arrows. */
export type ArrowKind = "applied" | "reaction" | "weight" | "inertial";
/** 🔄️ The kinds of moment arcs. */
export type ArcKind = "joint" | "reaction" | "applied";

/** 🎨️ The paint of a force arrow kind: red applied, teal reaction, grey weight, orange inertia, 90 % opaque. */
export function arrowPaint(theme: Theme, kind: ArrowKind): Paint {
  const o = overlayColours(theme);
  return { hex: kind === "applied" ? o.applied : kind === "reaction" ? o.reaction : kind === "weight" ? o.weight : o.joint, alpha: 0.9 };
}

/** 🖍️ The paint of a moment arc kind: orange joint, teal reaction, red applied, 90 % opaque. */
export function arcPaint(theme: Theme, kind: ArcKind): Paint {
  const o = overlayColours(theme);
  return { hex: kind === "joint" ? o.joint : kind === "reaction" ? o.reaction : o.applied, alpha: 0.9 };
}

/** ⚖️ The paint of the centre-of-mass sphere: the ink of the theme. */
export const massPaint = (theme: Theme): Paint => ({ hex: THEMES[theme].ink, alpha: 0.9 });
