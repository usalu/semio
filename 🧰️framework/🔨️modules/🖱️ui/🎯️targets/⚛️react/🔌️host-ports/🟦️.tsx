import { paintIconDrawing, prepareIconSvgMeshes } from "./🖋️drawing/🟦️.ts";
import {applyIconRenderTransform} from "./🧊️transform/🟦️.ts";
import { Canvas as ThreeCanvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei/core/OrbitControls.js";
import { Grid } from "@react-three/drei/core/Grid.js";
import { sceneHostPort, reactHostPort, flowHostPort, type ReactHostPort, type FlowHostPort, type SceneHostPort, setReactHostPort, setFlowHostPort, setSceneHostPort } from "@semio-tech/ui/elements/Ports";
import type { IconRenderPort, IconRenderRequest, IconRenderCamera, IconRenderResult, IconRenderShape } from "@semio-tech/ui-styling";
import { ephemeralMap } from "@semio-tech/framework-transient";
import * as THREE from "three";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { clone as cloneSkeleton } from "three/examples/jsm/utils/SkeletonUtils.js";
import { SVGRenderer } from "three/examples/jsm/renderers/SVGRenderer.js";
import * as React from "react";
import { cn } from "@semio-tech/ui/modules/class-name-composition";
import { useLevel } from "@semio-tech/ui/elements/Surface";
import { windowFrameChipLevel } from "@semio-tech/ui/elements/WindowChrome";
import { glassClass } from "@semio-tech/ui/modules/surface-presentation";

/** 🧊️ Host surface for three.js / R3F (implemented by 🔌️Adapters). */
export interface ThreeHostPort {
  readonly canvas: typeof ThreeCanvas;
  readonly drei: { OrbitControls: typeof OrbitControls; Grid: typeof Grid };
}
// 🧱️core-extracted: reactHostPort's `let` binding + default value now live in
// 🧱️elements/🔌️Ports/🟦️.tsx (imported above, in the 🔌️Ports region) — reassignment below
// goes through the imported setReactHostPort() setter, since an ES import binding can't be assigned to
// directly.

/** 🔌️ Default R3F host port wired to fiber/drei adapters. */
export let threeHostPort: ThreeHostPort = {
  canvas: sceneHostPort.fiber.canvas,
  drei: { OrbitControls: sceneHostPort.drei.OrbitControls, Grid: sceneHostPort.drei.Grid },
};

const defaultReactHostPort = reactHostPort;
const defaultFlowHostPort = flowHostPort;
const defaultThreeHostPort = threeHostPort;
const defaultSceneHostPort = sceneHostPort;

/** 🖥️ Overrides for {@link configureHostPorts}; an omitted/`undefined` key is left untouched (whatever port is currently installed keeps running). */
export type HostPortOverrides = Partial<{
  readonly react: ReactHostPort;
  readonly flow: FlowHostPort;
  readonly three: ThreeHostPort;
  readonly scene: SceneHostPort;
  readonly iconRender: IconRenderPort;
}>;

/** ☁️ Swaps one or more host ports; ESM importers cannot assign `export let` bindings directly,
 * so this is the only way to inject a test double or alternate adapter (e.g. Storybook's `withRenderer`
 * decorator) before a story renders. Ports are page-global (one React per page), so a merge — only the
 * keys you pass are touched, everything else keeps whatever is currently installed — is required rather
 * than a full reset: with two shells configuring ports independently (or nested calls within one shell),
 * an unconditional reset-to-default on an unrelated key would clobber the other caller's still-active
 * override. Returns a restore function that puts back exactly what was installed before this call. */
export function configureHostPorts(overrides: HostPortOverrides): () => void {
  const previous: HostPortOverrides = { react: reactHostPort, flow: flowHostPort, three: threeHostPort, scene: sceneHostPort, iconRender: iconRenderPort };
  if (overrides.react !== undefined) setReactHostPort(overrides.react);
  if (overrides.flow !== undefined) setFlowHostPort(overrides.flow);
  if (overrides.three !== undefined) threeHostPort = overrides.three;
  if (overrides.scene !== undefined) setSceneHostPort(overrides.scene);
  if (overrides.iconRender !== undefined) iconRenderPort = overrides.iconRender;
  return () => {
    if (overrides.react !== undefined) setReactHostPort(previous.react ?? defaultReactHostPort);
    if (overrides.flow !== undefined) setFlowHostPort(previous.flow ?? defaultFlowHostPort);
    if (overrides.three !== undefined) threeHostPort = previous.three ?? defaultThreeHostPort;
    if (overrides.scene !== undefined) setSceneHostPort(previous.scene ?? defaultSceneHostPort);
    if (overrides.iconRender !== undefined) iconRenderPort = previous.iconRender ?? defaultIconRenderPort;
  };
}

const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;
const GLB_RETRY_DELAYS_MS = [200, 800] as const;

type IconGlbLoad = {
  controller: AbortController;
  group: Promise<THREE.Group>;
  consumers: number;
  settled: boolean;
};

const ICON_RENDER_GLB_CACHE = ephemeralMap<string, IconGlbLoad>("framework.modules.ui.packages.typescript.targets.react.index.tsx.ICON_RENDER_GLB_CACHE");
const ICON_RENDER_MESH_OUTLINE_USER_DATA_KEY = "semio.iconRender.meshOutline";
const ICON_RENDER_EDGE_GEOMETRY_CACHE = new WeakMap<THREE.BufferGeometry, THREE.EdgesGeometry>();

/** 🌫️ Empty/`transparent` lets the host surface show through — mirrors world-3d environment JSON. */
export function isTransparentIconBackground(background?: string): boolean {
  return !background || background === "transparent";
}

export function resolveIconRenderStroke(stroke?: string): string | null {
  if (!stroke) return "#000000";
  const normalized = stroke.trim().toLowerCase();
  return normalized === "transparent" || normalized === "none" ? null : stroke;
}

function iconRenderEdgeGeometry(geometry: THREE.BufferGeometry): THREE.EdgesGeometry {
  let edges = ICON_RENDER_EDGE_GEOMETRY_CACHE.get(geometry);
  if (!edges) {
    edges = new THREE.EdgesGeometry(geometry);
    ICON_RENDER_EDGE_GEOMETRY_CACHE.set(geometry, edges);
  }
  return edges;
}

function applyIconMeshEdgeBorders(root: THREE.Object3D, borderColor: string): void {
  const targets: THREE.Mesh[] = [];
  root.traverse((object) => {
    if (!(object instanceof THREE.Mesh)) return;
    if (!object.geometry || object.children.some((child) => child.userData[ICON_RENDER_MESH_OUTLINE_USER_DATA_KEY])) return;
    targets.push(object);
  });
  for (const object of targets) {
    const outline = new THREE.LineSegments(iconRenderEdgeGeometry(object.geometry), new THREE.LineBasicMaterial({ color: new THREE.Color(borderColor) }));
    outline.userData[ICON_RENDER_MESH_OUTLINE_USER_DATA_KEY] = true;
    outline.scale.setScalar(1.001);
    object.add(outline);
  }
}

/** ☀️ Sun position on a sphere from azimuth/elevation degrees, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system. */
export function sunPositionFromAzimuthElevation(azimuthDeg: number, elevationDeg: number, distance = 120): [number, number, number] {
  const az = (azimuthDeg * Math.PI) / 180;
  const el = (elevationDeg * Math.PI) / 180;
  return [Math.cos(el) * Math.cos(az) * distance, Math.cos(el) * Math.sin(az) * distance, Math.sin(el) * distance];
}

class GlbTransportFailure extends Error {
  constructor(message: string, readonly retryable: boolean) {
    super(message);
  }
}

async function readGlbResponse(url: URL, signal: AbortSignal): Promise<ArrayBuffer> {
  try {
    const response = await fetch(url, { signal });
    if (!response.ok) {
      void response.body?.cancel().catch(() => {});
      const retryable = [408, 429, 500, 502, 503, 504].includes(response.status);
      throw new GlbTransportFailure(`GLB request returned HTTP ${response.status}`, retryable);
    }
    const bytes = await response.arrayBuffer();
    const length = response.headers.get("content-length");
    const encoding = response.headers.get("content-encoding");
    if (length !== null && (encoding === "identity" || response.type === "basic" && encoding === null) && bytes.byteLength < Number(length)) {
      throw new GlbTransportFailure("GLB response ended before its declared length", true);
    }
    return bytes;
  } catch (error) {
    signal.throwIfAborted();
    if (error instanceof TypeError) throw new GlbTransportFailure(error.message, true);
    throw error;
  }
}

function waitForGlbRetry(delayMs: number, signal: AbortSignal): Promise<void> {
  signal.throwIfAborted();
  return new Promise((resolve, reject) => {
    const abort = () => {
      clearTimeout(timer);
      reject(signal.reason);
    };
    const timer = setTimeout(() => {
      signal.removeEventListener("abort", abort);
      resolve();
    }, delayMs);
    signal.addEventListener("abort", abort, { once: true });
  });
}

async function fetchGlbBytes(url: URL, signal: AbortSignal): Promise<ArrayBuffer> {
  const http = url.protocol === "https:" || url.protocol === "http:";
  for (let attempt = 0; ; attempt++) {
    try {
      return await readGlbResponse(url, signal);
    } catch (error) {
      const delay = GLB_RETRY_DELAYS_MS[attempt];
      if (!http || delay === undefined || !(error instanceof GlbTransportFailure) || !error.retryable) throw error;
      await waitForGlbRetry(delay, signal);
    }
  }
}

async function fetchGlbGroup(url: string, signal: AbortSignal): Promise<THREE.Group> {
  signal.throwIfAborted();
  const loader = new GLTFLoader();
  const resourcePath = THREE.LoaderUtils.extractUrlBase(url);
  const source = new URL(loader.manager.resolveURL(url), document.baseURI);
  loader.manager.itemStart(url);
  try {
    const bytes = await fetchGlbBytes(source, signal);
    signal.throwIfAborted();
    const gltf = await loader.parseAsync(bytes, resourcePath);
    signal.throwIfAborted();
    const root = new THREE.Group();
    const frame = new THREE.Group();
    frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
    frame.add(gltf.scene);
    root.add(frame);
    return root;
  } catch (error) {
    if (!signal.aborted) loader.manager.itemError(url);
    throw error;
  } finally {
    loader.manager.itemEnd(url);
  }
}

function loadGlbGroup(url: string): IconGlbLoad {
  const cached = ICON_RENDER_GLB_CACHE.get(url);
  if (cached) return cached;
  const controller = new AbortController();
  const load: IconGlbLoad = { controller, group: Promise.resolve().then(() => fetchGlbGroup(url, controller.signal)), consumers: 0, settled: false };
  ICON_RENDER_GLB_CACHE.set(url, load);
  void load.group.then(
    () => { load.settled = true; },
    () => {
      load.settled = true;
      if (ICON_RENDER_GLB_CACHE.get(url) === load) ICON_RENDER_GLB_CACHE.delete(url);
    },
  );
  return load;
}

function waitForGlbGroup(url: string, signal?: AbortSignal): Promise<THREE.Group> {
  signal?.throwIfAborted();
  const load = loadGlbGroup(url);
  load.consumers++;
  return new Promise((resolve, reject) => {
    let released = false;
    const release = () => {
      if (released) return;
      released = true;
      signal?.removeEventListener("abort", abort);
      if (--load.consumers !== 0 || load.settled) return;
      if (ICON_RENDER_GLB_CACHE.get(url) === load) ICON_RENDER_GLB_CACHE.delete(url);
      load.controller.abort();
    };
    const abort = () => {
      release();
      reject(signal?.reason);
    };
    signal?.addEventListener("abort", abort, { once: true });
    void load.group.then(group => {
      release();
      resolve(group);
    }, error => {
      release();
      reject(error);
    });
  });
}

function applyIconMaterial(group: THREE.Object3D, material: IconRenderRequest["material"], raster: boolean): void {
  if (material) {
    group.traverse((obj) => {
      if (!(obj instanceof THREE.Mesh)) return;
      const mat = new THREE.MeshStandardMaterial({
        color: material.color ?? "#9aa0ab",
        metalness: material.metalness ?? 0,
        roughness: material.roughness ?? 1,
      });
      if (material.emissive) {
        mat.emissive.set(material.emissive);
        mat.emissiveIntensity = material.emissiveIntensity ?? 1;
      }
      obj.material = mat;
      obj.castShadow = true;
      obj.receiveShadow = true;
    });
  }
  const stroke = resolveIconRenderStroke(material?.stroke);
  if (raster && stroke) {
    applyIconMeshEdgeBorders(group, stroke);
  }
}

function buildIconScene(request: IconRenderRequest, model: THREE.Group): THREE.Scene {
  const scene = new THREE.Scene();
  if (request.format === "png" && !isTransparentIconBackground(request.background)) {
    scene.background = new THREE.Color(request.background);
  }
  scene.add(model);
  applyIconMaterial(scene, request.material, request.format === "png");
  const ambient = new THREE.AmbientLight(request.lights.ambientColor, request.lights.ambientIntensity);
  const sunPos = sunPositionFromAzimuthElevation(request.lights.sunAzimuth, request.lights.sunElevation);
  const sun = new THREE.DirectionalLight(request.lights.sunColor, request.lights.sunIntensity);
  sun.position.set(sunPos[0], sunPos[1], sunPos[2]);
  if (request.shadowEnabled) {
    sun.castShadow = true;
    sun.shadow.mapSize.set(1024, 1024);
  }
  scene.add(ambient, sun);
  return scene;
}

/** 📷️ Whether an icon shot uses parallel projection — mirrors world-3d `camera.projection`. */
export function iconRenderCameraIsOrthographic(camera: IconRenderCamera): boolean {
  return camera.projection === "orthographic";
}

/** 📏️ Orthographic zoom for a drei/R3F pixel frustum (`left = -width/2`, …) — twin of `worldProjectionOrthoZoom`. */
export function iconRenderOrthoZoomForRadius(radius: number, width: number, height: number, padding = 1.25): number {
  const padded = Math.max(radius * Math.max(padding, 1), 0.5);
  const zoomX = width * 0.5 / padded;
  const zoomY = height * 0.5 / padded;
  return Math.max(Math.min(zoomX, zoomY), 1e-3);
}

/** 👁️ Perspective eye distance so a bounding sphere fits the shot frustum — twin of `world3dFrameDistanceForRadius`. */
export function iconRenderPerspectiveDistanceForRadius(radius: number, fovDeg: number, aspect: number, padding = 1.25): number {
  const vertical = (fovDeg * Math.PI) / 360;
  const horizontal = Math.atan(Math.tan(vertical) * aspect);
  const half = Math.min(vertical, horizontal);
  return Math.max((Math.max(radius, 1e-4) / Math.sin(half)) * Math.max(padding, 1), radius + 0.1);
}

/** 🎯️ `request.camera`, or — with `request.fit` enabled — the same viewing direction re-targeted at
 * the model's bounding-sphere centre and backed off so the sphere fills the frame (`padding` ≥ 1). */
export function iconRenderCameraPose(request: IconRenderRequest, model: THREE.Object3D): IconRenderCamera {
  const camera = request.camera;
  if (!request.fit?.enabled) return camera;
  const sphere = new THREE.Box3().setFromObject(model).getBoundingSphere(new THREE.Sphere());
  if (!Number.isFinite(sphere.radius) || sphere.radius <= 0) return camera;
  const padding = request.fit.padding ?? 1.25;
  const direction = new THREE.Vector3(camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]);
  const length = direction.length();
  if (length <= 1e-6) direction.set(1, -1, 0.85);
  direction.normalize();
  const target: [number, number, number] = [sphere.center.x, sphere.center.y, sphere.center.z];
  if (iconRenderCameraIsOrthographic(camera)) {
    const distance = Math.max(length, sphere.radius + 0.1);
    return {
      ...camera,
      position: [target[0] + direction.x * distance, target[1] + direction.y * distance, target[2] + direction.z * distance],
      target,
      zoom: iconRenderOrthoZoomForRadius(sphere.radius, request.width, request.height, padding),
    };
  }
  const halfFov = ((camera.fov ?? 50) * Math.PI) / 360;
  const effectiveFov = (360 / Math.PI) * Math.atan(Math.tan(halfFov) / camera.zoom);
  const distance = iconRenderPerspectiveDistanceForRadius(sphere.radius, effectiveFov, request.width / request.height, padding);
  return { ...camera, position: [target[0] + direction.x * distance, target[1] + direction.y * distance, target[2] + direction.z * distance], target, zoom: camera.zoom };
}

