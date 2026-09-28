/** @emoji 🎨️ Actual Three SVGRenderer oracle for IconRender's SVG face-lighting profile. */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020";
import { chromium } from "playwright";
import { AmbientLight, BufferGeometry, DirectionalLight, Float32BufferAttribute, Mesh, MeshStandardMaterial, PerspectiveCamera, Scene, Vector3 } from "three";
import { SVGRenderer } from "three/examples/jsm/renderers/SVGRenderer.js";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎨️icon-svg-lighting/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/🎨️icon-svg-lighting/🔣️.json"), "utf8"));

function renderFill(record: any): string {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute([-1, -1, 0, 1, -1, 0, 0, 1, 0], 3));
  geometry.setAttribute("normal", new Float32BufferAttribute([...record.normal, ...record.normal, ...record.normal], 3));
  const material = new MeshStandardMaterial(record.material);
  const scene = new Scene();
  scene.add(new Mesh(geometry, material));
  scene.add(new AmbientLight(record.ambient.color, record.ambient.intensity));
  const sun = new DirectionalLight(record.sun.color, record.sun.intensity);
  sun.position.copy(new Vector3(...record.lightDirection));
  scene.add(sun);
  const camera = new PerspectiveCamera(50, 1, 0.1, 100);
  camera.position.set(0, 0, 5);
  camera.lookAt(0, 0, 0);
  const renderer = new SVGRenderer();
  renderer.setSize(64, 64);
  renderer.render(scene, camera);
  const path = [...renderer.domElement.querySelectorAll("path")].find((candidate) => candidate.getAttribute("style")?.includes("fill:rgb("));
  const fill = path?.getAttribute("style")?.match(/fill:(rgb\([^;]+\))/)?.[1];
  geometry.dispose();
  material.dispose();
  expect(fill, record.id).toBeDefined();
  return fill!;
}

