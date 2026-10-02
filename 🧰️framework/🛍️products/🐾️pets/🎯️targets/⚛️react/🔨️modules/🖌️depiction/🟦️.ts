/** 🖌️ The drawing of one actor as inline SVG: {@link depict} builds the element tree of a species once, {@link paint} applies a frame to it, {@link depictionMarkup} states the same drawing as text.
 *
 * The rules (design §6.2): one `<svg class="pet">` per actor, moved by its CSSOM `transform` and `opacity` and naming
 * what the actor does in `data-pet-activity` (for whoever watches the pets from outside: a test, a host's stylesheet);
 * one `<g>` per part carrying the matrix of its bone, numbers rounded to three decimals; the face (eyes squashed by
 * their lids, pupils where the frame puts them — the core lets them travel up to the outline of the white —, a mouth
 * bent by the mood) above the part the species names. Colours come from the classes of `🎨️.css` and the three palette
 * custom properties, never from attributes, because a presentation attribute cannot hold `var()` in every browser.
 *
 * Safe under a Content-Security-Policy without inline styles: elements are created with `createElementNS`, a frame
 * writes attributes and CSSOM properties only, and only those whose rounded value changed. No `innerHTML`, no `<style>`,
 * no `setAttribute("style")`, no ids.
 *
 * @see ../../🎨️.css — the paint classes
 * @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src-attr — what a policy blocks
 * @see https://www.w3.org/TR/css-transforms-1/#mathematical-description — the matrix convention
 */

import { ACTIVITIES, type ActorFrame, type Eye, type Mouth, type Part, type Species } from "@semio-tech/pets";

//#region 🔖️Rules
const SVG_NAMESPACE = "http://www.w3.org/2000/svg";
const REST_MATRIX = [1, 0, 0, 1, 0, 0] as const;
const FACE = "face";
const FACE_STROKE = "1.5";
const PART_STROKE = 2;
const LID_SQUASH = 0.9;
const MOUTH_BEND = 0.5;
const ACTOR_SLOTS = 6;
const EYE_SLOTS = 3;
const ACTIVITY = "data-pet-activity";

/** 🏷️ The attributes of an element, in the order they are written. */
type Attributes = readonly (readonly [name: string, value: string])[];

/** 🔷️ The elements a part can be. */
type ShapeTag = "path" | "ellipse" | "rect" | "line";

/** 🔢️ A number at the precision of the drawing: `digits` decimals as a power of ten (1000 = three decimals). */
function rounded(value: number, digits = 1000): number {
  return Math.round(value * digits) / digits;
}

/** 🧮️ `matrix(a b c d e f)` of six rounded numbers starting at `offset`. */
function matrixText(numbers: ArrayLike<number>, offset: number): string {
  return `matrix(${numbers[offset]} ${numbers[offset + 1]} ${numbers[offset + 2]} ${numbers[offset + 3]} ${numbers[offset + 4]} ${numbers[offset + 5]})`;
}

