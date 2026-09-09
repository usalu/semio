import { builtinModules } from "node:module";
import { resolve } from "node:path";

const extensionExternals = new Set(["vscode", ...builtinModules]);

/** 🕰️ Gives every VSIX entry a reproducible, ZIP-safe timestamp on every host. */
export function extensionPackageEnvironment(environment: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return { ...environment, SOURCE_DATE_EPOCH: "315532800", TZ: "UTC" };
}

/** 📦️Defines a dependency-bundled CommonJS entry while retaining VS Code and Node host modules. */
export function extensionBuildConfig(root: string, entry: string, outputDirectory: string, outputFile: string, watch: boolean) {
  const define = { "import.meta.vitest": "undefined" };
  return {
    configFile: false,
    define,
    plugins: [{
      name: "semio-extension-host-tests",
      enforce: "pre" as const,
      async transform(code: string, id: string) {
        if (!/\.[cm]?[jt]sx?(?:\?|$)/.test(id) || !code.includes("import.meta.vitest")) return;
        const { transformWithEsbuild } = await import("vite");
        const result = await transformWithEsbuild(code, id, { define, treeShaking: true, minifySyntax: true, sourcemap: false });
        return { code: result.code, map: null };
      },
    }],
    root,
    build: {
      emptyOutDir: true,
      lib: { entry: resolve(root, entry), formats: ["cjs" as const], fileName: () => outputFile },
      minify: false,
      outDir: resolve(root, outputDirectory),
      rollupOptions: { external: (id: string) => id.startsWith("node:") || extensionExternals.has(id) },
      sourcemap: false,
      target: "node22",
      watch: watch ? {} : undefined,
    },
  };
}
