/** 🕸️ The Drei and WebGPU grid renderers and the grid visual oracle run. */
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

async function renderGridDrei(fixture: any) {
  const rows: any[] = [];
  for (const testCase of fixture.cases) {
    const theme = fixture.themes.find((value: any) => value.id === testCase.themeId);
    if (!theme) throw new Error("Unknown grid theme " + testCase.themeId);
    const [width, height] = testCase.viewport;
    const host = document.createElement("div");
    host.style.width = width + "px";
    host.style.height = height + "px";
    document.body.appendChild(host);
    let resolveState: (state: any) => void = () => undefined;
    const stateReady = new Promise<any>((resolve) => (resolveState = resolve));
    const root = createRoot(host);
    const reference = fixture.reference;
    root.render(
      React.createElement(
        Canvas,
        {
          orthographic: testCase.projection === "orthographic",
          dpr: testCase.dpr,
          frameloop: "always",
          camera: {
            position: testCase.camera.position,
            up: testCase.camera.up,
            near: testCase.camera.near,
            far: testCase.camera.far,
            fov: testCase.camera.fov,
            zoom: testCase.camera.zoom,
          },
          gl: { antialias: false, alpha: true, premultipliedAlpha: false, preserveDrawingBuffer: true },
          onCreated: (state: any) => {
            state.camera.position.fromArray(testCase.camera.position);
            state.camera.up.fromArray(testCase.camera.up);
            state.camera.lookAt(new THREE.Vector3().fromArray(testCase.camera.target));
            state.camera.updateProjectionMatrix();
            state.camera.updateMatrixWorld();
            state.gl.setClearColor(0x000000, 0);
            state.gl.outputColorSpace = THREE.SRGBColorSpace;
            state.gl.toneMapping = THREE.ACESFilmicToneMapping;
            state.gl.toneMappingExposure = 1;
            resolveState(state);
          },
        },
        React.createElement(Grid, {
          args: reference.args,
          position: [0, 0, reference.planeZ],
          rotation: [Math.PI / 2, 0, 0],
          cellSize: reference.cellSize,
          cellThickness: reference.cellThickness,
          cellColor: theme.elementHex,
          sectionSize: reference.cellSize,
          sectionThickness: reference.sectionThickness,
          sectionColor: theme.elementHex,
          fadeDistance: reference.fadeDistance,
          fadeStrength: reference.fadeStrength,
          fadeFrom: reference.fadeFrom,
          followCamera: reference.followCamera,
          infiniteGrid: reference.infiniteGrid,
          side: THREE.DoubleSide,
          renderOrder: reference.renderOrder,
          onUpdate: (mesh: any) => {
            mesh.material.depthTest = reference.depthTest;
            mesh.material.depthWrite = reference.depthWrite;
          },
        }),
      ),
    );
    const state = await stateReady;
    for (let frame = 0; frame < 8; frame += 1) await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()));
    const gl = state.gl.getContext();
    for (const sample of testCase.samples) {
      const rgba = new Uint8Array(4);
      gl.readPixels(sample.physicalPoint[0], sample.physicalPoint[1], 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, rgba);
      if (gl.getError() !== gl.NO_ERROR) throw new Error("Drei grid readback failed for " + testCase.id + "/" + sample.id);
      rows.push({ id: testCase.id + "/" + sample.id, point: sample.physicalPoint, rgba8: [...rgba] });
    }
    root.unmount();
    host.remove();
  }
  return { status: "recorded-browser", producer: "installed @react-three/drei Grid", rows };
}

