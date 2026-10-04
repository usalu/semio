/** 🧰️ What an actor gets around with, drawn domain-neutrally: a parachute, a rope with its hook, a grappling gun, a ladder. {@link equip} builds the tools of one actor once, {@link paintTools} applies a frame to them; {@link rackLadders} and {@link paintLadders} do the same for the ladders that stand on the stage. The tilt of a whole drawing about its pivot is the depiction's ({@link tiltDegrees}); the tools follow it.
 *
 * The tools of an actor are two `<g class="pet-gear">` that belong inside the actor's `<svg class="pet">` — one before
 * its parts (what it wears on its back: the parachute, the carried ladder), one after them (what it holds: rope, hook,
 * gun): there they take the actor's place, size, opacity and palette for free. Their own transform undoes the mirroring
 * and the tilt of the drawing around them, so everything inside is laid out in the axes of the stage with the feet at
 * the origin, in the pet's own pixels: a rope runs to a point of the stage, a canopy stays above the grip whichever way
 * the pet swings or faces, and an aim, a sway and a lean are angles on screen (turns, clockwise, as CSS rotates).
 *
 * The rules of the depiction hold here too: elements are created with `createElementNS`, a frame writes attributes and
 * CSSOM properties only, and only those whose rounded value changed; a tool that is not in the frame is hidden by its
 * `visibility` attribute, never removed. No ids, no `<defs>`, no `<use>`, no `innerHTML`, no `<style>`. Colours come
 * from the paint classes of `🎨️.css`: lines in ink, the canopy and the gun in the species' accent.
 *
 * @see ../🖌️depiction/🟦️.ts — the drawing rules and the actor's own tree
 * @see ../../🎨️.css — the paint classes and the roots of standing ladders
 * @see https://www.w3.org/TR/css-transforms-1/#mathematical-description — the matrix convention
 * @see https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/visibility — how an unused tool is hidden
 */

import { cosTurns, sinTurns, type Point, type Species, type Turns } from "@semio-tech/pets";
import { maker, partShape, rounded, tiltDegrees, type Attributes, type Drawn } from "../🖌️depiction/🟦️.ts";

//#region 🔖️Rules
/** 🪂️ Half the width of the plain canopy when it is open, in widths of the species. */
export const CHUTE_SPAN = 0.75;

/** 🎈️ How far the rim of an open canopy floats above the grip, in heights of the species. */
export const CHUTE_RISE = 0.7;

/** ⛰️ How far the plain canopy rises above its rim, in heights of the species. */
export const CHUTE_DOME = 0.45;

/** 🤚️ How far ahead of its middle an actor holds a grappling gun, in widths of the species: at its side. */
export const GUN_REACH = 0.42;

/** 💪️ How far above its feet an actor holds a grappling gun, in heights of the species. */
export const GUN_HEIGHT = 0.5;

/** 🎒️ How far above its feet an actor carries a ladder, in heights of the species: the ladder is held at its middle there, and slides through the hold as far as its lower end would otherwise reach below the feet. */
export const LADDER_CARRY = 0.45;

/** ↔️ How far apart the two rails of a ladder are, in pixels. */
export const LADDER_WIDTH = 10;

/** 📏️ How far apart the rungs of a carried ladder are, in pixels; a standing ladder says how many rungs it has. */
export const LADDER_RUNG = 10.5;

/** 🎯️ How far the muzzle of a grappling gun lies ahead of the hand that holds it, in pixels: where its rope leaves. */
export const GUN_MUZZLE = 11;

const GEAR = "pet-gear";
const LADDERS = "pet-ladders";
const TOOL = "data-pet-tool";
const VISIBILITY = "visibility";
const LINE = "pet-fill-none pet-stroke-ink";
const CORDS = [0, 1 / 3, 2 / 3, 1] as const;
const CORD_STROKE = "1.25";
const PATH_TOKEN = /[a-zA-Z]|[-+]?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?/gu;
const PATH_ARITY: Readonly<Record<string, number>> = { M: 2, L: 2, T: 2, H: 1, V: 1, C: 6, S: 4, Q: 4, A: 7 };
const ROPE_STROKE = "1.5";
const RAIL_STROKE = "2";
const RUNG_STROKE = "1.5";
const CHUTE_STREAM = 0.4;
const CHUTE_FURL = 0.3;
const CHUTE_SQUASH = 0.25;
const SCALLOP = 0.16;
const PLACE = 100;
const AXIS = 100000;
const COUNTER = 0;
const CHUTE = 4;
const LADDER = 9;
const LADDER_SLOTS = 8;
const ROPE = LADDER + LADDER_SLOTS;
const HOOK = ROPE + 6;
const GUN = HOOK + 5;
const SLOTS = GUN + 5;

