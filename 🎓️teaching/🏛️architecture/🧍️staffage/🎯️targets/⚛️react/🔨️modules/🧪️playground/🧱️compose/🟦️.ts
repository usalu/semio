/** 🧱️ Composition of a playground into one SVG at true paper offsets: pure functions over already drawn parts.
 *
 * Every part is a scene drawn by the depiction (its own SVG, its own origin where world (0, 0) lies on the paper). A placement at `xM` metres goes to
 * `margin + xM · mmPerMetre − origin.x` millimetres, its ground point onto one shared ground line, so the exported file measures exactly what the screen shows at that drawing scale.
 * Mirrored placements flip around their own ground point. The file carries `<title>` and `<metadata>` with scale, view and asset ids.
 *
 * @see ../🟦️.tsx — the page that draws the parts and downloads the result
 * @see https://www.w3.org/TR/SVG2/struct.html#SVGElement — nested `svg` elements with x, y, width, height and viewBox
 */
import { SVG_NAMESPACE, mmPerMetre } from "@teaching/staffage";
import type { Language, PlanarViewpoint } from "@teaching/staffage";
import type { Background } from "../../💾️persistence/🟦️.ts";

/** 📍️ Where a placement projects on the line in a viewpoint, from its one 3D position (x along the line, z sideways, the asset's left +): the horizontal position along the paper in metres, the lift above the ground line in metres (plan only: paper up is +z), and the nearness that orders overlapping figures (larger is nearer and painted later). The side elevation looks along z (smaller z is nearer), the front elevation along -x (larger x is nearer), the plan has no overlap order. */
export function linePosition(viewpoint: PlanarViewpoint, placement: { readonly xM: number; readonly zM: number }, lengthM: number): { readonly horizontalM: number; readonly liftM: number; readonly nearness: number } {
  if (viewpoint === "front") return { horizontalM: lengthM / 2 + placement.zM, liftM: 0, nearness: placement.xM };
  if (viewpoint === "plan") return { horizontalM: placement.xM, liftM: placement.zM, nearness: 0 };
  return { horizontalM: placement.xM, liftM: 0, nearness: -placement.zM };
}

/** 🧩️ One drawn placement: its SVG markup and the paper metrics of its scene. */
export type ComposedPart = {
  readonly id: string;
  readonly asset: string;
  readonly title: string;
  readonly xM: number;
  readonly mirrored: boolean;
  readonly markup: string;
  readonly widthMm: number;
  readonly heightMm: number;
  readonly originXMm: number;
  readonly originYMm: number;
  readonly liftM?: number;
};

/** 🎛️ How the parts are put together. */
export type ComposeOptions = { readonly scale: number; readonly view: string; readonly viewpoint: string; readonly lengthM: number; readonly background: Background; readonly language: Language; readonly title: string; readonly marginMm?: number };

/** 📏️ The paper margin around the ground line, millimetres. */
export const MARGIN_MM = 8;

const PAPER: Readonly<Record<Exclude<Background, "transparent">, { readonly fill: string; readonly ink: string }>> = {
  paper: { fill: "#f4efe1", ink: "#1b1b1b" },
  dark: { fill: "#0b1a1f", ink: "#e8e4d4" },
  light: { fill: "#ffffff", ink: "#1b1b1b" },
};

