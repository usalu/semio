/** ✨️ The particles of a frame, painted in the tint of their species' state, and the dust where pets vanished: {@link stageEffects} builds the empty root of all particles of a layer, {@link stockEffects} the pools of a species, {@link paintEffects} applies the particles of a frame, {@link paintPuffs} its puffs of dust, {@link retireEffects} takes a species that left away and {@link strikeEffects} everything; {@link tintEffects} paints the tint of a state over the particles of a species, by the depiction's own rule for its palette ({@link tintPalette}).
 *
 * Particles are not actors: they live in one `<svg class="pet-effects">` beside the actors, in the units of the stage
 * (the root is scaled to the size pets are drawn at). Every species has one group there that carries its palette — so a
 * particle is painted with the classes of a part and follows the tint of its species' state and the theme —, and every
 * emitter of the species a pool of as many shape elements as it can have particles alive at once. A pool is built once,
 * from the emitter's shape and paints; afterwards a frame only writes the `transform` and the `opacity` of the
 * particles that changed and the `visibility` of the elements that came into use or fell out of it. No element is
 * created or removed while particles fly, nothing is written when nothing lives, and a particle the contract does not
 * allow (an unknown species or emitter, more than the emitter's `count`) is not drawn.
 *
 * The dust of a poof (design-v2 §18: a pet that has no legal place left vanishes in a puff) belongs to no species: it is
 * drawn domain-neutrally in ink and paper, behind every particle, as a short burst of blobs that spreads from the body
 * it replaces to its outline, rises a little and fades ({@link puffShape}). Its clouds are pooled like particles —
 * built when more puffs are in the air at once than ever before, at most {@link PUFF_CLOUDS}, never removed while the
 * layer lives — and written only where a rounded value changed.
 *
 * The rules of the depiction hold: `createElementNS`, attribute and CSSOM property writes only, no ids, no `<defs>`,
 * no `<use>`, no `innerHTML`, no `<style>`.
 *
 * @see ../🖌️depiction/🟦️.ts — how a shape is drawn and painted
 * @see ../../🎨️.css — the root of the particles and the paint classes
 * @see https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty — how a palette is written under a policy without inline styles
 * @see https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/visibility — how an unused element is hidden
 */

import type { Emitter, Slug, Species, Tint, Turns } from "@semio-tech/pets";
import { maker, partShape, rounded, tintPalette, type ShapeTag } from "../🖌️depiction/🟦️.ts";

//#region 🔖️Rules
const EFFECTS = "pet-effects";
const TROUPE = "data-pet-effects";
const EMITTER = "data-pet-emitter";
const VISIBILITY = "visibility";
const DUST = "data-pet-puffs";
const PUFF = "data-pet-puff";
const PLACE = 100;
const PARTICLE_SLOTS = 5;
const CLOUD_SLOTS = 3;
const BLOB_SLOTS = 3;

/** 🎇️ One particle of a frame: which emitter of which species it came from, where it is on the stage, how large, how far turned (clockwise on screen) and how opaque: the `ParticleFrame` of the contract. */
export type EffectParticle = { readonly species: Slug; readonly emitter: Slug; readonly x: number; readonly y: number; readonly scale: number; readonly rotation: Turns; readonly opacity: number };

/** 🧨️ What the painting needs of an emitter: its name, the shape and the paints of its particles and how many of them can be alive at once. */
export type EffectEmitter = Pick<Emitter, "id" | "shape" | "fill" | "stroke" | "strokeWidth" | "count">;

/** 🐣️ What the painting needs of a species: its name, its palette and its emitters. */
export type EffectSpecies = Pick<Species, "id" | "palette"> & { readonly emitters: readonly EffectEmitter[] };

/** 🫙️ The particles of one emitter: its elements in the order they are taken into use, how many of them the last frame used, how many this frame has taken so far, and five rounded numbers per element (place, size, turn, opacity) that are not a number before its first use. */
type Pool = { readonly elements: readonly SVGElementTagNameMap[ShapeTag][]; readonly painted: Float64Array; live: number; used: number };

/** 🎭️ Everything a species has among the particles: the group that carries its palette and the pool of every emitter stocked so far. */
type Troupe = { readonly group: SVGGElement; readonly pools: Map<Slug, Pool> };

/** 🌁️ One puff of dust as drawn: its group (placed at the middle of the puff, carrying its opacity and visibility), per blob an outline circle and a fill circle, and the rounded numbers on them — place (x, y) and opacity, then per blob its middle and radius —, not a number before its first use. */
type Cloud = { readonly group: SVGGElement; readonly outlines: readonly SVGCircleElement[]; readonly fills: readonly SVGCircleElement[]; readonly painted: Float64Array };

