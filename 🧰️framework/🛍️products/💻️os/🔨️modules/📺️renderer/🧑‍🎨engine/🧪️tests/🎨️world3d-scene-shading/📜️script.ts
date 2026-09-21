/** 🎨️ Actual Three/WebGPU pixel comparison for the shared World3d shading fixture. */
import Ajv from "ajv";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { createHash } from "node:crypto";
import { BundleScript } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

declare const THREE: any;

function paintedShaderFromProduction(shader: string): string {
  const replacements: readonly (readonly [string, string])[] = [
    [
      "@group(1) @binding(1) var shadow_sampler: sampler_comparison;",
      "@group(1) @binding(1) var shadow_sampler: sampler_comparison;\n@group(2) @binding(0) var paint_map: texture_2d<f32>;\n@group(2) @binding(1) var paint_sampler: sampler;",
    ],
    ["@location(2) color: vec4<f32>,\n}", "@location(2) color: vec4<f32>,\n@location(9) uv: vec2<f32>,\n}"],
    ["@location(3) world_position: vec3<f32>,\n}", "@location(3) world_position: vec3<f32>,\n@location(4) uv: vec2<f32>,\n}"],
    ["out.world_position = world_pos.xyz;\nreturn out;", "out.world_position = world_pos.xyz;\nout.uv = vertex.uv;\nreturn out;"],
    [
      "let emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0);",
      "let sampled = textureSample(paint_map, paint_sampler, in.uv);\nlet paint_color = world3d_linear_to_srgb(sampled.rgb);\nlet lit_color = in.color.rgb * paint_color;\nlet emissive = globals.material_emissive.rgb * globals.material.z + in.color.rgb * max(in.flags.y, 0.0);",
    ],
    [
      "let color = world3d_lighting(n, v, in.color.rgb, metalness, roughness, shadow_visibility) + emissive;\nreturn vec4<f32>(world3d_attachment_output(color), in.color.a);",
      "let color = world3d_lighting(n, v, lit_color, metalness, roughness, shadow_visibility) + emissive;\nreturn vec4<f32>(world3d_attachment_output(color), sampled.a * in.color.a);",
    ],
  ];
  for (const [anchor, replacement] of replacements) {
    const next = shader.replace(anchor, replacement);
    if (next === shader) throw new Error("Production painted shader anchor drifted: " + anchor);
    shader = next;
  }
  return shader;
}

