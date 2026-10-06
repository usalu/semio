/** 🦴️ Subject adapter of the rig-solving case: the pets rig module answers every committed vector.
 *
 * @see ./🥒️.feature
 * @see ../../🔨️modules/🦴️rig/🟦️.ts
 */
import { type AdapterContext, defineTestAdapter } from "../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type Affine, type Pose, compose, invert, restPose, solveRig, transform } from "../../🔨️modules/🦴️rig/🟦️.ts";
import type { Species } from "../../🧬️schema/🟦️.ts";

const VECTORS = "shared://🦴️rig-solving/🔣️.json";

type Vectors = {
  readonly products: readonly { readonly id: string; readonly parent: Affine; readonly local: Affine }[];
  readonly inverses: readonly { readonly id: string; readonly matrix: Affine }[];
  readonly points: readonly { readonly id: string; readonly matrix: Affine; readonly x: number; readonly y: number }[];
  readonly species: readonly Species[];
  readonly restPoses: readonly { readonly id: string; readonly species: string }[];
  readonly skeletons: readonly { readonly id: string; readonly species: string; readonly pose: Pose }[];
};

/** 🧫️ The committed vectors. */
function vectors(ctx: AdapterContext): Vectors {
  return JSON.parse(new TextDecoder().decode(ctx.inputBytes(VECTORS))) as Vectors;
}

/** 🧬️ The committed species of that id. */
function speciesOf(document: Vectors, id: string): Species {
  const species = document.species.find((candidate) => candidate.id === id);
  if (species === undefined) throw new Error(`the vectors name the unknown species ${id}`);
  return species;
}

const VIEW = new DataView(new ArrayBuffer(8));

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🧾️ The bit patterns of every product, inverse, carried point and solved skeleton. */
function patterns(document: Vectors): Record<string, Record<string, unknown>> {
  return {
    products: Object.fromEntries(document.products.map((vector) => [vector.id, compose(vector.parent, vector.local).map(bits)])),
    inverses: Object.fromEntries(document.inverses.map((vector) => [vector.id, invert(vector.matrix).map(bits)])),
    points: Object.fromEntries(
      document.points.map((vector) => {
        const carried = transform(vector.matrix, vector.x, vector.y);
        return [vector.id, { x: bits(carried.x), y: bits(carried.y) }];
      }),
    ),
    skeletons: Object.fromEntries(document.skeletons.map((vector) => [vector.id, solveRig(speciesOf(document, vector.species), vector.pose).map(bits)])),
  };
}

/** 🧍️ The rest pose of a species and the skeleton it solves to. */
function rested(species: Species): { pose: Pose; bones: number[] } {
  const pose = restPose(species);
  return { pose, bones: solveRig(species, pose) };
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    products: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).products.map((vector) => [vector.id, compose(vector.parent, vector.local)])) }) },
    inverses: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).inverses.map((vector) => [vector.id, invert(vector.matrix)])) }) },
    points: { subject: (ctx) => ({ projection: Object.fromEntries(vectors(ctx).points.map((vector) => [vector.id, transform(vector.matrix, vector.x, vector.y)])) }) },
    "rest-poses": {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.restPoses.map((vector) => [vector.id, rested(speciesOf(document, vector.species))])) };
      },
    },
    skeletons: {
      subject: (ctx) => {
        const document = vectors(ctx);
        return { projection: Object.fromEntries(document.skeletons.map((vector) => [vector.id, solveRig(speciesOf(document, vector.species), vector.pose)])) };
      },
    },
    "bit-patterns": { subject: (ctx) => ({ projection: patterns(vectors(ctx)) }) },
  },
});