const GUN_PIECES: readonly Drawn[] = [
  { shape: { kind: "line", x1: -1, y1: 2, x2: -3.5, y2: 8 }, fill: "none", stroke: "ink", strokeWidth: 3.5 },
  { shape: { kind: "rect", x: -5, y: -3, width: 13, height: 6, radius: 2 }, fill: "accent", stroke: "ink", strokeWidth: 1.5 },
  { shape: { kind: "rect", x: 8, y: -4, width: 3, height: 8, radius: 1 }, fill: "ink", stroke: "none" },
];

const HOOK_PIECES: readonly Drawn[] = [
  { shape: { kind: "line", x1: -6, y1: 0, x2: 0, y2: 0 }, fill: "none", stroke: "ink", strokeWidth: 1.75 },
  { shape: { kind: "path", d: "M 2 -6 C 5.5 -4 4.5 -0.5 0 0 C 4.5 0.5 5.5 4 2 6" }, fill: "none", stroke: "ink", strokeWidth: 1.75 },
];

/** 🪁️ An open parachute: `open` runs from 0 (packed) to 1 (open) and may overshoot while the canopy snaps open; `sway` leans canopy and cords about the grip. */
export type GearChute = { readonly kind: "chute"; readonly open: number; readonly sway: Turns };

/** 🪢️ A rope from the actor to the point (`x`, `y`) of the stage; its middle hangs `slack` pixels below the straight line (0 = taut, negative = bowed upwards). It leaves the muzzle of the gun while the frame holds one, else the pivot of the actor. */
export type GearRope = { readonly kind: "rope"; readonly x: number; readonly y: number; readonly slack: number };

/** 🪝️ The hook of a rope at the point (`x`, `y`) of the stage; it points the way the rope arrives, upwards without a rope. */
export type GearHook = { readonly kind: "hook"; readonly x: number; readonly y: number };

/** 🔫️ A grappling gun in the hand of the actor, pointing `aim` (0 = to the right, a quarter turn = down). */
export type GearGun = { readonly kind: "gun"; readonly aim: Turns };

/** 🪜️ A carried ladder of `length` pixels, held at its middle on the actor's back and leaning `lean` away from upright (a quarter turn = lying, its top to the right); raised towards upright its foot comes to stand at the height of the feet. */
export type GearCarriedLadder = { readonly kind: "ladder"; readonly lean: Turns; readonly length: number };

/** 🛠️ One tool of an actor in a frame: the `ToolFrame` of the contract. Of several tools of one kind the first is drawn. */
export type GearTool = GearChute | GearRope | GearHook | GearGun | GearCarriedLadder;

/** 🧍️ What the tools need of an actor's frame: its feet on the stage, the way it faces, how far the whole drawing is tilted (`tilt`, clockwise on screen whichever way it faces) about `pivot` (in the coordinates of the rig: feet at the origin, mirrored with the actor), and its tools. Everything in the units of the stage, before the size pets are drawn at. */
export type GearBearer = { readonly x: number; readonly y: number; readonly facing: 1 | -1; readonly tilt: Turns; readonly pivot: Point; readonly tools: readonly GearTool[] };

/** 🏗️ A ladder that stands on the stage, from its foot (`x0`, `y0`) to its top (`x1`, `y1`), with `rungs` rungs: the `LadderFrame` of the contract. */
export type GearLadder = { readonly x0: number; readonly y0: number; readonly x1: number; readonly y1: number; readonly rungs: number; readonly opacity: number };

/** 🧬️ What the tools need of a species: its box, the height of its scruff above its feet and the shape it flies as a canopy. A canopy of its own is authored around the point that floats {@link CHUTE_RISE} heights above the grip — the middle of its rim at the origin, the canopy above it (negative y) —, in the pet's pixels at its open size; its cords run to its rim ({@link canopyRim}). */
export type GearSpecies = Pick<Species, "size" | "grip" | "canopy">;