/** 🧺️ The dust of a layer: its group (behind every particle; `null` until the first puff), its clouds in the order they are taken into use and how many of them the last frame used. */
type Dust = { group: SVGGElement | null; readonly clouds: Cloud[]; live: number };
//#endregion 🔖️Rules

//#region 🔖️Particles
/** 🎆️ The particles of a layer and what was last painted on them: `element` is the `<svg class="pet-effects">` to place in a layer, in front of the actors; `troupes` holds per species its group and pools, `tints` the tint every species was last given, `live` how many particles the last frame drew, `scale` the size they are drawn at (not a number before the first frame that draws), `dust` the clouds of the puffs where pets vanished. */
export type Effects = { readonly element: SVGSVGElement; readonly troupes: Map<Slug, Troupe>; readonly tints: Map<Slug, Tint | undefined>; live: number; scale: number; readonly dust: Dust };

/** 🌌️ Builds the empty root of the particles in `document`. */
export function stageEffects(document: Document = globalThis.document): Effects {
  return {
    element: maker(document)("svg", [
      ["class", EFFECTS],
      ["focusable", "false"],
      ["overflow", "visible"],
    ]),
    troupes: new Map(),
    tints: new Map(),
    live: 0,
    scale: Number.NaN,
    dust: { group: null, clouds: [], live: 0 },
  };
}

/** 📏️ Scales the root to `scale`, the size pets are drawn at, when the rounded size changed. */
function scaled(effects: Effects, scale: number): void {
  const size = rounded(scale);
  if (size === effects.scale) return;
  effects.scale = size;
  effects.element.style.transform = `scale(${size})`;
}

/** 🎪️ The troupe of `species`, built on first need: a group that names the species and carries its palette under the tint it was last given. */
function troupeOf(effects: Effects, species: EffectSpecies): Troupe {
  const known = effects.troupes.get(species.id);
  if (known !== undefined) return known;
  const group = maker(effects.element.ownerDocument)("g", [[TROUPE, species.id]]);
  tintPalette(group, species.palette, effects.tints.get(species.id));
  effects.element.append(group);
  const troupe: Troupe = { group, pools: new Map() };
  effects.troupes.set(species.id, troupe);
  return troupe;
}

/** 🏺️ The pool of `emitter` in `troupe`, built on first need: as many hidden elements as the emitter can have particles alive at once, each its shape with its paints. */
function poolOf(troupe: Troupe, emitter: EffectEmitter): Pool {
  const known = troupe.pools.get(emitter.id);
  if (known !== undefined) return known;
  const create = maker(troupe.group.ownerDocument);
  const [tag, attributes] = partShape(emitter);
  const size = Math.max(0, Math.floor(emitter.count));
  const elements = Array.from({ length: size }, () => create(tag, [...attributes, [EMITTER, emitter.id], [VISIBILITY, "hidden"]]));
  troupe.group.append(...elements);
  const pool: Pool = { elements, painted: new Float64Array(size * PARTICLE_SLOTS).fill(Number.NaN), live: 0, used: 0 };
  troupe.pools.set(emitter.id, pool);
  return pool;
}

/** 📦️ Builds the pools of every emitter of `species` ahead of their first particle, so the frame that first shows them creates nothing. A species that is stocked already is left as it is. */
export function stockEffects(effects: Effects, species: EffectSpecies): void {
  if (species.emitters.length === 0) return;
  const troupe = troupeOf(effects, species);
  for (const emitter of species.emitters) poolOf(troupe, emitter);
}

/** 🌈️ Gives the particles of `species` the tint of its state (none restores its palette): at once when the species has particles on stage, and remembered for the moment it gets some. */
export function tintEffects(effects: Effects, species: EffectSpecies, tint?: Tint): void {
  effects.tints.set(species.id, tint);
  const troupe = effects.troupes.get(species.id);
  if (troupe !== undefined) tintPalette(troupe.group, species.palette, tint);
}