function renderSceneShading(fixture: any) {
  const [width, height] = fixture.scene.viewport;
  const profile = fixture.pixelOracle.profile;
  const renderer = new THREE.WebGLRenderer({ antialias: profile.antialias, alpha: true, premultipliedAlpha: profile.premultipliedAlpha, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setSize(width, height);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = fixture.reference.exposure;
  document.body.appendChild(renderer.domElement);
  const linear = (values: number[]) => new THREE.Color().setRGB(values[0], values[1], values[2], THREE.LinearSRGBColorSpace);
  renderer.setClearColor(linear(profile.clearRgba), profile.clearRgba[3]);
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.position.fromArray(fixture.scene.camera.position);
  camera.up.fromArray(fixture.scene.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
  const specifications: any[] = [];
  for (const row of fixture.provenanceCases) {
    const color = row.source === "semanticNeutral" ? new THREE.Color(fixture.appearances[row.appearance].panelHex) : linear(row.sourceLinear);
    specifications.push({ id: row.id, color, opacity: row.sourceLinear[3], lights: "fallback" });
  }
  for (const row of fixture.styleCases) specifications.push({ id: row.id, color: linear(row.baseColor), opacity: row.opacity, vertexColors: row.vertexColorWeight === 1, emissiveIntensity: row.emissiveIntensity, lights: "fallback" });
  const indirect = fixture.math.indirectLambert;
  specifications.push({ id: "indirect-lambert", color: linear(indirect.baseColor), metalness: indirect.metalness, ambient: indirect.irradiance });
  for (const row of fixture.math.directSamples)
    specifications.push({ id: row.id, color: linear(row.baseColor), metalness: row.metalness, roughness: row.roughness, normal: row.normal, view: row.view, direct: { position: row.light, color: row.radiance, intensity: 1 } });
  for (const row of fixture.math.acesInputs) specifications.push({ id: "aces-" + row.id, color: linear(row.linearRgb), basic: true });
  specifications.push({
    id: "sun-neutral-light",
    color: new THREE.Color(fixture.appearances.light.panelHex),
    direct: { position: fixture.scene.sunLight.direction, color: fixture.scene.sunLight.color, intensity: fixture.scene.sunLight.intensity },
    ambient: fixture.scene.fallbackLights.ambient.color,
    ambientIntensity: fixture.scene.fallbackLights.ambient.intensity,
  });
  const cases = specifications.map((specification) => ({ id: specification.id, clear: profile.clearRgba, layers: [specification] }));
  for (const row of fixture.compositingCases)
    cases.push({
      id: row.id,
      clear: row.clearLinearRgba,
      layers: row.layers.map((id: string) => {
        const specification = specifications.find((value) => value.id === id);
        if (!specification) throw new Error("Unknown layer " + id);
        return specification;
      }),
    });
  const rows = [];
  renderer.autoClear = false;
  for (const sample of cases) {
    renderer.setClearColor(linear(sample.clear), sample.clear[3]);
    renderer.clear();
    for (const [layerIndex, specification] of sample.layers.entries()) {
      const scene = new THREE.Scene();
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(fixture.scene.geometry.positions, 3));
      geometry.setAttribute("normal", new THREE.Float32BufferAttribute(specification.normal ? fixture.scene.geometry.normals.flatMap((_: number, index: number) => (index % 3 === 0 ? specification.normal : [])) : fixture.scene.geometry.normals, 3));
      geometry.setAttribute("color", new THREE.Float32BufferAttribute(fixture.scene.geometry.colors, 4));
      geometry.setIndex(fixture.scene.geometry.indices);
      const opacity = specification.opacity ?? 1;
      const material = specification.basic
        ? new THREE.MeshBasicMaterial({ color: specification.color, toneMapped: true })
        : new THREE.MeshStandardMaterial({
            color: specification.color,
            metalness: specification.metalness ?? fixture.scene.material.metalness,
            roughness: specification.roughness ?? fixture.scene.material.roughness,
            vertexColors: specification.vertexColors ?? false,
            opacity,
            transparent: opacity < 1,
            emissive: specification.emissiveIntensity ? specification.color : linear(fixture.scene.material.emissive),
            emissiveIntensity: specification.emissiveIntensity ?? fixture.scene.material.emissiveIntensity,
          });
      scene.add(new THREE.Mesh(geometry, material));
      if (specification.lights === "fallback") {
        const lights = fixture.scene.fallbackLights;
        scene.add(new THREE.AmbientLight(linear(lights.ambient.color), lights.ambient.intensity));
        const hemisphere = new THREE.HemisphereLight(linear(lights.hemisphere.sky), lights.hemisphere.groundHex ? new THREE.Color(lights.hemisphere.groundHex) : linear(lights.hemisphere.ground), lights.hemisphere.intensity);
        if (!lights.hemisphere.position) throw new Error("Hemisphere position must be explicit in the shared fixture");
        hemisphere.position.fromArray(lights.hemisphere.position);
        scene.add(hemisphere);
        for (const value of lights.directional) {
          const light = new THREE.DirectionalLight(linear(value.color), value.intensity);
          light.position.fromArray(value.position);
          scene.add(light);
        }
      }
      if (specification.ambient) scene.add(new THREE.AmbientLight(linear(specification.ambient), specification.ambientIntensity ?? 1));
      if (specification.direct) {
        const light = new THREE.DirectionalLight(linear(specification.direct.color), specification.direct.intensity);
        light.position.fromArray(specification.direct.position).multiplyScalar(10);
        scene.add(light);
      }
      camera.position.fromArray(specification.view ?? fixture.scene.camera.position);
      if (specification.view) camera.position.multiplyScalar(4);
      camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
      renderer.render(scene, camera);
      const rgba = new Uint8Array(4);
      const gl = renderer.getContext();
      gl.readPixels(profile.samplePosition[0], profile.samplePosition[1], 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, rgba);
      const error = gl.getError();
      if (error !== gl.NO_ERROR) throw new Error("WebGL readback failed: " + error);
      if (layerIndex === sample.layers.length - 1) rows.push({ id: sample.id, point: profile.samplePosition, rgba8: [...rgba] });
      geometry.dispose();
      material.dispose();
    }
  }
  const gl = renderer.getContext();
  const debug = gl.getExtension("WEBGL_debug_renderer_info");
  const result = {
    status: "recorded-browser",
    producer: "installed Three " + THREE.REVISION + " WebGLRenderer in Chromium",
    threeRevision: THREE.REVISION,
    viewport: [width, height],
    clearRgba: profile.clearRgba,
    antialias: renderer.getContext().getContextAttributes().antialias,
    premultipliedAlpha: profile.premultipliedAlpha,
    outputColorSpace: renderer.outputColorSpace,
    toneMapping: "ACESFilmic",
    exposure: renderer.toneMappingExposure,
    gpu: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
    rows,
  };
  renderer.dispose();
  return result;
}

function renderSceneShadingS2(fixture: any) {
  const [width, height] = fixture.scene.viewport;
  const profile = fixture.s2PixelOracle.profile;
  const renderer = new THREE.WebGLRenderer({ antialias: profile.antialias, alpha: true, premultipliedAlpha: profile.premultipliedAlpha, preserveDrawingBuffer: true });
  renderer.setPixelRatio(1);
  renderer.setSize(width, height);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = fixture.reference.exposure;
  renderer.autoClear = false;
  document.body.appendChild(renderer.domElement);
  const linear = (values: number[]) => new THREE.Color().setRGB(values[0], values[1], values[2], THREE.LinearSRGBColorSpace);
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.position.fromArray(fixture.scene.camera.position);
  camera.up.fromArray(fixture.scene.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
  camera.updateMatrixWorld();
  camera.updateProjectionMatrix();
  const rows: any[] = [];
  const standardGeometry = (center = [0, 0, 0]) => {
    const geometry = new THREE.BufferGeometry();
    geometry.setAttribute("position", new THREE.Float32BufferAttribute([
      center[0] - 1.5, center[1] - 1.5, center[2],
      center[0] + 1.5, center[1] - 1.5, center[2],
      center[0], center[1] + 1.5, center[2],
    ], 3));
    geometry.setAttribute("normal", new THREE.Float32BufferAttribute([0, 0, 1, 0, 0, 1, 0, 0, 1], 3));
    geometry.setIndex([0, 1, 2]);
    geometry.computeBoundingSphere();
    return geometry;
  };
  const addFallbackLights = (scene: any) => {
    const lights = fixture.scene.fallbackLights;
    scene.add(new THREE.AmbientLight(linear(lights.ambient.color), lights.ambient.intensity));
    const hemisphere = new THREE.HemisphereLight(linear(lights.hemisphere.sky), new THREE.Color(lights.hemisphere.groundHex), lights.hemisphere.intensity);
    hemisphere.position.fromArray(lights.hemisphere.position);
    scene.add(hemisphere);
    for (const value of lights.directional) {
      const light = new THREE.DirectionalLight(linear(value.color), value.intensity);
      light.position.fromArray(value.position);
      scene.add(light);
    }
  };
  const read = (point: readonly number[]) => {
    const rgba = new Uint8Array(4);
    const gl = renderer.getContext();
    gl.readPixels(point[0], point[1], 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, rgba);
    if (gl.getError() !== gl.NO_ERROR) throw new Error("WebGL readback failed at " + point.join(","));
    return [...rgba];
  };
  const render = (id: string, scene: any, point = profile.samplePosition) => {
    renderer.setClearColor(linear(profile.clearRgba), profile.clearRgba[3]);
    renderer.clear();
    renderer.render(scene, camera);
    rows.push({ id, point, rgba8: read(point) });
  };
  for (const row of fixture.materialCases) {
    const scene = new THREE.Scene();
    const geometry = standardGeometry();
    const material = new THREE.MeshStandardMaterial({
      color: new THREE.Color(fixture.appearances.light.panelHex),
      metalness: row.expected.metalness,
      roughness: row.expected.roughness,
      side: row.side === "double" ? THREE.DoubleSide : THREE.FrontSide,
    });
    if (material.depthWrite !== row.depthWrite) throw new Error("Material depth policy drifted for " + row.id);
    scene.add(new THREE.Mesh(geometry, material));
    addFallbackLights(scene);
    render(row.id, scene);
    geometry.dispose();
    material.dispose();
  }
  {
    const scene = new THREE.Scene();
    const resources: any[] = [];
    for (const row of fixture.mixedMaterialScope.cases) {
      const geometry = standardGeometry();
      const material = new THREE.MeshStandardMaterial({
        color: new THREE.Color(fixture.appearances.light.panelHex),
        metalness: row.expected.metalness,
        roughness: row.expected.roughness,
        side: row.geometryKind === "inline" ? THREE.DoubleSide : THREE.FrontSide,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.fromArray(row.position);
      mesh.scale.setScalar(fixture.mixedMaterialScope.scale);
      scene.add(mesh);
      resources.push(geometry, material);
    }
    addFallbackLights(scene);
    renderer.setClearColor(linear(profile.clearRgba), profile.clearRgba[3]);
    renderer.clear();
    renderer.render(scene, camera);
    for (const row of fixture.mixedMaterialScope.cases) rows.push({ id: row.id, point: row.samplePosition, rgba8: read(row.samplePosition) });
    for (const resource of resources) resource.dispose();
  }
  for (const row of fixture.textureCases) {
    const scene = new THREE.Scene();
    const geometry = standardGeometry();
    geometry.setAttribute("uv", new THREE.Float32BufferAttribute([0, 0, 1, 0, 0.5, 1], 2));
    geometry.setAttribute("color", new THREE.Float32BufferAttribute([...row.vertexColor, ...row.vertexColor, ...row.vertexColor], 4));
    const texture = new THREE.DataTexture(new Uint8Array(row.texelRgba8), 1, 1, THREE.RGBAFormat, THREE.UnsignedByteType);
    texture.colorSpace = row.mapColorSpace === "srgb" ? THREE.SRGBColorSpace : THREE.NoColorSpace;
    texture.minFilter = THREE.NearestFilter;
    texture.magFilter = THREE.NearestFilter;
    texture.needsUpdate = true;
    const color = linear(row.baseColor);
    const material = new THREE.MeshStandardMaterial({
      color,
      vertexColors: row.preserveVertexColor,
      map: texture,
      side: THREE.DoubleSide,
      metalness: 0,
      roughness: 1,
      emissive: row.preserveVertexColor ? new THREE.Color(0, 0, 0) : color,
      emissiveIntensity: row.preserveVertexColor ? 0 : row.emissiveIntensity,
      transparent: row.opacity < 1,
      opacity: row.opacity,
    });
    if (material.depthWrite !== row.depthWrite) throw new Error("Textured material depth policy drifted for " + row.id);
    scene.add(new THREE.Mesh(geometry, material));
    addFallbackLights(scene);
    render(row.id, scene);
    geometry.dispose();
    material.dispose();
    texture.dispose();
  }
  const conicVertex = `
varying vec3 vObjectPosition;
void main() {
  vObjectPosition = position;
  gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}
`;
  const conicFragment = `
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uColorC;
uniform float uAngle;
uniform float uOpacity;
varying vec3 vObjectPosition;
void main() {
  float a = atan(vObjectPosition.y, vObjectPosition.x) + uAngle;
  float t = fract(a / 6.28318530718);
  vec3 color;
  if (t < 0.333333) {
    color = mix(uColorA, uColorB, t / 0.333333);
  } else if (t < 0.666667) {
    color = mix(uColorB, uColorC, (t - 0.333333) / 0.333333);
  } else {
    color = mix(uColorC, uColorA, (t - 0.666667) / 0.333333);
  }
  gl_FragColor = vec4(color, uOpacity);
}
`;
  for (const row of fixture.celebration.cases) {
    const scene = new THREE.Scene();
    const geometry = standardGeometry(row.localPosition);
    const material = new THREE.ShaderMaterial({
      vertexShader: conicVertex,
      fragmentShader: conicFragment,
      uniforms: {
        uColorA: { value: new THREE.Color(fixture.celebration.stops[0].hex) },
        uColorB: { value: new THREE.Color(fixture.celebration.stops[1].hex) },
        uColorC: { value: new THREE.Color(fixture.celebration.stops[2].hex) },
        uAngle: { value: (row.phaseSeconds / fixture.celebration.spinSeconds) * Math.PI * 2 },
        uOpacity: { value: row.opacity },
      },
      transparent: row.opacity < 1,
      side: THREE.DoubleSide,
      depthWrite: row.depthWrite,
    });
    const mesh = new THREE.Mesh(geometry, material);
    mesh.position.set(-row.localPosition[0], -row.localPosition[1], -row.localPosition[2]);
    scene.add(mesh);
    render(row.id, scene);
    geometry.dispose();
    material.dispose();
  }
  const depthOrders: { readonly id: string; readonly order: string[] }[] = [];
  for (const row of fixture.depthOrderCases) {
    const scene = new THREE.Scene();
    const resources: any[] = [];
    const order: string[] = [];
    for (const layer of row.layers) {
      const geometry = standardGeometry(layer.localCenter);
      const material = new THREE.MeshStandardMaterial({
        color: linear(layer.color),
        metalness: 0,
        roughness: 1,
        transparent: true,
        opacity: layer.opacity,
      });
      if (material.depthWrite !== layer.depthWrite) throw new Error("Standard transparent depth policy drifted for " + layer.id);
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.fromArray(layer.position);
      mesh.onBeforeRender = () => order.push(layer.id);
      scene.add(mesh);
      resources.push(geometry, material);
    }
    addFallbackLights(scene);
    render(row.id, scene);
    if (JSON.stringify(order) !== JSON.stringify(row.expectedDrawOrder)) throw new Error("Three transparent draw order drifted for " + row.id + ": " + JSON.stringify(order));
    depthOrders.push({ id: row.id, order });
    for (const resource of resources) resource.dispose();
  }
  const gl = renderer.getContext();
  const debug = gl.getExtension("WEBGL_debug_renderer_info");
  const result = {
    status: "recorded-browser",
    producer: "installed Three " + THREE.REVISION + " S2 WebGLRenderer in Chromium",
    threeRevision: THREE.REVISION,
    viewport: [width, height],
    clearRgba: profile.clearRgba,
    antialias: renderer.getContext().getContextAttributes().antialias,
    premultipliedAlpha: profile.premultipliedAlpha,
    outputColorSpace: renderer.outputColorSpace,
    toneMapping: "ACESFilmic for standard materials; raw ShaderMaterial for celebration",
    exposure: renderer.toneMappingExposure,
    gpu: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
    depthOrders,
    rows,
  };
  renderer.dispose();
  return result;
}

async function renderWgpuShading(input: any) {
  const { fixture, shader } = input;
  const s2MaterialScope = input.mode === "s2-material-scope";
  const s2CurrentMesh = input.mode === "s2-current-mesh";
  const s2OrderedMesh = input.mode === "s2-ordered-mesh";
  const gpu = (navigator as any).gpu;
  if (!gpu) throw new Error("WebGPU is unavailable in the controlled browser context");
  const adapter = await gpu.requestAdapter();
  if (!adapter) throw new Error("WebGPU adapter unavailable");
  const device = await adapter.requestDevice();
  const errors: string[] = [];
  device.addEventListener("uncapturederror", (event: any) => errors.push(String(event.error)));
  const [width, height] = fixture.scene.viewport;
  const profile = (s2MaterialScope || s2CurrentMesh ? fixture.s2PixelOracle : fixture.pixelOracle).profile;
  const module = device.createShaderModule({ code: shader });
  const compilation = await module.getCompilationInfo();
  const failures = compilation.messages.filter((row: any) => row.type === "error");
  if (failures.length) throw new Error(JSON.stringify(failures));
  const format = "rgba8unorm";
  const storageFormat = "rgba8unorm";
  const uiFormat = "rgba8unorm-srgb";
  const texture = device.createTexture({ size: [width, height], format: storageFormat, viewFormats: [uiFormat, format], usage: 0x10 | 0x01 });
  const depth = device.createTexture({ size: [width, height], format: "depth24plus-stencil8", usage: 0x10 });
  const shadow = device.createTexture({ size: [1, 1], format: "depth32float", usage: 0x10 | 0x04 });
  const uniform = device.createBuffer({ size: 256, usage: 0x40 | 0x08 });
  const instance = device.createBuffer({ size: 96 * (s2MaterialScope || s2CurrentMesh || s2OrderedMesh ? 2 : 1), usage: 0x20 | 0x08 });
  const vertex = device.createBuffer({ size: (fixture.scene.geometry.positions.length / 3) * 48, usage: 0x20 | 0x08 });
  const indices = device.createBuffer({ size: 12, usage: 0x10 | 0x08 });
  device.queue.writeBuffer(indices, 0, new Uint32Array(fixture.scene.geometry.indices));
  const bytesPerRow = Math.ceil((width * 4) / 256) * 256;
  const readback = device.createBuffer({ size: bytesPerRow * height, usage: 0x01 | 0x08 });
  const layouts = [
    {
      arrayStride: 48,
      attributes: [
        { shaderLocation: 0, offset: 0, format: "float32x3" },
        { shaderLocation: 1, offset: 12, format: "float32x3" },
        { shaderLocation: 2, offset: 24, format: "float32x4" },
        { shaderLocation: 9, offset: 40, format: "float32x2" },
      ],
    },
    { arrayStride: 96, stepMode: "instance", attributes: Array.from({ length: 6 }, (_, i) => ({ shaderLocation: i + 3, offset: i * 16, format: "float32x4" })) },
  ];
  const pipelines: any[] = [];
  for (const transparent of [false, true])
    pipelines.push(
      await device.createRenderPipelineAsync({
        layout: "auto",
        vertex: { module, entryPoint: "vs_main", buffers: layouts },
        fragment: {
          module,
          entryPoint: "fs_main",
          targets: [{ format, ...(transparent ? { blend: { color: { srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha", operation: "add" }, alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha", operation: "add" } } } : {}) }],
        },
        primitive: { topology: "triangle-list", cullMode: transparent ? "back" : "none" },
        depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: !transparent || s2OrderedMesh, depthCompare: "less-equal" },
      }),
    );
  const groups = pipelines.map((pipeline) => [
    device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: uniform, size: 240 } }] }),
    device.createBindGroup({
      layout: pipeline.getBindGroupLayout(1),
      entries: [
        { binding: 0, resource: shadow.createView() },
        { binding: 1, resource: device.createSampler({ compare: "less-equal" }) },
      ],
    }),
  ]);
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.coordinateSystem = THREE.WebGPUCoordinateSystem;
  camera.up.fromArray(fixture.scene.camera.up);
  const identity = new THREE.Matrix4().elements;
  const specifications: any[] = [];
  for (const row of fixture.provenanceCases) specifications.push({ id: row.id, color: row.expectedNeutralLinear, opacity: row.sourceLinear[3], fallback: true });
  for (const row of fixture.styleCases) specifications.push({ id: row.id, color: row.baseColor, opacity: row.opacity, vertexWeight: row.vertexColorWeight, emissiveIntensity: row.emissiveIntensity, fallback: true });
  const indirect = fixture.math.indirectLambert;
  specifications.push({ id: "indirect-lambert", color: indirect.baseColor, metalness: indirect.metalness, ambient: indirect.irradiance });
  for (const row of fixture.math.directSamples)
    specifications.push({ id: row.id, color: row.baseColor, metalness: row.metalness, roughness: row.roughness, normal: row.normal, view: row.view, direction: row.light, sun: row.radiance, sunIntensity: 1 });
  for (const row of fixture.math.acesInputs) specifications.push({ id: "aces-" + row.id, color: row.linearRgb, emissiveIntensity: 1, metalness: 1 });
  specifications.push({
    id: "sun-neutral-light",
    color: fixture.appearances.light.panelLinear,
    direction: fixture.scene.sunLight.direction,
    sun: fixture.scene.sunLight.color,
    sunIntensity: fixture.scene.sunLight.intensity,
    ambient: fixture.scene.fallbackLights.ambient.color,
    ambientIntensity: fixture.scene.fallbackLights.ambient.intensity,
  });
  const currentCelebrationCases = fixture.celebration.cases.map((row: any) => ({
    id: row.id,
    clear: profile.clearRgba,
    layers: [{ color: fixture.celebration.stops[0].linear, emissiveIntensity: 0.55, fallback: true, opacity: row.opacity }],
  }));
  const depthCases = fixture.depthOrderCases.map((row: any) => {
    const drawOrder = s2OrderedMesh ? row.expectedDrawOrder : row.layers.map((layer: any) => layer.id);
    return {
      id: row.id,
      clear: profile.clearRgba,
      currentDrawOrder: drawOrder,
      layers: [
        {
          fallback: true,
          opacity: 0.5,
          instances: drawOrder.map((id: string) => row.layers.find((layer: any) => layer.id === id)).map((layer: any) => ({
            ...layer,
            position: layer.position.map((value: number, index: number) => value + layer.localCenter[index]),
            metalness: 0,
            roughness: 1,
          })),
        },
      ],
    };
  });
  const cases = s2MaterialScope
    ? [
        {
          id: "mixed-material-scope",
          clear: profile.clearRgba,
          layers: [
            {
              color: fixture.appearances.light.panelLinear,
              fallback: true,
              metalness: fixture.mixedMaterialScope.environment.metalness,
              roughness: fixture.mixedMaterialScope.environment.roughness,
              instances: fixture.mixedMaterialScope.cases,
              sampleRows: fixture.mixedMaterialScope.cases,
            },
          ],
        },
      ]
    : s2CurrentMesh
      ? [...currentCelebrationCases, ...depthCases]
      : s2OrderedMesh
        ? depthCases
        : specifications.map((specification) => ({ id: specification.id, clear: profile.clearRgba, layers: [specification] }));
  if (!s2MaterialScope && !s2CurrentMesh && !s2OrderedMesh)
    for (const row of fixture.compositingCases)
      cases.push({
        id: row.id,
        clear: row.clearLinearRgba,
        layers: row.layers.map((id: string) => {
          const specification = specifications.find((value) => value.id === id);
          if (!specification) throw new Error("Unknown layer " + id);
          return specification;
        }),
      });
  const rows = [];
  try {
    for (const sample of cases) {
      for (const [layerIndex, specification] of sample.layers.entries()) {
        camera.position.fromArray(specification.view ?? fixture.scene.camera.position);
        if (specification.view) camera.position.multiplyScalar(4);
        camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
        camera.updateMatrixWorld();
        camera.updateProjectionMatrix();
        const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse).elements;
        const ambient = specification.fallback ? fixture.scene.fallbackLights.ambient.color : (specification.ambient ?? [0, 0, 0]);
        const ambientIntensity = specification.fallback ? fixture.scene.fallbackLights.ambient.intensity : (specification.ambientIntensity ?? 1);
        const values = new Float32Array([
          ...viewProjection,
          ...identity,
          ...camera.position.toArray(),
          0,
          ...(specification.direction ?? [0, 0, 1]),
          0,
          ...ambient,
          ambientIntensity,
          ...(specification.sun ?? [0, 0, 0]),
          specification.sunIntensity ?? 0,
          specification.metalness ?? fixture.scene.material.metalness,
          specification.roughness ?? fixture.scene.material.roughness,
          fixture.scene.material.emissiveIntensity,
          specification.fallback ? 0 : 1,
          ...fixture.scene.material.emissive,
          0,
          0,
          0,
          0,
          1,
        ]);
        if (values.length !== 60) throw new Error("World3dGlobals stride changed");
        device.queue.writeBuffer(uniform, 0, values);
        const instanceSpecifications = specification.instances ?? [specification];
        const packedInstances: number[] = [];
        for (const value of instanceSpecifications) {
          const model = value.position
            ? new THREE.Matrix4().compose(new THREE.Vector3().fromArray(value.position), new THREE.Quaternion(), new THREE.Vector3().setScalar(value.scale ?? (s2MaterialScope ? fixture.mixedMaterialScope.scale : 1))).elements
            : identity;
          const color = value.color ?? specification.color;
          const opacity = value.opacity ?? specification.opacity ?? 1;
          const preserveVertexColor = value.vertexWeight ?? specification.vertexWeight ?? 0;
          const metalness = value.expected?.metalness ?? value.metalness ?? specification.metalness ?? fixture.scene.material.metalness;
          const roughness = value.expected?.roughness ?? value.roughness ?? specification.roughness ?? fixture.scene.material.roughness;
          packedInstances.push(...model, ...color.slice(0, 3), opacity, preserveVertexColor > 0.5 ? 1 : 0, value.emissiveIntensity ?? specification.emissiveIntensity ?? 0, metalness, roughness);
        }
        device.queue.writeBuffer(instance, 0, new Float32Array(packedInstances));
        const vertices: number[] = [];
        for (let index = 0; index < fixture.scene.geometry.positions.length / 3; index++)
          vertices.push(
            ...fixture.scene.geometry.positions.slice(index * 3, index * 3 + 3),
            ...(specification.normal ?? fixture.scene.geometry.normals.slice(index * 3, index * 3 + 3)),
            ...fixture.scene.geometry.colors.slice(index * 4, index * 4 + 4),
            0,
            0,
          );
        device.queue.writeBuffer(vertex, 0, new Float32Array(vertices));
        const encoder = device.createCommandEncoder();
        const loadOp = layerIndex === 0 ? "clear" : "load";
        if (layerIndex === 0) encoder.beginRenderPass({ colorAttachments: [{ view: texture.createView({ format: uiFormat }), clearValue: sample.clear, loadOp: "clear", storeOp: "store" }] }).end();
        const pass = encoder.beginRenderPass({
          colorAttachments: [{ view: texture.createView({ format }), loadOp: "load", storeOp: "store" }],
          depthStencilAttachment: { view: depth.createView(), depthClearValue: 1, depthLoadOp: loadOp, depthStoreOp: "store", stencilClearValue: 0, stencilLoadOp: loadOp, stencilStoreOp: "store" },
        });
        const opacity = specification.opacity ?? 1;
        const pipelineIndex = opacity < 1 ? 1 : 0;
        pass.setPipeline(pipelines[pipelineIndex]);
        pass.setBindGroup(0, groups[pipelineIndex][0]);
        pass.setBindGroup(1, groups[pipelineIndex][1]);
        pass.setVertexBuffer(0, vertex);
        pass.setVertexBuffer(1, instance);
        pass.setIndexBuffer(indices, "uint32");
        pass.drawIndexed(fixture.scene.geometry.indices.length, instanceSpecifications.length);
        pass.end();
        encoder.copyTextureToBuffer({ texture }, { buffer: readback, bytesPerRow }, [width, height]);
        device.queue.submit([encoder.finish()]);
        await readback.mapAsync(1);
        const mapped = new Uint8Array(readback.getMappedRange());
        if (layerIndex === sample.layers.length - 1) {
          const sampleRows = specification.sampleRows ?? [{ id: sample.id, samplePosition: profile.samplePosition }];
          for (const row of sampleRows) {
            const point = row.samplePosition;
            const offset = (height - 1 - point[1]) * bytesPerRow + point[0] * 4;
            rows.push({ id: row.id, point, rgba8: [...mapped.slice(offset, offset + 4)] });
          }
        }
        readback.unmap();
        if (errors.length) throw new Error(errors.join("\n"));
      }
    }
    return {
      status: "recorded-browser",
      producer: "Actual production WORLD3D_SHADER through Chromium WebGPU",
      viewport: [width, height],
      format,
      storageFormat,
      outputTransfer: "shader-linear-to-srgb-on-unorm-world-view",
      gpu: { vendor: adapter.info.vendor, architecture: adapter.info.architecture, device: adapter.info.device, description: adapter.info.description },
      currentDrawOrders: cases.filter((row: any) => row.currentDrawOrder).map((row: any) => ({ id: row.id, order: row.currentDrawOrder })),
      rows,
    };
  } finally {
    for (const resource of [texture, depth, shadow, uniform, instance, vertex, indices, readback]) resource.destroy();
    device.destroy();
  }
}

