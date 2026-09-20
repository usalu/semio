/** 🌿️ Ambient declarations for the npm packages `💻️os`'s own sources import that ship no types.
 *
 * Separate from `🧰️framework/📦️packages/🟦️typescript/🌿️ambient/🟦️.d.ts` because a `declare module`
 * block only applies inside the PROGRAM that contains its file, and this product's tsconfig includes
 * `🧰️framework/🛍️products/💻️os/**` only — the framework package's ambient file is never part of it.
 * Same discipline as that file: only the exact members the live call sites here use, and an entry
 * goes away the moment the package's real types become reachable.
 */

/** 🔢️ The same codec's IEEE-754 sibling, the reference oracle `🎒️pack`'s float scalars are witnessed against. */
declare module "@webassemblyjs/ieee754" {
  /** 🔢️ Encodes a double as its eight little-endian IEEE-754 bytes. */
  export function encodeF64(value: number): readonly number[];
  /** 🔢️ Decodes eight little-endian IEEE-754 bytes back to a double. */
  export function decodeF64(bytes: readonly number[]): number;
}

/** 🔗️ Spec-complete URL implementation, the independent oracle the deployment route and staging-root
 * path arithmetic are checked against (never the `node:url` the production code itself uses). */
declare module "whatwg-url" {
  /** 🔗️ Parses `url`, resolved against `base` when it is a relative reference. */
  export class URL {
    constructor(url: string, base?: string);
    readonly href: string;
    readonly pathname: string;
  }
}

/** 🧰️ Lodash's ES build, used only as a second implementation in tests and build scripts. */
declare module "lodash-es" {
  /** 🧰️ Rebuilds an object from `[key, value]` pairs — the oracle for `Object.fromEntries`. */
  export function fromPairs<Value>(pairs: readonly (readonly [PropertyKey, Value])[]): Record<string, Value>;
}

/** 🧰️ Lodash's `findIndex` imported as its own module, the deep-import form the rust package script uses. */
declare module "lodash-es/findIndex.js" {
  /** 🧰️ Index of the first element `predicate` accepts, or `-1`. */
  export default function findIndex<Element>(collection: readonly Element[], predicate: (value: Element, index: number, collection: readonly Element[]) => boolean): number;
}