/** 🗒️ The rounded numbers last written to a tree. */
type Memo = Float64Array | number[];

/** 🪵️ The elements of one ladder: the group that places it, its rails and its rungs. */
type LadderParts = { readonly group: SVGGElement; readonly rails: SVGPathElement; readonly rungs: SVGPathElement };

/** 🔁️ Stores `values` from `slot` on and tells whether any of them differs from what was there. */
function moved(painted: Memo, slot: number, values: readonly number[]): boolean {
  let changed = false;
  for (let index = 0; index < values.length; index++) {
    if (values[index] === painted[slot + index]) continue;
    painted[slot + index] = values[index]!;
    changed = true;
  }
  return changed;
}

/** 👻️ Shows or hides a tool by its `visibility` attribute, written only when it changes, and tells whether it shows. */
function reveal(element: Element, painted: Memo, slot: number, shown: boolean): boolean {
  const flag = shown ? 1 : 0;
  if (painted[slot] !== flag) {
    painted[slot] = flag;
    element.setAttribute(VISIBILITY, shown ? "visible" : "hidden");
  }
  return shown;
}

/** ☂️ What a canopy is drawn from, back to front, around the middle of its rim: the species' own shape in its accent, or the plain dome — scalloped between the cords, its middle gore in paper. */
function canopyPieces(species: GearSpecies): readonly Drawn[] {
  if (species.canopy !== undefined) return [{ shape: species.canopy, fill: "accent", stroke: "ink" }];
  const span = rounded(CHUTE_SPAN * species.size.width, PLACE);
  const dome = rounded(CHUTE_DOME * species.size.height, PLACE);
  const third = rounded(span / 3, PLACE);
  const crest = rounded((dome * 4) / 3, PLACE);
  const scallop = rounded(dome * SCALLOP, PLACE);
  const seam = rounded(dome * 0.6, PLACE);
  const bulge = rounded(third * 1.15, PLACE);
  return [
    { shape: { kind: "path", d: `M ${-span} 0 C ${-span} ${-crest} ${span} ${-crest} ${span} 0 Q ${third * 2} ${-scallop} ${third} 0 Q 0 ${-scallop} ${-third} 0 Q ${-third * 2} ${-scallop} ${-span} 0 Z` }, fill: "accent", stroke: "ink" },
    { shape: { kind: "path", d: `M 0 ${-dome} Q ${bulge} ${-seam} ${third} 0 Q 0 ${-scallop} ${-third} 0 Q ${-bulge} ${-seam} 0 ${-dome} Z` }, fill: "paper", stroke: "ink", strokeWidth: 1 },
  ];
}

/** ✏️ The points an outline passes through, read from SVG path data: the end of every segment (control points left out), absolute or relative, as far as the data can be read. */
function outlinePoints(d: string): Point[] {
  const tokens = d.match(PATH_TOKEN) ?? [];
  const points: Point[] = [];
  let command = "";
  let x = 0;
  let y = 0;
  let startX = 0;
  let startY = 0;
  let index = 0;
  while (index < tokens.length) {
    const token = tokens[index]!;
    if (/^[a-zA-Z]$/u.test(token)) {
      command = token;
      index++;
      if (command === "Z" || command === "z") {
        x = startX;
        y = startY;
      }
      continue;
    }
    const kind = command.toUpperCase();
    const arity = PATH_ARITY[kind];
    if (arity === undefined || index + arity > tokens.length) break;
    const relative = command !== kind;
    const values = tokens.slice(index, index + arity).map(Number);
    index += arity;
    if (kind === "H") x = (relative ? x : 0) + values[0]!;
    else if (kind === "V") y = (relative ? y : 0) + values[0]!;
    else {
      x = (relative ? x : 0) + values[arity - 2]!;
      y = (relative ? y : 0) + values[arity - 1]!;
    }
    if (kind === "M") {
      startX = x;
      startY = y;
      command = relative ? "l" : "L";
    }
    points.push({ x, y });
  }
  return points;
}