async function renderWgpuTextured(input: any) {
  const { fixture, shader } = input;
  const gpu = (navigator as any).gpu;
  if (!gpu) throw new Error("WebGPU is unavailable in the controlled browser context");
  const adapter = await gpu.requestAdapter();
  if (!adapter) throw new Error("WebGPU adapter unavailable");
  const device = await adapter.requestDevice();
  const [width, height] = fixture.scene.viewport;
  const format = "rgba8unorm";
  const uiFormat = "rgba8unorm-srgb";
  const output = device.createTexture({ size: [width, height], format, viewFormats: [uiFormat], usage: 0x10 | 0x01 });
  const depth = device.createTexture({ size: [width, height], format: "depth24plus-stencil8", usage: 0x10 });
  const shadow = device.createTexture({ size: [1, 1], format: "depth32float", usage: 0x10 | 0x04 });
  const uniform = device.createBuffer({ size: 256, usage: 0x40 | 0x08 });
  const instance = device.createBuffer({ size: 96, usage: 0x20 | 0x08 });
  const vertex = device.createBuffer({ size: 144, usage: 0x20 | 0x08 });
  const indices = device.createBuffer({ size: 12, usage: 0x10 | 0x08 });
  const readback = device.createBuffer({ size: 256 * height, usage: 0x01 | 0x08 });
  const module = device.createShaderModule({ code: shader });
  const compilation = await module.getCompilationInfo();
  const failures = compilation.messages.filter((row: any) => row.type === "error");
  if (failures.length) throw new Error(JSON.stringify(failures));
  const pipeline = await device.createRenderPipelineAsync({
    layout: "auto",
    vertex: {
      module,
      entryPoint: "vs_main",
      buffers: [
        {
          arrayStride: 48,
          attributes: [
            { shaderLocation: 0, offset: 0, format: "float32x3" },
            { shaderLocation: 1, offset: 12, format: "float32x3" },
            { shaderLocation: 2, offset: 24, format: "float32x4" },
            { shaderLocation: 9, offset: 40, format: "float32x2" },
          ],
        },
        { arrayStride: 96, stepMode: "instance", attributes: Array.from({ length: 6 }, (_, index) => ({ shaderLocation: index + 3, offset: index * 16, format: "float32x4" })) },
      ],
    },
    fragment: { module, entryPoint: "fs_main", targets: [{ format, blend: { color: { srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha", operation: "add" }, alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha", operation: "add" } } }] },
    primitive: { topology: "triangle-list", cullMode: "none" },
    depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: true, depthCompare: "less-equal" },
  });
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.coordinateSystem = THREE.WebGPUCoordinateSystem;
  camera.position.fromArray(fixture.scene.camera.position);
  camera.up.fromArray(fixture.scene.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
  camera.updateMatrixWorld();
  camera.updateProjectionMatrix();
  const identity = new THREE.Matrix4().elements;
  const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse).elements;
  device.queue.writeBuffer(indices, 0, new Uint32Array(fixture.scene.geometry.indices));
  const rows = [];
  try {
    for (const row of fixture.textureCases) {
      const values = new Float32Array([
        ...viewProjection,
        ...identity,
        ...camera.position.toArray(),
        0,
        0,
        0,
        1,
        0,
        ...fixture.scene.fallbackLights.ambient.color,
        fixture.scene.fallbackLights.ambient.intensity,
        0,
        0,
        0,
        0,
        0,
        1,
        fixture.scene.material.emissiveIntensity,
        0,
        ...fixture.scene.material.emissive,
        0,
        0,
        0,
        0,
        1,
      ]);
      if (values.length !== 60) throw new Error("World3dGlobals stride changed");
      device.queue.writeBuffer(uniform, 0, values);
      const packedVertices: number[] = [];
      for (let index = 0; index < fixture.scene.geometry.positions.length / 3; index++)
        packedVertices.push(
          ...fixture.scene.geometry.positions.slice(index * 3, index * 3 + 3),
          ...fixture.scene.geometry.normals.slice(index * 3, index * 3 + 3),
          ...row.vertexColor,
          ...(index === 0 ? [0, 0] : index === 1 ? [1, 0] : [0.5, 1]),
        );
      device.queue.writeBuffer(vertex, 0, new Float32Array(packedVertices));
      device.queue.writeBuffer(
        instance,
        0,
        new Float32Array([...identity, ...row.baseColor.slice(0, 3), row.opacity, row.preserveVertexColor ? 1 : 0, row.emissiveIntensity, 0, 1]),
      );
      const paint = device.createTexture({ size: [1, 1], format: "rgba8unorm-srgb", usage: 0x04 | 0x02 });
      device.queue.writeTexture({ texture: paint }, new Uint8Array(row.texelRgba8), { bytesPerRow: 4 }, [1, 1]);
      const groups = [
        device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: uniform, size: 240 } }] }),
        device.createBindGroup({
          layout: pipeline.getBindGroupLayout(1),
          entries: [
            { binding: 0, resource: shadow.createView() },
            { binding: 1, resource: device.createSampler({ compare: "less-equal" }) },
          ],
        }),
        device.createBindGroup({
          layout: pipeline.getBindGroupLayout(2),
          entries: [
            { binding: 0, resource: paint.createView() },
            { binding: 1, resource: device.createSampler({ minFilter: "nearest", magFilter: "nearest" }) },
          ],
        }),
      ];
      const encoder = device.createCommandEncoder();
      encoder.beginRenderPass({ colorAttachments: [{ view: output.createView({ format: uiFormat }), clearValue: fixture.s2PixelOracle.profile.clearRgba, loadOp: "clear", storeOp: "store" }] }).end();
      const pass = encoder.beginRenderPass({
        colorAttachments: [{ view: output.createView(), loadOp: "load", storeOp: "store" }],
        depthStencilAttachment: { view: depth.createView(), depthClearValue: 1, depthLoadOp: "clear", depthStoreOp: "store", stencilClearValue: 0, stencilLoadOp: "clear", stencilStoreOp: "store" },
      });
      pass.setPipeline(pipeline);
      groups.forEach((group, index) => pass.setBindGroup(index, group));
      pass.setVertexBuffer(0, vertex);
      pass.setVertexBuffer(1, instance);
      pass.setIndexBuffer(indices, "uint32");
      pass.drawIndexed(fixture.scene.geometry.indices.length, 1);
      pass.end();
      encoder.copyTextureToBuffer({ texture: output }, { buffer: readback, bytesPerRow: 256 }, [width, height]);
      device.queue.submit([encoder.finish()]);
      await readback.mapAsync(1);
      const mapped = new Uint8Array(readback.getMappedRange());
      const point = fixture.s2PixelOracle.profile.samplePosition;
      const offset = (height - 1 - point[1]) * 256 + point[0] * 4;
      rows.push({ id: row.id, point, rgba8: [...mapped.slice(offset, offset + 4)] });
      readback.unmap();
      paint.destroy();
    }
    return {
      status: "recorded-browser",
      producer: "Actual production world3d_painted_shader through Chromium WebGPU",
      viewport: [width, height],
      format,
      textureFormat: "rgba8unorm-srgb",
      outputTransfer: "paint bytes recovered from sRGB sampling, then standard lighting and ACES/sRGB output on unorm world view",
      gpu: { vendor: adapter.info.vendor, architecture: adapter.info.architecture, device: adapter.info.device, description: adapter.info.description },
      rows,
    };
  } finally {
    for (const resource of [output, depth, shadow, uniform, instance, vertex, indices, readback]) resource.destroy();
    device.destroy();
  }
}