/** 🔨️ Builds the three.js camera for an icon shot — perspective or orthographic to match the scene lane. */
export function buildIconCamera(request: IconRenderRequest, pose: IconRenderCamera = request.camera): THREE.Camera {
  const camera = iconRenderCameraIsOrthographic(pose)
    ? new THREE.OrthographicCamera(request.width / -2, request.width / 2, request.height / 2, request.height / -2, 0.1, 10_000)
    : new THREE.PerspectiveCamera(pose.fov ?? 50, request.width / request.height, 0.1, 10_000);
  camera.up.fromArray(pose.up ?? [0, 0, 1]);
  if (camera.up.lengthSq() === 0) camera.up.set(0, 1, 0);
  else camera.up.normalize();
  camera.position.set(...pose.position);
  const right = new THREE.Vector3(...pose.target).sub(camera.position).normalize().cross(camera.up);
  if (right.lengthSq() <= 1e-8) {
    if (Math.abs(camera.up.z) > 0.5) camera.up.set(0, 1, 0);
    else camera.up.set(0, 0, 1);
  }
  camera.lookAt(...pose.target);
  camera.zoom = pose.zoom;
  camera.updateProjectionMatrix();
  return camera;
}
/** 🧼️ Strips three.js SVGRenderer's default opaque clear color when the shot background is transparent. */
export function finalizeIconSvgMarkup(svgMarkup: string, options: { readonly background?: string }): string {
  if (!isTransparentIconBackground(options.background)) return svgMarkup;
  return svgMarkup.replace(/\sstyle="background-color:[^"]*"/gi, "");
}