/** 🧶️ The two ends of the rim of a species' canopy, left then right, in the canopy's own coordinates: the four cords of its parachute end evenly spread on the straight line between them. Of the plain canopy they are the ends of its dome; of a canopy of its own the ends of an ellipse's width, the lower corners of a rectangle, the ends of a line, and of an outline its leftmost and its rightmost point (the lower one where several are as far out) — the plain rim when the outline cannot be read. */
export function canopyRim(species: GearSpecies): readonly [left: Point, right: Point] {
  const span = rounded(CHUTE_SPAN * species.size.width, PLACE);
  const plain: readonly [Point, Point] = [
    { x: -span, y: 0 },
    { x: span, y: 0 },
  ];
  const shape = species.canopy;
  if (shape === undefined) return plain;
  if (shape.kind === "ellipse") {
    return [
      { x: shape.cx - shape.rx, y: shape.cy },
      { x: shape.cx + shape.rx, y: shape.cy },
    ];
  }
  if (shape.kind === "rect") {
    return [
      { x: shape.x, y: shape.y + shape.height },
      { x: shape.x + shape.width, y: shape.y + shape.height },
    ];
  }
  const points = shape.kind === "line" ? [{ x: shape.x1, y: shape.y1 }, { x: shape.x2, y: shape.y2 }] : outlinePoints(shape.d);
  let left: Point | undefined;
  let right: Point | undefined;
  for (const point of points) {
    if (!Number.isFinite(point.x) || !Number.isFinite(point.y)) return plain;
    if (left === undefined || point.x < left.x || (point.x === left.x && point.y > left.y)) left = point;
    if (right === undefined || point.x > right.x || (point.x === right.x && point.y > right.y)) right = point;
  }
  return left === undefined || right === undefined || left.x === right.x ? plain : [left, right];
}

/** 🚧️ The rails and the rungs of a ladder of `length` with `rungs` rungs, lying along the x axis from its foot at the origin: the rungs evenly spaced, half a spacing from both ends. */
function ladderTexts(length: number, rungs: number): readonly [rails: string, rungs: string] {
  const half = LADDER_WIDTH / 2;
  const steps: string[] = [];
  for (let rung = 0; rung < rungs; rung++) steps.push(`M ${rounded(((rung + 0.5) * length) / rungs, PLACE)} ${-half} V ${half}`);
  return [`M 0 ${-half} H ${length} M 0 ${half} H ${length}`, steps.join(" ")];
}

/** 🪚️ Places a ladder between its foot and its top and writes what changed since the numbers at `slot`: the matrix of its group (its axis and its foot), its rails and rungs (its length and their number) and its opacity. */
function paintLadder(parts: LadderParts, painted: Memo, slot: number, x0: number, y0: number, x1: number, y1: number, rungs: number, opacity: number): void {
  const dx = x1 - x0;
  const dy = y1 - y0;
  const length = Math.sqrt(dx * dx + dy * dy);
  const ux = length > 0 ? dx / length : 0;
  const uy = length > 0 ? dy / length : -1;
  if (moved(painted, slot, [rounded(ux, AXIS), rounded(uy, AXIS), rounded(x0, PLACE), rounded(y0, PLACE)])) parts.group.setAttribute("transform", `matrix(${painted[slot]} ${painted[slot + 1]} ${-painted[slot + 1]!} ${painted[slot]} ${painted[slot + 2]} ${painted[slot + 3]})`);
  if (moved(painted, slot + 4, [rounded(length, PLACE), Math.max(0, Math.floor(rungs))])) {
    const [rails, steps] = ladderTexts(painted[slot + 4]!, painted[slot + 5]!);
    parts.rails.setAttribute("d", rails);
    parts.rungs.setAttribute("d", steps);
  }
  if (moved(painted, slot + 6, [rounded(opacity)])) parts.group.setAttribute("opacity", String(painted[slot + 6]));
}
//#endregion 🔖️Rules

//#region 🔖️Tools
/** 🎽️ The tools of one actor and what was last painted on them: `back` is the `<g class="pet-gear">` to put before the parts of the actor's `<svg class="pet">` (the parachute, the carried ladder), `front` the one to put after them (rope, hook, gun); `chute` leans with the sway and holds the `cords` and the `canopy` (whose cords end between the two points of `rim`), `ladder` is the carried ladder, `rope`, `hook` and `gun` the grappling line. `painted` holds the rounded numbers on the tree and is not a number anywhere before the first frame, except that nothing shows and the carried ladder is opaque. */
export type Equipment = {
  readonly species: GearSpecies;
  readonly back: SVGGElement;
  readonly front: SVGGElement;
  readonly chute: SVGGElement;
  readonly cords: SVGPathElement;
  readonly canopy: SVGGElement;
  readonly rim: readonly [left: Point, right: Point];
  readonly ladder: LadderParts;
  readonly rope: SVGPathElement;
  readonly hook: SVGGElement;
  readonly gun: SVGGElement;
  readonly painted: Float64Array;
};

