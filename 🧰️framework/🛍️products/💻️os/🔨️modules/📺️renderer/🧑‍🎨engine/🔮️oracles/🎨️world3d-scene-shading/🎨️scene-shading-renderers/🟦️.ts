/** 🎨️ The browser-side scene shading renderers: the painted production shader reader, the scene shading renderer and its second scene variant. */
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

export function paintedShaderFromProduction(shader: string): string {
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

export function renderSceneShading(fixture: any) {
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

export function renderSceneShadingS2(fixture: any) {
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
  const standardGeometry = (center:readonly[number,number,number] = [0, 0, 0]) => {
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