/** 🧭️ Where an actor stands and which way it faces, as a CSS transform. */
function placementText(x: number, y: number, flip: number, scale: number): string {
  return `translate(${x}px, ${y}px) scale(${flip}, ${scale})`;
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

/** 🙂️ The height the middle of a mouth is pulled to by a mood between −1 and 1. */
function bendOf(mouth: Mouth, mood: number): number {
  return rounded(mouth.y + mouth.width * MOUTH_BEND * mood);
}

/** 🧩️ The element of a part: its geometry, its paint classes and its stroke width (none when it has no stroke). */
function partShape(part: Part): readonly [tag: ShapeTag, attributes: Attributes] {
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
//#endregion 🔖️Rules

//#region 🔖️Depiction
/** 🖼️ The element tree of one actor and what was last painted on it: `element` is the `<svg class="pet">` to place in a
 * layer; `followers` lists per bone (rig order) the groups that carry its matrix, `sockets` per eye the group its lid
 * squashes, `pupils` per eye its pupil, `mouth` the mouth of a species that has one. `painted` holds the rounded numbers
 * on the tree — place (x, y), horizontal and vertical scale, opacity, the index of the activity, six per bone, three per
 * eye (pupil x, y, openness), the bend of the mouth — and is not a number anywhere before the first frame. */
export type Depiction = {
  readonly species: Species;
  readonly element: SVGSVGElement;
  readonly followers: readonly (readonly SVGGElement[])[];
  readonly sockets: readonly SVGGElement[];
  readonly pupils: readonly SVGCircleElement[];
  readonly mouth: SVGPathElement | null;
  readonly painted: Float64Array;
};

/** 🏗️ Builds the element tree of `species` in `document`: every part and the face in drawing order, the palette as custom properties. The tree shows nothing sensible until {@link paint} applied a first frame. */
export function depict(species: Species, document: Document = globalThis.document): Depiction {
  const create = <Tag extends "svg" | "g" | "circle" | ShapeTag>(tag: Tag, attributes: Attributes): SVGElementTagNameMap[Tag] => {
    const created = document.createElementNS(SVG_NAMESPACE, tag);
    for (const [name, value] of attributes) created.setAttribute(name, value);
    return created;
  };
  const element = create("svg", rootAttributes(species));
  element.style.setProperty("--pet-body", species.palette.body);
  element.style.setProperty("--pet-accent", species.palette.accent);
  element.style.setProperty("--pet-detail", species.palette.detail);
  const followers: SVGGElement[][] = species.bones.map(() => []);
  const follow = (bone: string): SVGGElement => {
    const group = create("g", []);
    followers[species.bones.findIndex((candidate) => candidate.id === bone)]?.push(group);
    element.append(group);
    return group;
  };
  const sockets: SVGGElement[] = [];
  const pupils: SVGCircleElement[] = [];
  let mouth: SVGPathElement | null = null;
  for (const layer of layers(species)) {
    if (layer !== FACE) {
      follow(layer.bone).append(create(...partShape(layer)));
      continue;
    }
    for (const eye of species.face.eyes) {
      const socket = create("g", []);
      const pupil = create("circle", pupilAttributes(eye));
      socket.append(create("circle", whiteAttributes(eye)), pupil);
      follow(eye.bone).append(socket);
      sockets.push(socket);
      pupils.push(pupil);
    }
    if (species.face.mouth) {
      mouth = create("path", MOUTH_ATTRIBUTES);
      follow(species.face.mouth.bone).append(mouth);
    }
  }
  return { species, element, followers, sockets, pupils, mouth, painted: new Float64Array(ACTOR_SLOTS + species.bones.length * 6 + species.face.eyes.length * EYE_SLOTS + 1).fill(Number.NaN) };
}

/** 🎨️ Applies `frame` to the tree at `scale`: where the actor stands and faces, its opacity, what it does, the matrix of every bone, its pupils, lids and mouth. Writes an attribute or a CSSOM property only where the rounded value differs from what is painted. */
export function paint(depiction: Depiction, frame: ActorFrame, scale = 1): void {
  const { species, element, followers, sockets, pupils, mouth, painted } = depiction;
  const x = rounded(frame.x, 100);
  const y = rounded(frame.y, 100);
  const flip = rounded(frame.facing * scale);
  const size = rounded(scale);
  if (x !== painted[0] || y !== painted[1] || flip !== painted[2] || size !== painted[3]) {
    painted[0] = x;
    painted[1] = y;
    painted[2] = flip;
    painted[3] = size;
    element.style.transform = placementText(x, y, flip, size);
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
  for (let bone = 0; bone < followers.length; bone++) {
    const groups = followers[bone]!;
    if (groups.length === 0) continue;
    const slot = ACTOR_SLOTS + bone * 6;
    let moved = false;
    for (let entry = 0; entry < 6; entry++) {
      const value = rounded(frame.bones[bone * 6 + entry] ?? REST_MATRIX[entry]!);
      if (value === painted[slot + entry]) continue;
      painted[slot + entry] = value;
      moved = true;
    }
    if (!moved) continue;
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
    if (pupilX !== painted[slot]) {
      painted[slot] = pupilX;
      pupils[index]!.setAttribute("cx", String(pupilX));
    }
    if (pupilY !== painted[slot + 1]) {
      painted[slot + 1] = pupilY;
      pupils[index]!.setAttribute("cy", String(pupilY));
    }
    if (open !== painted[slot + 2]) {
      painted[slot + 2] = open;
      sockets[index]!.setAttribute("transform", socketText(species.face.eyes[index]!, open));
    }
  }
  if (mouth && species.face.mouth) {
    const slot = faceSlot + sockets.length * EYE_SLOTS;
    const bend = bendOf(species.face.mouth, frame.mood);
    if (bend !== painted[slot]) {
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
  const face = (): string => {
    const eyes = species.face.eyes.map((eye, index) => {
      const look = frame.eyes[index];
      const pupil = tagText("circle", [...pupilAttributes(eye), ["cx", String(rounded(look?.x ?? 0))], ["cy", String(rounded(look?.y ?? 0))]]);
      return following(eye.bone, tagText("g", [["transform", socketText(eye, openness(look?.lid ?? 0))]], tagText("circle", whiteAttributes(eye)) + pupil));
    });
    const mouth = species.face.mouth;
    return eyes.join("") + (mouth ? following(mouth.bone, tagText("path", [...MOUTH_ATTRIBUTES, ["d", mouthText(mouth, bendOf(mouth, frame.mood))]])) : "");
  };
  return tagText("svg", [...rootAttributes(species), [ACTIVITY, frame.activity]], layers(species).map((layer) => (layer === FACE ? face() : following(layer.bone, tagText(...partShape(layer))))).join(""));
}
//#endregion 🔖️Markup