/** 🧵️ Builds the tools of `species` in `document`, every one of them hidden: on the back the parachute (its cords behind its canopy) and the carried ladder, in front the rope, the hook and the gun. */
export function equip(species: GearSpecies, document: Document = globalThis.document): Equipment {
  const create = maker(document);
  const hidden = (kind: string): Attributes => [
    [TOOL, kind],
    [VISIBILITY, "hidden"],
  ];
  const back = create("g", [["class", GEAR]]);
  const front = create("g", [["class", GEAR]]);
  const chute = create("g", hidden("chute"));
  const cords = create("path", [
    ["class", LINE],
    ["stroke-width", CORD_STROKE],
  ]);
  const canopy = create("g", []);
  for (const piece of canopyPieces(species)) canopy.append(create(...partShape(piece)));
  chute.append(cords, canopy);
  const ladder = ladderParts(document, hidden("ladder"));
  const rope = create("path", [...hidden("rope"), ["class", LINE], ["stroke-width", ROPE_STROKE]]);
  const hook = create("g", hidden("hook"));
  for (const piece of HOOK_PIECES) hook.append(create(...partShape(piece)));
  const gun = create("g", hidden("gun"));
  for (const piece of GUN_PIECES) gun.append(create(...partShape(piece)));
  back.append(chute, ladder.group);
  front.append(rope, hook, gun);
  const painted = new Float64Array(SLOTS).fill(Number.NaN);
  for (const slot of [CHUTE, LADDER, ROPE, HOOK, GUN]) painted[slot] = 0;
  painted[LADDER + LADDER_SLOTS - 1] = 1;
  return { species, back, front, chute, cords, canopy, rim: canopyRim(species), ladder, rope, hook, gun, painted };
}

/** 🪛️ The elements of one ladder in `document`, its group carrying `attributes`. */
function ladderParts(document: Document, attributes: Attributes): LadderParts {
  const create = maker(document);
  const group = create("g", attributes);
  const rails = create("path", [
    ["class", LINE],
    ["stroke-width", RAIL_STROKE],
  ]);
  const rungs = create("path", [
    ["class", LINE],
    ["stroke-width", RUNG_STROKE],
  ]);
  group.append(rails, rungs);
  return { group, rails, rungs };
}

/** 🖌️ Applies the tools of `bearer` to the tree: hides what the frame does not hold, shows what it holds and writes what changed. An actor without tools whose tools are hidden already costs nothing and writes nothing.
 *
 * Both groups undo the mirroring and the tilt of the actor's drawing, so their inside is laid out in the axes of the
 * stage around the feet. The parachute hangs from the grip (the scruff, carried along by the tilt) and leans with its
 * sway; while it opens the canopy streams out narrow and close, and beyond 1 it is squashed as much as it is stretched.
 * The carried ladder is balanced at its middle and never reaches below the feet: raised, it slides through the hold
 * until its foot stands. The gun sits in the hand at the actor's side and keeps its handle down whichever way it
 * points; the rope leaves its muzzle — or, without a gun, the pivot the actor hangs from — and the hook points the way
 * the rope arrives.
 */
