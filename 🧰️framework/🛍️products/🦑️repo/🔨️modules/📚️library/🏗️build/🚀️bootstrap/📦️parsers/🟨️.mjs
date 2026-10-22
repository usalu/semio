import { createRequire } from "node:module";

const own = createRequire(import.meta.url);

/** 📦️ Loads the TypeScript compiler API from the parser package: the node resolution starts in the parser package directory and, when its install is absent, continues in the checkout root install. */
export const parserTypescriptV1 = () => own("typescript");
