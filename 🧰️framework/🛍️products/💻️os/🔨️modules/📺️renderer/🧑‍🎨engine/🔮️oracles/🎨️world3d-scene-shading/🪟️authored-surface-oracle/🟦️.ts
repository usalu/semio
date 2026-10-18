/** 🪟️ The authored surface reference renderer and the authored surface oracle run. */
import { oraclePixelDeltaV1, oracleSamplerV1, oracleSurfaceCasesV1, oracleBrowserDataPreludeV1 } from "../📋️data/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { createHash } from "node:crypto";
import { readModuleTreeSource } from "../../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { renderWgpuTextured } from "../🧊️wgpu-shading-renderers/🟦️.ts";

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

function renderAuthoredSurfaceReference(input: any) {
  const { fixture } = input;
  const [width, height] = fixture.scene.viewport;
  const renderer = new THREE.WebGLRenderer({ antialias: false, alpha: true, premultipliedAlpha: false, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setSize(width, height);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1;
  renderer.setClearColor(0, 0);
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.position.fromArray(fixture.scene.camera.position);
  camera.up.fromArray(fixture.scene.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
  const rows = [];
  try {
    for (const row of fixture.textureCases) {
      const scene = new THREE.Scene();
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(fixture.scene.geometry.positions, 3));
      geometry.setAttribute("normal", new THREE.Float32BufferAttribute(fixture.scene.geometry.normals, 3));
      geometry.setAttribute("color", new THREE.Float32BufferAttribute(Array.from({ length: 3 }, () => row.vertexColor).flat(), 4));
      geometry.setAttribute("uv", new THREE.Float32BufferAttribute([0, 0, 1, 0, 0.5, 1], 2));
      geometry.setAttribute("uv1", geometry.getAttribute("uv").clone());
      geometry.setIndex(row.reverse ? [0, 2, 1] : fixture.scene.geometry.indices);
      if(row.tangents)geometry.setAttribute("tangent",new THREE.Float32BufferAttribute(row.tangents,4));
      for(const [set,values] of Object.entries(row.uvSets ?? {}))geometry.setAttribute(Number(set) ? `uv${set}` : "uv",new THREE.Float32BufferAttribute(values,2));
      const material = worldSurfaceMaterial(row.id, row, true);
      const maps: any[] = [];
      for (const [role, bytes] of Object.entries(row.maps)) {
        const texture = new THREE.DataTexture(new Uint8Array(bytes as number[]), ...(row.mapSizes?.[role] ?? [1,1]), THREE.RGBAFormat);
        texture.colorSpace = role === "baseColor" || role === "emissive" ? THREE.SRGBColorSpace : THREE.NoColorSpace;
        texture.flipY = false;
        const sampler=oracleSamplerV1(role,row.textureSamplers);texture.channel=row.textureCoordinates?.[sampler.field] ?? 0;
        const wraps={33071:THREE.ClampToEdgeWrapping,33648:THREE.MirroredRepeatWrapping,10497:THREE.RepeatWrapping};const filters={9728:THREE.NearestFilter,9729:THREE.LinearFilter,9984:THREE.NearestMipmapNearestFilter,9985:THREE.LinearMipmapNearestFilter,9986:THREE.NearestMipmapLinearFilter,9987:THREE.LinearMipmapLinearFilter};texture.wrapS=wraps[sampler.wrapS];texture.wrapT=wraps[sampler.wrapT];texture.magFilter=filters[sampler.magFilter];texture.minFilter=filters[sampler.minFilter];texture.generateMipmaps=sampler.minFilter>=9984;
        texture.needsUpdate = true;
        maps.push(texture);
        if (role === "baseColor") material.map = texture;
        if (role === "metallicRoughness") material.metalnessMap = material.roughnessMap = texture;
        if (role === "normal") material.normalMap = texture;
        if (role === "occlusion") material.aoMap = texture;
        if (role === "emissive") material.emissiveMap = texture;
      }
      material.needsUpdate = true;
      scene.add(new THREE.Mesh(geometry, material));
      const fallback = fixture.scene.fallbackLights;
      scene.add(new THREE.AmbientLight(new THREE.Color().setRGB(...fallback.ambient.color), fallback.ambient.intensity));
      const hemisphere = new THREE.HemisphereLight(new THREE.Color().setRGB(...fallback.hemisphere.sky), new THREE.Color().setRGB(...fallback.hemisphere.ground), fallback.hemisphere.intensity);
      hemisphere.position.fromArray(fallback.hemisphere.position);
      scene.add(hemisphere);
      for (const specification of fallback.directional) {
        const light = new THREE.DirectionalLight(new THREE.Color().setRGB(...specification.color), specification.intensity);
        light.position.fromArray(specification.position);
        scene.add(light);
      }
      renderer.render(scene, camera);
      const pixel = new Uint8Array(4);
      const context = renderer.getContext();
      const point = fixture.s2PixelOracle.profile.samplePosition;
      context.readPixels(point[0], point[1], 1, 1, context.RGBA, context.UNSIGNED_BYTE, pixel);
      rows.push({ id: row.id, rgba8: [...pixel] });
      material.dispose();
      maps.forEach((texture) => texture.dispose());
      geometry.dispose();
    }
    return { producer: "Installed Three MeshStandardMaterial with production worldSurfaceMaterial", rows };
  } finally { renderer.dispose(); }
}

export async function runAuthoredSurfaceOracle(repoRoot: string, output: string): Promise<void> {
  await mkdir(output, { recursive: true });
  const engine = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
  const surface = JSON.parse(await readFile(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🧫️fixtures/🎨️world3d-inline-surface/🔣️.json"), "utf8"));
  const fixture = JSON.parse(await readFile(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json"), "utf8"));
  fixture.textureCases = oracleSurfaceCasesV1(surface).map(row => ({ ...row, preserveVertexColor: true, emissiveIntensity: 0 }));
  const shaderDirectory = join(output, "native-shader");
  await mkdir(shaderDirectory, { recursive: true });
  await runOwnedCommand("bun", ["repo", "run", "@semio-tech/ui-rs:test", "--", "wgpu-engine", "long", "--offline", "a_native_gpu_context_creates_every_pipeline_without_a_validation_error"], repoRoot, "authored-surface-native-shader", 1200000, { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: shaderDirectory, NEXTEST_SUCCESS_OUTPUT: "immediate" } });
  const shader = await readFile(join(shaderDirectory, "world3d-authored.wgsl"), "utf8");
  const mipShader=await readFile(join(shaderDirectory,"raster-mip.wgsl"),"utf8");
  const typescript = await import("typescript");
  const materialSource = readModuleTreeSource(join(engine, "🧱️elements/🌐️World3dHost/🟦️.tsx"));
  const syntax = typescript.createSourceFile("World3dHost.tsx", materialSource, typescript.ScriptTarget.Latest, true, typescript.ScriptKind.TSX);
  const declarations = syntax.statements.filter((statement) => typescript.isFunctionDeclaration(statement) && ["surfaceTuple", "worldSurfaceMaterial"].includes(statement.name?.text ?? ""));
  if (declarations.length !== 2) throw new Error("Production material constructor declarations were not found");
  const source = oracleBrowserDataPreludeV1()+'import * as THREE from "three";\nconst {MeshStandardMaterial, DoubleSide, FrontSide} = THREE;\n' + declarations.map((declaration) => declaration.getText(syntax)).join("\n") + '\nglobalThis.renderReference = ' + renderAuthoredSurfaceReference.toString() + ';\nglobalThis.renderNative = ' + renderWgpuTextured.toString();
  const build = await Bun.build({ entrypoints: ["semio-authored-surface"], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, plugins: [{ name: "authored-surface", setup(builder) {
    builder.onResolve({ filter: /^semio-authored-surface$/ }, () => ({ path: "semio-authored-surface", namespace: "oracle" }));
    builder.onResolve({ filter: /^three$/ }, () => ({ path: Bun.resolveSync("three", repoRoot) }));
    builder.onLoad({ filter: /.*/, namespace: "oracle" }, () => ({ contents: source, loader: "tsx" }));
  } }] });
  if (!build.success) throw new Error(build.logs.map(String).join("\n"));
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true, args: ["--ignore-gpu-blocklist", ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
  try {
    const page = await browser.newPage({ viewport: { width: 64, height: 64 }, deviceScaleFactor: 1 });
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    page.on("console", (message) => {});
    await page.route("https://semio-parity.invalid/**", (route) => route.fulfill({ contentType: "text/html", body: "<!doctype html><html><body></body></html>" }));
    await page.goto("https://semio-parity.invalid/");
    const bundle=build.outputs[0];
    if(!bundle)throw Error("Successful oracle bundle output required");
    await page.addScriptTag({ type: "module", content: await bundle.text() });
    await page.waitForFunction(() => typeof (globalThis as any).renderNative === "function");
    const input = { fixture, shader,mipShader, authored: true };
    const reference = await page.evaluate((input) => (globalThis as any).renderReference(input), input);
    const native = await page.evaluate((input) => (globalThis as any).renderNative(input), input);
    if (errors.length) throw new Error(errors.join("\n"));
    const differences = native.rows.map((row: any) => { const expected = reference.rows.find((value: any) => value.id === row.id)?.rgba8; return { id: row.id, actual: row.rgba8, expected, delta: oraclePixelDeltaV1(row.rgba8,expected) }; });
    await writeFile(join(output, "pixels.json"), JSON.stringify({ reference, native, differences, shaderSha256: createHash("sha256").update(shader).digest("hex") }, null, 2));
    await writeFile(join(output, "report.md"), "# Authored Surface Pixels\n\nActual production native WGSL and production React material constructor against installed Three.\n\n| Case | Native RGBA8 | Three RGBA8 | Delta |\n| --- | --- | --- | --- |\n" + differences.map((row: any) => `| ${row.id} | ${row.actual} | ${row.expected} | ${row.delta} |`).join("\n") + "\n");
    if (differences.some((row: any) => row.delta.some((value: number, index: number) => Math.abs(value) > (index < 3 ? 2 : 1)))) throw new Error("Authored surface pixels differ from installed Three; see retained artifacts");
  } finally { await browser.close(); }
}
