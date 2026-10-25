/** 🔀️ Types of the router command reading: specifier sets stay structural data. */
export type RouterCompilerV1 = any;
export function routerCommandSpecifiersV1(compiler: RouterCompilerV1, path: string, text: string, command: string): ReadonlySet<string> | undefined;
export function routerCommandWordV1(bound: { readonly script: string | null; readonly arguments: readonly string[] } | undefined): string | undefined;