export function paintTools(equipment: Equipment, bearer: GearBearer): void {
  const { species, painted } = equipment;
  let chute: GearChute | undefined;
  let rope: GearRope | undefined;
  let hook: GearHook | undefined;
  let gun: GearGun | undefined;
  let ladder: GearCarriedLadder | undefined;
  for (const tool of bearer.tools) {
    if (tool.kind === "chute") chute ??= tool;
    else if (tool.kind === "rope") rope ??= tool;
    else if (tool.kind === "hook") hook ??= tool;
    else if (tool.kind === "gun") gun ??= tool;
    else ladder ??= tool;
  }
  if (bearer.tools.length === 0 && painted[CHUTE] === 0 && painted[LADDER] === 0 && painted[ROPE] === 0 && painted[HOOK] === 0 && painted[GUN] === 0) return;
  const { width, height } = species.size;
  const facing = bearer.facing;
  const degrees = tiltDegrees(facing, bearer.tilt);
  const cos = cosTurns(degrees / 360);
  const sin = sinTurns(degrees / 360);
  const px = bearer.pivot.x;
  const py = bearer.pivot.y;
  const carried = (x: number, y: number): readonly [x: number, y: number] => [(cos * (x - px) - sin * (y - py) + px) * facing, sin * (x - px) + cos * (y - py) + py];
  if (bearer.tools.length > 0 && moved(painted, COUNTER, [facing, degrees, rounded(px, PLACE), rounded(py, PLACE)])) {
    const counter = `matrix(${rounded(cos * facing, AXIS)} ${rounded(-sin * facing, AXIS)} ${rounded(sin, AXIS)} ${rounded(cos, AXIS)} ${rounded(px - (cos * px + sin * py))} ${rounded(py - (cos * py - sin * px))})`;
    equipment.back.setAttribute("transform", counter);
    equipment.front.setAttribute("transform", counter);
  }
  if (reveal(equipment.chute, painted, CHUTE, chute !== undefined && rounded(chute.open) > 0) && chute !== undefined) {
    const [gx, gy] = carried(0, -species.grip);
    if (moved(painted, CHUTE + 1, [rounded(gx, PLACE), rounded(gy, PLACE), rounded(chute.sway * 360, PLACE)])) equipment.chute.setAttribute("transform", `translate(${painted[CHUTE + 1]} ${painted[CHUTE + 2]}) rotate(${painted[CHUTE + 3]})`);
    if (moved(painted, CHUTE + 4, [rounded(chute.open)])) {
      const open = painted[CHUTE + 4]!;
      const rise = rounded(-CHUTE_RISE * height * (CHUTE_STREAM + (1 - CHUTE_STREAM) * Math.min(open, 1)), PLACE);
      const wide = rounded(open > 1 ? 1 - CHUTE_SQUASH * (open - 1) : CHUTE_FURL + (1 - CHUTE_FURL) * open);
      equipment.canopy.setAttribute("transform", `translate(0 ${rise}) scale(${wide} ${open})`);
      const [left, right] = equipment.rim;
      equipment.cords.setAttribute("d", CORDS.map((cord) => `M 0 0 L ${rounded((left.x + (right.x - left.x) * cord) * wide, PLACE)} ${rounded(rise + (left.y + (right.y - left.y) * cord) * open, PLACE)}`).join(" "));
    }
  }
  if (reveal(equipment.ladder.group, painted, LADDER, ladder !== undefined && ladder.length > 0) && ladder !== undefined) {
    const [holdX, holdY] = carried(0, -LADDER_CARRY * height);
    const upright = cosTurns(ladder.lean);
    const ax = (sinTurns(ladder.lean) * ladder.length) / 2;
    const ay = (-upright * ladder.length) / 2;
    const sunk = Math.max(0, holdY + Math.abs(ay));
    const cx = holdX + (sunk * sinTurns(ladder.lean)) / (upright === 0 ? 1 : upright);
    const cy = holdY - sunk;
    paintLadder(equipment.ladder, painted, LADDER + 1, cx - ax, cy - ay, cx + ax, cy + ay, Math.max(1, Math.floor(ladder.length / LADDER_RUNG)), 1);
  }
  const [hx, hy] = carried(GUN_REACH * width, -GUN_HEIGHT * height);
  let endX = 0;
  let endY = -1;
  if (reveal(equipment.rope, painted, ROPE, rope !== undefined) && rope !== undefined) {
    const nx = gun === undefined ? px * facing : hx + cosTurns(gun.aim) * GUN_MUZZLE;
    const ny = gun === undefined ? py : hy + sinTurns(gun.aim) * GUN_MUZZLE;
    const fx = rope.x - bearer.x;
    const fy = rope.y - bearer.y;
    const sag = rounded(rope.slack, PLACE);
    const mx = (nx + fx) / 2;
    const my = (ny + fy) / 2 + 2 * sag;
    endX = fx - (sag === 0 ? nx : mx);
    endY = fy - (sag === 0 ? ny : my);
    if (moved(painted, ROPE + 1, [rounded(nx, PLACE), rounded(ny, PLACE), rounded(fx, PLACE), rounded(fy, PLACE), sag])) {
      const bend = sag === 0 ? "L" : `Q ${rounded(mx, PLACE)} ${rounded(my, PLACE)}`;
      equipment.rope.setAttribute("d", `M ${painted[ROPE + 1]} ${painted[ROPE + 2]} ${bend} ${painted[ROPE + 3]} ${painted[ROPE + 4]}`);
    }
  }
  if (reveal(equipment.hook, painted, HOOK, hook !== undefined) && hook !== undefined) {
    const reach = Math.sqrt(endX * endX + endY * endY);
    const ux = reach > 0 ? endX / reach : 0;
    const uy = reach > 0 ? endY / reach : -1;
    if (moved(painted, HOOK + 1, [rounded(ux, AXIS), rounded(uy, AXIS), rounded(hook.x - bearer.x, PLACE), rounded(hook.y - bearer.y, PLACE)])) equipment.hook.setAttribute("transform", `matrix(${painted[HOOK + 1]} ${painted[HOOK + 2]} ${-painted[HOOK + 2]!} ${painted[HOOK + 1]} ${painted[HOOK + 3]} ${painted[HOOK + 4]})`);
  }
  if (reveal(equipment.gun, painted, GUN, gun !== undefined) && gun !== undefined) {
    const upside = cosTurns(gun.aim) < 0 ? -1 : 1;
    if (moved(painted, GUN + 1, [rounded(hx, PLACE), rounded(hy, PLACE), rounded(gun.aim * 360, PLACE), upside])) equipment.gun.setAttribute("transform", `translate(${painted[GUN + 1]} ${painted[GUN + 2]}) rotate(${painted[GUN + 3]}) scale(1 ${upside})`);
  }
}
//#endregion 🔖️Tools

