/** 🖌️ The drawing of one actor as inline SVG: {@link depict} builds the element tree of a species once, {@link paint} applies a frame to it, {@link depictionMarkup} states the same drawing as text.
 *
 * The rules (design §6.2): one `<svg class="pet">` per actor, moved by its CSSOM `transform` — its feet, its facing,
 * its size and the tilt of the whole drawing about its pivot (design-v2 §22) — and `opacity`, coloured by the palette
 * custom properties under the tint of the state its species is in, and naming what the actor does, what carries it,
 * the state of its species it shows and its mood in `data-pet-activity`, `data-pet-footing`, `data-pet-state` and
 * `data-pet-mood` (for whoever watches the pets from outside: a test, a tool, a host's stylesheet);
 * one `<g>` per run of parts drawn one after the other on the same bone, carrying the matrix of that bone, numbers
 * rounded to three decimals and rewritten only once the drawing moved visibly; the face (eyes squashed by
 * their lids, pupils where the frame puts them — the core lets them travel up to the outline of the white —, a mouth
 * bent by the spirits) above the part the species names. Colours come from the classes of `🎨️.css` and the three palette
 * custom properties, never from attributes, because a presentation attribute cannot hold `var()` in every browser.
 *
 * Safe under a Content-Security-Policy without inline styles: elements are created with `createElementNS`, a frame
 * writes attributes and CSSOM properties only, and only those whose value changed visibly. No `innerHTML`, no `<style>`,
 * no `setAttribute("style")`, no ids. Every written attribute costs the page a style recalculation of its element, a
 * repaint and a raster of the pet: fewer groups and fewer writes are what keep pets cheap.
 *
 * @see ../../🎨️.css — the paint classes
 * @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src-attr — what a policy blocks
 * @see https://www.w3.org/TR/css-transforms-1/#mathematical-description — the matrix convention
 */

import { ACTIVITIES, FOOTINGS, MOODS, type ActorFrame, type Eye, type Mouth, type Palette, type Part, type Point, type Species, type Tint, type Turns } from "@semio-tech/pets";

//#region 🔖️Rules
/** 🌐️ The namespace every element of a drawing is created in. */
export const SVG_NAMESPACE = "http://www.w3.org/2000/svg";
const REST_MATRIX = [1, 0, 0, 1, 0, 0] as const;
const FACE = "face";
const FACE_STROKE = "1.5";
const PART_STROKE = 2;
const LID_SQUASH = 0.9;
const MOUTH_BEND = 0.5;
const ACTOR_SLOTS = 12;
const EYE_SLOTS = 3;
const ACTIVITY = "data-pet-activity";
const FOOTING = "data-pet-footing";
const STATE = "data-pet-state";
const MOOD = "data-pet-mood";
const PLACE = 100;
const TONES = ["body", "accent", "detail"] as const;

/** 🏷️ The attributes of an element, in the order they are written. */
export type Attributes = readonly (readonly [name: string, value: string])[];

/** 🔷️ The elements a part can be. */
export type ShapeTag = "path" | "ellipse" | "rect" | "line";

/** 🖍️ What is drawn of a part: its geometry and its paints. A particle of an emitter and the canopy of a parachute are drawn by the same rule. */
export type Drawn = Pick<Part, "shape" | "fill" | "stroke" | "strokeWidth">;

/** 🔢️ A number at the precision of the drawing: `digits` decimals as a power of ten (1000 = three decimals). */
export function rounded(value: number, digits = 1000): number {
  return Math.round(value * digits) / digits;
}

/** 🧱️ The maker of SVG elements in `document`: an element of `tag` carrying `attributes`, written in their order. */
export function maker(document: Document): <Tag extends "svg" | "g" | "circle" | ShapeTag>(tag: Tag, attributes: Attributes) => SVGElementTagNameMap[Tag] {
  return (tag, attributes) => {
    const created = document.createElementNS(SVG_NAMESPACE, tag);
    for (const [name, value] of attributes) created.setAttribute(name, value);
    return created;
  };
}

/** 🧮️ `matrix(a b c d e f)` of six rounded numbers starting at `offset`. */
function matrixText(numbers: ArrayLike<number>, offset: number): string {
  return `matrix(${numbers[offset]} ${numbers[offset + 1]} ${numbers[offset + 2]} ${numbers[offset + 3]} ${numbers[offset + 4]} ${numbers[offset + 5]})`;
}

/** 🧭️ Where an actor stands and which way it faces, as a CSS transform. */
export function placementText(x: number, y: number, flip: number, scale: number): string {
  return `translate(${x}px, ${y}px) scale(${flip}, ${scale})`;
}

