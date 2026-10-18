/** 🧊️ The browser-side WebGPU renderers of the shading, textured and celebration cases. */
/// <reference types="@webgpu/types" />
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

export async function renderWgpuShading(input: any) {
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
  ] as const);
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
        const selectedGroups=groups[pipelineIndex];
        if(!selectedGroups)throw Error("Actual pipeline bind groups required");
        pass.setBindGroup(0, selectedGroups[0]);
        pass.setBindGroup(1, selectedGroups[1]);
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

export async function renderWgpuTextured(input: any) {
  const { fixture, shader } = input;
  const authored = !!input.authored;
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
  const instance = device.createBuffer({ size: authored ? 128 : 112, usage: 0x20 | 0x08 });
  const vertex = device.createBuffer({ size: authored ? 288 : 144, usage: 0x20 | 0x08 });
  const indices = device.createBuffer({ size: 12, usage: 0x10 | 0x08 });
  const readback = device.createBuffer({ size: 256 * height, usage: 0x01 | 0x08 });
  const module = device.createShaderModule({ code: shader });
  const mipPipelines=new Map<string,any>();
  const compilation = await module.getCompilationInfo();
  const failures = compilation.messages.filter((row: any) => row.type === "error");
  if (failures.length) throw new Error(JSON.stringify(failures));
  const makePipeline = (blend: boolean, doubleSided: boolean) => device.createRenderPipelineAsync({
    layout: "auto",
    vertex: {
      module,
      entryPoint: "vs_main",
      buffers: [
        {
          arrayStride: authored ? 96 : 48,
          attributes: [
            { shaderLocation: 0, offset: 0, format: "float32x3" },
            { shaderLocation: 1, offset: 12, format: "float32x3" },
            { shaderLocation: 2, offset: 24, format: "float32x4" },
            ...(authored ? [{shaderLocation:9,offset:40,format:"float32x4"},{shaderLocation:11,offset:56,format:"float32x4"},{shaderLocation:12,offset:72,format:"float32x2"},{shaderLocation:13,offset:80,format:"float32x4"}] : [{shaderLocation:9,offset:40,format:"float32x2"}]),
          ],
        },
        { arrayStride: authored ? 128 : 112, stepMode: "instance", attributes: Array.from({ length: authored ? 8 : 7 }, (_, index) => ({ shaderLocation: index === 7 ? 15 : index === 6 ? 10 : index + 3, offset: index * 16, format: "float32x4" })) },
      ],
    },
    fragment: { module, entryPoint: "fs_main", targets: [{ format, ...(blend ? { blend: { color: { srcFactor: "src-alpha", dstFactor: "one-minus-src-alpha", operation: "add" }, alpha: { srcFactor: "one", dstFactor: "one-minus-src-alpha", operation: "add" } } } : {}) }] },
    primitive: { topology: "triangle-list", cullMode: doubleSided ? "none" : "back" },
    depthStencil: { format: "depth24plus-stencil8", depthWriteEnabled: !blend, depthCompare: "less-equal" },
  });
  const pipelines = await Promise.all([makePipeline(false, false), makePipeline(false, true), makePipeline(true, false), makePipeline(true, true)]);
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
      const pipeline = pipelines[(authored && row.alphaMode !== "BLEND" ? 0 : 2) + (authored && !row.doubleSided ? 0 : 1)];
      device.queue.writeBuffer(indices, 0, new Uint32Array(row.reverse ? [0, 2, 1] : fixture.scene.geometry.indices));
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
          ...(row.uvSets?.[row.textureCoordinates?.baseColorTexture ?? 0]?.slice(index*2,index*2+2) ?? (index === 0 ? [0, 0] : index === 1 ? [1, 0] : [0.5, 1])),
          ...(authored ? ["metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].flatMap(role=>{const set=row.textureCoordinates?.[role] ?? 0;return row.uvSets?.[set]?.slice(index*2,index*2+2) ?? (index === 0 ? [0, 0] : index === 1 ? [1, 0] : [0.5, 1]);}).concat(row.tangents?.slice(index*4,index*4+4) ?? [0,0,0,0]) : []),
        );
      device.queue.writeBuffer(vertex, 0, new Float32Array(packedVertices));
      device.queue.writeBuffer(
        instance,
        0,
        new Float32Array([...identity, ...row.baseColor.slice(0, 3), authored ? (row.alphaMode === "OPAQUE" ? 1 : row.baseColor[3]) : row.opacity, (row.preserveVertexColor ? 1 : 0) + (authored ? 4 + (row.alphaMode === "OPAQUE" ? 8 : 0) + (row.maps.normal ? 16 : 0) : 0), row.emissiveIntensity, row.metallic ?? 0, row.roughness ?? 1, ...(row.emissive ?? [0, 0, 0]), authored && row.alphaMode === "MASK" ? row.alphaCutoff : -1,...(authored ? [...(Array.isArray(row.normalScale)?row.normalScale:[row.normalScale ?? 1,row.normalScale ?? 1]),row.occlusionStrength ?? 1,0] : [])]),
      );
      const maps = (authored ? ["baseColor", "metallicRoughness", "normal", "occlusion", "emissive"] : ["baseColor"]).map((role, index) => {
        const size=row.mapSizes?.[role] ?? [1,1];const field=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"][index];const sampler=row.textureSamplers?.[field!] ?? {};const levels=input.mipShader && (sampler.minFilter ?? 9728)>=9984 ? 1+Math.floor(Math.log2(Math.max(...size))) : 1;const format=!authored || index===0 || index===4?"rgba8unorm-srgb":"rgba8unorm";const paint = device.createTexture({ size, format,mipLevelCount:levels, usage: 0x04 | 0x02 | 0x10 });
        device.queue.writeTexture({ texture: paint }, new Uint8Array(authored ? row.maps[role] ?? [255, 255, 255, 255] : row.texelRgba8), { bytesPerRow: size[0]*4 }, size);
        if(levels>1){let pipeline=mipPipelines.get(format);if(!pipeline){const module=device.createShaderModule({code:input.mipShader});pipeline=device.createRenderPipeline({layout:"auto",vertex:{module,entryPoint:"vs_main"},fragment:{module,entryPoint:"fs_main",targets:[{format}]},primitive:{topology:"triangle-list"}});mipPipelines.set(format,pipeline);}for(let level=1;level<levels;level++){const height=Math.max(1,size[1]>>level);for(let y=0;y<height;y+=8){const encoder=device.createCommandEncoder();const pass=encoder.beginRenderPass({colorAttachments:[{view:paint.createView({baseMipLevel:level,mipLevelCount:1}),loadOp:"load",storeOp:"store"}]});pass.setPipeline(pipeline);pass.setBindGroup(0,device.createBindGroup({layout:pipeline.getBindGroupLayout(0),entries:[{binding:0,resource:paint.createView({baseMipLevel:level-1,mipLevelCount:1})},{binding:1,resource:device.createSampler({minFilter:"linear",magFilter:"linear"})}]}));pass.setScissorRect(0,y,Math.max(1,size[0]>>level),Math.min(8,height-y));pass.draw(3);pass.end();device.queue.submit([encoder.finish()]);}}}
        return paint;
      });
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
          entries: maps.flatMap((paint, index) => [
            { binding: index * 2, resource: paint.createView() },
            { binding:index*2+1,resource:device.createSampler((()=>{const field=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"][index];const sampler=row.textureSamplers?.[field!] ?? {};const wrap=(mode:number)=>mode===10497?"repeat":mode===33648?"mirror-repeat":"clamp-to-edge";const min=sampler.minFilter ?? 9728;return {addressModeU:wrap(sampler.wrapS),addressModeV:wrap(sampler.wrapT),magFilter:sampler.magFilter===9729?"linear":"nearest",minFilter:[9729,9985,9987].includes(min)?"linear":"nearest",mipmapFilter:[9986,9987].includes(min)?"linear":"nearest",lodMaxClamp:min>=9984?32:0};})())},
          ]),
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
      maps.forEach((paint) => paint.destroy());
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

export async function renderWgpuCelebration(input: any) {
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