async function renderWgpuCelebration(input: any) {
  const { fixture, shader } = input;
  const gpu = (navigator as any).gpu;
  if (!gpu) throw new Error("WebGPU is unavailable in the controlled browser context");
  const adapter = await gpu.requestAdapter();
  if (!adapter) throw new Error("WebGPU adapter unavailable");
  const device = await adapter.requestDevice();
  const [width, height] = fixture.scene.viewport;
  const format = "rgba8unorm";
  const uiFormat = "rgba8unorm-srgb";
  const output = device.createTexture({ size: [width, height], format, viewFormats: [uiFormat], usage: 0x10 | 0x01 });
  const depth = device.createTexture({ size: [width, height], format: "depth24plus-stencil8", usage: 0x10 });
  const uniform = device.createBuffer({ size: 256, usage: 0x40 | 0x08 });
  const instance = device.createBuffer({ size: 128, usage: 0x20 | 0x08 });
  const vertex = device.createBuffer({ size: 144, usage: 0x20 | 0x08 });
  const indices = device.createBuffer({ size: 12, usage: 0x10 | 0x08 });
  const readback = device.createBuffer({ size: 256 * height, usage: 0x01 | 0x08 });
  const module = device.createShaderModule({ code: shader });
  const compilation = await module.getCompilationInfo();
  const failures = compilation.messages.filter((row: any) => row.type === "error");
  if (failures.length) throw new Error(JSON.stringify(failures));
  const layouts = [
    {
      arrayStride: 48,
      attributes: [
        { shaderLocation: 0, offset: 0, format: "float32x3" },
        { shaderLocation: 1, offset: 12, format: "float32x3" },
        { shaderLocation: 2, offset: 24, format: "float32x4" },
        { shaderLocation: 9, offset: 40, format: "float32x2" },
      ],
    },
    {
      arrayStride: 128,
      stepMode: "instance",
      attributes: [3, 4, 5, 6, 7, 8, 10, 11].map((shaderLocation, index) => ({ shaderLocation, offset: index * 16, format: "float32x4" })),
    },
  ];
  const pipelines = await Promise.all(
    [false, true].map((transparent) =>
      device.createRenderPipelineAsync({
        layout: "auto",
        vertex: { module, entryPoint: "vs_main", buffers: layouts },
        fragment: {
          module,
          entryPoint: "fs_main",
          targets: [{ format, ...(transparent ? { blend: { color: { srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha", operation: "add" }, alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha", operation: "add" } } } : {}) }],
        },
        primitive: { topology: "triangle-list", cullMode: "none" },
        depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: !transparent, depthCompare: "less-equal" },
      }),
    ),
  );
  const camera = new THREE.PerspectiveCamera(fixture.scene.camera.fov, width / height, 0.01, 100);
  camera.coordinateSystem = THREE.WebGPUCoordinateSystem;
  camera.position.fromArray(fixture.scene.camera.position);
  camera.up.fromArray(fixture.scene.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(fixture.scene.camera.target));
  camera.updateMatrixWorld();
  camera.updateProjectionMatrix();
  const identity = new THREE.Matrix4().elements;
  const viewProjection = new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse).elements;
  const values = new Float32Array([...viewProjection, ...identity, ...Array(28).fill(0)]);
  if (values.length !== 60) throw new Error("World3dGlobals stride changed");
  device.queue.writeBuffer(uniform, 0, values);
  device.queue.writeBuffer(indices, 0, new Uint32Array(fixture.scene.geometry.indices));
  const rows = [];
  try {
    for (const row of fixture.celebration.cases) {
      const vertices: number[] = [];
      for (let index = 0; index < fixture.scene.geometry.positions.length / 3; index++)
        vertices.push(
          ...fixture.scene.geometry.positions.slice(index * 3, index * 3 + 3).map((value: number, axis: number) => value + row.localPosition[axis]),
          ...fixture.scene.geometry.normals.slice(index * 3, index * 3 + 3),
          ...fixture.scene.geometry.colors.slice(index * 4, index * 4 + 4),
          0,
          0,
        );
      device.queue.writeBuffer(vertex, 0, new Float32Array(vertices));
      const model = new THREE.Matrix4().makeTranslation(-row.localPosition[0], -row.localPosition[1], -row.localPosition[2]).elements;
      const stops = fixture.celebration.stops.map((stop: any) => [...stop.linear, 1]);
      const angle = (row.phaseSeconds / fixture.celebration.spinSeconds) * Math.PI * 2;
      device.queue.writeBuffer(instance, 0, new Float32Array([...model, ...stops.flat(), angle, row.opacity, 0, 0]));
      const pipeline = pipelines[row.opacity < 1 ? 1 : 0];
      const group = device.createBindGroup({ layout: pipeline.getBindGroupLayout(0), entries: [{ binding: 0, resource: { buffer: uniform, size: 240 } }] });
      const encoder = device.createCommandEncoder();
      encoder.beginRenderPass({ colorAttachments: [{ view: output.createView({ format: uiFormat }), clearValue: fixture.s2PixelOracle.profile.clearRgba, loadOp: "clear", storeOp: "store" }] }).end();
      const pass = encoder.beginRenderPass({
        colorAttachments: [{ view: output.createView(), loadOp: "load", storeOp: "store" }],
        depthStencilAttachment: { view: depth.createView(), depthClearValue: 1, depthLoadOp: "clear", depthStoreOp: "store", stencilClearValue: 0, stencilLoadOp: "clear", stencilStoreOp: "store" },
      });
      pass.setPipeline(pipeline);
      pass.setBindGroup(0, group);
      pass.setVertexBuffer(0, vertex);
      pass.setVertexBuffer(1, instance);
      pass.setIndexBuffer(indices, "uint32");
      pass.drawIndexed(fixture.scene.geometry.indices.length, 1);
      pass.end();
      encoder.copyTextureToBuffer({ texture: output }, { buffer: readback, bytesPerRow: 256 }, [width, height]);
      device.queue.submit([encoder.finish()]);
      await readback.mapAsync(1);
      const mapped = new Uint8Array(readback.getMappedRange());
      const point = fixture.s2PixelOracle.profile.samplePosition;
      const offset = (height - 1 - point[1]) * 256 + point[0] * 4;
      rows.push({ id: row.id, point, rgba8: [...mapped.slice(offset, offset + 4)] });
      readback.unmap();
    }
    return {
      status: "recorded-browser",
      producer: "Actual production WORLD3D_CELEBRATION_SHADER through Chromium WebGPU",
      viewport: [width, height],
      format,
      outputTransfer: "raw ShaderMaterial color on unorm world view",
      gpu: { vendor: adapter.info.vendor, architecture: adapter.info.architecture, device: adapter.info.device, description: adapter.info.description },
      rows,
    };
  } finally {
    for (const resource of [output, depth, uniform, instance, vertex, indices, readback]) resource.destroy();
    device.destroy();
  }
}

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
          const points = [[-halfWidth, -halfHeight, 0], [halfWidth, -halfHeight, 0], [halfWidth, halfHeight, 0], [-halfWidth, halfHeight, 0]];
          const vertices: number[] = [];
          for (const [from, to] of [[0, 1], [1, 2], [2, 3], [3, 0]]) vertices.push(...points[from], ...color, ...points[to], ...color);
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
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🖼️reference-visual/🔣️.json");
  const output = outputDirectory ? resolve(outputDirectory) : undefined;
  if (output) await mkdir(output, { recursive: true });
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8"));
  const schema = JSON.parse(await readFile(resolve(dirname(fixtureArgument), fixture.$schema), "utf8"));
  const validate = new Ajv({ allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
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
  const javascript = await build.outputs[0].text();
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
      const shaderSource = await readFile(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs"), "utf8");
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
      return { id: row.id, actual: row.rgba8, expected, delta: expected ? row.rgba8.map((value: number, index: number) => value - expected[index]) : null };
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
    console.log("[DEBUG] Recorded " + result.rows.length + " actual " + (webgpu ? "WebGPU" : "Three") + " reference visual pixel samples");
    if (
      fixture.pixelOracle.status === "recorded" &&
      differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (index < 3 ? fixture.pixelOracle.maximumRgbError : 0)))
    )
      throw new Error("Reference visual browser pixels differ from the recorded Three fixture; see persisted report");
  } finally {
    await browser.close();
  }
}

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

