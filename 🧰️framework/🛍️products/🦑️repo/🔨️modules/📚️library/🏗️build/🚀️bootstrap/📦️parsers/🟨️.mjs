import { createRequire } from "node:module";
import { typescriptCompilerV1 } from "../🔷️typescript-compiler/🟨️.mjs";

const slot = createRequire(import.meta.url);

/** 📦️ Loads the TypeScript compiler API of the tooling through its one interface: the exact version the parser package declares, from the install slot of the parser package or the direct dependency of the checkout root, never from a path the process did not declare; the request `slot("typescript")` sits beside the manifest that pins the version, so a captured implementation admits it as the installed package at that version and every process shape (main, farm child, slice worker, captured loader) loads the same file; a wrong or incomplete install is the typed refusal `TypescriptCompilerRefusalV1`, retry never.
 * @see ../🔷️typescript-compiler/🟨️.mjs */
export const parserTypescriptV1 = () => typescriptCompilerV1({ load: () => slot("typescript") });
