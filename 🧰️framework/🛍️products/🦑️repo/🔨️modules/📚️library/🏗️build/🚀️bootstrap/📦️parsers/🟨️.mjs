import { typescriptCompilerV1 } from "../🔷️typescript-compiler/🟨️.mjs";

/** 📦️ Loads the TypeScript compiler API of the tooling through its one interface: the exact version the parser package declares, from the install slot of the parser package or the direct dependency of the checkout root, never by the bare name; a wrong or incomplete install is the typed refusal `TypescriptCompilerRefusalV1`, retry never.
 * @see ../🔷️typescript-compiler/🟨️.mjs */
export const parserTypescriptV1 = () => typescriptCompilerV1();