async function runGridVisualOracle(repoRoot: string, outputDirectory: string | undefined, webgpu: boolean): Promise<void> {
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🌐️grid-visual/🔣️.json");
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8"));
  const schema = JSON.parse(await readFile(resolve(dirname(fixtureArgument), fixture.$schema), "utf8"));
  const validate = new Ajv({ allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
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
  const javascript = await build.outputs[0].text();
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
      const source = await readFile(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs"), "utf8");
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
      return { id: row.id, point: row.point, actual: row.rgba8, expected, delta: expected ? row.rgba8.map((value: number, index: number) => value - expected[index]) : null };
    });
    result.differences = differences;
    if (output) {
      await writeFile(join(output, "pixels.json"), JSON.stringify(result, null, 2));
      await writeFile(join(output, "report.md"), ["# " + (webgpu ? "Production WGSL" : "Installed Drei") + " Grid Pixels", "", "| Case | Actual RGBA8 | Drei RGBA8 | Delta |", "| --- | --- | --- | --- |", ...differences.map((row: any) => "| " + row.id + " | " + row.actual.join(", ") + " | " + (row.expected?.join(", ") ?? "Unrecorded") + " | " + (row.delta?.join(", ") ?? "Unrecorded") + " |"), ""].join("\n"));
    }
    console.log("[DEBUG] Recorded " + result.rows.length + " actual " + (webgpu ? "production WGSL" : "installed Drei") + " grid pixel samples");
    if (fixture.pixelOracle.status === "recorded" && differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (index < 3 ? fixture.pixelOracle.maximumRgbError : 0)))) throw new Error("Grid pixels differ from the recorded installed-Drei fixture; see persisted report");
  } finally {
    await browser.close();
  }
}