//#region 🔖️Ladders
/** 🗄️ The ladders that stand on a stage and what was last painted on them: `element` is the `<svg class="pet-ladders">` to place in a layer, behind the actors; `ladders` are the ladders built so far, of which a frame shows as many as it holds. `painted` holds the size they are drawn at, then per ladder whether it shows and its rounded numbers. */
export type LadderRack = { readonly element: SVGSVGElement; readonly ladders: LadderParts[]; readonly painted: number[] };

/** 🧺️ Builds the empty rack of standing ladders in `document`. */
export function rackLadders(document: Document = globalThis.document): LadderRack {
  return {
    element: maker(document)("svg", [
      ["class", LADDERS],
      ["focusable", "false"],
      ["overflow", "visible"],
    ]),
    ladders: [],
    painted: [Number.NaN],
  };
}

/** 🧗️ Applies the standing `ladders` of a frame to the rack at `scale`, the size pets are drawn at: one group per ladder, two rails and its rungs between foot and top, perpendicular to the rails and evenly spaced. A ladder the stage sees for the first time is built once; a ladder that left is hidden, never removed; a frame that changes nothing writes nothing, and a stage without ladders costs nothing. */
export function paintLadders(rack: LadderRack, ladders: readonly GearLadder[], scale = 1): void {
  const { element, painted } = rack;
  if (ladders.length === 0 && rack.ladders.every((_, index) => painted[1 + index * LADDER_SLOTS] === 0)) return;
  if (moved(painted, 0, [rounded(scale)])) element.style.transform = `scale(${painted[0]})`;
  for (let index = rack.ladders.length; index < ladders.length; index++) {
    const built = ladderParts(element.ownerDocument, [
      [TOOL, "ladder"],
      [VISIBILITY, "hidden"],
    ]);
    rack.ladders.push(built);
    painted.push(0, ...Array.from({ length: LADDER_SLOTS - 1 }, () => Number.NaN));
    element.append(built.group);
  }
  for (const [index, parts] of rack.ladders.entries()) {
    const slot = 1 + index * LADDER_SLOTS;
    const ladder = ladders[index];
    if (reveal(parts.group, painted, slot, ladder !== undefined && ladder.rungs >= 1 && rounded(ladder.opacity) > 0) && ladder !== undefined) paintLadder(parts, painted, slot + 1, ladder.x0, ladder.y0, ladder.x1, ladder.y1, ladder.rungs, ladder.opacity);
  }
}
//#endregion 🔖️Ladders