const escapeXml = (text: string): string => text.replace(/[&<>"]/gu, (char) => `&#${char.charCodeAt(0)};`);
const fixed = (value: number): string => String(Number(value.toFixed(3)));

/** 🧲️ Rounds a position to the grid of `stepM` metres. */
export const snap = (xM: number, stepM = 0.05): number => Math.round(xM / stepM) * stepM;

/** 📐️ Where a part's nested `svg` sits on the paper: left and top in millimetres. */
export function partOffsetMm(part: Pick<ComposedPart, "xM" | "originXMm" | "originYMm" | "liftM">, scale: number, groundMm: number, marginMm = MARGIN_MM): { readonly xMm: number; readonly yMm: number } {
  const k = mmPerMetre({ denominator: scale });
  return { xMm: marginMm + part.xM * k - part.originXMm, yMm: groundMm - part.originYMm - (part.liftM ?? 0) * k };
}

/** 🏞️ The paper height above the ground line that holds the tallest part. */
export const aboveGroundMm = (parts: readonly ComposedPart[], scale = 100): number => Math.max(0, ...parts.map((part) => part.originYMm + (part.liftM ?? 0) * mmPerMetre({ denominator: scale })));

/** 📤️ The paper height below the ground line that holds the lowest part (a plan figure sideways of the line). */
export const belowGroundMm = (parts: readonly ComposedPart[], scale = 100): number => Math.max(0, ...parts.map((part) => part.heightMm - part.originYMm - (part.liftM ?? 0) * mmPerMetre({ denominator: scale })));

/** 🔧️ The inner content of an SVG document: everything between its root tags. */
export function innerSvg(markup: string): string {
  const open = /<svg\b[^>]*>/u.exec(markup);
  const end = markup.lastIndexOf("</svg>");
  return open === null || end < 0 ? "" : markup.slice(open.index + open[0].length, end);
}

function scaleBar(options: ComposeOptions, groundMm: number, ink: string): string {
  const k = mmPerMetre({ denominator: options.scale });
  const step = options.scale >= 500 ? 5 : options.scale >= 200 ? 2 : 1;
  const ticks: string[] = [];
  for (let metre = 0; metre <= options.lengthM + 1e-9; metre += step) {
    const x = MARGIN_MM + metre * k;
    ticks.push(`<line x1="${fixed(x)}" y1="${fixed(groundMm)}" x2="${fixed(x)}" y2="${fixed(groundMm + 2.5)}" stroke="${ink}" stroke-width="0.18"/>`, `<text x="${fixed(x)}" y="${fixed(groundMm + 6)}" font-size="2.5" font-family="sans-serif" text-anchor="middle" fill="${ink}">${metre} m</text>`);
  }
  return `<g id="scale-bar" data-step-m="${step}">${ticks.join("")}</g>`;
}

/** 🖼️ The whole composition as one SVG document, `width` and `height` in millimetres and a viewBox in the same unit. */
export function composeSvg(parts: readonly ComposedPart[], options: ComposeOptions): string {
  const k = mmPerMetre({ denominator: options.scale });
  const margin = options.marginMm ?? MARGIN_MM;
  const groundMm = margin + aboveGroundMm(parts, options.scale);
  const widthMm = 2 * margin + options.lengthM * k;
  const heightMm = groundMm + margin + 8 + belowGroundMm(parts, options.scale);
  const palette = PAPER[options.background === "transparent" ? "light" : options.background];
  const body = parts
    .map((part) => {
      const { xMm, yMm } = partOffsetMm(part, options.scale, groundMm, margin);
      const gx = margin + part.xM * k;
      const nested = `<svg x="${fixed(xMm)}" y="${fixed(yMm)}" width="${fixed(part.widthMm)}" height="${fixed(part.heightMm)}" viewBox="0 0 ${fixed(part.widthMm)} ${fixed(part.heightMm)}" overflow="visible">${innerSvg(part.markup)}</svg>`;
      const flip = part.mirrored ? ` transform="translate(${fixed(gx)} 0) scale(-1 1) translate(${fixed(-gx)} 0)"` : "";
      return `<g id="placement-${escapeXml(part.id)}" data-asset="${escapeXml(part.asset)}" data-x-m="${fixed(part.xM)}"${flip}><title>${escapeXml(part.title)}</title>${nested}</g>`;
    })
    .join("");
  const assets = [...new Set(parts.map((part) => part.asset))];
  const meta = `<metadata><staffage xmlns="${SVG_NAMESPACE}" scale="1:${fixed(options.scale)}" view="${escapeXml(options.view)}" viewpoint="${escapeXml(options.viewpoint)}" length-m="${fixed(options.lengthM)}" mm-per-metre="${fixed(k)}" language="${options.language}" assets="${escapeXml(assets.join(" "))}"/></metadata>`;
  const background = options.background === "transparent" ? "" : `<rect id="background" width="${fixed(widthMm)}" height="${fixed(heightMm)}" fill="${palette.fill}"/>`;
  const ground = `<line id="ground" x1="${fixed(margin)}" y1="${fixed(groundMm)}" x2="${fixed(margin + options.lengthM * k)}" y2="${fixed(groundMm)}" stroke="${palette.ink}" stroke-width="0.5"/>`;
  return `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" width="${fixed(widthMm)}mm" height="${fixed(heightMm)}mm" viewBox="0 0 ${fixed(widthMm)} ${fixed(heightMm)}"><title>${escapeXml(options.title)} 1:${fixed(options.scale)}</title>${meta}${background}${body}${ground}${scaleBar(options, groundMm, palette.ink)}</svg>\n`;
}
