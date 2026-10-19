// @vitest-environment node
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test } from "vitest";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../🎯️targets/⚛️react/📦️packages/🟦️typescript");
const manifest = JSON.parse(readFileSync(resolve(root, "package.json"), "utf8")) as { exports: Record<string, string> };
const config = JSON.parse(readFileSync(resolve(root, "tsconfig.json"), "utf8")) as { compilerOptions: { paths: Record<string, string[]> } };

test("package entry and self-alias resolve to the canonical React source", () => {
  const entry = manifest.exports["."];
  expect(entry).toBe("./🟦️.tsx");
  expect(config.compilerOptions.paths["@semio-tech/ui-react"]).toEqual([entry]);
  expect(existsSync(resolve(root, entry))).toBe(true);
  expect(readFileSync(resolve(root, entry), "utf8")).toContain('from "../../🟦️.tsx"');
  expect(existsSync(resolve(root, "../../🟦️.tsx"))).toBe(true);
});

test("introduction subpath and self-alias resolve to the independent introduction owner", () => {
  const entry = manifest.exports["./introduction"];
  expect(entry).toBe("./👋️introduction.ts");
  expect(config.compilerOptions.paths["@semio-tech/ui-react/introduction"]).toEqual([entry]);
  expect(readFileSync(resolve(root, entry), "utf8")).toContain('from "../../👋️introduction/🟦️.tsx"');
  expect(existsSync(resolve(root, "../../👋️introduction/🟦️.tsx"))).toBe(true);
});

test("chrome card imports omit the layered renderer until it is requested", async () => {
  const { build } = await import("vite");
  const chrome = resolve(root, "../../🪟️chrome/🟦️.ts").replaceAll("\\", "/");
  const layered = "/🧱️elements/🥞️LayeredOverview/🟦️.tsx";
  const renderLength = async (name: "OverviewCard" | "LayeredOverview"): Promise<number> => {
    const entry = "virtual:chrome-boundary";
    const result = await build({
      configFile: false,
      envFile: false,
      root,
      logLevel: "silent",
      plugins: [{ name: "chrome-boundary", resolveId: id => id === entry ? entry : undefined, load: id => id === entry ? `export { ${name} } from ${JSON.stringify(chrome)};` : undefined }],
      build: { write: false, minify: false, rollupOptions: { input: entry, preserveEntrySignatures: "strict", external: [/^react(?:\/|$)/, /^react-dom(?:\/|$)/, /^@semio-tech\//, /^i18next$/, /^react-i18next$/], output: { format: "es" } } },
    });
    const outputs = (Array.isArray(result) ? result : [result]).flatMap(output => "output" in output ? output.output : []);
    const chunks = outputs.filter(output => output.type === "chunk");
    expect(chunks.some(chunk => chunk.exports.includes(name))).toBe(true);
    return chunks.flatMap(chunk => Object.entries(chunk.modules)).filter(([id]) => id.replaceAll("\\", "/").endsWith(layered)).reduce((length, [, module]) => length + module.renderedLength, 0);
  };
  expect(await renderLength("OverviewCard")).toBe(0);
  expect(await renderLength("LayeredOverview")).toBeGreaterThan(0);
}, 30_000);
