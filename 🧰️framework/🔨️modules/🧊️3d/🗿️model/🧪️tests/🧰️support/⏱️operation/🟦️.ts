/** ⏱️ Helpers of the progress and cancellation law: the inputs of `🧫️fixtures/⏱️operation`, an operation that counts its polls and cancels at a chosen one, and the two assertions every suite shares. fast-check is the independent oracle that picks the poll at which to cancel. */
import fc from "fast-check";
import { expect } from "vitest";
import { CancelledError } from "../../../⏱️operation/🟦️.ts";
import { tessellate } from "../../../🔺️tessellation/🟦️.ts";
import type { ModelDocument, Operation, Quality, Solid } from "../../../🧬️schema/🟦️.ts";
import { fixture } from "../🟦️.ts";

/** 📋️ One case of the fixture. */
export type OperationCase = { readonly function: string; readonly input: string; readonly minimumPolls: number; readonly cancelAtPolls: readonly number[] };
/** 📄️ The fixture. */
export type OperationVector = {
  readonly interval: number;
  readonly quality: Quality;
  readonly document: { readonly id: string; readonly material: string; readonly node: string; readonly solid: Record<string, unknown> };
  readonly polygon: { readonly sides: number; readonly radius: number; readonly extrude: { readonly plane: "xy" | "yz" | "zx"; readonly from: number; readonly to: number } };
  readonly bytes: { readonly length: number; readonly seed: number; readonly multiplier: number; readonly modulus: number; readonly alphabet: number };
  readonly raster: { readonly width: number; readonly height: number; readonly preset: "axonometric" };
  readonly cases: readonly OperationCase[];
};

/** 📥️ The fixture of the law. */
export const vector = (): OperationVector => fixture<OperationVector>("⏱️operation");

/** 🏗️ The procedural document of one solid with its node and material. */
export const proceduralDocument = (solid: Record<string, unknown>): ModelDocument => {
  const { document } = vector();
  return {
    version: "semio.model/v1",
    id: document.id,
    title: { en: document.id, de: document.id },
    nodes: [{ id: document.node, parent: null, translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] }],
    solids: [{ node: document.node, material: document.material, ...solid } as unknown as Solid],
    meshes: [],
    skins: [],
    materials: [{ id: document.material, title: { en: "paper", de: "Papier" }, baseColor: [0.8, 0.8, 0.8, 1], metallic: 0, roughness: 0.8, emissive: [0, 0, 0], alphaMode: "opaque", doubleSided: false, unlit: false }],
    variants: [],
    poses: [],
    animations: [],
  };
};

/** 🥚️ The dense ellipsoid as a procedural document. */
export const denseDocument = (): ModelDocument => proceduralDocument(vector().document.solid);

/** 🧊️ The dense ellipsoid tessellated. */
export const denseResolved = (): Promise<ModelDocument> => tessellate(denseDocument(), vector().quality);

/** 🔺️ The regular polygon of the fixture. */
export const polygonOf = (): [number, number][] => {
  const { sides, radius } = vector().polygon;
  return Array.from({ length: sides }, (_, k) => [radius * Math.cos((2 * Math.PI * k) / sides), radius * Math.sin((2 * Math.PI * k) / sides)] as [number, number]);
};

/** 📐️ The polygon extruded as a procedural document. */
export const polygonDocument = (): ModelDocument => proceduralDocument({ kind: "extrude", id: "polygon-prism", profile: polygonOf(), ...vector().polygon.extrude });

/** 🔢️ The pseudo-random bytes of the fixture (minstd). */
export const bytesOf = (): Uint8Array => {
  const { length, seed, multiplier, modulus, alphabet } = vector().bytes;
  const out = new Uint8Array(length);
  let x = seed;
  for (let i = 0; i < length; i++) {
    x = (x * multiplier) % modulus;
    out[i] = x % alphabet;
  }
  return out;
};

/** 🔬️ An operation that counts its polls and answers true from poll `cancelAtPoll` on (never when null). */
export const probe = (cancelAtPoll: number | null = null): { readonly operation: Operation; readonly state: { polls: number; reports: number[] } } => {
  const state = { polls: 0, reports: [] as number[] };
  return { state, operation: { report: (fraction) => void state.reports.push(fraction), cancelled: () => ++state.polls >= (cancelAtPoll ?? Infinity) } };
};

/** 🧾️ Runs `run` uncancelled: it must poll at least `minimumPolls` times, report only fractions in [0, 1] and return the result. */
export async function expectPolling<T>(run: (operation: Operation) => T | Promise<T>, minimumPolls: number): Promise<Awaited<T>> {
  const { operation, state } = probe();
  const result = await run(operation);
  expect(state.polls).toBeGreaterThanOrEqual(minimumPolls);
  expect(state.reports.every((fraction) => fraction >= 0 && fraction <= 1)).toBe(true);
  return result;
}

/** 🛑️ Runs `run` cancelled at poll `cancelAtPoll`: it must fail with a `CancelledError` having polled exactly that often (it stops at the first positive answer) and return no result. */
export async function expectCancelledAt<T>(run: (operation: Operation) => T | Promise<T>, cancelAtPoll: number): Promise<void> {
  const { operation, state } = probe(cancelAtPoll);
  let returned = false;
  let error: unknown;
  try {
    await run(operation);
    returned = true;
  } catch (caught) {
    error = caught;
  }
  expect(returned, `a result was returned although cancelled at poll ${cancelAtPoll}`).toBe(false);
  expect(error).toBeInstanceOf(CancelledError);
  expect(state.polls).toBe(cancelAtPoll);
}

/** 🎲️ The whole law of one case: at least `minimumPolls` polls uncancelled, every listed poll and every poll fast-check picks below `minimumPolls` cancels exactly there. */
export async function expectLaw<T>(run: (operation: Operation) => T | Promise<T>, entry: OperationCase): Promise<Awaited<T>> {
  const result = await expectPolling(run, entry.minimumPolls);
  for (const poll of entry.cancelAtPolls) await expectCancelledAt(run, poll);
  await fc.assert(fc.asyncProperty(fc.integer({ min: 1, max: entry.minimumPolls }), (poll) => expectCancelledAt(run, poll)), { numRuns: 12 });
  return result;
}

/** 🔎️ The case of `name`. */
export const caseOf = (name: string): OperationCase => {
  const found = vector().cases.find((entry) => entry.function === name);
  if (!found) throw new Error(`the fixture has no case for ${name}`);
  return found;
};
