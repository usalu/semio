/** 🚧️ The production test boundary of the dev server's Vite build: strips the in-source test branches from production output. */
//#region BackboneVitePlugin
/** 🧹️ Eliminates in-source test branches before production asset URL collection. */
export function semioProductionTestBoundaryVitePlugin(): { name: string; enforce: "pre"; apply: "build"; transform(source: string, id: string): Promise<{ code: string; map: string } | null> } {
  return {
    name: "semio-production-test-boundary",
    enforce: "pre",
    apply: "build",
    async transform(source, id) {
      if (!source.includes("import.meta.vitest") || !/\.[cm]?[jt]sx?(?:[?#].*)?$/u.test(id)) return null;
      const { transformWithEsbuild } = await import("vite");
      const result = await transformWithEsbuild(source, id, { define: { "import.meta.vitest": "undefined" }, minifySyntax: true, target: "esnext", charset: "utf8", jsx: "preserve", sourcemap: true });
      return { code: result.code, map: JSON.stringify(result.map) };
    },
  };
}
//#endregion BackboneVitePlugin