/** 📐️ How far a drawing turns inside its own, possibly mirrored box for a tilt on screen, in degrees at the precision of the drawing: a mirrored drawing turns the other way round to lean the same way on screen. */
export function tiltDegrees(facing: number, tilt: Turns): number {
  return rounded((facing < 0 ? -tilt : tilt) * 360, PLACE);
}

/** 🎢️ Where an actor stands, which way it faces and how its whole drawing is tilted about `pivot`, as the CSS transform of its `<svg class="pet">`: `x`, `y`, `flip` (the facing times the size) and `scale` as {@link placementText} places an upright actor, then the turn of {@link tiltDegrees} about the pivot (in the coordinates of the rig: feet at the origin, mirrored with the actor). An actor that is not tilted gets exactly the transform of an upright one. */
export function tiltedPlacement(x: number, y: number, flip: number, scale: number, tilt: Turns, pivot: Point): string {
  const degrees = tiltDegrees(flip, tilt);
  const upright = placementText(x, y, flip, scale);
  if (degrees === 0) return upright;
  const px = rounded(pivot.x, PLACE);
  const py = rounded(pivot.y, PLACE);
  return `${upright} translate(${px}px, ${py}px) rotate(${degrees}deg) translate(${-px}px, ${-py}px)`;
}

/** 💡️ Paints `tint` over `palette` on `element` through the three palette custom properties: every colour the tint names replaces the species' own, every other one is the species' own again — so a state without a tint restores the palette. Writes a property only when its value changes. */
export function tintPalette(element: ElementCSSInlineStyle, palette: Palette, tint?: Tint): void {
  for (const tone of TONES) {
    const name = `--pet-${tone}`;
    const colour = tint?.[tone] ?? palette[tone];
    if (element.style.getPropertyValue(name) !== colour) element.style.setProperty(name, colour);
  }
}

/** 👁️ The place of an eye on its bone, squashed vertically to `open` (1 = open). */
function socketText(eye: Eye, open: number): string {
  return `translate(${eye.x} ${eye.y}) scale(1 ${open})`;
}

/** 👄️ The curve of a mouth whose middle is pulled to the height `bend`. */
function mouthText(mouth: Mouth, bend: number): string {
  return `M ${rounded(mouth.x - mouth.width / 2)} ${mouth.y} Q ${mouth.x} ${bend} ${rounded(mouth.x + mouth.width / 2)} ${mouth.y}`;
}

/** 😌️ How far a lid leaves its eye open: 1 open, 0.1 shut. */
function openness(lid: number): number {
  return rounded(1 - LID_SQUASH * lid);
}

/** 🙂️ The height the middle of a mouth is pulled to by spirits between −1 and 1. */
function bendOf(mouth: Mouth, spirits: number): number {
  return rounded(mouth.y + mouth.width * MOUTH_BEND * spirits);
}

/** 🧩️ The element of a part: its geometry, its paint classes and its stroke width (none when it has no stroke). */
export function partShape(part: Drawn): readonly [tag: ShapeTag, attributes: Attributes] {
  const shape = part.shape;
  const look: Attributes = [["class", `pet-fill-${part.fill} pet-stroke-${part.stroke}`], ...(part.stroke === "none" ? [] : [["stroke-width", String(part.strokeWidth ?? PART_STROKE)] as const])];
  switch (shape.kind) {
    case "path":
      return ["path", [["d", shape.d], ...look]];
    case "ellipse":
      return ["ellipse", [["cx", String(shape.cx)], ["cy", String(shape.cy)], ["rx", String(shape.rx)], ["ry", String(shape.ry)], ...look]];
    case "rect":
      return ["rect", [["x", String(shape.x)], ["y", String(shape.y)], ["width", String(shape.width)], ["height", String(shape.height)], ...(shape.radius === undefined ? [] : [["rx", String(shape.radius)] as const]), ...look]];
    case "line":
      return ["line", [["x1", String(shape.x1)], ["y1", String(shape.y1)], ["x2", String(shape.x2)], ["y2", String(shape.y2)], ...look]];
  }
}

/** 🪪️ What the root element of every depiction carries. */
function rootAttributes(species: Species): Attributes {
  return [
    ["class", "pet"],
    ["data-pet", species.id],
    ["focusable", "false"],
    ["overflow", "visible"],
  ];
}

/** ⚪️ What the white of an eye carries. */
function whiteAttributes(eye: Eye): Attributes {
  return [
    ["class", "pet-fill-paper pet-stroke-ink"],
    ["stroke-width", FACE_STROKE],
    ["r", String(eye.radius)],
  ];
}

