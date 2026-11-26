/** 🧭️ Private test oracle declaration: the WHATWG URL parser of the `whatwg-url` package, the only members the wasm output index test reads. */
declare module "whatwg-url" {
 export class URL {
  constructor(input: string, base?: string);
  readonly href: string;
  readonly pathname: string;
  readonly protocol: string;
  readonly searchParams: URLSearchParams;
 }
}
