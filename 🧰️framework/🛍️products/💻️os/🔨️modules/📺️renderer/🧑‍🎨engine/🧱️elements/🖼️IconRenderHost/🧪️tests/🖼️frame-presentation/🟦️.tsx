// @vitest-environment jsdom

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { compile } from "@tailwindcss/node";
import Ajv2020 from "ajv/dist/2020.js";
import { chromium } from "playwright";
import { cleanup, render, waitFor } from "@semio-tech/ui-react/test";
import { configureHostPorts, uiI18n } from "@semio-tech/ui-react";
import { afterEach, expect, it } from "vitest";
import { IconRenderHost } from "../../🟦️.tsx";
import fixture from "../../🧫️fixtures/🖼️frame-presentation/🔣️.json";

afterEach(cleanup);

it("validates the neutral icon frame cases", () => {
});

it("measures actual IconRenderHost frame, content and footer in Chromium", async () => {
  await uiI18n.changeLanguage("en");
  const restore = configureHostPorts({ iconRender: { render: async () => ({ dataUrl: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='256' height='128'/%3E" }) } });
  const stylesPath = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css");
  const compiler = await compile(readFileSync(stylesPath, "utf8"), { base: dirname(stylesPath), from: stylesPath, onDependency: () => {} });
  const fonts = [["Anta", "🚀️anta/🏛️latin/📖️regular"], ["Share Tech Mono", "⌨️share-tech-mono/🏛️latin/📖️regular"]].map(([family, path]) => ({ family, data: readFileSync(resolve(dirname(stylesPath), "../../../🖼️assets/🔤️fonts", path, "🔤️outline.ttf")).toString("base64") }));
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    for (const sample of fixture.cases) {
      const request = { assetUrl: "mesh://preview", camera: { position: [0, -10, 0], target: [0, 0, 0], zoom: 1 }, lights: { ambientIntensity: 1, ambientColor: "#fff", sunAzimuth: 0, sunElevation: 45, sunIntensity: 1, sunColor: "#fff" }, width: sample.width, height: sample.height, shape: sample.shape, format: "png" };
      const view = render(<IconRenderHost node={{ type: "componentScene", surfaceId: "frame-test", controllerId: "frame-test", componentKind: "icon-render", iconRender: { requestJson: JSON.stringify(request), ...(sample.footer ? { footer: sample.footer } : {}) } }} onAction={() => {}} />);
      await waitFor(() => expect(view.container.querySelector("img")).not.toBeNull());
      const host = view.container.firstElementChild!;
      const classes = [...new Set([host, ...host.querySelectorAll("[class]")].flatMap(node => [...node.classList]))];
      await page.setContent('<div class="semio-scope" style="position:relative;width:' + sample.container[0] + 'px;height:' + sample.container[1] + 'px">' + host.outerHTML + '</div>');
      await page.addStyleTag({ content: compiler.build(classes) });
      await page.evaluate(async sources => {
        const stripRemoteFaces = (sheet: CSSStyleSheet | CSSGroupingRule) => {
          for (let index = sheet.cssRules.length - 1; index >= 0; index--) {
            const rule = sheet.cssRules[index];
            if (rule instanceof CSSFontFaceRule && sources.some(source => rule.style.fontFamily.replaceAll('"', "").replaceAll("'", "") === source.family)) sheet.deleteRule(index);
            else if (rule instanceof CSSGroupingRule) stripRemoteFaces(rule);
          }
        };
        for (const sheet of document.styleSheets) stripRemoteFaces(sheet);
        for (const source of sources) {
          const face = new FontFace(source.family, Uint8Array.from(atob(source.data), value => value.charCodeAt(0)).buffer);
          document.fonts.add(await face.load());
        }
        await document.fonts.ready;
      }, fonts);
      const loadedFonts = await page.evaluate(() => ({ mono: document.fonts.check('10px "Share Tech Mono"'), body: document.fonts.check('11.2px "Anta"'), faces: [...document.fonts].filter(face => /Anta|Share/.test(face.family)).map(face => ({ family: face.family, status: face.status })) }));
      expect(loadedFonts.mono && loadedFonts.body, JSON.stringify(loadedFonts)).toBe(true);
      const observed = await page.locator(".semio-icon-render-host").evaluate(element => {
        const root = element.getBoundingClientRect();
        const box = (node: Element) => { const rect = node.getBoundingClientRect(); return [rect.x - root.x, rect.y - root.y, rect.width, rect.height]; };
        const frame = element.querySelector("[data-icon-shot-frame]")!;
        const style = getComputedStyle(frame);
        const footer = element.children.length > 1 ? element.lastElementChild! : null;
        const badge = frame.querySelector("span")!;
        const badgeStyle = getComputedStyle(badge);
        const lineText = new Map<number, string>();
        const walker = document.createTreeWalker(badge, NodeFilter.SHOW_TEXT);
        while (walker.nextNode()) {
          const node = walker.currentNode;
          for (let index = 0; index < node.textContent!.length; index++) {
            const range = document.createRange();
            range.setStart(node, index); range.setEnd(node, index + 1);
            const top = Math.round(range.getBoundingClientRect().top);
            lineText.set(top, (lineText.get(top) ?? "") + node.textContent![index]);
          }
        }
        return { frame: box(frame), content: box(frame.querySelector("img")!), radius: style.borderTopLeftRadius, border: Number.parseFloat(style.borderTopWidth), footer: footer ? box(footer) : null,
          badge: { bounds: box(badge), fontSize: Number.parseFloat(badgeStyle.fontSize), lineHeight: Number.parseFloat(badgeStyle.lineHeight), paddingX: Number.parseFloat(badgeStyle.paddingLeft), paddingY: Number.parseFloat(badgeStyle.paddingTop), fontFamily: badgeStyle.fontFamily, lines: [...lineText.values()].map(line => line.trim()) } };
      });
      sample.expected.badge.bounds.forEach((value, axis) => expect(Math.abs(observed.badge.bounds[axis] - value), sample.id + ":" + axis + ":" + JSON.stringify(observed.badge)).toBeLessThan(fixture.tolerance));
      expect(observed.badge.lines, sample.id).toEqual(sample.expected.badge.lines);
      for (const field of ["fontSize", "lineHeight", "paddingX", "paddingY"] as const) expect(observed.badge[field]).toBe(sample.expected.badge[field]);
      expect(observed.badge.fontFamily).toContain(sample.expected.badge.fontFamily);
      const probes = await page.locator("[data-icon-shot-frame] span").evaluate((badge, points) => {
        const bounds = badge.getBoundingClientRect();
        const previous = badge.style.pointerEvents;
        badge.style.pointerEvents = "auto";
        const visible = points.map(point => badge.contains(document.elementFromPoint(bounds.x + bounds.width * point.relative[0], bounds.y + bounds.height * point.relative[1])));
        badge.style.pointerEvents = previous;
        return visible;
      }, sample.expected.badge.probes);
      expect(probes).toEqual(sample.expected.badge.probes.map(point => point.visible));
      const contentProbes = await page.locator("[data-icon-shot-frame] img").evaluate((image, points) => {
        const bounds = image.getBoundingClientRect();
        const previous = image.style.pointerEvents;
        image.style.pointerEvents = "auto";
        const visible = points.map(point => image.contains(document.elementFromPoint(bounds.x + bounds.width * point.relative[0], bounds.y + bounds.height * point.relative[1])));
        image.style.pointerEvents = previous;
        return visible;
      }, sample.expected.contentProbes);
      expect(contentProbes).toEqual(sample.expected.contentProbes.map(point => point.visible));
      expect(observed.radius).toBe(sample.expected.radius);
      for (const field of ["frame", "content", "footer"] as const) {
        const expected = sample.expected[field];
        if (expected === null) expect(observed[field]).toBeNull();
        else expected.forEach((value, axis) => expect(Math.abs(observed[field]![axis] - value)).toBeLessThan(fixture.tolerance));
      }
      expect(observed.border).toBe(2);
      expect(observed.frame[2]).toBeLessThanOrEqual(sample.container[0]);
      expect(observed.frame[3]).toBeLessThanOrEqual(sample.container[1]);
      expect(observed.content[2]).toBeCloseTo(observed.frame[2] - 4, 1);
      expect(observed.content[3]).toBeCloseTo(observed.frame[3] - 4, 1);
      expect(observed.radius === "0px").toBe(sample.shape === "rectangle");
      cleanup();
    }
  } finally {
    await browser.close();
    restore();
  }
}, 90_000);

it("renders the production World mask and scene clear without painting outside the Icon viewport", async () => {
  const path = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs");
  const shader = readFileSync(path, "utf8").match(/pub const WORLD3D_POSTPROCESS_SHADER: &str = r#"([\s\S]*?)"#;/)![1];
  const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader"] });
  try {
    const page = await browser.newPage();
    await page.route("https://renderer.test/**", route => route.fulfill({ contentType: "text/html", body: "<!doctype html><title>World mask fixture</title>" }));
    await page.goto("https://renderer.test/");
    const observed = await page.evaluate(async ({ shader, cases }) => {
      const adapter = await navigator.gpu.requestAdapter();
      if (!adapter) throw new Error("WebGPU adapter is required for the World mask oracle");
      const device = await adapter.requestDevice();
      device.pushErrorScope("validation");
      const module = device.createShaderModule({ code: shader });
      const errors = (await module.getCompilationInfo()).messages.filter(message => message.type === "error");
      if (errors.length) throw new Error(errors.map(message => message.message).join("\n"));
      const pipeline = await device.createRenderPipelineAsync({ layout: "auto", vertex: { module, entryPoint: "vs_main" },
        fragment: { module, entryPoint: "fs_main", targets: [{ format: "rgba8unorm" }] }, primitive: { topology: "triangle-list" } });
      const globals = device.createBuffer({ size: 64, usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST });
      const sampler = device.createSampler({ magFilter: "linear", minFilter: "linear" });
      const rows = [];
      for (const sample of cases) {
        const [width, height] = sample.container;
        const rect = sample.expected.content;
        const texture = (background: boolean) => {
          const source = device.createTexture({ size: [width, height], format: "rgba8unorm", usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
          const data = new Uint8Array(width * height * 4);
          for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) data.set(background ? [13, x % 251, y % 251, 255] : [x % 251, 44, y % 251, 255], (y * width + x) * 4);
          device.queue.writeTexture({ texture: source }, data, { bytesPerRow: width * 4 }, [width, height]);
          return source;
        };
        const capture = texture(false), background = texture(true);
        const group = device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [
          { binding: 0, resource: { buffer: globals } }, { binding: 1, resource: capture.createView() }, { binding: 2, resource: sampler }, { binding: 3, resource: background.createView() },
        ] });
        for (const clear of [false, true]) {
          const data = new ArrayBuffer(64);
          new Float32Array(data).set([...rect, width, height, 0, 0]);
          new Uint32Array(data, 32, 4).set([0, Number(sample.shape === "ellipse"), Number(clear), 0]);
          new Float32Array(data, 48, 4).set([0.25, 0.5, 0.75, 1]);
          device.queue.writeBuffer(globals, 0, data);
          const target = device.createTexture({ size: [width, height], format: "rgba8unorm", usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
          const stride = Math.ceil(width * 4 / 256) * 256;
          const output = device.createBuffer({ size: stride * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
          const encoder = device.createCommandEncoder();
          const pass = encoder.beginRenderPass({ colorAttachments: [{ view: target.createView(), loadOp: "clear", storeOp: "store", clearValue: [0, 0, 1, 1] }] });
          pass.setPipeline(pipeline); pass.setBindGroup(0, group); pass.setViewport(rect[0], rect[1], rect[2], rect[3], 0, 1); pass.draw(6); pass.end();
          encoder.copyTextureToBuffer({ texture: target }, { buffer: output, bytesPerRow: stride }, [width, height]);
          device.queue.submit([encoder.finish()]);
          await output.mapAsync(GPUMapMode.READ);
          const pixels = new Uint8Array(output.getMappedRange());
          rows.push({ id: sample.id, clear, outside: [...pixels.slice(0, 4)], probes: sample.expected.contentProbes.map(point => {
            const x = Math.floor(rect[0] + rect[2] * point.relative[0]), y = Math.floor(rect[1] + rect[3] * point.relative[1]);
            return { x, y, rgba: [...pixels.slice(y * stride + x * 4, y * stride + x * 4 + 4)] };
          }) });
          output.unmap(); output.destroy(); target.destroy();
        }
        capture.destroy(); background.destroy();
      }
      const fault = await device.popErrorScope();
      device.destroy();
      if (fault) throw new Error(fault.message);
      return rows;
    }, { shader, cases: fixture.cases });
    for (const row of observed) {
      expect(row.outside, row.id).toEqual([0, 0, 255, 255]);
      const sample = fixture.cases.find(sample => sample.id === row.id)!;
      row.probes.forEach((point, index) => {
        const expected = row.clear ? [64, 128, 191, 255] : sample.expected.contentProbes[index].visible ? [point.x % 251, 44, point.y % 251, 255] : [13, point.x % 251, point.y % 251, 255];
        point.rgba.forEach((channel, axis) => expect(Math.abs(channel - expected[axis]), row.id + ":" + row.clear + ":" + index + ":" + axis).toBeLessThanOrEqual(1));
      });
    }
  } finally {
    await browser.close();
  }
}, 90_000);


it("renders the production UI shader with the same ellipse badge clipping as Chromium", async () => {
  const path = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs");
  const shader = readFileSync(path, "utf8").match(/pub const UI_SHADER: &str = r#"([\s\S]*?)"#;/)![1];
  const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader"] });
  try {
    const page = await browser.newPage();
    await page.route("https://renderer.test/**", route => route.fulfill({ contentType: "text/html", body: "<!doctype html><title>UI shader fixture</title>" }));
    await page.goto("https://renderer.test/");
    const observed = await page.evaluate(async ({ shader, cases }) => {
      const adapter = await navigator.gpu.requestAdapter();
      if (!adapter) throw new Error("WebGPU adapter is required for the production shader oracle");
      const device = await adapter.requestDevice();
      device.pushErrorScope("validation");
      const module = device.createShaderModule({ code: shader });
      const compilation = await module.getCompilationInfo();
      const errors = compilation.messages.filter(message => message.type === "error").map(message => message.message);
      if (errors.length) throw new Error(errors.join("\n"));
      const pipeline = await device.createRenderPipelineAsync({ layout: "auto",
        vertex: { module, entryPoint: "vs_main", buffers: [
          { arrayStride: 8, attributes: [{ shaderLocation: 0, offset: 0, format: "float32x2" }] },
          { arrayStride: 80, stepMode: "instance", attributes: [1, 2, 3, 4, 5].map((shaderLocation, index) => ({ shaderLocation, offset: index * 16, format: "float32x4" })) },
        ] }, fragment: { module, entryPoint: "fs_main", targets: [{ format: "rgba8unorm" }] }, primitive: { topology: "triangle-list" } });
      const buffer = (values: number[], usage: number) => {
        const result = device.createBuffer({ size: values.length * 4, usage: usage | GPUBufferUsage.COPY_DST });
        device.queue.writeBuffer(result, 0, new Float32Array(values));
        return result;
      };
      const corners = buffer([0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1], GPUBufferUsage.VERTEX);
      const globals = buffer([400, 300, 0, 0], GPUBufferUsage.UNIFORM);
      const glyph = device.createTexture({ size: [1, 1], format: "r8unorm", usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
      const icon = device.createTexture({ size: [1, 1], format: "rgba8unorm", usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
      device.queue.writeTexture({ texture: glyph }, new Uint8Array([255]), {}, [1, 1]);
      device.queue.writeTexture({ texture: icon }, new Uint8Array([255, 255, 255, 255]), {}, [1, 1]);
      const sampler = device.createSampler({ magFilter: "linear", minFilter: "linear" });
      const group = device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [
        { binding: 0, resource: { buffer: globals } }, { binding: 1, resource: glyph.createView() }, { binding: 2, resource: sampler }, { binding: 3, resource: icon.createView() }, { binding: 4, resource: sampler },
      ] });
      const rows = [];
      for (const sample of cases) {
        const width = sample.container[0], height = sample.container[1];
        device.queue.writeBuffer(globals, 0, new Float32Array([width, height, 0, 0]));
        const ellipse = sample.shape === "ellipse" ? sample.expected.content : [0, 0, 0, 0];
        for (const kind of [3, 2]) {
          const instance = buffer([...sample.expected.badge.bounds, 1, 1, 1, 1, 0, 0, kind, 0, 0, 0, 1, 1, ...ellipse], GPUBufferUsage.VERTEX);
          const texture = device.createTexture({ size: [width, height], format: "rgba8unorm", usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC });
          const stride = Math.ceil(width * 4 / 256) * 256;
          const output = device.createBuffer({ size: stride * height, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
          const encoder = device.createCommandEncoder();
          const pass = encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView(), loadOp: "clear", storeOp: "store", clearValue: [0, 0, 0, 0] }] });
          const content = sample.expected.content;
          pass.setScissorRect(Math.floor(content[0]), Math.floor(content[1]), Math.ceil(content[0] + content[2]) - Math.floor(content[0]), Math.ceil(content[1] + content[3]) - Math.floor(content[1]));
          pass.setPipeline(pipeline); pass.setBindGroup(0, group); pass.setVertexBuffer(0, corners); pass.setVertexBuffer(1, instance); pass.draw(6); pass.end();
          encoder.copyTextureToBuffer({ texture }, { buffer: output, bytesPerRow: stride }, [width, height]);
          device.queue.submit([encoder.finish()]);
          await output.mapAsync(GPUMapMode.READ);
          const pixels = new Uint8Array(output.getMappedRange());
          const rect = sample.expected.badge.bounds;
          rows.push({ id: sample.id, kind, alpha: sample.expected.badge.probes.map(point => pixels[Math.floor(rect[1] + rect[3] * point.relative[1]) * stride + Math.floor(rect[0] + rect[2] * point.relative[0]) * 4 + 3]) });
          output.unmap(); output.destroy(); texture.destroy(); instance.destroy();
        }
      }
      const fault = await device.popErrorScope();
      device.destroy();
      if (fault) throw new Error(fault.message);
      return rows;
    }, { shader, cases: fixture.cases });
    for (const row of observed) {
      const sample = fixture.cases.find(sample => sample.id === row.id)!;
      expect(row.alpha.map(alpha => alpha > 127), row.id + ":" + row.kind).toEqual(sample.expected.badge.probes.map(point => point.visible));
    }
  } finally {
    await browser.close();
  }
}, 90_000);
