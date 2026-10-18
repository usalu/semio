/** 📐️ The scene shading oracle fixture type and the scene shading oracle run. */
/// <reference types="@webgpu/types" />
import { oraclePixelDeltaV1 } from "../📋️data/🟦️.ts";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { paintedShaderFromProduction, renderSceneShading, renderSceneShadingS2 } from "../🎨️scene-shading-renderers/🟦️.ts";
import { renderWgpuCelebration, renderWgpuShading, renderWgpuTextured } from "../🧊️wgpu-shading-renderers/🟦️.ts";

declare const THREE: any;
declare const worldSurfaceMaterial: (name: string, material: any, vertexColors: boolean) => any;

/** 🌐️ The names the browser bundle's own prelude supplies to the oracle bodies below. Each
 * `run*Oracle` stringifies one of these functions (`Function.prototype.toString`) and prepends the
 * matching `import` lines before Bun bundles the pair for the page, so these bindings exist in the
 * BROWSER and are never resolved in this driver process — exactly like `THREE` above. */
declare const React: typeof import("react");
declare const createRoot: typeof import("react-dom/client").createRoot;
declare const Canvas: React.ComponentType<Record<string, unknown>>;
declare const Grid: React.ComponentType<Record<string, unknown>>;

/** 🔬️ Renders one producer without substituting computed values for GPU readback. */
/** 🎨️ What this oracle reads off the committed scene-shading fixture. `JSON.parse` answers `unknown`
 * in this program, so the corpus is named rather than assumed. */
type SceneShadingOracleFixture = {
    readonly pixelOracle: {
    readonly status: string;
    readonly producer: string;
    readonly profile: { readonly maximumRgbError: number };
    readonly rows: readonly { readonly id: string; readonly rgba8: readonly number[] }[];
  };
  readonly s2PixelOracle: {
    readonly status: string;
    readonly producer: string;
    readonly profile: { readonly maximumRgbError: number };
    readonly rows: readonly { readonly id: string; readonly rgba8: readonly number[] }[];
  };
};

