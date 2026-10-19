/** 🔭️ The reference visual renderers and the reference visual oracle run. */
/// <reference types="@webgpu/types" />
import { oraclePixelDeltaV1 } from "../📋️data/🟦️.ts";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { createHash } from "node:crypto";
import type { PixelOracleFixture } from "../🧾️pixel-oracle-fixture/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

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

async function renderReferenceVisualWgpu(input: any) {
  const fixture = input.fixture;
  const [width, height] = fixture.scene.viewport;
  const gpu = (navigator as any).gpu;
  if (!gpu) throw new Error("WebGPU is unavailable in the controlled browser context");
  const adapter = await gpu.requestAdapter();
  if (!adapter) throw new Error("WebGPU adapter unavailable");
  const device = await adapter.requestDevice();
  const format = "rgba8unorm";
  const output = device.createTexture({ size: [width, height], format, usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC, viewFormats: ["rgba8unorm-srgb"] });
  const outputView = output.createView({ format });
  const clearView = output.createView({ format: "rgba8unorm-srgb" });
  const depth = device.createTexture({ size: [width, height], format: "depth24plus-stencil8", usage: GPUTextureUsage.RENDER_ATTACHMENT });
  const uniform = device.createBuffer({ size: 256, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
  const plane = device.createBuffer({ size: 6 * 5 * 4, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST });
  const instance = device.createBuffer({ size: 96, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST });
  const line = device.createBuffer({ size: 8 * 7 * 4, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST });
  const readback = device.createBuffer({ size: 256 * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
  const texture = device.createTexture({ size: [1, 1], format: "rgba8unorm-srgb", usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
  device.queue.writeTexture({ texture }, new Uint8Array(fixture.scene.textureRgba8), { bytesPerRow: 256 }, [1, 1]);
  const sampler = device.createSampler({ magFilter: "linear", minFilter: "linear", addressModeU: "clamp-to-edge", addressModeV: "clamp-to-edge" });
  const globalsLayout = device.createBindGroupLayout({ entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: "uniform", minBindingSize: 240 } }] });
  const textureLayout = device.createBindGroupLayout({ entries: [{ binding: 0, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: "float" } }, { binding: 1, visibility: GPUShaderStage.FRAGMENT, sampler: { type: "filtering" } }] });
  const globals = device.createBindGroup({ layout: globalsLayout, entries: [{ binding: 0, resource: { buffer: uniform, size: 240 } }] });
  const textureGroup = device.createBindGroup({ layout: textureLayout, entries: [{ binding: 0, resource: texture.createView() }, { binding: 1, resource: sampler }] });
  const texturedModule = device.createShaderModule({ code: input.texturedShader });
  const lineModule = device.createShaderModule({ code: input.lineShader });
  for (const module of [texturedModule, lineModule]) {
    const errors = (await module.getCompilationInfo()).messages.filter((message: any) => message.type === "error");
    if (errors.length) throw new Error(JSON.stringify(errors));
  }
  const blend = { color: { srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha", operation: "add" }, alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha", operation: "add" } };
  const texturedPipeline = await device.createRenderPipelineAsync({
    layout: device.createPipelineLayout({ bindGroupLayouts: [globalsLayout, textureLayout] }),
    vertex: {
      module: texturedModule,
      entryPoint: "vs_main",
      buffers: [
        { arrayStride: 20, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x3" }, { shaderLocation: 1, offset: 12, format: "float32x2" }] },
        { arrayStride: 96, stepMode: "instance", attributes: [3, 4, 5, 6, 7, 8].map((shaderLocation, index) => ({ shaderLocation, offset: index * 16, format: "float32x4" })) },
      ],
    },
    fragment: { module: texturedModule, entryPoint: "fs_main", targets: [{ format, blend }] },
    primitive: { topology: "triangle-list", cullMode: "none" },
    depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: false, depthCompare: "less-equal" },
  });
  const linePipeline = await device.createRenderPipelineAsync({
    layout: device.createPipelineLayout({ bindGroupLayouts: [globalsLayout] }),
    vertex: { module: lineModule, entryPoint: "vs_main", buffers: [{ arrayStride: 28, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x3" }, { shaderLocation: 1, offset: 12, format: "float32x4" }] }] },
    fragment: { module: lineModule, entryPoint: "fs_main", targets: [{ format, blend }] },
    primitive: { topology: "line-list", cullMode: "none" },
    depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: false, depthCompare: "less-equal" },
  });
  const extent = fixture.scene.cameraExtent;
  const camera = new THREE.OrthographicCamera(-extent, extent, extent, -extent, 0.01, 100);
  camera.coordinateSystem = THREE.WebGPUCoordinateSystem;
  camera.position.set(0, 0, 4);
  camera.lookAt(0, 0, 0);
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld();
  const identity = new THREE.Matrix4().elements;
  const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse).elements;
  const globalsValues = new Float32Array([...viewProjection, ...identity, 0, 0, 4, 0, ...Array(20).fill(0), 0, 0, 0, 1]);
  if (globalsValues.length !== 60) throw new Error("Reference World3dGlobals stride changed: " + globalsValues.length);
  device.queue.writeBuffer(uniform, 0, globalsValues);
  const [planeWidth, planeHeight] = fixture.scene.planeSize;
  device.queue.writeBuffer(plane, 0, new Float32Array([
    -0.5, -0.5, 0, 0, 1, 0.5, -0.5, 0, 1, 1, 0.5, 0.5, 0, 1, 0,
    -0.5, -0.5, 0, 0, 1, 0.5, 0.5, 0, 1, 0, -0.5, 0.5, 0, 0, 0,
  ]));
  const clearLinear = new THREE.Color(fixture.scene.clearHex);
  const linear = (hex: string) => new THREE.Color(hex).toArray();
  const rows: any[] = [];
  try {
    for (const row of fixture.stateCases) {
      const theme = fixture.themes.find((value: any) => value.id === row.themeId);
      const sourceProfile = fixture.sourceProfiles.find((value: any) => value.id === row.sourceProfileId);
      const visible = !row.hidden || row.revealed;
      const selectable = !row.hidden && !row.locked;
      const asHover = (row.hidden && row.revealed) || (row.hovered && !row.locked);
      const selected = row.selected && !row.locked && !row.hidden;
      const baseOpacity = row.locked && !row.hidden ? row.baseOpacity * fixture.reference.lockedOpacityScale : row.baseOpacity;
      const contentOpacity = selected ? baseOpacity * fixture.reference.selectedContentOpacityScale : baseOpacity;
      const backgroundSemantic = selected ? "activeBase" : asHover ? "hoverBase" : null;
      const outlineSemantic = selected ? "activeBase" : asHover ? "accentSecondary" : null;
      const outlineOpacity = selected ? 1 : asHover ? 0.9 : 0;
      if (JSON.stringify({ visible, selectable, interactionId: selectable ? fixture.identityCases[0].id : null, contentOpacity, backgroundSemantic, outlineSemantic, outlineOpacity }) !== JSON.stringify(row.expected))
        throw new Error("Reference WGPU state derivation drifted for " + row.id);
      const encoder = device.createCommandEncoder();
      {
        const pass = encoder.beginRenderPass({
          colorAttachments: [{ view: clearView, loadOp: "clear", storeOp: "store", clearValue: { r: clearLinear.r, g: clearLinear.g, b: clearLinear.b, a: 1 } }],
          depthStencilAttachment: { view: depth.createView(), depthLoadOp: "clear", depthStoreOp: "store", depthClearValue: 1, stencilLoadOp: "clear", stencilStoreOp: "store", stencilClearValue: 0 },
        });
        pass.end();
      }
      if (visible) {
        const background = backgroundSemantic ? [...linear(theme[backgroundSemantic].hex), 1] : [0, 0, 0, 0];
        const sourceTransfer = sourceProfile.textureColorSpace === "SRGBColorSpace" ? 1 : 0;
        const model = new THREE.Matrix4().makeScale(planeWidth, planeHeight, 1).elements;
        device.queue.writeBuffer(instance, 0, new Float32Array([...model, ...background, contentOpacity, sourceTransfer, 0, 0]));
        const pass = encoder.beginRenderPass({
          colorAttachments: [{ view: outputView, loadOp: "load", storeOp: "store" }],
          depthStencilAttachment: { view: depth.createView(), depthLoadOp: "load", depthStoreOp: "store", stencilLoadOp: "load", stencilStoreOp: "store" },
        });
        pass.setPipeline(texturedPipeline);
        pass.setBindGroup(0, globals);
        pass.setBindGroup(1, textureGroup);
        pass.setVertexBuffer(0, plane);
        pass.setVertexBuffer(1, instance);
        pass.draw(6, 1);
        pass.end();
        if (outlineSemantic) {
          const halfWidth = planeWidth * 0.5 * 1.002;
          const halfHeight = planeHeight * 0.5 * 1.002;
          const color = [...linear(theme[outlineSemantic].hex), outlineOpacity];
          const points:readonly[readonly[number,number,number],readonly[number,number,number],readonly[number,number,number],readonly[number,number,number]] = [[-halfWidth, -halfHeight, 0], [halfWidth, -halfHeight, 0], [halfWidth, halfHeight, 0], [-halfWidth, halfHeight, 0]];
          const vertices: number[] = [];
          for (const [from, to] of [[0, 1], [1, 2], [2, 3], [3, 0]] as const) vertices.push(...points[from], ...color, ...points[to], ...color);
          device.queue.writeBuffer(line, 0, new Float32Array(vertices));
          const outlinePass = encoder.beginRenderPass({
            colorAttachments: [{ view: outputView, loadOp: "load", storeOp: "store" }],
            depthStencilAttachment: { view: depth.createView(), depthLoadOp: "load", depthStoreOp: "store", stencilLoadOp: "load", stencilStoreOp: "store" },
          });
          outlinePass.setPipeline(linePipeline);
          outlinePass.setBindGroup(0, globals);
          outlinePass.setVertexBuffer(0, line);
          outlinePass.draw(8, 1);
          outlinePass.end();
        }
      }
      encoder.copyTextureToBuffer({ texture: output }, { buffer: readback, bytesPerRow: 256, rowsPerImage: height }, [width, height]);
      device.queue.submit([encoder.finish()]);
      await readback.mapAsync(GPUMapMode.READ);
      const mapped = new Uint8Array(readback.getMappedRange());
      for (const sample of row.samples) {
        const offset = (height - 1 - sample.point[1]) * 256 + sample.point[0] * 4;
        rows.push({ id: sample.id, point: sample.point, rgba8: [...mapped.slice(offset, offset + 4)] });
      }
      readback.unmap();
    }
    return {
      status: "recorded-browser",
      producer: "Actual production WORLD3D_TEXTURED_SHADER/WORLD3D_LINES_SHADER through Chromium WebGPU",
      viewport: [width, height],
      format,
      outputTransfer: "compatible UNORM world view with manual Basic OETF; clear through sRGB view",
      gpu: { vendor: adapter.info.vendor, architecture: adapter.info.architecture, device: adapter.info.device, description: adapter.info.description },
      rows,
    };
  } finally {
    for (const resource of [output, depth, uniform, plane, instance, line, readback, texture]) resource.destroy();
    device.destroy();
  }
}

function renderReferenceVisual(fixture: any) {
  const [width, height] = fixture.scene.viewport;
  const renderer = new THREE.WebGLRenderer({
    alpha: true,
    antialias: fixture.scene.antialias,
    premultipliedAlpha: fixture.scene.premultipliedAlpha,
    preserveDrawingBuffer: true,
  });
  renderer.setPixelRatio(1);
  renderer.setSize(width, height);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1;
  renderer.autoClear = false;
  document.body.appendChild(renderer.domElement);
  const extent = fixture.scene.cameraExtent;
  const camera = new THREE.OrthographicCamera(-extent, extent, extent, -extent, 0.01, 100);
  camera.position.set(0, 0, 4);
  camera.lookAt(0, 0, 0);
  camera.updateProjectionMatrix();
  camera.updateMatrixWorld();
  const read = (point: readonly number[]) => {
    const rgba = new Uint8Array(4);
    const gl = renderer.getContext();
    gl.readPixels(point[0], point[1], 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, rgba);
    if (gl.getError() !== gl.NO_ERROR) throw new Error("Reference visual WebGL readback failed at " + point.join(","));
    return [...rgba];
  };
  const identityA = fixture.identityCases[0];
  const identityB = fixture.identityCases[1];
  if (identityA.id === identityB.id || identityA.url !== identityB.url || identityA.expectedInteractionId !== identityA.id || identityB.expectedInteractionId !== identityB.id)
    throw new Error("Authored reference identity fixture no longer distinguishes ids from a shared URL");
  const imageProfile = fixture.sourceProfiles.find((profile: any) => profile.id === "world-image");
  const canvasProfile = fixture.sourceProfiles.find((profile: any) => profile.id === "explicit-canvas-raster");
  if (!imageProfile?.reachableFromWorld3dHost || imageProfile.route !== "TextureLoader" || imageProfile.textureColorSpace !== "NoColorSpace")
    throw new Error("World image profile drifted from the reachable TextureLoader NoColorSpace contract");
  if (canvasProfile?.reachableFromWorld3dHost || canvasProfile.route !== "CanvasTexture" || canvasProfile.textureColorSpace !== "SRGBColorSpace")
    throw new Error("Explicit SVG/PDF canvas profile must remain distinct and currently unreachable from World3dHost");
  const naturalAspect = fixture.geometry.sourceNaturalSize[0] / fixture.geometry.sourceNaturalSize[1];
  const planeSize = [fixture.geometry.widthWorld, fixture.geometry.widthWorld / naturalAspect];
  const geometryPlane = new THREE.PlaneGeometry(...planeSize);
  geometryPlane.translate(...fixture.geometry.origin);
  geometryPlane.computeBoundingBox();
  const bounds = geometryPlane.boundingBox;
  const corners = [
    [bounds.min.x, bounds.min.y, bounds.min.z],
    [bounds.max.x, bounds.min.y, bounds.min.z],
    [bounds.max.x, bounds.max.y, bounds.max.z],
    [bounds.min.x, bounds.max.y, bounds.max.z],
  ];
  geometryPlane.dispose();
  const close = (left: number, right: number) => Math.abs(left - right) <= 1e-5;
  if (fixture.geometry.originMeaning !== "plane-center" || !planeSize.every((value: number, index: number) => close(value, fixture.geometry.expectedPlaneSize[index])) || corners.some((corner, index) => corner.some((value, axis) => !close(value, fixture.geometry.expectedCorners[index][axis]))))
    throw new Error("Reference natural-aspect centered-plane geometry drifted: " + JSON.stringify({ planeSize, corners }));
  const rows: any[] = [];
  const resources: any[] = [];
  try {
    for (const row of fixture.stateCases) {
      const theme = fixture.themes.find((value: any) => value.id === row.themeId);
      const sourceProfile = fixture.sourceProfiles.find((value: any) => value.id === row.sourceProfileId);
      if (!theme || !sourceProfile) throw new Error("Unknown reference fixture theme or source profile for " + row.id);
      const visible = !row.hidden || row.revealed;
      const selectable = !row.hidden && !row.locked;
      const asHover = (row.hidden && row.revealed) || (row.hovered && !row.locked);
      const showSelectedOutline = row.selected && !row.locked && !row.hidden;
      const baseOpacity = row.locked && !row.hidden ? row.baseOpacity * fixture.reference.lockedOpacityScale : row.baseOpacity;
      const contentOpacity = showSelectedOutline ? baseOpacity * fixture.reference.selectedContentOpacityScale : baseOpacity;
      const backgroundSemantic = showSelectedOutline ? "activeBase" : asHover ? "hoverBase" : null;
      const outlineSemantic = showSelectedOutline ? "activeBase" : asHover ? "accentSecondary" : null;
      const outlineOpacity = showSelectedOutline ? 1 : asHover ? 0.9 : 0;
      const interactionId = selectable ? identityA.id : null;
      const actualState = { visible, selectable, interactionId, contentOpacity, backgroundSemantic, outlineSemantic, outlineOpacity };
      if (JSON.stringify(actualState) !== JSON.stringify(row.expected)) throw new Error("Reference state contract drifted for " + row.id + ": " + JSON.stringify(actualState));
      renderer.setClearColor(new THREE.Color(fixture.scene.clearHex), 1);
      renderer.clear();
      if (visible) {
        const scene = new THREE.Scene();
        const plane = new THREE.PlaneGeometry(...fixture.scene.planeSize);
        resources.push(plane);
        if (backgroundSemantic) {
          const backgroundMaterial = new THREE.MeshBasicMaterial({
            color: new THREE.Color(theme[backgroundSemantic].hex),
            transparent: true,
            opacity: 1,
            depthWrite: false,
            side: THREE.DoubleSide,
            toneMapped: false,
          });
          const background = new THREE.Mesh(plane, backgroundMaterial);
          background.renderOrder = fixture.reference.renderOrder.background;
          scene.add(background);
          resources.push(backgroundMaterial);
        }
        const texture = new THREE.DataTexture(new Uint8Array(fixture.scene.textureRgba8), 1, 1, THREE.RGBAFormat, THREE.UnsignedByteType);
        texture.colorSpace = sourceProfile.textureColorSpace === "SRGBColorSpace" ? THREE.SRGBColorSpace : THREE.NoColorSpace;
        texture.minFilter = THREE.NearestFilter;
        texture.magFilter = THREE.NearestFilter;
        texture.generateMipmaps = false;
        texture.needsUpdate = true;
        const contentMaterial = new THREE.MeshBasicMaterial({
          map: texture,
          transparent: true,
          opacity: contentOpacity,
          depthWrite: false,
          side: THREE.DoubleSide,
          toneMapped: false,
        });
        if (contentMaterial.depthWrite !== fixture.reference.depthWrite || contentMaterial.side !== THREE.DoubleSide || contentMaterial.toneMapped !== fixture.reference.toneMapped)
          throw new Error("Reference MeshBasicMaterial policy drifted for " + row.id);
        const content = new THREE.Mesh(plane, contentMaterial);
        content.renderOrder = fixture.reference.renderOrder.content;
        scene.add(content);
        resources.push(texture, contentMaterial);
        if (outlineSemantic) {
          const edges = new THREE.EdgesGeometry(plane);
          const outlineMaterial = new THREE.LineBasicMaterial({ color: new THREE.Color(theme[outlineSemantic].hex), transparent: true, opacity: outlineOpacity, depthWrite: false });
          const outline = new THREE.LineSegments(edges, outlineMaterial);
          outline.renderOrder = fixture.reference.renderOrder.outline;
          outline.scale.setScalar(1.002);
          scene.add(outline);
          resources.push(edges, outlineMaterial);
        }
        renderer.render(scene, camera);
      }
      for (const sample of row.samples) rows.push({ id: sample.id, point: sample.point, rgba8: read(sample.point) });
    }
    const gl = renderer.getContext();
    const debug = gl.getExtension("WEBGL_debug_renderer_info");
    return {
      status: "recorded-browser",
      producer: "installed Three " + THREE.REVISION + " reference visual WebGLRenderer in Chromium",
      threeRevision: THREE.REVISION,
      viewport: [width, height],
      outputColorSpace: renderer.outputColorSpace,
      toneMapping: "ACESFilmic; reference MeshBasic materials toneMapped=false",
      antialias: renderer.getContext().getContextAttributes().antialias,
      premultipliedAlpha: fixture.scene.premultipliedAlpha,
      gpu: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
      sourceProfiles: fixture.sourceProfiles,
      rows,
    };
  } finally {
    for (const resource of resources) resource.dispose();
    renderer.dispose();
  }
}

export async function runReferenceVisualOracle(repoRoot: string, outputDirectory?: string, webgpu = false): Promise<void> {
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json");
  const output = outputDirectory ? resolve(outputDirectory) : undefined;
  if (output) await mkdir(output, { recursive: true });
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8")) as PixelOracleFixture;
  const threePath = Bun.resolveSync("three", repoRoot);
  const program = webgpu ? renderReferenceVisualWgpu : renderReferenceVisual;
  const source = 'import * as THREE from "three";\nglobalThis.renderReferenceVisual = ' + program.toString();
  const build = await Bun.build({
    entrypoints: ["semio-reference-visual-oracle"],
    target: "browser",
    format: "iife",
    plugins: [
      {
        name: "ticket-reference-visual-oracle",
        setup(builder) {
          builder.onResolve({ filter: /^semio-reference-visual-oracle$/ }, () => ({ path: "semio-reference-visual-oracle", namespace: "oracle" }));
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
  const playwrightSpecifier = "playwright";
  const { chromium } = (await import(playwrightSpecifier)) as typeof import("playwright");
  const browser = await chromium.launch({ headless: true, args: ["--ignore-gpu-blocklist", ...(webgpu ? ["--enable-unsafe-webgpu"] : []), ...(process.platform === "darwin" ? ["--use-angle=metal"] : [])] });
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
      const shaderSource = readModuleTreeSource(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs"));
      const shader = (name: string) => shaderSource.match(new RegExp(`pub const ${name}: &str = r#"([\\s\\S]*?)"#;`))?.[1];
      const texturedShader = shader("WORLD3D_TEXTURED_SHADER");
      const lineShader = shader("WORLD3D_LINES_SHADER");
      if (!texturedShader || !lineShader) throw new Error("Production reference shader constants not found");
      shaderSha256 = createHash("sha256").update(texturedShader + lineShader).digest("hex");
      input = { fixture, texturedShader, lineShader };
    }
    const result = await page.evaluate((input) => (globalThis as any).renderReferenceVisual(input), input);
    result.shaderSha256 = shaderSha256;
    if (errors.length) throw new Error(errors.join("\n"));
    const expectedRows = fixture.pixelOracle.rows;
    if (fixture.pixelOracle.status === "recorded" && JSON.stringify(result.rows.map((row: any) => row.id)) !== JSON.stringify(expectedRows.map((row: any) => row.id)))
      throw new Error("Reference visual pixel identities differ from the recorded fixture");
    const differences = result.rows.map((row: any) => {
      const expected = expectedRows.find((value: any) => value.id === row.id)?.rgba8;
      return { id: row.id, actual: row.rgba8, expected, delta: expected ? oraclePixelDeltaV1(row.rgba8,expected) : null };
    });
    result.differences = differences;
    if (output) {
      await writeFile(join(output, "pixels.json"), JSON.stringify(result, null, 2));
      await writeFile(
        join(output, "report.md"),
        [
          "# " + (webgpu ? "Production WGSL WebGPU" : "Three WebGL") + " Reference Visual Pixels",
          "",
          (webgpu ? "Actual production-WGSL WebGPU" : "Actual installed-Three") + " pixels for the schema-first reference identity, appearance, and Basic texture-transfer fixture.",
          "",
          "GPU: " + JSON.stringify(result.gpu),
          "",
          "| Case | Actual RGBA8 | Recorded RGBA8 | Delta |",
          "| --- | --- | --- | --- |",
          ...differences.map((row: any) => "| " + row.id + " | " + row.actual.join(", ") + " | " + (row.expected?.join(", ") ?? "Unrecorded") + " | " + (row.delta?.join(", ") ?? "Unrecorded") + " |"),
          "",
        ].join("\n"),
      );
    }
    console.log("Recorded " + result.rows.length + " actual " + (webgpu ? "WebGPU" : "Three") + " reference visual pixel samples");
    if (
      fixture.pixelOracle.status === "recorded" &&
      differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (index < 3 ? fixture.pixelOracle.maximumRgbError : 0)))
    )
      throw new Error("Reference visual browser pixels differ from the recorded Three fixture; see persisted report");
  } finally {
    await browser.close();
  }
}
