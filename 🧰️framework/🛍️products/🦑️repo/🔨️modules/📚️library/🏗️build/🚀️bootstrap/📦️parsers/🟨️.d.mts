/** 📦️ Declares the one parser the build bootstrap loads before any other package exists: the TypeScript compiler API. */
export const parserTypescriptV1:()=>typeof import("typescript");
