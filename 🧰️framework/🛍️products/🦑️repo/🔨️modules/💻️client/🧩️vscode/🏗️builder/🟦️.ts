import { buildModePolicy, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { builtinModules, createRequire } from "node:module";
import { resolve } from "node:path";

const extensionExternals = new Set(["vscode", ...builtinModules]);

/** 🕰️ Gives every VSIX entry a reproducible, ZIP-safe timestamp on every host. */
export function extensionPackageEnvironment(environment: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return { ...environment, SOURCE_DATE_EPOCH: "315532800", TZ: "UTC" };
}

/** 📦️Defines a dependency-bundled CommonJS entry while retaining VS Code and Node host modules. */
export function extensionBuildConfig(root: string, entry: string, outputDirectory: string, outputFile: string, mode: BuildMode) {
  const policy = buildModePolicy(mode);
  const define = { "import.meta.vitest": "undefined" };
  return {
    configFile: false as const,
    define,
    plugins: [{
      name: "semio-extension-host-tests",
      enforce: "pre" as const,
      async transform(code: string, id: string) {
        if (!/\.[cm]?[jt]sx?(?:\?|$)/.test(id) || !code.includes("import.meta.vitest")) return;
        const ts = createRequire(resolve(root, "package.json"))("typescript"), source = ts.createSourceFile(id, code, ts.ScriptTarget.Latest, true, /\.[jt]sx(?:\?|$)/u.test(id) ? ts.ScriptKind.TSX : /\.[cm]?js(?:\?|$)/u.test(id) ? ts.ScriptKind.JS : ts.ScriptKind.TS), edits: { start: number; end: number; replacement: string }[] = [];
        const visit = (node: any) => {
          if (ts.isIfStatement(node) && node.expression.getText(source).replaceAll(/\s/gu, "") === "import.meta.vitest") { edits.push({ start: node.getStart(source), end: node.end, replacement: node.elseStatement?.getText(source) ?? "" }); return; }
          ts.forEachChild(node, visit);
        };
        visit(source); let stripped = code;
        for (const edit of edits.sort((left, right) => right.start - left.start)) stripped = stripped.slice(0, edit.start) + edit.replacement + stripped.slice(edit.end);
        return { code: stripped, map: null };
      },
    }],
    root,
    build: {
      emptyOutDir: true,
      lib: { entry: resolve(root, entry), formats: ["cjs" as const], fileName: () => outputFile },
      minify: policy.optimize ? "esbuild" as const : false as const,
      outDir: resolve(root, outputDirectory),
      rollupOptions: { treeshake: policy.optimize, external: (id: string) => id.startsWith("node:") || extensionExternals.has(id) },
      sourcemap: policy.sourceMap,
      target: "node22",
    },
  };
}