/** ⚫️ What a pupil carries before a frame places it. */
function pupilAttributes(eye: Eye): Attributes {
  return [
    ["class", "pet-pupil"],
    ["r", String(eye.pupil)],
  ];
}

const MOUTH_ATTRIBUTES: Attributes = [
  ["class", "pet-fill-none pet-stroke-ink"],
  ["stroke-width", FACE_STROKE],
];

/** 🗂️ What a species is drawn from, back to front: its parts, and its face after the part it names (last when it names none). */
function layers(species: Species): readonly (Part | typeof FACE)[] {
  const drawn: (Part | typeof FACE)[] = [];
  for (const part of species.parts) {
    drawn.push(part);
    if (part.id === species.face.above) drawn.push(FACE);
  }
  if (!drawn.includes(FACE)) drawn.push(FACE);
  return drawn;
}

/** 🔩️ One thing a bone carries in a drawing: a part, an eye or the mouth. */
type Piece = { readonly kind: "part"; readonly part: Part } | { readonly kind: "eye"; readonly eye: Eye } | { readonly kind: "mouth"; readonly mouth: Mouth };

/** 🦴️ A run of pieces drawn one after the other that follow the same bone, and so share one group. */
type Run = { readonly bone: string; readonly pieces: readonly Piece[] };

/** 🧷️ What a species is drawn from, back to front ({@link layers}, the face as its eyes and its mouth), as runs of pieces that follow the same bone: one group carries the matrix of a run, so a bone that moves is written once per run, not once per piece. */
function runs(species: Species): readonly Run[] {
  const drawn: { readonly bone: string; readonly pieces: Piece[] }[] = [];
  const add = (bone: string, piece: Piece): void => {
    const last = drawn.at(-1);
    if (last?.bone === bone) last.pieces.push(piece);
    else drawn.push({ bone, pieces: [piece] });
  };
  for (const layer of layers(species)) {
    if (layer !== FACE) {
      add(layer.bone, { kind: "part", part: layer });
      continue;
    }
    for (const eye of species.face.eyes) add(eye.bone, { kind: "eye", eye });
    if (species.face.mouth) add(species.face.mouth.bone, { kind: "mouth", mouth: species.face.mouth });
  }
  return drawn;
}

/** 📏️ The least distance a drawn point must move, in pixels at pet size 1, before a frame writes it anew: a bone's offset, a pupil, the middle of a mouth. A twentieth of a pixel is far below what an eye can tell, yet most frames of a resting or breathing pet move its bones by less; holding the drawing until it moved that far spares the page the style, paint and raster of a write nobody can see. */
export const PAINT_SHIFT = 0.05;

/** 🌀️ The least change of a linear entry of a bone's matrix (its turn, stretch and skew) or of a lid's openness before a frame writes it anew: on a bone at most twenty-five pixels long that moves its tip by a twentieth of a pixel ({@link PAINT_SHIFT}). */
export const PAINT_LINEAR = 0.002;

/** 👣️ Whether `value` is at least `step` away from what is painted (`known`), or nothing is painted yet. */
function visibly(value: number, known: number, step: number): boolean {
  return !(Math.abs(value - known) < step);
}
//#endregion 🔖️Rules

//#region 🔖️Depiction
/** 🖼️ The element tree of one actor and what was last painted on it: `element` is the `<svg class="pet">` to place in a
 * layer; `followers` lists per bone (rig order) the groups that carry its matrix (one per run of pieces that follow
 * it), `sockets` per eye the group its lid squashes, `pupils` per eye its pupil, `mouth` the mouth of a species that
 * has one. `painted` holds the rounded numbers
 * on the tree — place (x, y), horizontal and vertical scale, opacity, the index of the activity, the tilt in degrees and
 * the pivot it turns about (x, y; 0 while upright), the index of the state, the indices of the footing and the mood, six
 * per bone, three per eye (pupil x, y, openness), the bend of the mouth — and is not a number anywhere before the first
 * frame. */
export type Depiction = {
  readonly species: Species;
  readonly element: SVGSVGElement;
  readonly followers: readonly (readonly SVGGElement[])[];
  readonly sockets: readonly SVGGElement[];
  readonly pupils: readonly SVGCircleElement[];
  readonly mouth: SVGPathElement | null;
  readonly painted: Float64Array;
};

