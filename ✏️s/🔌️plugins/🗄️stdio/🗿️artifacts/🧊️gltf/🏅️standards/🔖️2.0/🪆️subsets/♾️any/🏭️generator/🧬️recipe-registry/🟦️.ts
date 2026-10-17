/** 🧬️ glTF fixture generator: the recipe type, the registry every family registers into and the document helpers the recipes share. */
import type { Doc } from "../🏗️base-document/🟦️.ts";

//#region 🧬️Recipes
export type Recipe = { id: string; mutationId: string; outcome: "applied"; family: string; notes: string; build: (base: Doc) => { before: Doc; after: Doc } };

export function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

export function bufferView(doc: Doc, byteLength: number): number {
  const buffer0 = 0;
  const view = { buffer: buffer0, byteOffset: 0, byteLength };
  doc.bufferViews ??= [];
  doc.bufferViews.push(view);
  return doc.bufferViews.length - 1;
}
export function minimalAccessor(doc: Doc): number {
  const view = bufferView(doc, 12);
  doc.accessors ??= [];
  doc.accessors.push({ bufferView: view, componentType: 5126, count: 1, type: "SCALAR" });
  return doc.accessors.length - 1;
}

export const RECIPES: Recipe[] = [];
export function recipe(id: string, mutationId: string, family: string, notes: string, build: Recipe["build"]): void {
  RECIPES.push({ id, mutationId, outcome: "applied", family, notes, build });
}
//#endregion 🧬️Recipes