/** 🔬️ Renders one producer without substituting computed values for GPU readback. */
/** 🎨️ What this oracle reads off the committed scene-shading fixture. `JSON.parse` answers `unknown`
 * in this program, so the corpus is named rather than assumed. */
type SceneShadingOracleFixture = {
  readonly $schema: string;
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
  const fixtureArgument = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json");
  const output = outputDirectory ? resolve(outputDirectory) : undefined;
  if (output) await mkdir(output, { recursive: true });
  const playwrightSpecifier = "playwright";
  // 🧭️ The specifier is indirect on purpose (playwright is a dev-only dependency), which erases the
  // module type and makes every callback parameter below an implicit `any`.
  const { chromium } = (await import(playwrightSpecifier)) as typeof import("playwright");
  const fixture = JSON.parse(await readFile(fixtureArgument, "utf8")) as SceneShadingOracleFixture;
  const schema = JSON.parse(await readFile(resolve(dirname(fixtureArgument), fixture.$schema), "utf8"));
  const validate = new Ajv({ allErrors: true }).compile(schema);
  if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
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
  const javascript = await build.outputs[0].text();
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
      const shaderSource = await readFile(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs"), "utf8");
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
      return { id: row.id, actual: row.rgba8, expected, delta: expected ? row.rgba8.map((value: number, index: number) => value - expected[index]) : null };
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
    console.log("[DEBUG] Recorded " + result.rows.length + " actual " + command + " pixel samples");
    if (
      oracle.status === "recorded" &&
      differences.some((row: any) => row.delta === null || row.delta.some((value: number, index: number) => Math.abs(value) > (webgpu && index < 3 ? oracle.profile.maximumRgbError : 0)))
    )
      throw new Error("Browser pixel samples differ from the recorded Three fixture; see persisted report");
  } finally {
    await browser.close();
  }
}

