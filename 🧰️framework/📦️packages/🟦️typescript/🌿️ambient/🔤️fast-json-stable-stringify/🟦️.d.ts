/** 🔤️ Real surface of `fast-json-stable-stringify`, the deterministic-JSON oracle the ordered-value
 * and taxonomy contracts are checked against.
 *
 * The package ships an `index.d.ts` declaring a single-argument `stringify(obj)`, which its own
 * runtime contradicts: the second parameter carries the key comparator every byte-order oracle in
 * this repo passes (`{ cmp }`). This declaration states the shape that is actually called and is
 * routed in through the framework `tsconfig.json` `paths` entry. Delete it the moment upstream ships
 * correct types.
 *
 * @see https://github.com/epoberezkin/fast-json-stable-stringify
 */
declare function stringify(
  value: unknown,
  options?: {
    readonly cmp?: (a: { readonly key: string; readonly value: unknown }, b: { readonly key: string; readonly value: unknown }) => number;
    readonly cycles?: boolean;
  },
): string;

export = stringify;