export async function runSceneShadingOracle(
  repoRoot: string,
  command: "shading-oracle" | "shading-s2-oracle" | "shading-wgpu" | "shading-s2-wgpu" | "shading-s2-current-mesh" | "shading-s2-ordered-mesh" | "shading-s2-painted" | "shading-s2-celebration",
  outputDirectory?: string,
): Promise<void> {
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json");
  const output = outputDirectory ? resolve(outputDirectory) : undefined;
  if (output) await mkdir(output, { recursive: true });
  const playwrightSpecifier = "playwright";
  // 🧭️ The specifier is indirect on purpose (playwright is a dev-only dependency), which erases the
  // module type and makes every callback parameter below an implicit `any`.
  const { chromium } = (await import(playwrightSpecifier)) as typeof import("playwright");
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8")) as SceneShadingOracleFixture;
  const threePath = Bun.resolveSync("three", repoRoot);
  const webgpu = ["shading-wgpu", "shading-s2-wgpu", "shading-s2-current-mesh", "shading-s2-ordered-mesh", "shading-s2-painted", "shading-s2-celebration"].includes(command);
  const program = command === "shading-s2-painted" ? renderWgpuTextured : command === "shading-s2-celebration" ? renderWgpuCelebration : webgpu ? renderWgpuShading : command === "shading-s2-oracle" ? renderSceneShadingS2 : renderSceneShading;
  const source = 'import * as THREE from "three";\nglobalThis.renderSceneShading = ' + program.toString();
  const build = await Bun.build({
    entrypoints: ["semio-shading-oracle"],
    target: "browser",
    format: "iife",
    plugins: [
      {
        name: "ticket-shading-oracle",
        setup(builder) {
          builder.onResolve({ filter: /^semio-shading-oracle$/ }, () => ({ path: "semio-shading-oracle", namespace: "oracle" }));
          builder.onResolve({ filter: /^three$/ }, () => ({ path: threePath }));
          builder.onLoad({ filter: /.*/, namespace: "oracle" }, () => ({ contents: source, loader: "js" }));
        },
      },
    ],
  });
  if (!build.success) throw new Error(build.logs.map(String).join("\n"));
  const bundle=build.outputs[0];
  if(!bundle)throw Error("Successful oracle bundle output required");
  const javascript = await bundle.text();
  if (output) await writeFile(join(output, "oracle.js"), javascript);
  const browser = await chromium.launch({ headless: true, args: ["--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
  try {
    const page = await browser.newPage({ viewport: { width: 64, height: 64 }, deviceScaleFactor: 1 });
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    await page.route("https://semio-parity.invalid/**", (route) => route.fulfill({ contentType: "text/html", body: "<!doctype html><html><body></body></html>" }));
    await page.goto("https://semio-parity.invalid/");
    await page.addScriptTag({ content: javascript });
    let input: any = fixture;
    let shaderSha256: string | undefined;
    if (webgpu) {
      const shaderSource = await readFile(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs"), "utf8");
      const shaderName = command === "shading-s2-celebration" ? "WORLD3D_CELEBRATION_SHADER" : "WORLD3D_SHADER";
      const shader = shaderSource.match(new RegExp(`pub const ${shaderName}: &str = r#"([\\s\\S]*?)"#;`))?.[1];
      if (!shader) throw new Error(`Production ${shaderName} constant not found`);
      const exactShader = command === "shading-s2-painted" ? paintedShaderFromProduction(shader) : shader;
      shaderSha256 = createHash("sha256").update(exactShader).digest("hex");
      input = {
        fixture,
        shader: exactShader,
        ...(command === "shading-s2-wgpu"
          ? { mode: "s2-material-scope" }
          : command === "shading-s2-current-mesh"
            ? { mode: "s2-current-mesh" }
            : command === "shading-s2-ordered-mesh"
              ? { mode: "s2-ordered-mesh" }
              : {}),
      };
    }
    const result = await page.evaluate((input) => (globalThis as any).renderSceneShading(input), input);
    result.shaderSha256 = shaderSha256;
    if (errors.length) throw new Error(errors.join("\n"));
    const oracle = command.startsWith("shading-s2-") ? fixture.s2PixelOracle : fixture.pixelOracle;
    const expectedRows =
      command === "shading-s2-wgpu"
        ? oracle.rows.filter((row: any) => row.id.startsWith("mixed-scope-"))
        : command === "shading-s2-current-mesh"
          ? oracle.rows.filter((row: any) => row.id.startsWith("celebrated-") || row.id.startsWith("standard-transparent-"))
          : command === "shading-s2-ordered-mesh"
            ? oracle.rows.filter((row: any) => row.id.startsWith("standard-transparent-"))
            : command === "shading-s2-painted"
              ? oracle.rows.filter((row: any) => row.id.startsWith("paint-texture-"))
              : command === "shading-s2-celebration"
                ? oracle.rows.filter((row: any) => row.id.startsWith("celebrated-"))
                : oracle.rows;
    if (oracle.status === "recorded" && JSON.stringify(result.rows.map((row: any) => row.id)) !== JSON.stringify(expectedRows.map((row: any) => row.id)))
      throw new Error("Pixel case identities differ from the recorded fixture: " + JSON.stringify(result.rows.map((row: any) => row.id)) + " != " + JSON.stringify(expectedRows.map((row: any) => row.id)));
    const differences = result.rows.map((row: any) => {
      const expected = expectedRows.find((value: any) => value.id === row.id)?.rgba8;
      return { id: row.id, actual: row.rgba8, expected, delta: expected ? oraclePixelDeltaV1(row.rgba8,expected) : null };
    });
    result.differences = differences;
    if (output) await writeFile(join(output, "pixels.json"), JSON.stringify(result, null, 2));
    if (output)
      await writeFile(
        join(output, "report.md"),
        [
          "# " + (webgpu ? "Production WGSL WebGPU" : command === "shading-s2-oracle" ? "Three WebGL S2" : "Three WebGL") + " Shading Pixels",
          "",
          "Actual browser pixels from the shared fixture. WebGPU uses production WGSL and matching mesh buffers, uniforms, and blends in a controlled offscreen pipeline; full renderer integration is a separate gate.",
          "",
          "GPU: " + JSON.stringify(result.gpu),
          "",
          "Shader SHA256: " + (shaderSha256 ?? "Three installed shader"),
          "",
          "| Case | Actual RGBA8 | Three RGBA8 | Delta |",
          "| --- | --- | --- | --- |",
          ...differences.map((row: any) => "| " + row.id + " | " + row.actual.join(", ") + " | " + (row.expected?.join(", ") ?? "Unrecorded") + " | " + (row.delta?.join(", ") ?? "Unrecorded") + " |"),
          "",
        ].join("\n"),
      );
    console.log("Recorded " + result.rows.length + " actual " + command + " pixel samples");
    if (
      oracle.status === "recorded" &&
      differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (webgpu && index < 3 ? oracle.profile.maximumRgbError : 0)))
    )
      throw new Error("Browser pixel samples differ from the recorded Three fixture; see persisted report");
  } finally {
    await browser.close();
  }
}