async function renderGridWgpu(input: any) {
  const { fixture, shader } = input;
  const adapter = await navigator.gpu?.requestAdapter();
  if (!adapter) throw new Error("WebGPU adapter unavailable for grid oracle");
  const device = await adapter.requestDevice();
  const module = device.createShaderModule({ code: shader });
  const compilation = await module.getCompilationInfo();
  const errors = compilation.messages.filter((message: any) => message.type === "error");
  if (errors.length) throw new Error(errors.map((message: any) => message.message).join("\n"));
  const globalsLayout = device.createBindGroupLayout({ entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } }] });
  const gridLayout = device.createBindGroupLayout({ entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, buffer: { type: "uniform" } }] });
  const pipeline = await device.createRenderPipelineAsync({
    layout: device.createPipelineLayout({ bindGroupLayouts: [globalsLayout, gridLayout] }),
    vertex: { module, entryPoint: "vs_main", buffers: [{ arrayStride: 20, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x3" }] }] },
    fragment: {
      module,
      entryPoint: "fs_main",
      targets: [{ format: "rgba8unorm", blend: { color: { operation: "add", srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha" }, alpha: { operation: "add", srcFactor: "one", dstFactor: "one-minus-src-alpha" } } }],
    },
    primitive: { topology: "triangle-list", cullMode: "none" },
    depthStencil: { format: "depth24plus", depthWriteEnabled: false, depthCompare: "less-equal" },
  });
  const plane = new Float32Array([-0.5, -0.5, 0, 0, 1, 0.5, -0.5, 0, 1, 1, 0.5, 0.5, 0, 1, 0, -0.5, -0.5, 0, 0, 1, 0.5, 0.5, 0, 1, 0, -0.5, 0.5, 0, 0, 0]);
  const vertexBuffer = device.createBuffer({ size: plane.byteLength, usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST });
  device.queue.writeBuffer(vertexBuffer, 0, plane);
  const rows: any[] = [];
  for (const testCase of fixture.cases) {
    const theme = fixture.themes.find((value: any) => value.id === testCase.themeId);
    if (!theme) throw new Error("Unknown grid theme " + testCase.themeId);
    const camera = testCase.projection === "orthographic"
      ? new THREE.OrthographicCamera(-testCase.viewport[0] / (2 * testCase.camera.zoom), testCase.viewport[0] / (2 * testCase.camera.zoom), testCase.viewport[1] / (2 * testCase.camera.zoom), -testCase.viewport[1] / (2 * testCase.camera.zoom), testCase.camera.near, testCase.camera.far)
      : new THREE.PerspectiveCamera(testCase.camera.fov, testCase.viewport[0] / testCase.viewport[1], testCase.camera.near, testCase.camera.far);
    camera.coordinateSystem = THREE.WebGPUCoordinateSystem;
    camera.position.fromArray(testCase.camera.position);
    camera.up.fromArray(testCase.camera.up);
    camera.lookAt(new THREE.Vector3().fromArray(testCase.camera.target));
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse).toArray();
    const globals = new Float32Array(64);
    globals.set(viewProjection, 0);
    globals.set(testCase.camera.position, 32);
    globals[59] = 1;
    const grid = new Float32Array(64);
    grid.set([fixture.reference.planeZ, fixture.reference.cellSize, fixture.reference.cellThickness, fixture.reference.fadeDistance], 0);
    grid.set([testCase.camera.position[0], testCase.camera.position[1], fixture.reference.planeZ, fixture.reference.fadeStrength], 4);
    grid.set(theme.elementLinear, 8);
    const globalsBuffer = device.createBuffer({ size: globals.byteLength, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
    const gridBuffer = device.createBuffer({ size: grid.byteLength, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
    device.queue.writeBuffer(globalsBuffer, 0, globals);
    device.queue.writeBuffer(gridBuffer, 0, grid);
    const globalsBind = device.createBindGroup({ layout: globalsLayout, entries: [{ binding: 0, resource: { buffer: globalsBuffer } }] });
    const gridBind = device.createBindGroup({ layout: gridLayout, entries: [{ binding: 0, resource: { buffer: gridBuffer } }] });
    const width = testCase.viewport[0] * testCase.dpr;
    const height = testCase.viewport[1] * testCase.dpr;
    const color = device.createTexture({ size: [width, height], format: "rgba8unorm", usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
    const depth = device.createTexture({ size: [width, height], format: "depth24plus", usage: GPUTextureUsage.RENDER_ATTACHMENT });
    const bytesPerRow = Math.ceil((width * 4) / 256) * 256;
    const readback = device.createBuffer({ size: bytesPerRow * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const encoder = device.createCommandEncoder();
    const pass = encoder.beginRenderPass({
      colorAttachments: [{ view: color.createView(), clearValue: [0, 0, 0, 0], loadOp: "clear", storeOp: "store" }],
      depthStencilAttachment: { view: depth.createView(), depthClearValue: 1, depthLoadOp: "clear", depthStoreOp: "discard" },
    });
    pass.setPipeline(pipeline);
    pass.setViewport(0, 0, width, height, 0, 1);
    pass.setScissorRect(0, 0, width, height);
    pass.setBindGroup(0, globalsBind);
    pass.setBindGroup(1, gridBind);
    pass.setVertexBuffer(0, vertexBuffer);
    pass.draw(6);
    pass.end();
    encoder.copyTextureToBuffer({ texture: color }, { buffer: readback, bytesPerRow, rowsPerImage: height }, [width, height]);
    device.queue.submit([encoder.finish()]);
    await readback.mapAsync(GPUMapMode.READ);
    const bytes = new Uint8Array(readback.getMappedRange());
    for (const sample of testCase.samples) {
      const offset = (height - 1 - sample.physicalPoint[1]) * bytesPerRow + sample.physicalPoint[0] * 4;
      rows.push({ id: testCase.id + "/" + sample.id, point: sample.physicalPoint, rgba8: [...bytes.slice(offset, offset + 4)] });
    }
    readback.unmap();
    readback.destroy();
    globalsBuffer.destroy();
    gridBuffer.destroy();
    color.destroy();
    depth.destroy();
  }
  vertexBuffer.destroy();
  device.destroy();
  return { status: "recorded-browser", producer: "production WORLD3D_GRID_SHADER", rows };
}

export async function runGridVisualOracle(repoRoot: string, outputDirectory: string | undefined, webgpu: boolean): Promise<void> {
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/🌐️grid-visual/🔣️.json");
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8")) as PixelOracleFixture;
  const output = outputDirectory ? resolve(outputDirectory) : undefined;
  if (output) await mkdir(output, { recursive: true });
  const threePath = Bun.resolveSync("three", repoRoot);
  const imports = webgpu
    ? 'import * as THREE from "three";\nglobalThis.renderGridVisual = ' + renderGridWgpu.toString()
    : 'import React from "react";\nimport { createRoot } from "react-dom/client";\nimport { Canvas } from "@react-three/fiber";\nimport { Grid } from "drei-grid";\nimport * as THREE from "three";\nglobalThis.renderGridVisual = ' + renderGridDrei.toString();
  const build = await Bun.build({
    entrypoints: ["semio-grid-visual-oracle"], target: "browser", format: "iife",
    plugins: [{ name: "ticket-grid-visual-oracle", setup(builder) {
      builder.onResolve({ filter: /^semio-grid-visual-oracle$/ }, () => ({ path: "semio-grid-visual-oracle", namespace: "oracle" }));
      builder.onResolve({ filter: /^three$/ }, () => ({ path: threePath }));
      builder.onResolve({ filter: /^drei-grid$/ }, () => ({ path: Bun.resolveSync("@react-three/drei/core/Grid.js", repoRoot) }));
      builder.onLoad({ filter: /.*/, namespace: "oracle" }, () => ({ contents: imports, loader: "js", resolveDir: repoRoot }));
    } }],
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
    const page = await browser.newPage({ viewport: { width: 256, height: 256 }, deviceScaleFactor: 1 });
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    await page.route("https://semio-parity.invalid/**", (route) => route.fulfill({ contentType: "text/html", body: "<!doctype html><html><body style='margin:0'></body></html>" }));
    await page.goto("https://semio-parity.invalid/");
    await page.addScriptTag({ content: javascript });
    let input: any = fixture;
    let shaderSha256: string | undefined;
    if (webgpu) {
      const source = readModuleTreeSource(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🎨️shaders/🦀️.rs"));
      const shader = source.match(/pub const WORLD3D_GRID_SHADER: &str = r#"([\s\S]*?)"#;/)?.[1];
      if (!shader) throw new Error("Production WORLD3D_GRID_SHADER constant not found");
      shaderSha256 = createHash("sha256").update(shader).digest("hex");
      input = { fixture, shader };
    }
    const result = await page.evaluate((value) => (globalThis as any).renderGridVisual(value), input);
    result.shaderSha256 = shaderSha256;
    if (errors.length) throw new Error(errors.join("\n"));
    const expectedRows = fixture.pixelOracle.rows;
    if (fixture.pixelOracle.status === "recorded" && JSON.stringify(result.rows.map((row: any) => row.id)) !== JSON.stringify(expectedRows.map((row: any) => row.id))) throw new Error("Grid pixel case identities differ from the recorded fixture");
    const differences = result.rows.map((row: any) => {
      const expected = expectedRows.find((value: any) => value.id === row.id)?.rgba8;
      return { id: row.id, point: row.point, actual: row.rgba8, expected, delta: expected ? oraclePixelDeltaV1(row.rgba8,expected) : null };
    });
    result.differences = differences;
    if (output) {
      await writeFile(join(output, "pixels.json"), JSON.stringify(result, null, 2));
      await writeFile(join(output, "report.md"), ["# " + (webgpu ? "Production WGSL" : "Installed Drei") + " Grid Pixels", "", "| Case | Actual RGBA8 | Drei RGBA8 | Delta |", "| --- | --- | --- | --- |", ...differences.map((row: any) => "| " + row.id + " | " + row.actual.join(", ") + " | " + (row.expected?.join(", ") ?? "Unrecorded") + " | " + (row.delta?.join(", ") ?? "Unrecorded") + " |"), ""].join("\n"));
    }
    console.log("Recorded " + result.rows.length + " actual " + (webgpu ? "production WGSL" : "installed Drei") + " grid pixel samples");
    if (fixture.pixelOracle.status === "recorded" && differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (index < 3 ? fixture.pixelOracle.maximumRgbError : 0)))) throw new Error("Grid pixels differ from the recorded installed-Drei fixture; see persisted report");
  } finally {
    await browser.close();
  }
}