describe("🎨️ Icon SVG face lighting", () => {
  it("validates the neutral SVG profile", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.profile).toBe("svgFlatLit");
  });

  it("matches actual SVGRenderer colors and its ignored PBR/intensity parameters", () => {
    for (const record of fixture.cases) expect(renderFill(record), record.id).toBe(record.expectedFill);
    expect(fixture.cases[0].expectedFill).toBe(fixture.cases[1].expectedFill);
  });

  it("renders the actual production SVG profile through Chromium WebGPU", async () => {
    const shaderPath = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs");
    const shader = readFileSync(shaderPath, "utf8").match(/pub const WORLD3D_SHADER: &str = r#"([\s\S]*?)"#;/)![1];
    const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader"] });
    try {
      const page = await browser.newPage();
      await page.route("https://renderer.test/**", (route) => route.fulfill({ contentType: "text/html", body: "<!doctype html><title>Icon SVG lighting fixture</title>" }));
      await page.goto("https://renderer.test/");
      const observed = await page.evaluate(async ({ shader, cases }) => {
        const adapter = await navigator.gpu.requestAdapter();
        if (!adapter) throw new Error("WebGPU adapter is required for the Icon SVG lighting oracle");
        const device = await adapter.requestDevice();
        device.pushErrorScope("validation");
        const module = device.createShaderModule({ code: shader });
        const failures = (await module.getCompilationInfo()).messages.filter((message) => message.type === "error");
        if (failures.length) throw new Error(failures.map((message) => message.message).join("\n"));
        const pipeline = await device.createRenderPipelineAsync({
          layout: "auto",
          vertex: {
            module,
            entryPoint: "vs_main",
            buffers: [
              { arrayStride: 48, attributes: [
                { shaderLocation: 0, offset: 0, format: "float32x3" }, { shaderLocation: 1, offset: 12, format: "float32x3" },
                { shaderLocation: 2, offset: 24, format: "float32x4" }, { shaderLocation: 9, offset: 40, format: "float32x2" },
              ] },
              { arrayStride: 112, stepMode: "instance", attributes: [...Array.from({ length: 6 }, (_, index) => ({ shaderLocation: index + 3, offset: index * 16, format: "float32x4" as const })), { shaderLocation: 10, offset: 96, format: "float32x4" as const }] },
            ],
          },
          fragment: { module, entryPoint: "fs_main", targets: [{ format: "rgba8unorm" }] },
          primitive: { topology: "triangle-list", cullMode: "none" },
        });
        const buffer = (size: number, usage: number, values?: Float32Array<ArrayBuffer>) => {
          const result = device.createBuffer({ size, usage: usage | GPUBufferUsage.COPY_DST });
          if (values) device.queue.writeBuffer(result, 0, values);
          return result;
        };
        const vertices = buffer(144, GPUBufferUsage.VERTEX, new Float32Array([
          -0.75, -0.75, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
          0.75, -0.75, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
          0, 0.75, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
        ]));
        const globals = buffer(256, GPUBufferUsage.UNIFORM);
        const instance = buffer(112, GPUBufferUsage.VERTEX);
        const identity = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
        const shadow = device.createTexture({ size: [1, 1], format: "depth32float", usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.RENDER_ATTACHMENT });
        const groups = [
          device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: globals, size: 240 } }] }),
          device.createBindGroup({ layout: pipeline.getBindGroupLayout(1), entries: [
            { binding: 0, resource: shadow.createView() }, { binding: 1, resource: device.createSampler({ compare: "less-equal" }) },
          ] }),
        ];
        const width = 64, height = 64, bytesPerRow = 256;
        const target = device.createTexture({ size: [width, height], format: "rgba8unorm", usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
        const readback = device.createBuffer({ size: bytesPerRow * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
        const linear = (hex: string) => hex.match(/[0-9a-f]{2}/gi)!.map((part) => Number.parseInt(part, 16) / 255).map((value) => value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4);
        const rows: Array<{ id: string; profile: string; rgba: number[] }> = [];
        for (const record of cases) {
          for (const profile of ["svgFlatLit", "rasterPbr"]) {
            const material = linear(record.material.color), emissive = linear(record.material.emissive);
            const packed = new Float32Array([...identity, ...material, 1, 0, 0, record.material.metalness, record.material.roughness, 0, 0, 0, -1]);
            device.queue.writeBuffer(instance, 0, packed);
            const values = new Float32Array([
              ...identity, ...identity, 0, 0, 5, 0, ...record.lightDirection, 0,
              ...linear(record.ambient.color), record.ambient.intensity, ...linear(record.sun.color), record.sun.intensity,
              record.material.metalness, record.material.roughness, record.material.emissiveIntensity, 1,
              ...emissive, 0, 0, Number(profile === "svgFlatLit"), 0, 1,
            ]);
            if (values.length !== 60) throw new Error("World3dGlobals stride changed");
            device.queue.writeBuffer(globals, 0, values);
            const encoder = device.createCommandEncoder();
            const pass = encoder.beginRenderPass({ colorAttachments: [{ view: target.createView(), clearValue: [0, 0, 0, 0], loadOp: "clear", storeOp: "store" }] });
            pass.setPipeline(pipeline); pass.setBindGroup(0, groups[0]); pass.setBindGroup(1, groups[1]); pass.setVertexBuffer(0, vertices); pass.setVertexBuffer(1, instance); pass.draw(3); pass.end();
            encoder.copyTextureToBuffer({ texture: target }, { buffer: readback, bytesPerRow }, [width, height]);
            device.queue.submit([encoder.finish()]);
            await readback.mapAsync(GPUMapMode.READ);
            const offset = 32 * bytesPerRow + 32 * 4;
            rows.push({ id: record.id, profile, rgba: [...new Uint8Array(readback.getMappedRange()).slice(offset, offset + 4)] });
            readback.unmap();
          }
        }
        const fault = await device.popErrorScope();
        for (const resource of [vertices, globals, instance, shadow, target, readback]) resource.destroy();
        device.destroy();
        if (fault) throw new Error(fault.message);
        return rows;
      }, { shader, cases: fixture.cases });
      for (const record of fixture.cases) {
        const actual = observed.find((row) => row.id === record.id && row.profile === fixture.profile)!;
        const expected = record.expectedFill.match(/\d+/g)!.map(Number);
        actual.rgba.slice(0, 3).forEach((value, index) => expect(Math.abs(value - expected[index]), record.id + ":" + index + ":" + JSON.stringify(actual.rgba)).toBeLessThanOrEqual(1));
        expect(actual.rgba[3], record.id).toBe(255);
      }
      const rasterLow = observed.find((row) => row.id === fixture.cases[0].id && row.profile === "rasterPbr")!.rgba;
      const rasterHigh = observed.find((row) => row.id === fixture.cases[1].id && row.profile === "rasterPbr")!.rgba;
      expect(rasterLow).not.toEqual(rasterHigh);
      expect(observed.filter((row) => row.profile === fixture.profile && row.id.startsWith("front-")).map((row) => row.rgba)).toEqual([
        observed.find((row) => row.id === fixture.cases[0].id && row.profile === fixture.profile)!.rgba,
        observed.find((row) => row.id === fixture.cases[0].id && row.profile === fixture.profile)!.rgba,
      ]);
    } finally {
      await browser.close();
    }
  }, 90_000);
});