/** 🧪️ Validates the recorded reference and production WGSL on an actual browser GPU. */
export class SceneShadingPixelCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
    if (segments.length === 1 && segments[0] === "reference-visual") {
      await runReferenceVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-reference-visual", "three-reference") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "reference-visual-wgpu") {
      await runReferenceVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-reference-visual", "production-wgpu") : undefined, true);
      return;
    }
    if (segments.length === 1 && segments[0] === "grid-visual") {
      await runGridVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-grid-visual", "installed-drei") : undefined, false);
      return;
    }
    if (segments.length === 1 && segments[0] === "grid-visual-wgpu") {
      await runGridVisualOracle(this.repoRoot, artifacts ? join(artifacts, "world3d-grid-visual", "production-wgpu") : undefined, true);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-reference") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-oracle", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-oracle") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-material-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-wgpu", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-wgpu") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-current-mesh-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-current-mesh", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-current-mesh") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-ordered-mesh-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-ordered-mesh", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-ordered-mesh") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-painted-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-painted", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-painted") : undefined);
      return;
    }
    if (segments.length === 1 && segments[0] === "s2-celebration-wgpu") {
      await runSceneShadingOracle(this.repoRoot, "shading-s2-celebration", artifacts ? join(artifacts, "world3d-scene-shading", "shading-s2-celebration") : undefined);
      return;
    }
    if (segments.length) throw new Error("scene-shading-pixel-check accepts only reference-visual, reference-visual-wgpu, grid-visual, grid-visual-wgpu, s2-reference, s2-material-wgpu, s2-current-mesh-wgpu, s2-ordered-mesh-wgpu, s2-painted-wgpu, or s2-celebration-wgpu");
    for (const command of ["shading-oracle", "shading-wgpu"] as const) await runSceneShadingOracle(this.repoRoot, command, artifacts ? join(artifacts, "world3d-scene-shading", command) : undefined);
  }
}