/** 🎉️ Applies the `particles` of a frame at `scale`, the size pets are drawn at; `kinds` names the species a particle may come from. The n-th particle of an emitter in the frame takes the n-th element of its pool: its `transform` (place to hundredths of a pixel, turn to hundredths of a degree, size to thousandths) and its `opacity` are written only when they changed, its `visibility` only when the element comes into use; elements the frame no longer needs are hidden. A frame without particles after a frame without particles writes nothing and costs nothing. */
export function paintEffects(effects: Effects, kinds: ReadonlyMap<Slug, EffectSpecies>, particles: readonly EffectParticle[], scale = 1): void {
  if (particles.length === 0 && effects.live === 0) return;
  scaled(effects, scale);
  let drawn = 0;
  for (const particle of particles) {
    const species = kinds.get(particle.species);
    const emitter = species?.emitters.find((candidate) => candidate.id === particle.emitter);
    if (species === undefined || emitter === undefined) continue;
    const pool = poolOf(troupeOf(effects, species), emitter);
    if (pool.used >= pool.elements.length) continue;
    const index = pool.used++;
    const element = pool.elements[index]!;
    const painted = pool.painted;
    const slot = index * PARTICLE_SLOTS;
    const x = rounded(particle.x, PLACE);
    const y = rounded(particle.y, PLACE);
    const turn = rounded(particle.rotation * 360, PLACE);
    const grown = rounded(particle.scale);
    const opacity = rounded(particle.opacity);
    if (x !== painted[slot] || y !== painted[slot + 1] || turn !== painted[slot + 2] || grown !== painted[slot + 3]) {
      painted[slot] = x;
      painted[slot + 1] = y;
      painted[slot + 2] = turn;
      painted[slot + 3] = grown;
      element.setAttribute("transform", `translate(${x} ${y}) rotate(${turn}) scale(${grown})`);
    }
    if (opacity !== painted[slot + 4]) {
      painted[slot + 4] = opacity;
      element.setAttribute("opacity", String(opacity));
    }
    if (index >= pool.live) element.setAttribute(VISIBILITY, "visible");
    drawn++;
  }
  for (const troupe of effects.troupes.values()) {
    for (const pool of troupe.pools.values()) {
      for (let index = pool.used; index < pool.live; index++) pool.elements[index]!.setAttribute(VISIBILITY, "hidden");
      pool.live = pool.used;
      pool.used = 0;
    }
  }
  effects.live = drawn;
}

/** 👋️ Takes the particles of the species `id` away, for a species that left the stage: its group with every pool, and the tint it was given. Its particles of the last frame no longer count as alive. */
export function retireEffects(effects: Effects, id: Slug): void {
  const troupe = effects.troupes.get(id);
  effects.tints.delete(id);
  if (troupe === undefined) return;
  for (const pool of troupe.pools.values()) effects.live -= pool.live;
  troupe.group.remove();
  effects.troupes.delete(id);
}

/** 🧹️ Ends the particles and the dust: removes the root with everything in it from its layer and forgets every pool, tint and cloud, so nothing is left behind and a later frame would start anew. */
export function strikeEffects(effects: Effects): void {
  for (const troupe of effects.troupes.values()) troupe.group.remove();
  effects.troupes.clear();
  effects.tints.clear();
  effects.dust.group?.remove();
  effects.dust.group = null;
  effects.dust.clouds.length = 0;
  effects.dust.live = 0;
  effects.element.remove();
  effects.live = 0;
  effects.scale = Number.NaN;
}
//#endregion 🔖️Particles

//#region 🔖️Dust
/** 💭️ One puff of dust of a frame: the middle and the size of the body a pet vanished from, in stage coordinates, and how far the dust has spread and faded (`phase`, 0 the tick the pet vanished, towards 1 as it clears): the `PuffFrame` of the contract. */
export type EffectPuff = { readonly x: number; readonly y: number; readonly width: number; readonly height: number; readonly phase: number };

/** ☁️ A puff of dust at its phase: where its middle is on the stage, how opaque it is, and its blobs — the middle of each relative to the middle of the puff and its radius, in stage pixels; the first blob is the middle one. */
export type PuffShape = { readonly x: number; readonly y: number; readonly opacity: number; readonly blobs: readonly { readonly x: number; readonly y: number; readonly r: number }[] };

/** 🌫️ The most puffs of dust drawn at once; of a frame with more, the first ones are drawn. Poofs are the last resort of the stage and must stay rare. */
export const PUFF_CLOUDS = 8;

/** 🫧️ How many blobs ring the middle blob of a puff. */
export const PUFF_RING = 7;

/** ⬆️ How far a puff rises while it clears, in heights of the body it replaces. */
export const PUFF_RISE = 0.25;

/** 🖊️ The stroke width of the outline circles of a puff, in stage pixels: half of it shows around the union of its blobs. */
export const PUFF_OUTLINE = 3;