/** 🏗️ Builds the element tree of `species` in `document`: every part and the face in drawing order, one group per run of pieces that follow the same bone ({@link runs}), the palette as custom properties. The tree shows nothing sensible until {@link paint} applied a first frame. */
export function depict(species: Species, document: Document = globalThis.document): Depiction {
  const create = maker(document);
  const element = create("svg", rootAttributes(species));
  tintPalette(element, species.palette);
  const followers: SVGGElement[][] = species.bones.map(() => []);
  const sockets: SVGGElement[] = [];
  const pupils: SVGCircleElement[] = [];
  let mouth: SVGPathElement | null = null;
  for (const run of runs(species)) {
    const group = create("g", []);
    followers[species.bones.findIndex((candidate) => candidate.id === run.bone)]?.push(group);
    element.append(group);
    for (const piece of run.pieces) {
      if (piece.kind === "part") group.append(create(...partShape(piece.part)));
      else if (piece.kind === "eye") {
        const socket = create("g", []);
        const pupil = create("circle", pupilAttributes(piece.eye));
        socket.append(create("circle", whiteAttributes(piece.eye)), pupil);
        group.append(socket);
        sockets.push(socket);
        pupils.push(pupil);
      } else {
        mouth = create("path", MOUTH_ATTRIBUTES);
        group.append(mouth);
      }
    }
  }
  return { species, element, followers, sockets, pupils, mouth, painted: new Float64Array(ACTOR_SLOTS + species.bones.length * 6 + species.face.eyes.length * EYE_SLOTS + 1).fill(Number.NaN) };
}

/** 🎨️ Applies `frame` to the tree at `scale`: where the actor stands and faces and how far its whole drawing is tilted about its pivot ({@link tiltedPlacement}: a mirrored drawing turns the other way inside its box, so it leans the same way on screen), its opacity, the tint of its species' state over the palette (an unknown state or one without a tint shows the palette), what it does, what carries it, the state it shows (no `data-pet-state` for a state its species does not have) and its mood, the matrix of every bone, its pupils, lids and the mouth its spirits bend. Writes an attribute or a CSSOM property only where the rounded value differs from what is painted — the matrix of a bone, a pupil, a lid and the mouth only once the drawing moved by at least {@link PAINT_SHIFT} or {@link PAINT_LINEAR} since it was written, so the painted drawing never strays from the frame by more than that and a pet that only breathes writes a fraction of its frames; the palette and the state's name only when the state changes. */
export function paint(depiction: Depiction, frame: ActorFrame, scale = 1): void {
  const { species, element, followers, sockets, pupils, mouth, painted } = depiction;
  const x = rounded(frame.x, PLACE);
  const y = rounded(frame.y, PLACE);
  const flip = rounded(frame.facing * scale);
  const size = rounded(scale);
  const degrees = tiltDegrees(flip, frame.tilt);
  const pivotX = degrees === 0 ? 0 : rounded(frame.pivot.x, PLACE);
  const pivotY = degrees === 0 ? 0 : rounded(frame.pivot.y, PLACE);
  if (x !== painted[0] || y !== painted[1] || flip !== painted[2] || size !== painted[3] || degrees !== painted[6] || pivotX !== painted[7] || pivotY !== painted[8]) {
    painted[0] = x;
    painted[1] = y;
    painted[2] = flip;
    painted[3] = size;
    painted[6] = degrees;
    painted[7] = pivotX;
    painted[8] = pivotY;
    element.style.transform = tiltedPlacement(x, y, flip, size, frame.tilt, frame.pivot);
  }
  const state = species.states.findIndex((candidate) => candidate.id === frame.state);
  const restated = state !== painted[9];
  if (restated) {
    painted[9] = state;
    tintPalette(element, species.palette, species.states[state]?.tint);
  }
  const opacity = rounded(frame.opacity);
  if (opacity !== painted[4]) {
    painted[4] = opacity;
    element.style.opacity = String(opacity);
  }
  const activity = ACTIVITIES.indexOf(frame.activity);
  if (activity !== painted[5]) {
    painted[5] = activity;
    element.setAttribute(ACTIVITY, frame.activity);
  }
  const footing = FOOTINGS.indexOf(frame.footing);
  if (footing !== painted[10]) {
    painted[10] = footing;
    element.setAttribute(FOOTING, frame.footing);
  }
  if (restated) {
    const shown = species.states[state];
    if (shown === undefined) element.removeAttribute(STATE);
    else element.setAttribute(STATE, shown.id);
  }
  const mood = MOODS.indexOf(frame.mood);
  if (mood !== painted[11]) {
    painted[11] = mood;
    element.setAttribute(MOOD, frame.mood);
  }
  for (let bone = 0; bone < followers.length; bone++) {
    const groups = followers[bone]!;
    if (groups.length === 0) continue;
    const slot = ACTOR_SLOTS + bone * 6;
    let moved = false;
    for (let entry = 0; entry < 6 && !moved; entry++) moved = visibly(rounded(frame.bones[bone * 6 + entry] ?? REST_MATRIX[entry]!), painted[slot + entry]!, entry < 4 ? PAINT_LINEAR : PAINT_SHIFT);
    if (!moved) continue;
    for (let entry = 0; entry < 6; entry++) painted[slot + entry] = rounded(frame.bones[bone * 6 + entry] ?? REST_MATRIX[entry]!);
    const text = matrixText(painted, slot);
    for (const group of groups) group.setAttribute("transform", text);
  }
  const faceSlot = ACTOR_SLOTS + followers.length * 6;
  for (let index = 0; index < sockets.length; index++) {
    const look = frame.eyes[index];
    const slot = faceSlot + index * EYE_SLOTS;
    const pupilX = rounded(look?.x ?? 0);
    const pupilY = rounded(look?.y ?? 0);
    const open = openness(look?.lid ?? 0);
    if (visibly(pupilX, painted[slot]!, PAINT_SHIFT)) {
      painted[slot] = pupilX;
      pupils[index]!.setAttribute("cx", String(pupilX));
    }
    if (visibly(pupilY, painted[slot + 1]!, PAINT_SHIFT)) {
      painted[slot + 1] = pupilY;
      pupils[index]!.setAttribute("cy", String(pupilY));
    }
    if (visibly(open, painted[slot + 2]!, PAINT_LINEAR)) {
      painted[slot + 2] = open;
      sockets[index]!.setAttribute("transform", socketText(species.face.eyes[index]!, open));
    }
  }
  if (mouth && species.face.mouth) {
    const slot = faceSlot + sockets.length * EYE_SLOTS;
    const bend = bendOf(species.face.mouth, frame.spirits);
    if (visibly(bend, painted[slot]!, PAINT_SHIFT)) {
      painted[slot] = bend;
      mouth.setAttribute("d", mouthText(species.face.mouth, bend));
    }
  }
}
//#endregion 🔖️Depiction