async function renderIconSvg(scene: THREE.Scene, camera: THREE.Camera, request: IconRenderRequest, signal?: AbortSignal): Promise<IconRenderResult> {
  const geometry = await prepareIconSvgMeshes(scene, signal);
  try {
    const renderer = new SVGRenderer();
    renderer.autoClear = false;
    renderer.setSize(request.width, request.height);
    if (request.drawing?.fill !== false && request.drawing?.fillOpacity !== 0) renderer.render(scene, camera);
    const svgElement = renderer.domElement;
    await paintIconDrawing(svgElement, scene, camera, request, iconRenderEdgeGeometry, resolveIconRenderStroke(request.material?.stroke), signal);
    const svgMarkup = new XMLSerializer().serializeToString(svgElement);
    const dataUrl = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svgMarkup)}`;
    return { dataUrl, svgMarkup };
  } finally {
    for (const mesh of geometry) mesh.dispose();
  }
}

async function renderIconPng(scene: THREE.Scene, camera: THREE.Camera, width: number, height: number, shadowEnabled: boolean): Promise<IconRenderResult> {
  const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, preserveDrawingBuffer: true });
  renderer.setSize(width, height);
  renderer.shadowMap.enabled = shadowEnabled;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  renderer.render(scene, camera);
  const dataUrl = renderer.domElement.toDataURL("image/png");
  renderer.dispose();
  return { dataUrl };
}

/** ⭕️ Clips rendered SVG markup to an axis-aligned ellipse inscribed in the shot bounds. */
export function clipIconSvgMarkupToEllipse(svgMarkup: string, width: number, height: number): string {
  const clipId = "semio-icon-ellipse-clip";
  if (svgMarkup.includes(`id="${clipId}"`)) return svgMarkup;
  const openMatch = svgMarkup.match(/^<svg\b([^>]*?)(\/?)>/i);
  const closeIdx = svgMarkup.lastIndexOf("</svg>");
  if (!openMatch || (closeIdx < 0 && openMatch[2] !== "/")) return svgMarkup;
  const openingAttributes=openMatch[1];if(openingAttributes===undefined)return svgMarkup;
  const parsed=openingAttributes.match(/\bviewBox\s*=\s*["']([^"']+)["']/i)?.[1]?.trim().split(/[\s,]+/).map(Number);
  let bounds:readonly[number,number,number,number]=[0,0,width,height];
  if(parsed?.length===4){const[x,y,w,h]=parsed;if(x!==undefined&&y!==undefined&&w!==undefined&&h!==undefined&&[x,y,w,h].every(Number.isFinite)&&w>0&&h>0)bounds=[x,y,w,h];}
  const[x,y,boxWidth,boxHeight]=bounds;
  const clipDef = `<clipPath id="${clipId}"><ellipse cx="${x + boxWidth / 2}" cy="${y + boxHeight / 2}" rx="${boxWidth / 2}" ry="${boxHeight / 2}"/></clipPath>`;
  let background = "";
  const attributes=openingAttributes.replace(/\sstyle\s*=\s*(["'])(.*?)\1/i, (_attribute, quote: string, value: string) => {
    const style = value.replace(/(?:^|;)\s*background-color\s*:\s*([^;]+);?/i, (_declaration, color: string) => {
      background = color.trim().replaceAll("&", "&amp;").replaceAll('"', "&quot;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
      return ";";
    }).replace(/^;+|;+$/g, "").trim();
    return style ? ` style=${quote}${style}${quote}` : "";
  });
  let body = openMatch[2] === "/" ? "" : svgMarkup.slice(openMatch[0].length, closeIdx);
  let defs = "";
  const defsMatch = body.match(/^<defs[^>]*>[\s\S]*?<\/defs>/i);
  if (defsMatch) {
    defs = defsMatch[0].replace(/<defs([^>]*)>/i, `<defs$1>${clipDef}`);
    body = body.slice(defsMatch[0].length);
  } else {
    defs = `<defs>${clipDef}</defs>`;
  }
  const fill = background ? `<rect x="${x}" y="${y}" width="${boxWidth}" height="${boxHeight}" fill="${background}"/>` : "";
  return `<svg${attributes}>${defs}<g clip-path="url(#${clipId})">${fill}${body}</g></svg>`;
}

async function clipIconPngDataUrlToEllipse(dataUrl: string, width: number, height: number): Promise<string> {
  if (typeof document === "undefined") return dataUrl;
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => {
      const canvas = document.createElement("canvas");
      canvas.width = width;
      canvas.height = height;
      const context = canvas.getContext("2d");
      if (!context) {
        reject(new Error("2d canvas unavailable"));
        return;
      }
      context.clearRect(0, 0, width, height);
      context.beginPath();
      context.ellipse(width / 2, height / 2, width / 2, height / 2, 0, 0, Math.PI * 2);
      context.clip();
      context.drawImage(image, 0, 0, width, height);
      resolve(canvas.toDataURL("image/png"));
    };
    image.onerror = () => reject(new Error("Failed to load icon png for ellipse mask"));
    image.src = dataUrl;
  });
}

async function applyIconRenderShape(result: IconRenderResult, shape: IconRenderShape | undefined, width: number, height: number): Promise<IconRenderResult> {
  if (!shape || shape === "rectangle") return result;
  if (result.svgMarkup) {
    const svgMarkup = clipIconSvgMarkupToEllipse(result.svgMarkup, width, height);
    return { dataUrl: `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svgMarkup)}`, svgMarkup };
  }
  const dataUrl = await clipIconPngDataUrlToEllipse(result.dataUrl, width, height);
  return { dataUrl };
}

/** 🖌️ Default three.js-backed icon render port (SVGRenderer + WebGL PNG), reassignable via {@link configureHostPorts}. */
export let iconRenderPort: IconRenderPort = {
  async render(request: IconRenderRequest, signal?: AbortSignal): Promise<IconRenderResult> {
    const loaded = await waitForGlbGroup(request.assetUrl, signal);
    signal?.throwIfAborted();
    const model = cloneSkeleton(loaded) as THREE.Group;
    applyIconRenderTransform(model,request.transform);
    const scene = buildIconScene(request, model);
    const camera = buildIconCamera(request, iconRenderCameraPose(request, model));
    const result = request.format === "svg" ? await renderIconSvg(scene, camera, request, signal) : await renderIconPng(scene, camera, request.width, request.height, request.shadowEnabled === true);
    signal?.throwIfAborted();
    return applyIconRenderShape(result, request.shape, request.width, request.height);
  },
};

const defaultIconRenderPort = iconRenderPort;

/** 🖼️ Aspect-ratio style keeping a W×H frame inside its container. */
export function iconShotFrameStyle(width: number, height: number): React.CSSProperties {
  const landscape = width >= height;
  return {
    aspectRatio: `${width} / ${height}`,
    maxHeight: "100%",
    maxWidth: "100%",
    width: landscape ? "100%" : "auto",
    height: landscape ? "auto" : "100%",
  };
}

/** 🎞️ Frame mask class for an icon shot shape. */
export function iconShotFrameClass(shape: IconRenderShape): string {
  return shape === "ellipse" ? "rounded-[50%]" : "rounded-none";
}

/** 🎭️ Centered shot guide with a shape mask and inset dimensions. */
export function IconShotFrame({
  width,
  height,
  shape = "rectangle",
  className,
  background,
  badge = true,
  children,
}: {
  readonly width: number;
  readonly height: number;
  readonly shape?: IconRenderShape;
  readonly className?: string;
  readonly background?: string;
  readonly badge?: boolean;
  readonly children?: React.ReactNode;
}): React.ReactNode {
  const chipLevel = windowFrameChipLevel(useLevel());
  const ellipse = shape === "ellipse";
  return (
    <div className={cn("pointer-events-none absolute inset-0 flex items-center justify-center", className)}>
      <div
        className={cn("relative box-border overflow-hidden border-solid border-[color:var(--border-emphasized-color)]", iconShotFrameClass(shape))}
        data-icon-shot-frame
        data-icon-shot-shape={shape}
        style={{
          ...iconShotFrameStyle(width, height),
          ...(background ? { background } : {}),
          borderWidth: "var(--stroke-hairline)",
        }}
      >
        {children}
        {badge ? (
          <span
            data-glass-chip=""
            data-level={chipLevel}
            className={cn("pointer-events-none absolute bottom-0 max-w-full border-solid border-[color:var(--border-normal-color)] px-single font-mono text-tiny text-foreground", glassClass, ellipse ? "inset-x-0 flex justify-center pb-single" : "end-0")}
            style={{ borderBlockStartWidth: "var(--stroke-hairline)", borderInlineStartWidth: ellipse ? undefined : "var(--stroke-hairline)" }}
          >
            <span className={cn("block truncate", ellipse && "max-w-[60%]")}>{width}×{height}</span>
          </span>
        ) : null}
      </div>
    </div>
  );
}