/** 🌬️ The dust of `puff` at its phase (held to [0, 1]): it rises by {@link PUFF_RISE} heights of the body and fades as `1 − phase²`; a middle blob of half the mean half-size of the body and {@link PUFF_RING} blobs on an ellipse of the body's half-width and half-height — the even ones 0.36, the odd ones 0.28 of that size —, which spreads from 0.45 of it to the outline, eased out (`1 − (1 − phase)²`), and turns by a twentieth of a turn while every blob shrinks to half its size. The first blob of the ring lies straight to the right of the middle at phase 0. */
export function puffShape(puff: EffectPuff): PuffShape {
  const phase = Math.min(Math.max(puff.phase, 0), 1);
  const spread = 1 - (1 - phase) * (1 - phase);
  const size = (puff.width + puff.height) / 4;
  const shrink = 1 - 0.5 * phase;
  const reach = 0.45 + 0.55 * spread;
  const blobs = [{ x: 0, y: 0, r: 0.5 * size * shrink }];
  for (let index = 0; index < PUFF_RING; index++) {
    const radians = (index / PUFF_RING + 0.05 * phase) * 2 * Math.PI;
    blobs.push({ x: (Math.cos(radians) * reach * puff.width) / 2, y: (Math.sin(radians) * reach * puff.height) / 2, r: (index % 2 === 0 ? 0.36 : 0.28) * size * shrink });
  }
  return { x: puff.x, y: puff.y - PUFF_RISE * puff.height * phase, opacity: 1 - phase * phase, blobs };
}

/** 🏭️ The `index`-th cloud of the dust, built on first need: the group of the dust goes behind every particle, a cloud is a hidden group with an outline circle in ink per blob and, above them, a fill circle in paper per blob, so its outline runs round the union of its blobs. */
function cloudOf(effects: Effects, index: number): Cloud {
  const dust = effects.dust;
  const known = dust.clouds[index];
  if (known !== undefined) return known;
  const create = maker(effects.element.ownerDocument);
  if (dust.group === null) {
    dust.group = create("g", [[DUST, ""]]);
    effects.element.prepend(dust.group);
  }
  const blobs = PUFF_RING + 1;
  const outlines = Array.from({ length: blobs }, () => create("circle", [["class", "pet-fill-ink pet-stroke-ink"], ["stroke-width", String(PUFF_OUTLINE)]]));
  const fills = Array.from({ length: blobs }, () => create("circle", [["class", "pet-fill-paper pet-stroke-none"]]));
  const group = create("g", [[PUFF, ""], [VISIBILITY, "hidden"]]);
  group.append(...outlines, ...fills);
  dust.group.append(group);
  const cloud: Cloud = { group, outlines, fills, painted: new Float64Array(CLOUD_SLOTS + blobs * BLOB_SLOTS).fill(Number.NaN) };
  dust.clouds.push(cloud);
  return cloud;
}

/** 💨️ Applies the `puffs` of a frame at `scale`, the size pets are drawn at: the n-th puff (up to {@link PUFF_CLOUDS}) takes the n-th cloud; its `transform` (place to hundredths of a pixel) and `opacity`, and the middle and radius of every blob of {@link puffShape} are written only when they changed, its `visibility` only when the cloud comes into use; clouds the frame no longer needs are hidden. A frame without puffs after a frame without puffs writes nothing. */
export function paintPuffs(effects: Effects, puffs: readonly EffectPuff[], scale = 1): void {
  const dust = effects.dust;
  if (puffs.length === 0 && dust.live === 0) return;
  scaled(effects, scale);
  const shown = Math.min(puffs.length, PUFF_CLOUDS);
  for (let index = 0; index < shown; index++) {
    const cloud = cloudOf(effects, index);
    const shape = puffShape(puffs[index]!);
    const painted = cloud.painted;
    const x = rounded(shape.x, PLACE);
    const y = rounded(shape.y, PLACE);
    const opacity = rounded(shape.opacity);
    if (x !== painted[0] || y !== painted[1]) {
      painted[0] = x;
      painted[1] = y;
      cloud.group.setAttribute("transform", `translate(${x} ${y})`);
    }
    if (opacity !== painted[2]) {
      painted[2] = opacity;
      cloud.group.setAttribute("opacity", String(opacity));
    }
    for (const [blob, { x: bx, y: by, r }] of shape.blobs.entries()) {
      const slot = CLOUD_SLOTS + blob * BLOB_SLOTS;
      const values = [rounded(bx, PLACE), rounded(by, PLACE), rounded(r, PLACE)] as const;
      for (const [entry, name] of (["cx", "cy", "r"] as const).entries()) {
        if (values[entry] === painted[slot + entry]) continue;
        painted[slot + entry] = values[entry];
        cloud.outlines[blob]!.setAttribute(name, String(values[entry]));
        cloud.fills[blob]!.setAttribute(name, String(values[entry]));
      }
    }
    if (index >= dust.live) cloud.group.setAttribute(VISIBILITY, "visible");
  }
  for (let index = shown; index < dust.live; index++) dust.clouds[index]!.group.setAttribute(VISIBILITY, "hidden");
  dust.live = shown;
}
//#endregion 🔖️Dust