//#region 🔖️Markup
/** 🔏️ A text as the value of an attribute. */
function escaped(text: string): string {
  return text.replace(/&/gu, "&amp;").replace(/</gu, "&lt;").replace(/>/gu, "&gt;").replace(/"/gu, "&quot;");
}

/** 🔤️ An element as text. */
function tagText(tag: string, attributes: Attributes, children = ""): string {
  return `<${tag}${attributes.map(([name, value]) => ` ${name}="${escaped(value)}"`).join("")}>${children}</${tag}>`;
}

/** 📜️ The drawing of `species` at `frame` as SVG text: the tree {@link depict} builds once {@link paint} applied the frame, without what only the CSSOM carries (place, opacity and palette). For tests and tools; a page sets the palette custom properties on an ancestor. */
export function depictionMarkup(species: Species, frame: ActorFrame): string {
  const following = (bone: string, children: string): string => {
    const index = species.bones.findIndex((candidate) => candidate.id === bone);
    const matrix = REST_MATRIX.map((rest, entry) => rounded(frame.bones[index * 6 + entry] ?? rest));
    return tagText("g", index < 0 ? [] : [["transform", matrixText(matrix, 0)]], children);
  };
  const pieceText = (piece: Piece): string => {
    if (piece.kind === "part") return tagText(...partShape(piece.part));
    if (piece.kind === "mouth") return tagText("path", [...MOUTH_ATTRIBUTES, ["d", mouthText(piece.mouth, bendOf(piece.mouth, frame.spirits))]]);
    const look = frame.eyes[species.face.eyes.indexOf(piece.eye)];
    const pupil = tagText("circle", [...pupilAttributes(piece.eye), ["cx", String(rounded(look?.x ?? 0))], ["cy", String(rounded(look?.y ?? 0))]]);
    return tagText("g", [["transform", socketText(piece.eye, openness(look?.lid ?? 0))]], tagText("circle", whiteAttributes(piece.eye)) + pupil);
  };
  const state = species.states.find((candidate) => candidate.id === frame.state);
  const named: Attributes = [[ACTIVITY, frame.activity], [FOOTING, frame.footing], ...(state === undefined ? [] : [[STATE, state.id] as const]), [MOOD, frame.mood]];
  return tagText("svg", [...rootAttributes(species), ...named], runs(species).map((run) => following(run.bone, run.pieces.map(pieceText).join(""))).join(""));
}
//#endregion 🔖️Markup
