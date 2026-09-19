/** 🌿️ Ambient declarations for the npm packages `🧰️framework/🔨️modules/**` imports that ship no types.
 *
 * Each package below is a real runtime dependency with a live call site in this program; none of them
 * publishes a `.d.ts` that `moduleResolution: "bundler"` can reach, and none of them has an
 * `@types/*` companion installed (`bun add -d @types/semver` fails repo-wide for the same workspace
 * reason recorded in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts`).
 * Declared here only: the exact members those call sites use, so this file cannot drift into a
 * hand-maintained copy of the packages' full surfaces. Delete an entry the moment its real types
 * become installable.
 */

/** 🔢️ LEB128 varint codec, the reference oracle the replication wire format is verified against. */
declare module "@webassemblyjs/leb128" {
  /** 🔢️ Encodes an unsigned 32-bit value as an LEB128 byte sequence. */
  export function encodeU32(value: number): Uint8Array;
  /** 🔢️ Decodes an unsigned 64-bit LEB128 value starting at `index`. */
  export function decodeUInt64(encoded: Uint8Array, index: number): { readonly value: bigint; readonly nextIndex: number };
}

/** 🔢️ The same codec's inner module, imported directly for `encodeUIntBuffer`. */
declare module "@webassemblyjs/leb128/lib/leb.js" {
  /** 🔢️ Encodes an already little-endian byte buffer as an LEB128 byte sequence. */
  export function encodeUIntBuffer(buffer: Uint8Array): Uint8Array;
  /** 🔢️ Decodes an unsigned 64-bit LEB128 value starting at `index`. */
  export function decodeUInt64(encoded: Uint8Array, index: number): { readonly value: bigint; readonly nextIndex: number };
  const leb: { readonly encodeUIntBuffer: typeof encodeUIntBuffer; readonly decodeUInt64: typeof decodeUInt64 };
  export default leb;
}

/** 🏷️ Semantic-version matcher, the reference oracle the kernel's version-requirement adapter is checked against. */
declare module "semver" {
  /** 🏷️ Reports whether `version` satisfies the `range` expression. */
  export function satisfies(version: string, range: string): boolean;
  const semver: { readonly satisfies: typeof satisfies };
  export default semver;
}

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

/** 🕸️ Vite's `?url` asset query: a wasm-pack payload imported as a served URL rather than a module.
 *
 * `🧊️3d`'s brep loader hands this URL to `initFlow({ module_or_path })`; the bundler rewrites the
 * specifier at build time, so only the type side needs declaring. */
declare module "*.wasm?url" {
  const url: string;
  export default url;
}
