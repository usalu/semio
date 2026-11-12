/** 🧰️ Shared helpers of the FBX encoder tests: fixture loading, hashing and the inert operation. */
import { createHash } from "node:crypto";
import { Matrix4, Quaternion, Vector3 } from "three";
import type { ModelDocument, Node as ModelNode, Operation } from "../../../../🧬️schema/🟦️.ts";
import { directoryNear, fixtureNames, readFixture } from "../../../../🧪️tests/🧰️support/🧫️reader/🟦️.ts";
import type { FbxOptions } from "../../🟦️.ts";

/** 🧫️ One encoding case of a fixture: options plus the structural digests every implementation must reproduce. */
export type FbxCase = {
  readonly name: string;
  readonly options: FbxOptions;
  readonly expected: { readonly byteLength: number; readonly sha256: string; readonly objectCounts: Record<string, number>; readonly notices: readonly string[]; readonly connections: readonly string[]; readonly outline: readonly string[] };
};
/** 📄️ A fixture file of one document. */
export type FbxFixture = { readonly document: ModelDocument; readonly cases: readonly FbxCase[] };

const FIXTURES = directoryNear(import.meta.url, "../../🧫️fixtures");

/** 🗂️ The fixture folders, discovered on disk so folder names are never retyped. */
export function fixtureFolders(): string[] {
  return fixtureNames(FIXTURES);
}

/** 📥️ Loads the fixture of the folder whose name contains `part`. */
export function fixture<T = FbxFixture>(part: string): T {
  const folder = fixtureFolders().find((name) => name.includes(part));
  if (!folder) throw new Error(`no fixture folder contains ${part}`);
  return readFixture<T>(FIXTURES, folder);
}

/** 📋️ The names (without emoji) of the fixtures holding a document with encoding cases. */
export const DOCUMENT_FIXTURES = ["rigged-arm", "static-cube", "rotation-continuity"] as const;

/** 🔏️ Lower-case hex SHA-256. */
export function sha256(bytes: Uint8Array): string {
  return createHash("sha256").update(bytes).digest("hex");
}

/** ⏱️ An operation that is never cancelled and ignores progress. */
export const NO_OPERATION: Operation = { report() {}, cancelled: () => false };

/** 🧮️ The local matrix of a node, composed by three (the independent oracle of the encoder's own matrices). */
export function localMatrix(node: ModelNode): Matrix4 {
  return new Matrix4().compose(new Vector3(...node.translation), new Quaternion(...node.rotation).normalize(), new Vector3(...node.scale));
}

/** 🌳️ The world matrices of the nodes of a document (parents first), composed by three. */
export function worldMatricesOf(document: ModelDocument): Map<string, Matrix4> {
  const world = new Map<string, Matrix4>();
  for (const node of document.nodes) world.set(node.id, node.parent === null ? localMatrix(node) : world.get(node.parent)!.clone().multiply(localMatrix(node)));
  return world;
}
