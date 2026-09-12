// #region 🧲️Header

// 💻️ framework/ui/js/react/index.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Shared export surface for elements ui primitives.

// #endregion 🧲️Header

// #region 🔌️Adapters

import type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, Node as FlowNode, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import {
  activeUiTheme,
  builtinUiThemes,
  domSizePx,
  elementStateAttributes,
  parseUiTheme,
  readSizeVarPx,
  resolveColorHex,
  resolveElementFillKind,
  resolveElementState,
  resolveSemanticColorHex,
  resolveSpatialAxisColors,
  semanticVar,
  semioTheme,
  serializeUiTheme,
  setActiveUiTheme,
  sizeVar,
  STYLING_COMPACT_ROOT_PX,
  STYLING_DOM,
  subscribeActiveUiTheme,
  themeColorVar,
  tokenVar,
  uiSpacingPx,
  type ElementFillKind,
  type UiElementState,
  type UiState,
  type UiStatus,
  type UiTheme,
} from "@semio-tech/ui-styling";
// 🚧️W3-interim: explicit re-export so 🧱️elements/<Element>/ leaf files (which only ever import from
// this barrel, never straight from "@semio-tech/ui-styling") can resolve these — a bare `import {...}
// from "@semio-tech/ui-styling"` above does not itself make a name part of this module's public surface.
export { resolveColorHex, resolveSemanticColorHex, resolveSpatialAxisColors, themeColorVar, tokenVar, uiSpacingPx };
import {
  CANVAS_HOVER_SOURCE_CANVAS,
  CANVAS_HOVER_SOURCE_PICK_MENU,
  canvasHoverFocusFromTarget,
  canvasPickTargetKey,
  effectiveActionArgs,
  missingRequiredArgs,
  SHELL_LOCALES,
  SHELL_TERMINOLOGIES,
  type ActionArgDef,
  type CanvasHoverFocus,
  type CanvasPickRequest,
  type CanvasPickTarget,
  type DialogDefinition,
  type DockSkeleton,
  type DockTabSkeleton,
  type IntroductionCursor,
  type IntroductionDefinition,
  type IntroductionDemonstration,
  type IntroductionGesture,
  type IntroductionKeyModifier,
  type IntroductionLogo,
  type IntroductionPlacement,
  type IntroductionPoint,
  type IntroductionStepDefinition,
  type ShellLocale,
  type ShellTerminology,
  type StoragePort,
  createBrowserStoragePort,
  createMemoryStoragePort,
  panelTabFirstDraggableElementId,
  panelTabElementId,
  windowElementId,
  pickMostSpecificCanvasTarget,
  sortCanvasPickTargetsGeneralFirst,
  START_TUTORIAL_ACTION_ID,
  RECORD_TUTORIAL_ACTION_ID,
  TUTORIAL_CONVERGE_MS,
  type TutorialDefinition,
  type TutorialChapter,
  type TutorialUiSnapshot,
  type TutorialUiChange,
  type TutorialCameraKeyframe,
  type TutorialCameraState,
  type TutorialEasing,
  type TutorialEvent,
  type TutorialArtifactEvent,
  type TutorialGestureCue,
  type TutorialOverlayRect,
  type WindowLayout,
  ephemeralBox,
  ephemeralMap,
  ephemeralSet,
} from "@semio-tech/framework";
import i18next from "i18next";
import * as React from "react";
import * as ResizablePrimitive from "react-resizable-panels";
import * as THREE from "three";
// 🚧️W8-interim: explicit re-export of the raw React/react-dom/three runtime values and types that
// s plugins previously imported straight from those packages — plugins depend on this package
// already, so they no longer need "react"/"react-dom"/"three" in their own `dependencies`.
export { Fragment, createContext, useCallback, useContext, useEffect, useLayoutEffect, useMemo, useRef, useState, act } from "react";
export type { CSSProperties, FC, KeyboardEvent, MouseEvent as ReactMouseEvent, ReactNode, RefObject } from "react";
export { createRoot } from "react-dom/client";
export type { Root } from "react-dom/client";
export type { BufferGeometry, Camera as ThreeCamera, Group, MeshStandardMaterial, Object3D, Ray, Scene as ThreeScene, Vector3 } from "three";

import { closestCenter, DndContext, DragEndEvent, PointerSensor, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
import { SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Clone, Edges, GizmoHelper, GizmoViewport, Grid, Line as DreiLine, OrbitControls, OrthographicCamera, Outlines, PerspectiveCamera, Text as DreiText, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas as ThreeCanvas, createPortal as r3fCreatePortal, ThreeEvent, useFrame, useStore, useThree } from "@react-three/fiber";
import { rankFuzzyItems, type FuzzySearchField, type FuzzySearchOptions, type FuzzySearchResult } from "../../../../🔨️modules/🔎️fuzzy-ranking/🟦️.ts";
import {
  applyNodeChanges,
  Background,
  BackgroundVariant,
  BaseEdge,
  ConnectionMode,
  getBezierPath,
  Handle,
  MiniMap,
  Position,
  ReactFlow,
  ReactFlowProvider,
  SelectionMode,
  useInternalNode,
  useReactFlow,
  useStoreApi,
  ViewportPortal,
} from "@xyflow/react";
import { ICONS, assertUniqueIconConceptAssignments, isIconName, resolveCatalogIconSvgFromTheme, shortcodeCatalogKey, shortcodeEmoji, type IconName } from "@semio-tech/assets";
import { isMetabolismIconName, METABOLISM_ICONS, resolveMetabolismIconSvgFromTheme, type MetabolismIconName } from "@semio-tech/assets";
export type { IconName, MetabolismIconName };
import { createPortal } from "react-dom";
import { createRoot, type Root } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import { I18nextProvider, initReactI18next, useTranslation } from "react-i18next";
// 🕹️wave-0: imported directly from the module's own source (not via `@semio-tech/framework`) — the
// `🛂️manifest` module already re-exports a same-named, owned-schema-generated `MergeMode`/`SelectionMode`
// family through that barrel, so a second barrel export of the hand-written mirror would collide.
import { type MergeMode } from "../../../../../🕹️interaction/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🔌️Ports
// 🧱️core-extracted: ReactHostPort + reactHostPort moved to 🧱️elements/🔌️Ports/🟦️.tsx (a
// module-top-level circular-import fix, see that file's header comment) — imported below, not redefined.
import { reactHostPort, setReactHostPort, type ReactHostPort } from "../../../../🧱️elements/🔌️Ports/🟦️.tsx";
export { reactHostPort, type ReactHostPort };
export {
  interactiveJobPort,
  type InteractiveJobDescriptor,
  type InteractiveJobLease,
  type InteractiveJobPage,
  type InteractiveJobPort,
  type InteractiveJobPortSnapshot,
  type InteractiveJobTerminal,
} from "../../../../🧱️elements/🔌️Ports/🟦️";

/** @emoji 🧊️ Host surface for three.js / R3F (implemented by 🔌️Adapters). */
export interface ThreeHostPort {
  readonly canvas: typeof ThreeCanvas;
  readonly drei: { OrbitControls: typeof OrbitControls; Grid: typeof Grid };
}

// 🧱️core-extracted: SceneHostPort + sceneHostPort moved to 🧱️elements/🔌️Ports/🟦️.tsx (same
// module-top-level circular-import fix as reactHostPort above, triggered by Scene's top-level
// `sceneHostPort.drei.Line` read) — imported below, not redefined.
import { sceneHostPort, setSceneHostPort, type SceneHostPort } from "../../../../🧱️elements/🔌️Ports/🟦️.tsx";
export { sceneHostPort, type SceneHostPort };
// #endregion 🔌️Ports

// #region 🔌️PortWiring
import { flowHostPort, setFlowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider } from "../../../../🧱️elements/🔌️Ports/🟦️.tsx";
export { flowHostPort, type FlowHostPort, HostReactFlow, HostReactFlowProvider };
// 🧱️core-extracted: reactHostPort's `let` binding + default value now live in
// 🧱️elements/🔌️Ports/🟦️.tsx (imported above, in the 🔌️Ports region) — reassignment below
// goes through the imported setReactHostPort() setter, since an ES import binding can't be assigned to
// directly.

/** @emoji 🔌️ Default R3F host port wired to fiber/drei adapters. */
export let threeHostPort: ThreeHostPort = {
  canvas: ThreeCanvas,
  drei: { OrbitControls, Grid },
};

const defaultReactHostPort = reactHostPort;
const defaultFlowHostPort = flowHostPort;
const defaultThreeHostPort = threeHostPort;
const defaultSceneHostPort = sceneHostPort;

/** @emoji 🔌️ Overrides for {@link configureHostPorts}; an omitted/`undefined` key is left untouched (whatever port is currently installed keeps running). */
export type HostPortOverrides = Partial<{
  readonly react: ReactHostPort;
  readonly flow: FlowHostPort;
  readonly three: ThreeHostPort;
  readonly scene: SceneHostPort;
  readonly iconRender: IconRenderPort;
}>;

/** @emoji 🔌️ Swaps one or more host ports; ESM importers cannot assign `export let` bindings directly,
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
// #endregion 🔌️PortWiring

// #region 🐚️ShellScope
import { type SelectionModeStore, type ShellScope, createShellScope, ShellScopeContext, ShellScopeProvider, useShellScope, useShellScopeOptional, shellScopeStorageOrBrowserFallback } from "../../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
export { type SelectionModeStore, type ShellScope, createShellScope, ShellScopeContext, ShellScopeProvider, useShellScope, useShellScopeOptional, shellScopeStorageOrBrowserFallback };
// #endregion 🐚️ShellScope

import { registerShellActivityRoot, activeShellRoot, useShellKeydown, useIsActiveShellRoot, NULL_SHELL_ROOT_REF } from "../../../../🧱️elements/🐚️ShellScope/🟦️.tsx";
export { registerShellActivityRoot, activeShellRoot, useShellKeydown, useIsActiveShellRoot, NULL_SHELL_ROOT_REF };

// #region 🔖️IconRenderPort
export type { IconRenderCamera, IconRenderFormat, IconRenderShape, IconRenderLights, IconRenderMaterial, IconRenderPort, IconRenderRequest, IconRenderResult, ThemeAppearanceName, ThemePaletteGroup, UiTheme } from "@semio-tech/ui-styling";
export { activeUiTheme, applyUiThemeToRoot, builtinUiThemes, clearUiThemeFromRoot, parseUiTheme, resolveThemeAppearancePalettes, semioTheme, serializeUiTheme, setActiveUiTheme, subscribeActiveUiTheme } from "@semio-tech/ui-styling";

import type { IconRenderPort, IconRenderRequest, IconRenderResult, IconRenderShape } from "@semio-tech/ui-styling";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { SVGRenderer } from "three/examples/jsm/renderers/SVGRenderer.js";

const GLB_MESH_FRAME_ROTATION_X = Math.PI / 2;
const ICON_RENDER_GLB_CACHE = ephemeralMap<string, Promise<THREE.Group>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.ICON_RENDER_GLB_CACHE");

/** @emoji ☀️ Sun position on a sphere from azimuth/elevation degrees, see https://en.wikipedia.org/wiki/Horizontal_coordinate_system. */
export function sunPositionFromAzimuthElevation(azimuthDeg: number, elevationDeg: number, distance = 120): [number, number, number] {
  const az = (azimuthDeg * Math.PI) / 180;
  const el = (elevationDeg * Math.PI) / 180;
  return [Math.cos(el) * Math.cos(az) * distance, Math.cos(el) * Math.sin(az) * distance, Math.sin(el) * distance];
}

async function loadGlbGroup(url: string): Promise<THREE.Group> {
  const cached = ICON_RENDER_GLB_CACHE.get(url);
  if (cached) return cached.then((group) => group.clone(true));
  const pending = new Promise<THREE.Group>((resolve, reject) => {
    const loader = new GLTFLoader();
    loader.load(
      url,
      (gltf) => {
        const root = new THREE.Group();
        const frame = new THREE.Group();
        frame.rotation.x = GLB_MESH_FRAME_ROTATION_X;
        frame.add(gltf.scene);
        root.add(frame);
        resolve(root);
      },
      undefined,
      reject,
    );
  });
  ICON_RENDER_GLB_CACHE.set(url, pending);
  return pending.then((group) => group.clone(true));
}

function applyIconMaterial(group: THREE.Object3D, material?: IconRenderRequest["material"]): void {
  if (!material) return;
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

function buildIconScene(request: IconRenderRequest, model: THREE.Group): THREE.Scene {
  const scene = new THREE.Scene();
  if (request.background) scene.background = new THREE.Color(request.background);
  scene.add(model.clone(true));
  applyIconMaterial(scene, request.material);
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

function buildIconCamera(request: IconRenderRequest): THREE.PerspectiveCamera {
  const camera = new THREE.PerspectiveCamera(request.camera.fov ?? 50, request.width / request.height, 0.1, 10_000);
  const up = request.camera.up ?? [0, 0, 1];
  camera.up.set(up[0], up[1], up[2]);
  camera.position.set(request.camera.position[0], request.camera.position[1], request.camera.position[2]);
  camera.lookAt(request.camera.target[0], request.camera.target[1], request.camera.target[2]);
  camera.zoom = request.camera.zoom;
  camera.updateProjectionMatrix();
  return camera;
}

async function renderIconSvg(scene: THREE.Scene, camera: THREE.PerspectiveCamera, width: number, height: number): Promise<IconRenderResult> {
  const renderer = new SVGRenderer();
  renderer.setSize(width, height);
  renderer.render(scene, camera);
  const svgElement = renderer.domElement;
  const svgMarkup = new XMLSerializer().serializeToString(svgElement);
  const dataUrl = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svgMarkup)}`;
  return { dataUrl, svgMarkup };
}

async function renderIconPng(scene: THREE.Scene, camera: THREE.PerspectiveCamera, width: number, height: number, shadowEnabled: boolean): Promise<IconRenderResult> {
  const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, preserveDrawingBuffer: true });
  renderer.setSize(width, height);
  renderer.shadowMap.enabled = shadowEnabled;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  renderer.render(scene, camera);
  const dataUrl = renderer.domElement.toDataURL("image/png");
  renderer.dispose();
  return { dataUrl };
}

/** @emoji ⭕️ Clips rendered SVG markup to an axis-aligned ellipse inscribed in the shot bounds. */
export function clipIconSvgMarkupToEllipse(svgMarkup: string, width: number, height: number): string {
  const clipId = "semio-icon-ellipse-clip";
  if (svgMarkup.includes(`id="${clipId}"`)) return svgMarkup;
  const openMatch = svgMarkup.match(/^<svg([^>]*)>/i);
  const closeIdx = svgMarkup.lastIndexOf("</svg>");
  if (!openMatch || closeIdx < 0) return svgMarkup;
  const clipDef = `<clipPath id="${clipId}"><ellipse cx="${width / 2}" cy="${height / 2}" rx="${width / 2}" ry="${height / 2}"/></clipPath>`;
  let body = svgMarkup.slice(openMatch[0].length, closeIdx);
  let defs = "";
  const defsMatch = body.match(/^<defs[^>]*>[\s\S]*?<\/defs>/i);
  if (defsMatch) {
    defs = defsMatch[0].replace(/<defs([^>]*)>/i, `<defs$1>${clipDef}`);
    body = body.slice(defsMatch[0].length);
  } else {
    defs = `<defs>${clipDef}</defs>`;
  }
  return `<svg${openMatch[1]}>${defs}<g clip-path="url(#${clipId})">${body}</g></svg>`;
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

/** @emoji 🖼️ Default three.js-backed icon render port (SVGRenderer + WebGL PNG), reassignable via {@link configureHostPorts}. */
export let iconRenderPort: IconRenderPort = {
  async render(request: IconRenderRequest): Promise<IconRenderResult> {
    const model = await loadGlbGroup(request.assetUrl);
    const scene = buildIconScene(request, model);
    const camera = buildIconCamera(request);
    const result = request.format === "svg" ? await renderIconSvg(scene, camera, request.width, request.height) : await renderIconPng(scene, camera, request.width, request.height, request.shadowEnabled === true);
    return applyIconRenderShape(result, request.shape, request.width, request.height);
  },
};

const defaultIconRenderPort = iconRenderPort;

/** @emoji 🖼️ Aspect-ratio style keeping a W×H frame inside its container. */
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

/** @emoji 🖼️ Frame mask class for an icon shot shape. */
export function iconShotFrameClass(shape: IconRenderShape): string {
  return shape === "ellipse" ? "rounded-full" : "rounded-none";
}

/** @emoji 🖼️ Centered shot frame overlay with shape mask and W×H badge. */
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
  return (
    <div className={cn("pointer-events-none absolute inset-0 flex items-center justify-center", className)}>
      <div
        className={cn("relative box-border overflow-hidden border-2 border-accent shadow-[0_0_0_1px_color-mix(in_srgb,var(--foreground)_20%,transparent)]", iconShotFrameClass(shape))}
        data-icon-shot-frame
        data-icon-shot-shape={shape}
        style={{
          ...iconShotFrameStyle(width, height),
          ...(background ? { background } : {}),
        }}
      >
        {children}
        {badge ? (
          <span className="pointer-events-none absolute bottom-1 right-1 rounded-sm bg-background/80 px-1 font-mono text-[10px] text-muted-foreground">
            {width}×{height} · {shape}
          </span>
        ) : null}
      </div>
    </div>
  );
}
// #endregion 🔖️IconRenderPort

// #region 🖼️ReferenceMedia
/** @emoji 🖼️ Reference plane media kind for infinite-world grid underlays. */
export type ReferenceMediaKind = "image" | "svg" | "pdf";

/** @emoji 🖼️ Source descriptor for {@link ReferenceMediaPort.loadReferenceTexture}. */
export interface ReferenceMediaSource {
  readonly url: string;
  readonly mediaKind: ReferenceMediaKind;
  readonly page?: number;
}

/** @emoji 🖼️ Loaded reference texture with intrinsic pixel dimensions. */
export interface ReferenceMediaLoadResult {
  readonly texture: THREE.Texture;
  readonly width: number;
  readonly height: number;
}

/** @emoji 🖼️ Port for rasterizing png/svg/pdf paths into three.js textures. */
export interface ReferenceMediaPort {
  loadReferenceTexture(source: ReferenceMediaSource): Promise<ReferenceMediaLoadResult>;
}

const REFERENCE_MEDIA_RASTER_MAX = 4096;

/** @emoji 🔎️ Infers reference media kind from a URL path extension. */
export function referenceMediaKindFromUrl(url: string): ReferenceMediaKind | null {
  const ext = url.split(/[?#]/, 1)[0]?.split(".").pop()?.toLowerCase() ?? "";
  if (["png", "jpg", "jpeg", "gif", "webp", "bmp", "avif", "tif", "tiff"].includes(ext)) {
    return "image";
  }
  if (ext === "svg") {
    return "svg";
  }
  if (ext === "pdf") {
    return "pdf";
  }
  return null;
}

async function rasterizeReferenceImage(url: string): Promise<{ readonly canvas: HTMLCanvasElement; readonly width: number; readonly height: number }> {
  const image = new Image();
  image.crossOrigin = "anonymous";
  await new Promise<void>((resolve, reject) => {
    image.onload = () => resolve();
    image.onerror = () => reject(new Error(`reference image load failed: ${url}`));
    image.src = url;
  });
  const width = Math.max(1, Math.min(REFERENCE_MEDIA_RASTER_MAX, image.naturalWidth || 512));
  const height = Math.max(1, Math.min(REFERENCE_MEDIA_RASTER_MAX, image.naturalHeight || 512));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("reference canvas 2d unavailable");
  }
  ctx.drawImage(image, 0, 0, width, height);
  return { canvas, width, height };
}

const referencePdfWorkerReady = ephemeralBox<Promise<void> | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.referencePdfWorkerReady", null);

async function loadReferencePdfModule(): Promise<typeof import("pdfjs-dist")> {
  const pdfjs = await import("pdfjs-dist");
  if (!referencePdfWorkerReady.current) {
    referencePdfWorkerReady.current = Promise.resolve().then(() => {
      pdfjs.GlobalWorkerOptions.workerSrc = new URL("pdfjs-dist/build/pdf.worker.min.mjs", import.meta.url).toString();
    });
  }
  await referencePdfWorkerReady.current;
  return pdfjs;
}

async function rasterizeReferencePdf(url: string, pageNumber: number): Promise<{ readonly canvas: HTMLCanvasElement; readonly width: number; readonly height: number }> {
  const pdfjs = await loadReferencePdfModule();
  const doc = await pdfjs.getDocument(url).promise;
  const page = await doc.getPage(Math.max(1, Math.min(pageNumber, doc.numPages)));
  const viewport = page.getViewport({ scale: 1 });
  const scale = Math.min(1, REFERENCE_MEDIA_RASTER_MAX / Math.max(viewport.width, viewport.height, 1));
  const scaled = page.getViewport({ scale });
  const canvas = document.createElement("canvas");
  canvas.width = Math.ceil(scaled.width);
  canvas.height = Math.ceil(scaled.height);
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("reference pdf canvas 2d unavailable");
  }
  await page.render({ canvas, canvasContext: ctx, viewport: scaled }).promise;
  return { canvas, width: canvas.width, height: canvas.height };
}

function referenceCanvasTexture(canvas: HTMLCanvasElement): THREE.Texture {
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.needsUpdate = true;
  return texture;
}

async function loadReferenceImageTexture(url: string): Promise<ReferenceMediaLoadResult> {
  const loader = new THREE.TextureLoader();
  loader.setCrossOrigin("anonymous");
  const texture = await loader.loadAsync(url);
  const image = texture.image as { naturalWidth?: number; width?: number; naturalHeight?: number; height?: number };
  return {
    texture,
    width: Math.max(1, image.naturalWidth ?? image.width ?? 1),
    height: Math.max(1, image.naturalHeight ?? image.height ?? 1),
  };
}

/** @emoji 🖼️ Default {@link ReferenceMediaPort} wired through {@link sceneHostPort}. */
export let referenceMediaPort: ReferenceMediaPort = {
  async loadReferenceTexture(source) {
    if (source.mediaKind === "image") {
      return loadReferenceImageTexture(source.url);
    }
    if (source.mediaKind === "svg") {
      const raster = await rasterizeReferenceImage(source.url);
      return { texture: referenceCanvasTexture(raster.canvas), width: raster.width, height: raster.height };
    }
    const raster = await rasterizeReferencePdf(source.url, source.page ?? 1);
    return { texture: referenceCanvasTexture(raster.canvas), width: raster.width, height: raster.height };
  },
};
// #endregion 🖼️ReferenceMedia

// #region 🔌️PortWiringAliases
export const HostThreeCanvas = threeHostPort.canvas;
export const HostSceneCanvas = sceneHostPort.fiber.canvas;
export type { ThreeEvent };

/** @emoji 🌳️ Typography for measure tree leaf labels. */
export const windowMeasureTreeLeafLabelClass = "text-tiny font-normal text-element group-hover:text-emphasized transition-colors";

/** @emoji 🌳️ Typography for measure tree group headers. */
export const windowMeasureTreeGroupLabelClass = "text-tiny font-semibold uppercase tracking-wide text-element group-hover:text-emphasized";

/** @emoji 📑️ Panel tab label beside the icon. */
export const panelTabLabelClass = "min-w-0 truncate text-xs leading-none";

/** @emoji 📑️ Panel tab icon slot — defers dimensions to the tab icon (12px). */
export const panelTabIconSlotClass = "inline-flex shrink-0 items-center justify-center leading-none";

/** @emoji 🎯️ Label/icon emphasis paired with {@link dropZoneReadyFillClass} so text stays legible on the fill. */
export const dropZoneReadyTextClass = "text-emphasized";

/** @emoji 🎯️ Passive drop-zone fill — secondary accent, kept visually distinct from the stronger primary-accent indicator on the actively hovered target. */
export const dropZoneReadyFillClass = "bg-[var(--accent-secondary)]";

/** @emoji Combined passive drop-zone treatment (fill + emphasized text/icons). */
export const dropZoneReadyClass = cn(dropZoneReadyFillClass, dropZoneReadyTextClass);

/** @emoji 🎨️ Active/on: primary fill + active border + emphasized content (never the transient hover fill). */

/** @emoji 🌀️ Maps shell chrome {@link UiStatus} to the shared border ring utilities. */

import { cn } from "../../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
export { cn };
import {
  waitingBorderClass,
  waitingBorderActiveClass,
  loadingBorderClass,
  loadingBorderActiveClass,
  waitingBorderStateClass,
  loadingBorderStateClass,
  chromeStatusBorderClass,
  loadingBorderElementClass,
  waitingBorderElementClass,
} from "../../../../🔨️modules/🌀️status-border-presentation/🟦️.ts";
export { waitingBorderClass, waitingBorderActiveClass, loadingBorderClass, loadingBorderActiveClass, waitingBorderStateClass, loadingBorderStateClass, chromeStatusBorderClass, loadingBorderElementClass, waitingBorderElementClass };
import {
  interactiveControlTransitionClass,
  interactiveOnClass,
  interactiveTabActiveClass,
  groupHoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
  hoverExcludingHandleBgFillClass,
  interactiveHoverClass,
  interactiveHoverFillClass,
  interactiveActiveFillClass,
  interactiveActiveBorderClass,
} from "../../../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
export {
  interactiveControlTransitionClass,
  interactiveOnClass,
  interactiveTabActiveClass,
  groupHoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
  hoverExcludingHandleBgFillClass,
  interactiveHoverClass,
  interactiveHoverFillClass,
  interactiveActiveFillClass,
  interactiveActiveBorderClass,
};
import { formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "../../../../🔨️modules/📝️form-control-presentation/🟦️.ts";
export { formControlFocusBorderClass, uiFormControlBrowserDefaultProps };
import { borderNormalBottomClass, borderNormalClass, borderElementClass } from "../../../../🔨️modules/📏️border-presentation/🟦️.ts";
export { borderNormalBottomClass, borderNormalClass, borderElementClass };
import { veilClass, glassClass, surfaceClass } from "../../../../🔨️modules/🌈️surface-presentation/🟦️.ts";
export { veilClass, glassClass, surfaceClass };
import { menuListItemClassName } from "../../../../🔨️modules/📋️menu-item-presentation/🟦️.ts";
export { menuListItemClassName };
import { shellFloorPaints, shellFloorFillClass } from "../../../../🔨️modules/🏠️shell-floor-presentation/🟦️.ts";
export { shellFloorPaints, shellFloorFillClass };
import {
  chromeControlItemBaseClass,
  chromeControlItemClass,
  chromeControlTabItemClass,
  modeDockTabClassName,
  windowPaneChromeToggleClass,
  chromeControlGroupShellClass,
  chromeControlGroupClass,
  chromeControlItemOnClass,
  chromeControlTabActiveClass,
} from "../../../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";
export { chromeControlItemBaseClass, chromeControlItemClass, chromeControlTabItemClass, modeDockTabClassName, windowPaneChromeToggleClass, chromeControlGroupShellClass, chromeControlGroupClass, chromeControlItemOnClass, chromeControlTabActiveClass };

// #region 🔖️SelectionMarquee
/** @emoji ⬚️ Canonical area-select overlay coverage (drag right-to-left = partial). */
export type SelectionMarqueeCoverage = "partial" | "full";

export type SelectionMarqueeRect = {
  readonly x: number | string;
  readonly y: number | string;
  readonly width: number | string;
  readonly height: number | string;
};

export type SelectionMarqueePoint = {
  readonly x: number;
  readonly y: number;
};

export type SelectionMarqueeProps = {
  readonly coverage: SelectionMarqueeCoverage;
  readonly className?: string;
} & ({ readonly shape: "rect"; readonly rect: SelectionMarqueeRect } | { readonly shape: "polygon"; readonly points: readonly SelectionMarqueePoint[] });

/** @emoji ⬚️ Shared SVG marquee for spatial area selection (primary fill/stroke; dashed when partial). */
export function SelectionMarquee(props: SelectionMarqueeProps): React.ReactElement {
  const { coverage, className } = props;
  const svgClass = cn("selection-marquee pointer-events-none absolute inset-0 h-full w-full overflow-visible", className);
  if (props.shape === "rect") {
    const { rect } = props;
    return (
      <svg className={svgClass} data-coverage={coverage} aria-hidden>
        <rect x={rect.x} y={rect.y} width={rect.width} height={rect.height} />
      </svg>
    );
  }
  const points = props.points.map((point) => `${point.x},${point.y}`).join(" ");
  return (
    <svg className={svgClass} data-coverage={coverage} aria-hidden>
      <polygon points={points} />
    </svg>
  );
}

export const SELECTION_DRAG_DIRECTION_THRESHOLD_PX = 2;

export type SelectionMarqueeMethod = "lasso" | "rectangle";

/** @emoji 🖱️ Crossing selection when the drag ends left of the start (partial overlap). */
export function marqueeIsCrossing(startX: number, endX: number): boolean {
  return endX < startX;
}

/** @emoji 🖱️ Lasso uses the first horizontal step; rectangle compares start vs end. */
export function marqueeIsCrossingFromPath(path: readonly SelectionMarqueePoint[], method: SelectionMarqueeMethod = "rectangle"): boolean {
  const start = path[0];
  if (!start) return false;
  if (method === "lasso") {
    for (const point of path.slice(1)) {
      const dx = point.x - start.x;
      if (Math.abs(dx) < SELECTION_DRAG_DIRECTION_THRESHOLD_PX) continue;
      return dx < 0;
    }
  }
  const end = path[path.length - 1] ?? start;
  return marqueeIsCrossing(start.x, end.x);
}

/** @emoji 🖱️ Maps drag direction to marquee coverage (rectangle endpoints). */
export function marqueeCoverageFromDrag(startX: number, endX: number): SelectionMarqueeCoverage {
  return marqueeIsCrossing(startX, endX) ? "partial" : "full";
}

/** @emoji 🖱️ Maps gesture path to marquee coverage (lasso first horizontal step). */
export function marqueeCoverageFromPath(path: readonly SelectionMarqueePoint[], method: SelectionMarqueeMethod = "rectangle"): SelectionMarqueeCoverage {
  return marqueeIsCrossingFromPath(path, method) ? "partial" : "full";
}

/** @emoji 🖱️ Resolves marquee coverage for rectangle or lasso gestures. */
export function marqueeCoverageFromGesture(input: { readonly method: SelectionMarqueeMethod; readonly startX: number; readonly endX: number; readonly path: readonly SelectionMarqueePoint[] }): SelectionMarqueeCoverage {
  if (input.method === "lasso" && input.path.length > 0) {
    return marqueeCoverageFromPath(input.path, "lasso");
  }
  return marqueeCoverageFromDrag(input.startX, input.endX);
}

/** @emoji 🎯️ Maps shift/ctrl modifiers to a marquee-drag `MergeMode` (ctrl+shift → invertive).
 * `persistentMode` — a shell's own {@link SelectionModeStore} value, e.g. `useShellScope().selection.get()`
 * — takes precedence when set to something other than `"replace"`, mirroring the toolbar toggle in
 * `SelectionUtilityOptions`; omitted, behaves as if the toolbar is at its `"replace"` setting (was: read a
 * page-global `(globalThis).__selectionMode`, so one shell's toolbar choice silently applied to every
 * other mounted shell's marquee gestures too). ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
 * W3a: unified onto the `🕹️interaction` module's `MergeMode` (was a bespoke `SelectionMergeMode` union
 * with `"default"` instead of `"replace"`) — never produces `"range"`, since a marquee drag has no
 * ordered/anchored topology to range over (unlike `interactionMergeFromModifiers`'s click-select shift
 * mapping below). */
export function marqueeModeFromModifiers(modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean }, persistentMode?: MergeMode): MergeMode {
  if (persistentMode && persistentMode !== "replace") {
    return persistentMode;
  }
  const shift = modifiers.shiftKey === true;
  const ctrl = modifiers.ctrlKey === true || modifiers.metaKey === true;
  if (shift && ctrl) return "invertive";
  if (shift) return "additive";
  if (ctrl) return "subtractive";
  return "replace";
}

/** @emoji 🎯️ Applies a marquee-drag `MergeMode` when committing ids — `"range"` has no meaning without
 * an ordered topology here (see `marqueeModeFromModifiers`'s doc), so it falls back to `"replace"`. */
export function selectionMergeIds(mode: MergeMode, current: readonly string[], incoming: readonly string[]): string[] {
  const currentSet = new Set(current);
  const incomingSet = new Set(incoming);
  if (mode === "replace" || mode === "range") return [...incomingSet];
  if (mode === "additive") {
    for (const id of incomingSet) currentSet.add(id);
    return [...currentSet];
  }
  if (mode === "subtractive") {
    for (const id of incomingSet) currentSet.delete(id);
    return [...currentSet];
  }
  for (const id of incomingSet) {
    if (currentSet.has(id)) currentSet.delete(id);
    else currentSet.add(id);
  }
  return [...currentSet];
}

/**
 * 🎯️ The ONE modifier→merge policy for `🕹️interaction`'s `nextSelection`/`nextHover` picks — every
 * surface (Tree rows, the Shell, canvas/world hosts) routes its raw pointer/keyboard modifiers through
 * this single mapping instead of hand-rolling shift/ctrl/alt checks per surface: shift → `"range"`,
 * mod/ctrl/cmd → `"invertive"`, alt → `"subtractive"`, no modifier → `"replace"`. Priority is shift, then
 * ctrl/meta, then alt — a chord that holds more than one of these picks the first that matches, so shift
 * always wins a range pick even with ctrl also held. Shares its `MergeMode` vocabulary with
 * `marqueeModeFromModifiers` above (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W3a
 * collapsed the old, differently-prioritized `SelectionMergeMode` duplicate into this one) — the two
 * functions differ only in priority/never-range, matching their different gestures (marquee drag vs.
 * click-select).
 */
export function interactionMergeFromModifiers(modifiers: { readonly shiftKey?: boolean; readonly ctrlKey?: boolean; readonly metaKey?: boolean; readonly altKey?: boolean }): MergeMode {
  if (modifiers.shiftKey === true) return "range";
  if (modifiers.ctrlKey === true || modifiers.metaKey === true) return "invertive";
  if (modifiers.altKey === true) return "subtractive";
  return "replace";
}

export type ScreenRect = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

export function screenRectContainsRect(outer: ScreenRect, inner: ScreenRect): boolean {
  return inner.x >= outer.x && inner.y >= outer.y && inner.x + inner.width <= outer.x + outer.width && inner.y + inner.height <= outer.y + outer.height;
}

export function screenRectIntersectsRect(a: ScreenRect, b: ScreenRect): boolean {
  return a.x <= b.x + b.width && a.x + a.width >= b.x && a.y <= b.y + b.height && a.y + a.height >= b.y;
}

export function screenRectFromPoints(points: readonly { readonly x: number; readonly y: number }[]): ScreenRect | null {
  if (!points.length) return null;
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  const maxX = Math.max(...xs);
  const maxY = Math.max(...ys);
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}
// #endregion 🔖️SelectionMarquee

// #region 🔖️CanvasPickMenu
export type CanvasPickMenuProps = {
  readonly request: CanvasPickRequest | null;
  readonly hoveredKey: string | null;
  readonly onHoverKey: (key: string | null) => void;
  readonly onPick: (target: CanvasPickTarget) => void;
  readonly onDismiss: () => void;
  readonly renderRow?: (target: CanvasPickTarget, active: boolean) => React.ReactNode;
  readonly title?: UiLabel;
};

export { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_PICK_MENU, canvasHoverFocusFromTarget, canvasPickTargetKey, pickMostSpecificCanvasTarget, sortCanvasPickTargetsGeneralFirst } from "@semio-tech/framework";
export { windowElementId, panelTabElementId, panelTabFirstDraggableElementId } from "@semio-tech/framework";
export type { CanvasHoverFocus, CanvasPickRequest, CanvasPickTarget } from "@semio-tech/framework";

/** @emoji 🎯️ Fixed DOM pick list for overlapping canvas targets (not painted on the infinite canvas). */
export function CanvasPickMenu({ request, hoveredKey, onHoverKey, onPick, onDismiss, renderRow, title }: CanvasPickMenuProps): React.ReactNode {
  const menuRef = React.useRef<HTMLDivElement | null>(null);
  const selectTargetLabel = useLabel("ui.common.selectTarget");
  const resolvedTitle = title ?? selectTargetLabel;
  // 🐚️ Falls back to `document.body` outside any shell — inside one, portals into that shell's own
  // overlay layer instead so this menu never visually escapes into another mounted shell's stacking context.
  const shellScope = useShellScopeOptional();

  React.useEffect(() => {
    if (!request) return;
    const onPointerDown = (event: PointerEvent) => {
      const menu = menuRef.current;
      if (event.target instanceof globalThis.Node && menu?.contains(event.target)) return;
      onDismiss();
    };
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") onDismiss();
    };
    window.addEventListener("pointerdown", onPointerDown, true);
    window.addEventListener("keydown", onKeyDown, true);
    return () => {
      window.removeEventListener("pointerdown", onPointerDown, true);
      window.removeEventListener("keydown", onKeyDown, true);
    };
  }, [onDismiss, request]);

  if (!request || request.targets.length === 0) return null;
  const sorted = sortCanvasPickTargetsGeneralFirst(request.targets);
  const body = sorted.map((target) => {
    const key = canvasPickTargetKey(target);
    const active = hoveredKey === key;
    return (
      <button
        key={key}
        type="button"
        role="menuitem"
        aria-selected={active}
        className={cn(floatingMenuItemClass, active && "bg-active-base text-emphasized")}
        onPointerEnter={() => onHoverKey(key)}
        onPointerLeave={() => onHoverKey(null)}
        onPointerDown={(event) => {
          event.preventDefault();
          event.stopPropagation();
          onPick(target);
        }}
      >
        {renderRow ? (
          renderRow(target, active)
        ) : (
          <>
            <span className="text-muted-foreground">{target.domain}</span> <code className="text-foreground">{target.label ?? target.id}</code>
          </>
        )}
      </button>
    );
  });
  if (typeof document === "undefined") return null;
  return createPortal(
    <div
      ref={menuRef}
      role="menu"
      className={cn("fixed z-tutorial w-layout-floating-menu-sm max-h-layout-preview-md overflow-y-auto p-single", floatingMenuSurfaceClass)}
      style={{
        left: Math.min(request.client.x + 8, window.innerWidth - 230),
        top: Math.min(request.client.y + 8, window.innerHeight - 220),
      }}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <div className="text-muted-foreground px-single py-half text-2xs">{resolvedTitle}</div>
      {body}
    </div>,
    shellScope?.portalLayerRef.current ?? document.body,
  );
}

export type UseCanvasPickInteractionOptions = {
  readonly resolveTargetsAtClient: (client: { readonly x: number; readonly y: number }) => readonly CanvasPickTarget[];
  readonly onHoverFocus: (focus: CanvasHoverFocus) => void;
  readonly onSelectTarget: (target: CanvasPickTarget, request: CanvasPickRequest) => void;
  readonly clickThresholdPx?: number;
};

export type CanvasPickInteraction = {
  readonly pickMenu: CanvasPickRequest | null;
  readonly menuHoveredKey: string | null;
  readonly pickMenuOpen: boolean;
  readonly onCanvasPointerDown: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerMove: (client: { readonly x: number; readonly y: number }) => void;
  readonly onCanvasPointerLeave: () => void;
  readonly onCanvasPointerUp: (client: { readonly x: number; readonly y: number }, modifiers?: Readonly<Record<string, boolean>>) => void;
  readonly dismissPickMenu: () => void;
  readonly onMenuHoverKey: (key: string | null) => void;
  readonly onMenuPick: (target: CanvasPickTarget) => void;
};

/** @emoji 🎯️ Shared pointer routing for canvas hover (most-specific) and click disambiguation menus. */
export function useCanvasPickInteraction({ resolveTargetsAtClient, onHoverFocus, onSelectTarget, clickThresholdPx = 4 }: UseCanvasPickInteractionOptions): CanvasPickInteraction {
  const [pickMenu, setPickMenu] = React.useState<CanvasPickRequest | null>(null);
  const [menuHoveredKey, setMenuHoveredKey] = React.useState<string | null>(null);
  const pointerDownRef = React.useRef<{ readonly x: number; readonly y: number } | null>(null);

  const dismissPickMenu = React.useCallback(() => {
    setPickMenu(null);
    setMenuHoveredKey(null);
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [onHoverFocus]);

  const onCanvasPointerMove = React.useCallback(
    (client: { readonly x: number; readonly y: number }) => {
      if (pickMenu) return;
      const targets = resolveTargetsAtClient(client);
      const specific = pickMostSpecificCanvasTarget(targets);
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, specific));
    },
    [onHoverFocus, pickMenu, resolveTargetsAtClient],
  );

  const onCanvasPointerLeave = React.useCallback(() => {
    if (pickMenu) return;
    onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_CANVAS, null));
  }, [onHoverFocus, pickMenu]);

  const onCanvasPointerDown = React.useCallback((client: { readonly x: number; readonly y: number }) => {
    pointerDownRef.current = client;
  }, []);

  const onCanvasPointerUp = React.useCallback(
    (client: { readonly x: number; readonly y: number }, modifiers: Readonly<Record<string, boolean>> = {}) => {
      const start = pointerDownRef.current;
      pointerDownRef.current = null;
      if (start) {
        const dx = client.x - start.x;
        const dy = client.y - start.y;
        if (Math.hypot(dx, dy) > clickThresholdPx) return;
      }
      const targets = resolveTargetsAtClient(client);
      if (targets.length === 0) {
        dismissPickMenu();
        return;
      }
      if (targets.length === 1) {
        dismissPickMenu();
        onSelectTarget(targets[0]!, { targets, client, modifiers });
        return;
      }
      const request: CanvasPickRequest = { targets, client, modifiers };
      setPickMenu(request);
      const first = sortCanvasPickTargetsGeneralFirst(targets)[0]!;
      const key = canvasPickTargetKey(first);
      setMenuHoveredKey(key);
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_PICK_MENU, first));
    },
    [clickThresholdPx, dismissPickMenu, onHoverFocus, onSelectTarget, resolveTargetsAtClient],
  );

  const onMenuHoverKey = React.useCallback(
    (key: string | null) => {
      setMenuHoveredKey(key);
      if (!pickMenu) return;
      const target = key ? (pickMenu.targets.find((row) => canvasPickTargetKey(row) === key) ?? null) : null;
      onHoverFocus(canvasHoverFocusFromTarget(CANVAS_HOVER_SOURCE_PICK_MENU, target));
    },
    [onHoverFocus, pickMenu],
  );

  const onMenuPick = React.useCallback(
    (target: CanvasPickTarget) => {
      if (!pickMenu) return;
      onSelectTarget(target, pickMenu);
      dismissPickMenu();
    },
    [dismissPickMenu, onSelectTarget, pickMenu],
  );

  return {
    pickMenu,
    menuHoveredKey,
    pickMenuOpen: pickMenu !== null,
    onCanvasPointerMove,
    onCanvasPointerLeave,
    onCanvasPointerUp,
    dismissPickMenu,
    onMenuHoverKey,
    onMenuPick,
    onCanvasPointerDown,
  };
}
// #endregion 🔖️CanvasPickMenu

// #region 🔖️Icon
import {
  resolveIconSizePx,
  decodeIcon,
  encodeIcon,
  classifyIconSelectorMode,
  resolveIconUrlsInBoardJson,
  resolveCatalogIconSvg,
  resolveMetabolismIconSvg,
  renderControlIcon,
  iconSvgMarkup,
  iconMaskImage,
  Icon,
  createIconComponent,
  AddIcon,
  AlertCircleIcon,
  ArrowLeftIcon,
  AwardIcon,
  BookIcon,
  BoxIcon,
  CameraIcon,
  ChatIcon,
  CheckIcon,
  CheckIconAlt,
  ChevronDownIcon,
  ChevronDownIconAlt,
  ChevronLeftIcon,
  ChevronRightIcon,
  ChevronUpIcon,
  ChevronsUpDownIcon,
  CircleDotIcon,
  CloseIcon,
  CloseIconAlt,
  CodeIcon,
  ComponentIcon,
  ConnectionIcon,
  ConnectorIcon,
  CopyIcon,
  DetailsIcon,
  DiagramIcon,
  DisconnectIcon,
  DocumentIcon,
  ExternalLinkIcon,
  FileArchiveIcon,
  FileCodeIcon,
  FileImageIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTypeIcon,
  FileVideoIcon,
  FilterIcon,
  FindInViewIcon,
  FocusIcon,
  FolderIcon,
  FolderOpenIcon,
  GlobeIcon,
  GripVerticalIcon,
  HandIcon,
  HashIcon,
  HomeIcon,
  HudIcon,
  HudPanelIcon,
  InfoIcon,
  IntersectIcon,
  LandmarkIcon,
  LassoIcon,
  LayoutIcon,
  LayoutGridIcon,
  LeftSidePanelIcon,
  LightbulbIcon,
  LinkIcon,
  LoaderIcon,
  LocalKitIcon,
  Maximize2Icon,
  MessageCircle,
  MessageSquareIcon,
  Minimize2Icon,
  MonitorIcon,
  MoonIcon,
  MoreHorizontalIcon,
  MousePointerIcon,
  MoveIcon,
  NavigateBackIcon,
  NavigateForwardIcon,
  NavigateUpIcon,
  PanelRightIcon,
  PauseIcon,
  PieceIcon,
  PlayIcon,
  PlugIcon,
  PlusIcon,
  PortIcon,
  Puzzle2dIconFileImportIcon,
  Puzzle2dIconMathGlyphIcon,
  Puzzle2dIconRasterGlyphIcon,
  RecordIcon,
  RemoteKitIcon,
  RemoveIcon,
  ResetIcon,
  RightSidePanelIcon,
  SceneIcon,
  SearchIcon,
  SelectUtilityIcon,
  Settings2Icon,
  SettingsIcon,
  SkipBackIcon,
  SkipForwardIcon,
  SmartphoneIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  StatsIcon,
  StopIcon,
  SunIcon,
  TabletIcon,
  TableViewIcon,
  TemporaryKitIcon,
  TriangleAlertIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
  UsersIcon,
  UtilitiesIcon,
  UtilityBarIcon,
  WorkbenchIcon,
  Cursor,
  type IconSizeToken,
  type IconSelectorMode,
  type IconSource,
  type ControlIcon,
  type IconProps,
} from "../../../../🧱️elements/🔣️Icons/🟦️.tsx";
export {
  resolveIconSizePx,
  decodeIcon,
  encodeIcon,
  classifyIconSelectorMode,
  resolveIconUrlsInBoardJson,
  resolveCatalogIconSvg,
  resolveMetabolismIconSvg,
  renderControlIcon,
  iconSvgMarkup,
  iconMaskImage,
  Icon,
  createIconComponent,
  AddIcon,
  AlertCircleIcon,
  ArrowLeftIcon,
  AwardIcon,
  BookIcon,
  BoxIcon,
  CameraIcon,
  ChatIcon,
  CheckIcon,
  CheckIconAlt,
  ChevronDownIcon,
  ChevronDownIconAlt,
  ChevronLeftIcon,
  ChevronRightIcon,
  ChevronUpIcon,
  ChevronsUpDownIcon,
  CircleDotIcon,
  CloseIcon,
  CloseIconAlt,
  CodeIcon,
  ComponentIcon,
  ConnectionIcon,
  ConnectorIcon,
  CopyIcon,
  DetailsIcon,
  DiagramIcon,
  DisconnectIcon,
  DocumentIcon,
  ExternalLinkIcon,
  FileArchiveIcon,
  FileCodeIcon,
  FileImageIcon,
  FileJsonIcon,
  FileSpreadsheetIcon,
  FileTypeIcon,
  FileVideoIcon,
  FilterIcon,
  FindInViewIcon,
  FocusIcon,
  FolderIcon,
  FolderOpenIcon,
  GlobeIcon,
  GripVerticalIcon,
  HandIcon,
  HashIcon,
  HomeIcon,
  HudIcon,
  HudPanelIcon,
  InfoIcon,
  IntersectIcon,
  LandmarkIcon,
  LassoIcon,
  LayoutIcon,
  LayoutGridIcon,
  LeftSidePanelIcon,
  LightbulbIcon,
  LinkIcon,
  LoaderIcon,
  LocalKitIcon,
  Maximize2Icon,
  MessageCircle,
  MessageSquareIcon,
  Minimize2Icon,
  MonitorIcon,
  MoonIcon,
  MoreHorizontalIcon,
  MousePointerIcon,
  MoveIcon,
  NavigateBackIcon,
  NavigateForwardIcon,
  NavigateUpIcon,
  PanelRightIcon,
  PauseIcon,
  PieceIcon,
  PlayIcon,
  PlugIcon,
  PlusIcon,
  PortIcon,
  Puzzle2dIconFileImportIcon,
  Puzzle2dIconMathGlyphIcon,
  Puzzle2dIconRasterGlyphIcon,
  RecordIcon,
  RemoteKitIcon,
  RemoveIcon,
  ResetIcon,
  RightSidePanelIcon,
  SceneIcon,
  SearchIcon,
  SelectUtilityIcon,
  Settings2Icon,
  SettingsIcon,
  SkipBackIcon,
  SkipForwardIcon,
  SmartphoneIcon,
  SortAscendingIcon,
  SortDescendingIcon,
  StatsIcon,
  StopIcon,
  SunIcon,
  TabletIcon,
  TableViewIcon,
  TemporaryKitIcon,
  TriangleAlertIcon,
  TutorialIcon,
  TypeIcon,
  UserIcon,
  UsersIcon,
  UtilitiesIcon,
  UtilityBarIcon,
  WorkbenchIcon,
  Cursor,
  type IconSizeToken,
  type IconSelectorMode,
  type IconSource,
  type ControlIcon,
  type IconProps,
};

/** @emoji 🌀️ Waiting ring matching the element's current state color; empty when not waiting. */

/** @emoji 📋️ Hover row styling for menus, selects, comboboxes, and context menus. */

import {
  createDOMEventBinding,
  getElementById,
  queryElement,
  ContextMenu,
  contextMenuOrdinals,
  contextMenuItemsAtLevel,
  contextMenuItemAtPath,
  findContextMenuCheckedPath,
  contextMenuNavigationFromKey,
  moveContextMenuActivePath,
  contextMenuPathForOrdinal,
  contextMenuOpenSubmenuPath,
  isContextMenuPointerTarget,
  contextMenuDigitFromKey,
  findCheckedContextMenuItem,
  ContextMenuController,
  readDomTextSelection,
  isPointerEventOnDomTextSelection,
  isDomTextEditableTarget,
  buildTextSelectionContextMenuItems,
  copyDomTextSelection,
  cutDomTextSelection,
  pasteDomTextSelection,
  selectAllDomText,
  TextSelectionContextMenuHost,
  type ContextMenuItem,
  type ContextMenuProps,
  type ContextMenuControllerProps,
  type ContextMenuNavDirection,
  type TextSelectionContextMenuLabels,
  type TextSelectionContextMenuActions,
} from "../../../../🧱️elements/🖱️ContextMenu/🟦️.tsx";
export {
  createDOMEventBinding,
  getElementById,
  queryElement,
  ContextMenu,
  contextMenuOrdinals,
  contextMenuItemsAtLevel,
  contextMenuItemAtPath,
  findContextMenuCheckedPath,
  contextMenuNavigationFromKey,
  moveContextMenuActivePath,
  contextMenuPathForOrdinal,
  contextMenuOpenSubmenuPath,
  isContextMenuPointerTarget,
  contextMenuDigitFromKey,
  findCheckedContextMenuItem,
  ContextMenuController,
  readDomTextSelection,
  isPointerEventOnDomTextSelection,
  isDomTextEditableTarget,
  buildTextSelectionContextMenuItems,
  copyDomTextSelection,
  cutDomTextSelection,
  pasteDomTextSelection,
  selectAllDomText,
  TextSelectionContextMenuHost,
  type ContextMenuItem,
  type ContextMenuProps,
  type ContextMenuControllerProps,
  type ContextMenuNavDirection,
  type TextSelectionContextMenuLabels,
  type TextSelectionContextMenuActions,
};
// #endregion 🖱️ContextMenu

// #region UiDriver
import {
  type UiDriverLabels,
  type UiDriverLabelTier,
  type UiDriverDrag,
  type UiDriverReveal,
  type UiDriverTooltips,
  type UiDriverHotkeys,
  type UiDriver,
  DEFAULT_UI_DRIVER,
  COMPACT_UI_DRIVER,
  builtinUiDrivers,
  parseUiDriver,
  serializeUiDriver,
  resolveUiDriver,
  UI_CHROME_DRIVER_STORAGE_KEY,
  readStoredUiDriverId,
  writeStoredUiDriverId,
  UI_CUSTOM_DRIVERS_STORAGE_KEY,
  readStoredUiCustomDrivers,
  writeStoredUiCustomDrivers,
  readStoredUiDriver,
  setUiDriverProvider,
  activeUiDriver,
  useUiDriver,
  UiDriverProvider,
  useUiDriverDragSurface,
  useNativeDragArm,
  useUiDriverTooltips,
  setControlLabelIdResolver,
  resolveControlLabelId,
  panelKindFromPanelToggleControlId,
  isInternalChromeControlId,
  humanizeControlSegment,
  humanizeControlId,
  humanizeEngagementStepId,
} from "../../../../🧱️elements/🚗️UiDriver/🟦️.tsx";
export type { UiDriverLabels, UiDriverLabelTier, UiDriverDrag, UiDriverReveal, UiDriverTooltips, UiDriverHotkeys, UiDriver };
export {
  DEFAULT_UI_DRIVER,
  COMPACT_UI_DRIVER,
  builtinUiDrivers,
  parseUiDriver,
  serializeUiDriver,
  resolveUiDriver,
  UI_CHROME_DRIVER_STORAGE_KEY,
  readStoredUiDriverId,
  writeStoredUiDriverId,
  UI_CUSTOM_DRIVERS_STORAGE_KEY,
  readStoredUiCustomDrivers,
  writeStoredUiCustomDrivers,
  readStoredUiDriver,
  setUiDriverProvider,
  activeUiDriver,
  useUiDriver,
  UiDriverProvider,
  useUiDriverDragSurface,
  useNativeDragArm,
  useUiDriverTooltips,
  setControlLabelIdResolver,
  resolveControlLabelId,
  panelKindFromPanelToggleControlId,
  isInternalChromeControlId,
  humanizeControlSegment,
  humanizeControlId,
  humanizeEngagementStepId,
};
// #endregion UiDriver

// #region ⌨️UiKeybindings
import { parseKeybindingChords, formatKeybindingShortcut, ariaKeyshortcutsText } from "../../../../🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts";
import { formatControlTooltipText } from "../../../../🔨️modules/💡️control-tooltip-presentation/🟦️.ts";
import {
  buildKeysByActionId,
  SHELL_KEYBINDINGS,
  composeControlKeybindings,
  UiKeybindingsProvider,
  useUiKeybindingsByControlId,
  resolveControlKeybindingRaw,
  useControlHotkey,
  useControlKeybinding,
  useHotkeys,
  type ControlKeybindingAction,
  type ControlKeybindingDefinition,
  type ControlKeybindingCallback,
  type ControlKeybindingOptions,
  type ControlKeybindingDependencies,
} from "../../../../🔨️modules/🕹️control-keybinding-context/🟦️.tsx";
import { ControlHotkeyBadge, type ControlHotkeyBadgeProps } from "../../../../🔨️modules/⌨️control-hotkey-presentation/🟦️.tsx";
import { readStoredUiKeybindingOverrides, writeStoredUiKeybindingOverrides } from "../../../../🔨️modules/💾️keybinding-persistence/🟦️.ts";
export {
  parseKeybindingChords,
  formatKeybindingShortcut,
  ariaKeyshortcutsText,
  formatControlTooltipText,
  buildKeysByActionId,
  SHELL_KEYBINDINGS,
  composeControlKeybindings,
  UiKeybindingsProvider,
  useUiKeybindingsByControlId,
  resolveControlKeybindingRaw,
  useControlHotkey,
  useControlKeybinding,
  ControlHotkeyBadge,
  readStoredUiKeybindingOverrides,
  writeStoredUiKeybindingOverrides,
};
export type { ControlKeybindingAction, ControlKeybindingDefinition, ControlKeybindingCallback, ControlKeybindingOptions, ControlKeybindingDependencies, ControlHotkeyBadgeProps };

/** @emoji ⌨️ Maps dock {@link Anchor} values to {@link SHELL_KEYBINDINGS} control ids. */
export const SHELL_PANEL_ANCHOR_KEY_IDS: Readonly<Record<Anchor, keyof typeof SHELL_KEYBINDINGS & string>> = {
  "top-left": "ui.shell.panelAnchor.topLeft",
  "top-middle": "ui.shell.panelAnchor.topMiddle",
  "top-right": "ui.shell.panelAnchor.topRight",
  "right-middle": "ui.shell.panelAnchor.rightMiddle",
  "bottom-right": "ui.shell.panelAnchor.bottomRight",
  "bottom-middle": "ui.shell.panelAnchor.bottomMiddle",
  "bottom-left": "ui.shell.panelAnchor.bottomLeft",
  "left-middle": "ui.shell.panelAnchor.leftMiddle",
};

// #endregion ⌨️UiKeybindings

// #region 🌈️SurfaceChrome
/** @emoji 🌈️ Document-level UI chrome shared by Elements shells: appearance (system/light/dark), device (desktop/tablet/mobile), and driver — mirrors sketchpad `Appearance` / `Device` behavior on `documentElement`. */
export type ElementsSurfaceAppearance = "system" | "light" | "dark";
export type ElementsSurfaceDevice = "desktop" | "tablet" | "mobile";

/** @emoji 📱️ Shared viewport breakpoint below which shells switch to the automatic mobile device. */
export const UI_MOBILE_MEDIA_QUERY = "(max-width: 767px)";

export interface ElementsSurfaceChromeInput {
  appearance: ElementsSurfaceAppearance;
  device: ElementsSurfaceDevice;
  driver: UiDriver;
}

/** @emoji 🐚️ Resolves an explicit surface-chrome root (a shell's own root — e.g. its `ShellScope.rootRef`)
 * or falls back to `document.documentElement` for the page-owning case; every entry point below takes
 * this same optional-root shape so a single-shell page's existing call sites (which pass none) keep
 * their exact current behavior unchanged. `undefined` in a non-browser environment (SSR/vitest without
 * a document) rather than throwing. */
function resolveElementsSurfaceChromeRoot(root?: HTMLElement): HTMLElement | undefined {
  return root ?? (typeof document !== "undefined" ? document.documentElement : undefined);
}

/** @emoji 🐚️ Paints a surface-chrome root's own background/foreground/color-scheme — every root, not
 * just `documentElement`, so an embedded shell's own `.semio-scope` div is visually correct even before
 * any descendant renders. When the root IS `documentElement` (the page-owning case), also mirrors onto
 * `document.body` exactly as before this was made root-scoped — unchanged behavior for that case. */
function applyElementsSurfaceChromeBaseColors(root: HTMLElement, scheme: "light" | "dark"): void {
  root.style.backgroundColor = "var(--base)";
  root.style.color = "var(--foreground)";
  root.style.colorScheme = scheme;
  if (typeof document !== "undefined" && root === document.documentElement && document.body) {
    document.body.style.backgroundColor = "var(--base)";
    document.body.style.color = "var(--foreground)";
    document.body.style.colorScheme = scheme;
  }
}

function clearElementsSurfaceChromeBaseColors(root: HTMLElement): void {
  root.style.backgroundColor = "";
  root.style.color = "";
  root.style.colorScheme = "";
  if (typeof document !== "undefined" && root === document.documentElement && document.body) {
    document.body.style.backgroundColor = "";
    document.body.style.color = "";
    document.body.style.colorScheme = "";
  }
}

/** @emoji 🌓️ Resolves whether {@link ElementsSurfaceAppearance} is dark for the current system preference. */
export function resolveElementsSurfaceChromeDark(appearance: ElementsSurfaceAppearance): boolean {
  if (appearance === "dark") return true;
  if (appearance === "light") return false;
  if (typeof window === "undefined") return false;
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

/** @emoji 🌓️ True when a surface-chrome root (`document.documentElement` by default) currently carries
 * the dark surface chrome class. */
export function isElementsSurfaceChromeDarkApplied(rootOverride?: HTMLElement): boolean {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  return root?.classList.contains("dark") ?? false;
}

type ElementsSurfaceChromeLease = { readonly id: number; readonly input: ElementsSurfaceChromeInput };

const elementsSurfaceChromeLeaseSeq = ephemeralBox("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeLeaseSeq", 0);
/** @emoji 🐚️ One independent lease stack per surface-chrome root — was a single page-global stack, which
 * meant a second mounted shell's appearance/driver/device lease silently won (last-wins) over the
 * first's for the WHOLE page instead of just its own subtree. */
const elementsSurfaceChromeLeasesByRoot = ephemeralMap<HTMLElement, ElementsSurfaceChromeLease[]>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeLeasesByRoot");
const elementsSurfaceChromeDeferredClearFrames = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeDeferredClearFrames");
const elementsSurfaceChromeDomBindings = ephemeralBox<ReturnType<typeof createDOMEventBinding> | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeDomBindings", null);
const elementsSurfaceChromeSystemListenersInstalled = ephemeralBox("framework.modules.ui.packages.typescript.targets.react.index.tsx.elementsSurfaceChromeSystemListenersInstalled", false);

function activeElementsSurfaceChromeInput(root: HTMLElement): ElementsSurfaceChromeInput | undefined {
  const leases = elementsSurfaceChromeLeasesByRoot.get(root);
  return leases?.[leases.length - 1]?.input;
}

function syncElementsSurfaceChromeProviders(input: ElementsSurfaceChromeInput | undefined): void {
  if (input) {
    setUiDriverProvider(() => input.driver);
    return;
  }
  setUiDriverProvider(() => readStoredUiDriver(createBrowserStoragePort()));
}

function applyElementsSurfaceChromeDriverDom(root: HTMLElement, driver: UiDriver): void {
  root.dataset.uiDriver = driver.id;
  root.dataset.uiLabels = driver.labels;
  root.dataset.uiDrag = driver.drag;
  root.dataset.uiChromeReveal = driver.chrome;
  root.dataset.uiGumballReveal = driver.gumball;
  root.dataset.uiTooltips = driver.tooltips;
  syncUiChromeRevealController(root, driver.chrome);
}

function clearElementsSurfaceChromeDriverDom(root: HTMLElement): void {
  delete root.dataset.uiDriver;
  delete root.dataset.uiLabels;
  delete root.dataset.uiDrag;
  delete root.dataset.uiChromeReveal;
  delete root.dataset.uiGumballReveal;
  delete root.dataset.uiTooltips;
  teardownUiChromeRevealController(root);
}

// #region 🫥️ChromeReveal
/** @emoji 🫥️ Extra radius (px) around a reveal region's own rect that still counts as "inside" — makes the invisible-until-hovered bar reachable. */
const CHROME_REVEAL_ACTIVATION_BAND_PX = 24;
/** @emoji 🫥️ Screen-edge band (px) that reveals a region anchored to that edge (navbar top, footer bottom), even before the cursor reaches the region's own rect. */
const CHROME_REVEAL_EDGE_BAND_PX = 8;

/** @emoji 🐚️ One independent reveal controller per surface-chrome root — was a single page-global
 * controller, which meant hovering ANY mounted shell revealed hover-reveal chrome for EVERY shell that
 * had opted into it (and a pointer-move over shell B's DOM would drive shell A's reveal state). */
const chromeRevealBindingsByRoot = ephemeralMap<HTMLElement, ReturnType<typeof createDOMEventBinding>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealBindingsByRoot");
const chromeRevealFrameByRoot = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealFrameByRoot");
const chromeRevealLastPointByRoot = ephemeralMap<HTMLElement, { x: number; y: number }>("framework.modules.ui.packages.typescript.targets.react.index.tsx.chromeRevealLastPointByRoot");

function chromeRevealStackAncestor(region: HTMLElement): HTMLElement | null {
  return region.closest<HTMLElement>('[data-slot="window-chrome-stack"], [data-slot="mode-dock-stack"]');
}

function chromeRevealRegionRevealed(region: HTMLElement, x: number, y: number): boolean {
  const rect = region.getBoundingClientRect();
  if (x >= rect.left - CHROME_REVEAL_ACTIVATION_BAND_PX && x <= rect.right + CHROME_REVEAL_ACTIVATION_BAND_PX && y >= rect.top - CHROME_REVEAL_ACTIVATION_BAND_PX && y <= rect.bottom + CHROME_REVEAL_ACTIVATION_BAND_PX) {
    return true;
  }
  const regionName = region.dataset.uiRevealRegion;
  if (regionName === "navbar" && y <= CHROME_REVEAL_EDGE_BAND_PX) return true;
  if (regionName === "footer" && typeof window !== "undefined" && y >= window.innerHeight - CHROME_REVEAL_EDGE_BAND_PX) return true;
  if (regionName === "window-cap") {
    const stack = chromeRevealStackAncestor(region);
    if (stack) {
      const stackRect = stack.getBoundingClientRect();
      if (x >= stackRect.left && x <= stackRect.right && y >= stackRect.top && y <= stackRect.top + CHROME_REVEAL_EDGE_BAND_PX) return true;
    }
  }
  return false;
}

function applyChromeRevealAtPoint(root: HTMLElement, x: number, y: number): void {
  root.querySelectorAll<HTMLElement>("[data-ui-reveal-region]").forEach((region) => {
    if (chromeRevealRegionRevealed(region, x, y)) region.dataset.uiRevealed = "true";
    else delete region.dataset.uiRevealed;
  });
}

function scheduleChromeRevealUpdate(root: HTMLElement): void {
  if (chromeRevealFrameByRoot.has(root) || typeof requestAnimationFrame === "undefined") return;
  const frame = requestAnimationFrame(() => {
    chromeRevealFrameByRoot.delete(root);
    const point = chromeRevealLastPointByRoot.get(root);
    if (point) applyChromeRevealAtPoint(root, point.x, point.y);
  });
  chromeRevealFrameByRoot.set(root, frame);
}

function ensureUiChromeRevealController(root: HTMLElement): void {
  if (chromeRevealBindingsByRoot.has(root) || typeof window === "undefined") return;
  const bindings = createDOMEventBinding();
  bindings.listen(window, "pointermove", (event: PointerEvent) => {
    // 🐚️ `pointermove` only ever bubbles to `window` (never scoped to a subtree), so this root only
    // reacts to points actually over its own DOM — otherwise hovering shell B would reveal shell A's chrome.
    if (!(event.target instanceof globalThis.Node) || !root.contains(event.target)) return;
    chromeRevealLastPointByRoot.set(root, { x: event.clientX, y: event.clientY });
    scheduleChromeRevealUpdate(root);
  });
  bindings.listen(root, "focusin", (event: FocusEvent) => {
    const region = (event.target as HTMLElement | null)?.closest<HTMLElement>("[data-ui-reveal-region]");
    if (region) region.dataset.uiRevealed = "true";
  });
  bindings.listen(root, "focusout", (event: FocusEvent) => {
    const region = (event.target as HTMLElement | null)?.closest<HTMLElement>("[data-ui-reveal-region]");
    const related = event.relatedTarget as globalThis.Node | null;
    if (region && (!related || !region.contains(related))) delete region.dataset.uiRevealed;
  });
  chromeRevealBindingsByRoot.set(root, bindings);
}

function teardownUiChromeRevealController(root: HTMLElement): void {
  chromeRevealBindingsByRoot.get(root)?.dispose();
  chromeRevealBindingsByRoot.delete(root);
  const frame = chromeRevealFrameByRoot.get(root);
  if (frame !== undefined && typeof cancelAnimationFrame !== "undefined") cancelAnimationFrame(frame);
  chromeRevealFrameByRoot.delete(root);
  chromeRevealLastPointByRoot.delete(root);
  root.querySelectorAll<HTMLElement>("[data-ui-reveal-region][data-ui-revealed]").forEach((region) => delete region.dataset.uiRevealed);
}

/** @emoji 🫥️ Ensures the pointer/focus reveal tracker is installed iff the driver wants hover-reveal chrome; called whenever driver DOM attrs are (re)applied. */
function syncUiChromeRevealController(root: HTMLElement, chrome: UiDriverReveal): void {
  if (chrome === "hover") ensureUiChromeRevealController(root);
  else teardownUiChromeRevealController(root);
}
// #endregion 🫥️ChromeReveal

function clearElementsSurfaceChromeDom(root: HTMLElement): void {
  root.classList.remove("dark");
  root.classList.remove("touch");
  delete root.dataset.uiDevice;
  clearElementsSurfaceChromeDriverDom(root);
  delete root.dataset.uiAppearance;
  clearElementsSurfaceChromeBaseColors(root);
}

function applyElementsSurfaceChromeAppearanceDom(root: HTMLElement, appearance: ElementsSurfaceAppearance): void {
  const dark = resolveElementsSurfaceChromeDark(appearance);
  root.classList.toggle("dark", dark);
  root.dataset.uiAppearance = dark ? "dark" : "light";
  applyElementsSurfaceChromeBaseColors(root, dark ? "dark" : "light");
}

/** @emoji 🌓️ Applies `.dark`/`color-scheme` to a root (`document.documentElement` by default) before
 * React/CSS load (play/static entries); does not register a surface-chrome lease. */
export function bootstrapElementsSurfaceChromeDocument(appearance: ElementsSurfaceAppearance = "system", rootOverride?: HTMLElement): void {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  if (!root) return;
  cancelElementsSurfaceChromeDeferredClear(root);
  applyElementsSurfaceChromeAppearanceDom(root, appearance);
}

function cancelElementsSurfaceChromeDeferredClear(root: HTMLElement): void {
  const frame = elementsSurfaceChromeDeferredClearFrames.get(root);
  if (frame === undefined || typeof cancelAnimationFrame === "undefined") {
    elementsSurfaceChromeDeferredClearFrames.delete(root);
    return;
  }
  cancelAnimationFrame(frame);
  elementsSurfaceChromeDeferredClearFrames.delete(root);
}

function scheduleElementsSurfaceChromeDeferredClear(root: HTMLElement): void {
  if (typeof requestAnimationFrame === "undefined") {
    if (!elementsSurfaceChromeLeasesByRoot.has(root)) clearElementsSurfaceChromeDom(root);
    return;
  }
  cancelElementsSurfaceChromeDeferredClear(root);
  const frame = requestAnimationFrame(() => {
    elementsSurfaceChromeDeferredClearFrames.delete(root);
    if (!elementsSurfaceChromeLeasesByRoot.has(root)) clearElementsSurfaceChromeDom(root);
  });
  elementsSurfaceChromeDeferredClearFrames.set(root, frame);
}

function applyElementsSurfaceChromeDom(root: HTMLElement, input: ElementsSurfaceChromeInput): void {
  cancelElementsSurfaceChromeDeferredClear(root);
  applyElementsSurfaceChromeAppearanceDom(root, input.appearance);
  root.dataset.uiDevice = input.device;
  root.classList.toggle("touch", input.device !== "desktop");
  applyElementsSurfaceChromeDriverDom(root, input.driver);
}

function syncElementsSurfaceChromeDomFromLeaseStack(root: HTMLElement): void {
  const input = activeElementsSurfaceChromeInput(root);
  if (!input) {
    clearElementsSurfaceChromeDom(root);
    return;
  }
  applyElementsSurfaceChromeDom(root, input);
}

/** @emoji 🐚️ One shared `matchMedia` listener re-applies EVERY root with an active `appearance: "system"`
 * lease when the OS preference flips — the media query itself is genuinely page-global (there is only
 * one system preference), but each root's lease stack (and therefore whether it even has a "system"
 * lease) stays independent. */
function ensureElementsSurfaceChromeSystemListeners(): void {
  if (elementsSurfaceChromeSystemListenersInstalled.current || typeof window === "undefined" || typeof document === "undefined") {
    return;
  }
  elementsSurfaceChromeSystemListenersInstalled.current = true;
  const bindings = createDOMEventBinding();
  if (typeof window.matchMedia !== "function") {
    installElementsSurfaceBrowserDefaultSuppression(bindings);
    elementsSurfaceChromeDomBindings.current = bindings;
    return;
  }
  const mq = window.matchMedia("(prefers-color-scheme: dark)");
  const onSystemAppearanceChange = (): void => {
    for (const [root, leases] of elementsSurfaceChromeLeasesByRoot) {
      const input = leases[leases.length - 1]?.input;
      if (!input || input.appearance !== "system") continue;
      applyElementsSurfaceChromeDom(root, input);
    }
  };
  bindings.listen(mq, "change", onSystemAppearanceChange);
  installElementsSurfaceBrowserDefaultSuppression(bindings);
  elementsSurfaceChromeDomBindings.current = bindings;
}

/**
 * @emoji 🌈️ Imperative surface chrome controller for class-based shells; returns a cleanup that reverts
 * DOM state, browser default input, and the active driver. `rootOverride` scopes this lease to one
 * shell's own root (e.g. its `ShellScope.rootRef`) — omitted, it falls back to `document.documentElement`
 * (the single-shell-per-page case, unchanged from before this was made root-scoped).
 */
export function applyElementsSurfaceChrome(input: ElementsSurfaceChromeInput, rootOverride?: HTMLElement): () => void {
  const root = resolveElementsSurfaceChromeRoot(rootOverride);
  if (!root) return () => {};
  const lease: ElementsSurfaceChromeLease = { id: ++elementsSurfaceChromeLeaseSeq.current, input };
  const leases = elementsSurfaceChromeLeasesByRoot.get(root) ?? [];
  leases.push(lease);
  elementsSurfaceChromeLeasesByRoot.set(root, leases);
  ensureElementsSurfaceChromeSystemListeners();
  syncElementsSurfaceChromeProviders(input);
  syncElementsSurfaceChromeDomFromLeaseStack(root);
  return () => {
    const current = elementsSurfaceChromeLeasesByRoot.get(root);
    const index = current?.findIndex((entry) => entry.id === lease.id) ?? -1;
    if (current && index >= 0) current.splice(index, 1);
    if (!current || current.length === 0) {
      elementsSurfaceChromeLeasesByRoot.delete(root);
      syncElementsSurfaceChromeProviders(undefined);
      scheduleElementsSurfaceChromeDeferredClear(root);
      return;
    }
    syncElementsSurfaceChromeProviders(activeElementsSurfaceChromeInput(root));
    syncElementsSurfaceChromeDomFromLeaseStack(root);
  };
}

/**
 * @emoji 🌓️ Syncs a surface-chrome root (`dark`, `touch`, `data-ui-device`, `data-ui-driver` + axis
 * attrs), base colors, and {@link setUiDriverProvider}; returns `mobile` for {@link AppProps.mobile}.
 * `root` scopes this to one shell — omitted, targets `document.documentElement` as before. Callers reading
 * this from a ref (e.g. `ShellScope.rootRef.current`) must re-render once that ref attaches (`FrameworkOsShell`
 * bumps state in its callback ref for exactly this) — a ref OBJECT in this hook's own deps would never
 * re-trigger the effect once populated, since the object's identity never changes.
 */
export function useElementsSurfaceChrome({ appearance, device, driver }: ElementsSurfaceChromeInput, root?: HTMLElement): { mobile: boolean } {
  reactHostPort.useLayoutEffect(() => applyElementsSurfaceChrome({ appearance, device, driver }, root), [appearance, device, driver, root]);

  return { mobile: device === "mobile" };
}

/**
 * @emoji 🌓️ Observes a surface-chrome root's appearance attributes and runs `sync` on mount and whenever
 * they change. Holds `sync` in a ref so callers can pass an inline arrow without retriggering the effect
 * every render (React 19: unstable `sync` identity → effect → `paintOverlays`/`setState` → re-render →
 * Maximum update depth). `root` scopes the observed element — omitted, observes `document.documentElement`;
 * see {@link useElementsSurfaceChrome}'s doc for why this takes a resolved element, not a ref.
 */
export function useCanvasAppearanceSync(sync: () => void, enabled = true, root?: HTMLElement): void {
  const syncRef = reactHostPort.useRef(sync);
  syncRef.current = sync;
  reactHostPort.useEffect(() => {
    const observedRoot = resolveElementsSurfaceChromeRoot(root);
    if (!enabled || !observedRoot || typeof MutationObserver === "undefined") return;
    const run = () => syncRef.current();
    run();
    const observer = new MutationObserver(run);
    observer.observe(observedRoot, { attributes: true, attributeFilter: ["class", "style", "data-ui-appearance", "data-ui-theme"] });
    return () => observer.disconnect();
  }, [enabled, root]);
}

/** @emoji 🧪️ Clears every surface-chrome root's leases and DOM overrides between vitest cases (tests only
 * ever exercise the default `document.documentElement` root, but this clears all of them defensively). */
export function resetElementsSurfaceChromeForTests(): void {
  for (const root of elementsSurfaceChromeDeferredClearFrames.keys()) cancelElementsSurfaceChromeDeferredClear(root);
  for (const root of elementsSurfaceChromeLeasesByRoot.keys()) clearElementsSurfaceChromeDom(root);
  elementsSurfaceChromeLeasesByRoot.clear();
  syncElementsSurfaceChromeProviders(undefined);
  const root = resolveElementsSurfaceChromeRoot();
  if (root) clearElementsSurfaceChromeDom(root);
}

// #region 🎛️UiChromePrefs

/** @emoji 🌓️ Storage key for surface appearance (system/light/dark). */
export const UI_CHROME_APPEARANCE_STORAGE_KEY = "ui.chrome.appearance";

/** @emoji 🌓️ Reads persisted surface appearance from the given shell's storage — a required param
 * (not a `localStorage` default) since two shells on one page must never read/write each other's
 * appearance through a shared key. */
export function readStoredUiChromeAppearance(storage: StoragePort): ElementsSurfaceAppearance {
  const raw = storage.get(UI_CHROME_APPEARANCE_STORAGE_KEY);
  if (raw === "light" || raw === "dark" || raw === "system") return raw;
  return "system";
}

/** @emoji 🌓️ Persists surface appearance to the given shell's storage. */
export function writeStoredUiChromeAppearance(storage: StoragePort, appearance: ElementsSurfaceAppearance): void {
  storage.set(UI_CHROME_APPEARANCE_STORAGE_KEY, appearance);
}

/** @emoji 📐️ User-selectable layout device; mobile is automatic and excluded here. */
export type UiChromeLayout = "desktop" | "tablet";

/** @emoji 📐️ Storage key for the user-selected desktop/tablet layout. */
export const UI_CHROME_LAYOUT_STORAGE_KEY = "ui.chrome.layout";

/** @emoji 📐️ Reads the persisted layout preference from the given shell's storage, defaulting to desktop. */
export function readStoredUiChromeLayout(storage: StoragePort): UiChromeLayout {
  return storage.get(UI_CHROME_LAYOUT_STORAGE_KEY) === "tablet" ? "tablet" : "desktop";
}

/** @emoji 📐️ Persists the layout preference to the given shell's storage. */
export function writeStoredUiChromeLayout(storage: StoragePort, layout: UiChromeLayout): void {
  storage.set(UI_CHROME_LAYOUT_STORAGE_KEY, layout);
}

/** @emoji 🌐️ Storage key for the active UI locale. */
export const UI_CHROME_LOCALE_STORAGE_KEY = "ui.chrome.locale";

/** @emoji 🌐️ Reads the persisted UI locale from the given shell's storage, if any. */
export function readStoredUiChromeLocale(storage: StoragePort): UiLocale | null {
  const raw = storage.get(UI_CHROME_LOCALE_STORAGE_KEY);
  return raw === "en" || raw === "de" ? raw : null;
}

/** @emoji 🌐️ Persists the active UI locale to the given shell's storage. */
export function writeStoredUiChromeLocale(storage: StoragePort, locale: UiLocale): void {
  storage.set(UI_CHROME_LOCALE_STORAGE_KEY, locale);
}

/** @emoji 🗣️ Id of the always-available default terminology (no term substitutions). */
export const UI_TERMINOLOGY_NATIVE = "native";

/** @emoji 🗣️ Storage key for the active app terminology id. */
export const UI_CHROME_TERMINOLOGY_STORAGE_KEY = "ui.chrome.terminology";

/** @emoji 🗣️ Reads the persisted terminology id from the given shell's storage, defaulting to native. */
export function readStoredUiChromeTerminology(storage: StoragePort): string {
  return storage.get(UI_CHROME_TERMINOLOGY_STORAGE_KEY) || UI_TERMINOLOGY_NATIVE;
}

/** @emoji 🗣️ Persists the active terminology id to the given shell's storage. */
export function writeStoredUiChromeTerminology(storage: StoragePort, id: string): void {
  storage.set(UI_CHROME_TERMINOLOGY_STORAGE_KEY, id);
}

/** @emoji 🎨️ Storage key for the active theme id (builtin or `custom.<slug>`). */
export const UI_CHROME_THEME_ID_STORAGE_KEY = "ui.chrome.theme";

/** @emoji 🎨️ Reads the persisted active theme id from the given shell's storage, if any. */
export function readStoredUiChromeThemeId(storage: StoragePort): string | null {
  return storage.get(UI_CHROME_THEME_ID_STORAGE_KEY);
}

/** @emoji 🎨️ Persists the active theme id to the given shell's storage. */
export function writeStoredUiChromeThemeId(storage: StoragePort, id: string): void {
  storage.set(UI_CHROME_THEME_ID_STORAGE_KEY, id);
}

/** @emoji 🎨️ Storage key for a full snapshot of the active theme (boot-time fallback before builtin/custom lookup resolves). */
export const UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY = "ui.chrome.theme.snapshot";

/** @emoji 🎨️ Reads the persisted active theme snapshot from the given shell's storage; discards it silently if invalid. */
export function readStoredUiChromeThemeSnapshot(storage: StoragePort): UiTheme | null {
  const raw = storage.get(UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY);
  if (!raw) return null;
  try {
    return parseUiTheme(JSON.parse(raw));
  } catch {
    return null;
  }
}

/** @emoji 🎨️ Persists a full snapshot of the active theme to the given shell's storage. */
export function writeStoredUiChromeThemeSnapshot(storage: StoragePort, theme: UiTheme): void {
  storage.set(UI_CHROME_THEME_SNAPSHOT_STORAGE_KEY, serializeUiTheme(theme));
}

/** @emoji 🎨️ Storage key for the user's saved custom themes, keyed by theme id. */
export const UI_CUSTOM_THEMES_STORAGE_KEY = "ui.themes.custom";

/** @emoji 🎨️ Reads the user's saved custom themes from the given shell's storage; discards any entry that fails to parse. */
export function readStoredUiCustomThemes(storage: StoragePort): Record<string, UiTheme> {
  const raw = storage.get(UI_CUSTOM_THEMES_STORAGE_KEY);
  if (!raw) return {};
  try {
    const parsed = JSON.parse(raw) as Record<string, unknown>;
    const out: Record<string, UiTheme> = {};
    for (const [id, value] of Object.entries(parsed)) {
      try {
        out[id] = parseUiTheme(value);
      } catch {
        /* drop invalid saved theme */
      }
    }
    return out;
  } catch {
    return {};
  }
}

/** @emoji 🎨️ Persists the user's saved custom themes to the given shell's storage. */
export function writeStoredUiCustomThemes(storage: StoragePort, themes: Record<string, UiTheme>): void {
  storage.set(UI_CUSTOM_THEMES_STORAGE_KEY, JSON.stringify(themes));
}

/** @emoji 🧵️ Storage key for WASM compute worker thread count (`ui.chrome.*` namespace). */
export const UI_COMPUTE_WORKER_COUNT_STORAGE_KEY = "ui.compute.workerCount";

/** @emoji 🧵️ Default compute workers: `navigator.hardwareConcurrency` or 1. */
export function defaultComputeWorkerCount(): number {
  if (typeof navigator !== "undefined" && typeof navigator.hardwareConcurrency === "number" && navigator.hardwareConcurrency > 0) {
    return navigator.hardwareConcurrency;
  }
  return 1;
}

/** @emoji 🧵️ Reads persisted compute worker count from the given shell's storage. */
export function readStoredComputeWorkerCount(storage: StoragePort): number {
  const raw = storage.get(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY);
  if (raw == null || raw === "") return defaultComputeWorkerCount();
  const parsed = Number.parseInt(raw, 10);
  if (!Number.isFinite(parsed) || parsed < 1) return defaultComputeWorkerCount();
  return parsed;
}

/** @emoji 🧵️ Persists compute worker count to the given shell's storage. */
export function writeStoredComputeWorkerCount(storage: StoragePort, count: number): void {
  storage.set(UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, String(Math.max(1, Math.floor(count))));
}

/** @emoji 🧵️ True when SharedArrayBuffer thread pools are available. */
export function isCrossOriginIsolatedRuntime(): boolean {
  return typeof crossOriginIsolated !== "undefined" && crossOriginIsolated === true;
}

/** @emoji 🧵️ Effective worker count after cross-origin isolation fallback. */
export function effectiveComputeWorkerCount(storage: StoragePort, requested = readStoredComputeWorkerCount(storage)): number {
  if (!isCrossOriginIsolatedRuntime()) return 1;
  return Math.max(1, Math.floor(requested));
}

/** @emoji 🎓️ Storage key prefix for whether an app's introduction has already been shown on this device. */
export const UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX = "ui.introduction.seen.";

/** @emoji 🎓️ Reads whether `appId`'s introduction has already been shown — auto-start checks this once
 * per app; replaying stays available via the `startIntroduction` action regardless of this flag. */
export function readStoredIntroductionSeen(storage: StoragePort, appId: string): boolean {
  if (!appId) return false;
  return storage.get(`${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${appId}`) === "true";
}

/** @emoji 🎓️ Marks `appId`'s introduction as shown so it stops auto-starting on future launches. */
export function writeStoredIntroductionSeen(storage: StoragePort, appId: string): void {
  if (!appId) return;
  storage.set(`${UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX}${appId}`, "true");
}

import {
  WINDOW_SILHOUETTE_GEOMETRY_SCHEMA,
  WINDOW_SILHOUETTE_PATH_INSET,
  WINDOW_SILHOUETTE_CHIP_EPSILON,
  normalizeWindowSilhouetteChips,
  normalizeWindowSilhouetteMetrics,
  windowSilhouetteEdgePoints,
  windowSilhouetteEdgePointsRtl,
  windowSilhouetteOutline,
  simplifyWindowSilhouetteOutline,
  windowSilhouetteOutlineViolations,
  windowSilhouettePathFromOutline,
  windowSilhouettePath,
  windowSilhouetteContentClipPath,
  windowSilhouetteSafeClearances,
  windowSilhouetteBodyRegion,
  windowSilhouetteGlassRegions,
  windowSilhouetteContentRegions,
  windowSilhouetteRegionContains,
  windowSilhouetteContains,
  pendingWindowSilhouetteMetrics,
  createWindowSilhouetteGeometry,
  type WindowSilhouetteChip,
  type WindowSilhouetteEdge,
  type WindowSilhouetteMetrics,
  type WindowSilhouettePoint,
  type WindowSilhouetteDock,
  type WindowSilhouetteRegion,
  type WindowSilhouetteSafeClearances,
  type PendingWindowSilhouetteMetrics,
  type WindowSilhouetteGeometry,
} from "../../../../🧱️elements/🔲️WindowSilhouette/🟦️.tsx";
export {
  WINDOW_SILHOUETTE_GEOMETRY_SCHEMA,
  WINDOW_SILHOUETTE_PATH_INSET,
  WINDOW_SILHOUETTE_CHIP_EPSILON,
  normalizeWindowSilhouetteChips,
  normalizeWindowSilhouetteMetrics,
  windowSilhouetteEdgePoints,
  windowSilhouetteEdgePointsRtl,
  windowSilhouetteOutline,
  simplifyWindowSilhouetteOutline,
  windowSilhouetteOutlineViolations,
  windowSilhouettePathFromOutline,
  windowSilhouettePath,
  windowSilhouetteContentClipPath,
  windowSilhouetteSafeClearances,
  windowSilhouetteBodyRegion,
  windowSilhouetteGlassRegions,
  windowSilhouetteContentRegions,
  windowSilhouetteRegionContains,
  windowSilhouetteContains,
  pendingWindowSilhouetteMetrics,
  createWindowSilhouetteGeometry,
  type WindowSilhouetteChip,
  type WindowSilhouetteEdge,
  type WindowSilhouetteMetrics,
  type WindowSilhouettePoint,
  type WindowSilhouetteDock,
  type WindowSilhouetteRegion,
  type WindowSilhouetteSafeClearances,
  type PendingWindowSilhouetteMetrics,
  type WindowSilhouetteGeometry,
};
import { ChromeControlHint } from "../../../../🧱️elements/💡️ChromeControlHint/🟦️.tsx";
export { ChromeControlHint };
// #endregion 🎛️UiChromeCompact

// #endregion 🌈️SurfaceChrome

// #region 🪁️I18n Resources

// Domain-neutral UI translation bundles (settings, tooltip, generic shell `ui.*` ids).
// Product-specific bundles (e.g. compose sketchpad) register via {@link registerUiTranslationBundles}.

// #region 🔑️Schema & Keys
// Type/key-derivation machinery: locale codes, label shapes, the deep dot-path key type, and compile-time key-coverage checks.

/** @emoji 🪁️ Supported UI locale codes — the single source is `@semio-tech/framework`'s
 * `ShellLocale`, so a brand's `locks.locale` and this chrome bundle's coverage can never drift apart. */
// #region UiLabel
import { uiDataLabel, type UiLabel } from "../../../../🧱️elements/🎗️UiLabel/🟦️.tsx";
export { uiDataLabel, type UiLabel };
// #endregion UiLabel

import {
  type UiLocale,
  type UiLabelPair,
  type UiLabelValue,
  type UiRibbonParentCategory,
  type UiRibbonParentEntries,
  type DeepUiTranslationKeys,
  type UiTranslationSchema,
  type UiTranslationKey,
  type AssertUiRibbonParentKeysCovered,
  type AssertUiSettingsLanguageKeysCovered,
  type UiChromeTerminologyId,
  type AssertUiSettingsTerminologyKeysCovered,
  type UiTranslateFn,
  type UiI18nPort,
  type UiRegisteredTranslationKey,
  UI_RIBBON_PARENT_CATEGORIES,
} from "../../../../🧱️elements/📚️I18n/🟦️.tsx";
export type {
  UiLocale,
  UiLabelPair,
  UiLabelValue,
  UiRibbonParentCategory,
  UiRibbonParentEntries,
  DeepUiTranslationKeys,
  UiTranslationSchema,
  UiTranslationKey,
  AssertUiRibbonParentKeysCovered,
  AssertUiSettingsLanguageKeysCovered,
  UiChromeTerminologyId,
  AssertUiSettingsTerminologyKeysCovered,
  UiTranslateFn,
  UiI18nPort,
  UiRegisteredTranslationKey,
};
export { UI_RIBBON_PARENT_CATEGORIES };

const _assertUiRibbonParentKeys: AssertUiRibbonParentKeysCovered<UiRibbonParentCategory> = true;

const _assertUiSettingsLanguageKeys: AssertUiSettingsLanguageKeysCovered<UiLocale> = true;

const _assertUiSettingsTerminologyKeys: AssertUiSettingsTerminologyKeysCovered<UiChromeTerminologyId> = true;

/** @emoji 🏷️ Compile-time check that every brand-lockable {@link ShellLocale} has a complete chrome bundle — closes the loop from a brand's `locks.locale` through `ShellLocale` to an actual translated `uiChromeTranslationBundles` entry. */
type AssertShellBrandLocalesBundled<L extends string> = L extends keyof typeof uiChromeTranslationBundles ? true : false;
const _assertShellBrandLocalesBundled: AssertShellBrandLocalesBundled<ShellLocale> = true;

// #endregion 🔑️Schema & Keys

// #region 🇩️🇪️ German Bundle
// German (`de`) translation bundle: ribbon-parent labels here, plus the nested `de` translation tree further below
// inside {@link uiChromeTranslationBundles} (kept as one object so both locales satisfy the same schema).

const uiRibbonParentDe: UiRibbonParentEntries = {
  history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
  hand: { label: { normal: "Hand", beginner: "Hand" } },
  selection: { label: { normal: "Auswahl", beginner: "Auswahl" } },
  lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
  filter: { label: { normal: "Filter", beginner: "Filter" } },
  open: { label: { normal: "Öffnen", beginner: "Öffnen" } },
  save: { label: { normal: "Speichern", beginner: "Speichern" } },
  transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
  transform: { label: { normal: "Transformieren", beginner: "Transformieren" } },
  create: { label: { normal: "Erstellen", beginner: "Erstellen" } },
  view: { label: { normal: "Ansicht", beginner: "Ansicht" } },
  actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
  settings: { label: { normal: "Einstellungen", beginner: "Einstellungen" } },
  methods: { label: { normal: "Methoden", beginner: "Methoden" } },
  mode: { label: { normal: "Modus", beginner: "Modus" } },
  targets: { label: { normal: "Ziele", beginner: "Ziele" } },
  export: { label: { normal: "Export", beginner: "Export" } },
  tools: { label: { normal: "Werkzeuge", beginner: "Werkzeuge" } },
  utilities: { label: { normal: "Hilfsmittel", beginner: "Hilfsmittel" } },
  sync: { label: { normal: "Sync", beginner: "Sync" } },
};

// #endregion 🇩️🇪️ German Bundle

// #region 🇬️🇧️ English Bundle
// English (`en`) translation bundle: ribbon-parent labels here, plus the nested `en` translation tree further below
// inside {@link uiChromeTranslationBundles} (kept as one object so both locales satisfy the same schema).

const uiRibbonParentEn: UiRibbonParentEntries = {
  history: { label: { normal: "History", beginner: "History" } },
  hand: { label: { normal: "Hand", beginner: "Hand" } },
  selection: { label: { normal: "Selection", beginner: "Selection" } },
  lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
  filter: { label: { normal: "Filter", beginner: "Filter" } },
  open: { label: { normal: "Open", beginner: "Open" } },
  save: { label: { normal: "Save", beginner: "Save" } },
  transfer: { label: { normal: "Transfer", beginner: "Transfer" } },
  transform: { label: { normal: "Transform", beginner: "Transform" } },
  create: { label: { normal: "Create", beginner: "Create" } },
  view: { label: { normal: "View", beginner: "View" } },
  actions: { label: { normal: "Actions", beginner: "Actions" } },
  settings: { label: { normal: "Settings", beginner: "Settings" } },
  methods: { label: { normal: "Methods", beginner: "Methods" } },
  mode: { label: { normal: "Mode", beginner: "Mode" } },
  targets: { label: { normal: "Targets", beginner: "Targets" } },
  export: { label: { normal: "Export", beginner: "Export" } },
  tools: { label: { normal: "Tools", beginner: "Tools" } },
  utilities: { label: { normal: "Utilities", beginner: "Utilities" } },
  sync: { label: { normal: "Sync", beginner: "Sync" } },
};

// #endregion 🇬️🇧️ English Bundle

export const uiChromeTranslationBundles = {
  // #region 🇩️🇪️ German Bundle
  de: {
    translation: {
      ui: {
        nav: {
          back: {
            label: {
              normal: "Zurück",
              beginner: "Zurück",
            },
          },
          forward: {
            label: {
              normal: "Vorwärts",
              beginner: "Vorwärts",
            },
          },
          up: {
            label: {
              normal: "Eine Ebene hoch",
              beginner: "Eine Ebene hoch",
            },
          },
        },
        search: {
          toggle: {
            label: {
              normal: "Suche",
              beginner: "Suche",
            },
          },
          close: {
            label: {
              normal: "Suche schließen",
              beginner: "Suche schließen",
            },
          },
          title: {
            label: {
              normal: "Suche",
              beginner: "Suche",
            },
          },
          description: {
            label: {
              normal: "Nach Elementen suchen",
              beginner: "Nach Elementen suchen",
            },
          },
          placeholder: {
            label: {
              normal: "Suchen...",
              beginner: "Suchen...",
            },
          },
          empty: {
            label: {
              normal: "Keine Ergebnisse gefunden.",
              beginner: "Keine Ergebnisse gefunden.",
            },
          },
          category: {
            panels: { label: { normal: "Panels", beginner: "Panels" } },
            windows: { label: { normal: "Fenster", beginner: "Fenster" } },
            catalogue: { label: { normal: "Katalog", beginner: "Katalog" } },
            // 🏠️ "Space" here is a deliberate, deferred duplicate of the host plugin's own manifest label
            // (`App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space"))`,
            // ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:869) — reading it from there via
            // `resolveManifestLabel(hostApp.label, …)` (the same pattern `appWindowLabel` already uses)
            // is the correct fix, EXCEPT `ShellHost/🟦️.tsx`'s `hostApp` lookup (line ~1132,
            // `manifest.apps.find(app => app.id === hostConfig?.hostAppId)`) is ALREADY always
            // `undefined`: `hostConfig.hostAppId` is the raw Cargo.toml `host = { shell = "studio" }`
            // alias, never the real dialect-derived `AppDefinition.id`
            // (`s.space.studio@1/*#editor`) — a pre-existing bug the same file's own w4-h comment
            // (lines 4112-4121) already documents and declines to fix. Wiring this label to
            // `hostApp?.label` today would render an EMPTY category header, not "Space" — a regression.
            // Fix plan (needs a new field, not a string-matching workaround): add
            // `AppDefinition.host_role: Option<HostRole>` (`Landing`/`Host`) in
            // `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (~3034), a `.host_role(...)` builder
            // method on `AppBuilder`/forwarded by `EditorBuilder`/`ViewerBuilder`
            // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), set it in
            // `create_home_app()`/`create_space_app()`, regenerate the TS mirror, then have
            // `ShellHost/🟦️.tsx:1132-1133` match on `hostRole` instead of the broken id
            // comparison. See
            // `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/STUBS-AND-PLACEHOLDERS-COMPLETION/📓️hostapp-label-layering.md`.
            hostApp: { label: { normal: "Space", beginner: "Space" } },
            navigation: { label: { normal: "Navigation", beginner: "Navigation" } },
          },
        },
        palette: {
          undo: { label: { normal: "Rückgängig", beginner: "Rückgängig" } },
          redo: { label: { normal: "Wiederholen", beginner: "Wiederholen" } },
          goHome: { label: { normal: "Nach Hause", beginner: "Nach Hause" } },
          spawnPrefix: { label: { normal: "Erzeugen", beginner: "Erzeugen" } },
        },
        panel: {
          artifact: { label: { normal: "Dokument", beginner: "Dokument" } },
          catalogue: { label: { normal: "Katalog", beginner: "Katalog" } },
          inspection: { label: { normal: "Inspektion", beginner: "Inspektion" } },
          parameters: { label: { normal: "Parameter", beginner: "Parameter" } },
          artifactEmpty: { label: { normal: "—", beginner: "—" } },
          spawnedAppsSuffix: { label: { normal: "gestartete App(s)", beginner: "gestartete App(s)" } },
          sync: { label: { normal: "Synchronisierung", beginner: "Synchronisierung" } },
          actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
          history: { label: { normal: "Verlauf", beginner: "Verlauf" } },
        },
        tree: {
          drag: {
            sort: { label: { normal: "Sortieren", beginner: "Zeile sortieren" } },
            sortTarget: { label: { normal: "Linksklick gedrückt halten, um {{target}} zu ziehen", beginner: "Linksklick gedrückt halten, um {{target}} zu ziehen" } },
            transfer: { label: { normal: "Verschieben", beginner: "In ein Fenster ziehen" } },
            transferTarget: { label: { normal: "Linksklick gedrückt halten, um {{target}} zu ziehen", beginner: "Linksklick gedrückt halten, um {{target}} zu ziehen" } },
          },
        },
        find: {
          toggle: {
            label: {
              normal: "Finden",
              beginner: "Im aktuellen Kontext finden",
            },
          },
          title: {
            label: {
              normal: "Finden",
              beginner: "Finden",
            },
          },
          description: {
            label: {
              normal: "Elemente in dieser Ansicht finden",
              beginner: "Elemente in dieser Ansicht finden",
            },
          },
          placeholder: {
            label: {
              normal: "Finden...",
              beginner: "Finden...",
            },
          },
          empty: {
            label: {
              normal: "Keine Ergebnisse gefunden.",
              beginner: "Keine Ergebnisse gefunden.",
            },
          },
        },
        fullscreen: {
          toggle: {
            label: {
              normal: "Vollbild",
              beginner: "Vollbild",
            },
          },
          exit: {
            label: {
              normal: "Vollbild beenden",
              beginner: "Vollbild beenden",
            },
          },
        },
        mobilePanel: {
          toggle: {
            label: {
              normal: "Panel",
              beginner: "Panel",
            },
          },
          app: {
            label: {
              normal: "App",
              beginner: "App",
            },
          },
        },
        panelToggle: {
          topLeft: {
            label: {
              normal: "Oben links",
              beginner: "Oben links",
            },
          },
          topRight: {
            label: {
              normal: "Oben rechts",
              beginner: "Oben rechts",
            },
          },
          bottomLeft: {
            label: {
              normal: "Unten links",
              beginner: "Unten links",
            },
          },
          bottomRight: {
            label: {
              normal: "Unten rechts",
              beginner: "Unten rechts",
            },
          },
          display: {
            label: {
              normal: "Anzeige",
              beginner: "Anzeige",
            },
          },
          command: {
            label: {
              normal: "Befehl",
              beginner: "Befehl",
            },
          },
          tool: {
            label: {
              normal: "Werkzeug",
              beginner: "Werkzeug",
            },
          },
          overview: {
            label: {
              normal: "Übersicht",
              beginner: "Übersicht",
            },
          },
          workbench: {
            label: {
              normal: "Arbeitsbereich",
              beginner: "Arbeitsbereich",
            },
          },
          details: {
            label: {
              normal: "Details",
              beginner: "Details",
            },
          },
          settings: {
            label: {
              normal: "Einstellungen",
              beginner: "Einstellungen",
            },
          },
          chat: {
            label: {
              normal: "Chat",
              beginner: "Chat",
            },
          },
          plugins: {
            label: {
              normal: "Plugins",
              beginner: "Plugins",
            },
          },
        },
        display: {
          tab: {
            windows: { label: { normal: "Fenster", beginner: "Fenster" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
          },
          saveLayout: { label: { normal: "Layout speichern", beginner: "Layout speichern" } },
          saveLayoutPlaceholder: { label: { normal: "Layoutname", beginner: "Layoutname" } },
          saveCurrentLayout: { label: { normal: "Aktuelles Layout speichern", beginner: "Aktuelles Layout speichern" } },
          deleteLayout: { label: { normal: "Löschen", beginner: "Löschen" } },
          emptyShell: {
            label: {
              normal: "Fenster aus Anzeige in der Navigationsleiste hierher ziehen oder ein gespeichertes Layout wiederherstellen.",
              beginner: "Fenster aus Anzeige in der Navigationsleiste hierher ziehen oder ein gespeichertes Layout wiederherstellen.",
            },
          },
          layouts: { label: { normal: "Layouts", beginner: "Layouts" } },
          saved: { label: { normal: "Gespeichert", beginner: "Gespeichert" } },
          unavailable: { label: { normal: "Anzeige nicht verfügbar", beginner: "Anzeige nicht verfügbar" } },
        },
        settings: {
          tab: {
            general: { label: { normal: "Allgemein", beginner: "Allgemein" } },
            driver: { label: { normal: "Treiber", beginner: "Treiber" } },
            app: { label: { normal: "App", beginner: "App" } },
            appearance: { label: { normal: "Design", beginner: "Design" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
            language: { label: { normal: "Sprache", beginner: "Sprache" } },
            terminology: { label: { normal: "Terminologie", beginner: "Terminologie" } },
            theme: { label: { normal: "Thema", beginner: "Thema" } },
            keybindings: { label: { normal: "Tastenkürzel", beginner: "Tastenkürzel" } },
          },
          appearance: {
            light: { label: { normal: "Hell", beginner: "Hell" } },
            dark: { label: { normal: "Dunkel", beginner: "Dunkel" } },
            system: { label: { normal: "System", beginner: "System" } },
          },
          language: {
            en: { label: { normal: "English", beginner: "English" } },
            de: { label: { normal: "Deutsch", beginner: "Deutsch" } },
          },
          terminology: {
            native: { label: { normal: "Nativ", beginner: "Nativ" } },
            reuse: { label: { normal: "Wiederverwendung", beginner: "Wiederverwendung" } },
          },
          app: {
            name: { label: { normal: "Name", beginner: "Name" } },
            id: { label: { normal: "App-ID", beginner: "App-ID" } },
            controller: { label: { normal: "Controller", beginner: "Controller" } },
            plugin: { label: { normal: "Plugin", beginner: "Plugin" } },
          },
          theme: {
            select: { label: { normal: "Thema", beginner: "Thema" } },
            save: { label: { normal: "Speichern unter", beginner: "Speichern unter" } },
            savePlaceholder: { label: { normal: "Themenname", beginner: "Themenname" } },
            reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
            export: { label: { normal: "Exportieren", beginner: "Exportieren" } },
            import: { label: { normal: "Importieren", beginner: "Importieren" } },
            delete: { label: { normal: "Löschen", beginner: "Löschen" } },
            colors: { label: { normal: "Farben", beginner: "Farben" } },
            spacing: { label: { normal: "Abstand", beginner: "Abstand" } },
            fonts: { label: { normal: "Schriftarten", beginner: "Schriftarten" } },
            strokes: { label: { normal: "Strichstärken", beginner: "Strichstärken" } },
            radii: { label: { normal: "Rundungen", beginner: "Rundungen" } },
            opacities: { label: { normal: "Deckkraft", beginner: "Deckkraft" } },
            metrics: { label: { normal: "Masse", beginner: "Masse" } },
            appearances: { label: { normal: "Erscheinungsbilder", beginner: "Erscheinungsbilder" } },
            dirty: { label: { normal: "Nicht gespeichert", beginner: "Nicht gespeichert" } },
            appearance: {
              light: { label: { normal: "Hell", beginner: "Hell" } },
              dark: { label: { normal: "Dunkel", beginner: "Dunkel" } },
            },
            group: {
              board: { label: { normal: "Board", beginner: "Board" } },
              map: { label: { normal: "Karte", beginner: "Karte" } },
              canvas: { label: { normal: "Leinwand", beginner: "Leinwand" } },
              chrome: { label: { normal: "Oberfläche", beginner: "Oberfläche" } },
            },
          },
          unavailable: { label: { normal: "Einstellungen nicht verfügbar", beginner: "Einstellungen nicht verfügbar" } },
          resetDock: { label: { normal: "Panels zurücksetzen", beginner: "Panels zurücksetzen" } },
        },
        plugins: {
          status: {
            available: { label: { normal: "Verfügbar", beginner: "Verfügbar" } },
            installing: { label: { normal: "Wird installiert…", beginner: "Wird installiert…" } },
            loaded: { label: { normal: "Geladen", beginner: "Geladen" } },
            failed: { label: { normal: "Fehlgeschlagen", beginner: "Fehlgeschlagen" } },
            reloading: { label: { normal: "Wird neu geladen…", beginner: "Wird neu geladen…" } },
          },
          action: {
            install: { label: { normal: "Installieren", beginner: "Installieren" } },
            uninstall: { label: { normal: "Deinstallieren", beginner: "Deinstallieren" } },
            reload: { label: { normal: "Neu laden", beginner: "Neu laden" } },
          },
          waitingForHost: { label: { normal: "Warte auf Host-Programm…", beginner: "Warte auf Host-Programm…" } },
          unavailable: { label: { normal: "Plugins nicht verfügbar", beginner: "Plugins nicht verfügbar" } },
          source: { label: { normal: "Quelle", beginner: "Quelle" } },
        },
        command: {
          introduceApp: { label: { normal: "App vorstellen", beginner: "App vorstellen" } },
          playTutorial: { label: { normal: "Tutorial abspielen", beginner: "Tutorial abspielen" } },
          recordTutorial: { label: { normal: "Tutorial aufnehmen", beginner: "Tutorial aufnehmen" } },
          setAppearance: { label: { normal: "Erscheinungsbild festlegen", beginner: "Erscheinungsbild festlegen" } },
          setTheme: { label: { normal: "Thema festlegen", beginner: "Thema festlegen" } },
          setLayout: { label: { normal: "Layout festlegen", beginner: "Layout festlegen" } },
          setLocale: { label: { normal: "Sprache festlegen", beginner: "Sprache festlegen" } },
          setTerminology: { label: { normal: "Terminologie festlegen", beginner: "Terminologie festlegen" } },
          setDriver: { label: { normal: "Treiber festlegen", beginner: "Treiber festlegen" } },
        },
        shellCommand: {
          dockMove: { label: { normal: "Panel-Tab verschieben", beginner: "Panel-Tab verschieben" } },
          windowResize: { label: { normal: "Fenster skalieren", beginner: "Fenster skalieren" } },
          windowMove: { label: { normal: "Fenster neu anordnen", beginner: "Fenster neu anordnen" } },
          windowActivate: { label: { normal: "Fenster aktivieren", beginner: "Fenster aktivieren" } },
          windowClose: { label: { normal: "Fenster schließen", beginner: "Fenster schließen" } },
          windowSplit: { label: { normal: "Fenster teilen", beginner: "Fenster teilen" } },
          windowOpenInNewWindow: { label: { normal: "In neuem Fenster öffnen", beginner: "In neuem Fenster öffnen" } },
          panelToggle: { label: { normal: "Panel umschalten", beginner: "Panel umschalten" } },
          panelTab: { label: { normal: "Panel-Tab wechseln", beginner: "Panel-Tab wechseln" } },
        },
        ribbon: {
          group: {
            parent: {
              label: {
                normal: "Hilfsmittel",
                beginner: "Hilfsmittel",
              },
            },
          },
          parent: uiRibbonParentDe,
        },
        selection: {
          method: { label: { normal: "Methode", beginner: "Methode" } },
          mode: { label: { normal: "Modus", beginner: "Modus" } },
          rectangle: { label: { normal: "Rechteck", beginner: "Rechteck" } },
          lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
          selective: { label: { normal: "Selektiv", beginner: "Selektiv" } },
          additive: { label: { normal: "Additiv", beginner: "Additiv" } },
          subtractive: { label: { normal: "Subtraktiv", beginner: "Subtraktiv" } },
          invertive: { label: { normal: "Invertierend", beginner: "Invertierend" } },
        },
        windowFault: {
          title: { label: { normal: "Fenster reagiert nicht", beginner: "Fenster reagiert nicht" } },
          abiMismatch: { label: { normal: "Plugin-Modul passt nicht zur Host-Schnittstelle", beginner: "Das Plugin ist veraltet und passt nicht mehr zum Programm" } },
          interactiveCeiling: { label: { normal: "Plugin-Schritt hat die interaktive Zeitgrenze überschritten", beginner: "Das Plugin hat für einen Schritt zu lange gebraucht" } },
          clock: { label: { normal: "Keine monotone Uhr verfügbar", beginner: "Die Zeitmessung des Systems steht nicht zur Verfügung" } },
          pluginInternal: { label: { normal: "Interner Plugin-Laufzeitfehler", beginner: "Im Plugin ist ein interner Fehler aufgetreten" } },
          installFailed: { label: { normal: "Plugin konnte nicht installiert werden", beginner: "Das Plugin liess sich nicht laden" } },
          unknown: { label: { normal: "Unbekannte Fehlerursache", beginner: "Die Ursache ist unbekannt" } },
        },
        common: {
          mixedValues: {
            label: {
              normal: "Gemischt",
              beginner: "Gemischt",
            },
          },
          name: { label: { normal: "Name", beginner: "Name" } },
          save: { label: { normal: "Speichern", beginner: "Speichern" } },
          loading: { label: { normal: "Lädt…", beginner: "Lädt…" } },
          loadingPlugins: { label: { normal: "Plugins werden geladen…", beginner: "Plugins werden geladen…" } },
          renderError: { label: { normal: "Renderfehler", beginner: "Renderfehler" } },
          noPluginsLoaded: { label: { normal: "Keine Plugins geladen", beginner: "Keine Plugins geladen" } },
          missingWindow: { label: { normal: "Fehlendes Fenster", beginner: "Fehlendes Fenster" } },
          home: { label: { normal: "Startseite", beginner: "Startseite" } },
          backToWorkflow: { label: { normal: "Zurück zum Workflow", beginner: "Zurück zum Workflow" } },
          execute: { label: { normal: "Ausführen", beginner: "Ausführen" } },
          reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
          windowOptions: { label: { normal: "Fensteroptionen", beginner: "Fensteroptionen" } },
          focus: { label: { normal: "Fokussieren", beginner: "Fokussieren" } },
          unfocus: { label: { normal: "Fokus aufheben", beginner: "Fokus aufheben" } },
          example: { label: { normal: "Beispiel", beginner: "Beispiel" } },
          noExample: { label: { normal: "Kein Beispiel", beginner: "Kein Beispiel" } },
          loadingSurface: { label: { normal: "Oberfläche wird geladen…", beginner: "Oberfläche wird geladen…" } },
          unknownComponent: { label: { normal: "Unbekannte Komponente", beginner: "Unbekannte Komponente" } },
          select: { label: { normal: "Auswählen", beginner: "Auswählen" } },
          commandPalette: { label: { normal: "Befehlspalette", beginner: "Befehlspalette" } },
          searchForCommand: { label: { normal: "Nach einem Befehl suchen…", beginner: "Nach einem Befehl suchen…" } },
          find: { label: { normal: "Finden…", beginner: "Finden…" } },
          noData: { label: { normal: "Keine Daten", beginner: "Keine Daten" } },
          noFileSystemNodes: { label: { normal: "Keine Dateisystemknoten", beginner: "Keine Dateisystemknoten" } },
          selectTarget: { label: { normal: "Ziel auswählen", beginner: "Ziel auswählen" } },
          selectOption: { label: { normal: "Option auswählen…", beginner: "Option auswählen…" } },
          noOptionsFound: { label: { normal: "Keine Optionen gefunden.", beginner: "Keine Optionen gefunden." } },
          close: { label: { normal: "Schließen", beginner: "Schließen" } },
          newWindow: { label: { normal: "Neues Fenster", beginner: "Neues Fenster" } },
          minimize: { label: { normal: "Minimieren", beginner: "Minimieren" } },
          maximize: { label: { normal: "Maximieren", beginner: "Maximieren" } },
          action: { label: { normal: "Aktion", beginner: "Aktion" } },
          actions: { label: { normal: "Aktionen", beginner: "Aktionen" } },
          utilities: { label: { normal: "Hilfsmittel", beginner: "Hilfsmittel" } },
          retry: { label: { normal: "Erneut versuchen", beginner: "Erneut versuchen" } },
          somethingWentWrong: { label: { normal: "Etwas ist schiefgelaufen", beginner: "Etwas ist schiefgelaufen" } },
          doubleClickToEdit: { label: { normal: "Zum Bearbeiten doppelklicken", beginner: "Zum Bearbeiten doppelklicken" } },
          importFile: { label: { normal: "Datei importieren…", beginner: "Datei importieren…" } },
          clear: { label: { normal: "Leeren", beginner: "Leeren" } },
          collapse: { label: { normal: "Einklappen", beginner: "Einklappen" } },
          expand: { label: { normal: "Ausklappen", beginner: "Ausklappen" } },
          cancel: { label: { normal: "Abbrechen", beginner: "Abbrechen" } },
          error: { label: { normal: "Fehler", beginner: "Fehler" } },
        },
        window: {
          close: { label: { normal: "Schließen", beginner: "Schließen" } },
          focus: { label: { normal: "Fokussieren", beginner: "Fokussieren" } },
          unfocus: { label: { normal: "Fokus aufheben", beginner: "Fokus aufheben" } },
          newWindow: { label: { normal: "Neues Fenster", beginner: "Neues Fenster" } },
        },
        contextMenu: {
          more: { label: { normal: "Mehr", beginner: "Mehr" } },
          select: { label: { normal: "Auswählen", beginner: "Auswählen" } },
          deselect: { label: { normal: "Abwählen", beginner: "Abwählen" } },
          selectAll: { label: { normal: "Alles auswählen", beginner: "Alles auswählen" } },
          clearSelection: { label: { normal: "Auswahl aufheben", beginner: "Auswahl aufheben" } },
          selectSameKind: { label: { normal: "Gleiche Art auswählen", beginner: "Gleiche Art auswählen" } },
          duplicate: { label: { normal: "Duplizieren", beginner: "Duplizieren" } },
          delete: { label: { normal: "Löschen", beginner: "Löschen" } },
          zoomToSelection: { label: { normal: "Auf Auswahl zoomen", beginner: "Auf Auswahl zoomen" } },
          focusZoom: { label: { normal: "Fokus / Zoom darauf", beginner: "Fokus / Zoom darauf" } },
          openSource: { label: { normal: "Quelle öffnen", beginner: "Quelle öffnen" } },
          fitWorld: { label: { normal: "Welt einpassen", beginner: "Welt einpassen" } },
          cut: { label: { normal: "Ausschneiden", beginner: "Ausschneiden" } },
          copy: { label: { normal: "Kopieren", beginner: "Kopieren" } },
          paste: { label: { normal: "Einfügen", beginner: "Einfügen" } },
          rename: { label: { normal: "Umbenennen", beginner: "Umbenennen" } },
          formatDocument: { label: { normal: "Dokument formatieren", beginner: "Dokument formatieren" } },
          lintDocument: { label: { normal: "Dokument prüfen", beginner: "Dokument prüfen" } },
          suggestCompletions: { label: { normal: "Vervollständigungen vorschlagen", beginner: "Vervollständigungen vorschlagen" } },
          selectToken: { label: { normal: "Token auswählen", beginner: "Token auswählen" } },
          selectLine: { label: { normal: "Zeile auswählen", beginner: "Zeile auswählen" } },
          hide: { label: { normal: "Ausblenden", beginner: "Ausblenden" } },
          show: { label: { normal: "Einblenden", beginner: "Einblenden" } },
          lock: { label: { normal: "Sperren", beginner: "Sperren" } },
          unlock: { label: { normal: "Entsperren", beginner: "Entsperren" } },
        },
        host: {
          emptyScene: { label: { normal: "Keine Szene", beginner: "Keine Szene" } },
          preview: { label: { normal: "Vorschau", beginner: "Vorschau" } },
          sourceAvailable: { label: { normal: "Quelle verfügbar", beginner: "Quelle verfügbar" } },
          blockImage: { label: { normal: "Bild", beginner: "Bild" } },
          blockTable: { label: { normal: "Tabelle", beginner: "Tabelle" } },
          blockMath: { label: { normal: "Mathe", beginner: "Mathe" } },
          blockInk: { label: { normal: "Tinte", beginner: "Tinte" } },
          blockGroup: { label: { normal: "Gruppe", beginner: "Gruppe" } },
          blockText: { label: { normal: "Text", beginner: "Text" } },
          checkingPlacement: { label: { normal: "Prüfe kollisionsfreie Platzierungen…", beginner: "Prüfe kollisionsfreie Platzierungen…" } },
          noPlacement: { label: { normal: "Keine kollisionsfreie Platzierung an diesem Verbinder", beginner: "Keine kollisionsfreie Platzierung an diesem Verbinder" } },
          canvasUnavailable: { label: { normal: "Leinwand nicht verfügbar", beginner: "Leinwand nicht verfügbar" } },
          rendering: { label: { normal: "Wird gerendert…", beginner: "Wird gerendert…" } },
          documentPlaceholder: { label: { normal: "Dokument", beginner: "Dokument" } },
          languageDocument: { label: { normal: "{{language}}-Dokument", beginner: "{{language}}-Dokument" } },
          iconShot: { label: { normal: "Symbolbild", beginner: "Symbolbild" } },
          projection: { label: { normal: "Projektion", beginner: "Projektion" } },
          frameVisible: { label: { normal: "Sichtbares einpassen", beginner: "Sichtbares einpassen" } },
          perspective: { label: { normal: "Perspektivisch", beginner: "Perspektivisch" } },
          orthographic: { label: { normal: "Orthografisch", beginner: "Orthografisch" } },
        },
        chat: {
          readyFor: { label: { normal: "Chat ist bereit für {{title}}.", beginner: "Chat ist bereit für {{title}}." } },
          localOnly: { label: { normal: "Nachrichten bleiben lokal in diesem Panel, bis ein verbundener Assistent hinzugefügt wird.", beginner: "Nachrichten bleiben lokal in diesem Panel, bis ein verbundener Assistent hinzugefügt wird." } },
          instructions: {
            label: { normal: "Lokaler Chat für {{title}}. Eingabetaste zum Senden, Umschalt+Eingabetaste für eine neue Zeile.", beginner: "Lokaler Chat für {{title}}. Eingabetaste zum Senden, Umschalt+Eingabetaste für eine neue Zeile." },
          },
          placeholder: { label: { normal: "Nachricht für {{title}} schreiben…", beginner: "Nachricht für {{title}} schreiben…" } },
          savedLocally: { label: { normal: "Lokal gespeichert: „{{preview}}“", beginner: "Lokal gespeichert: „{{preview}}“" } },
          send: { label: { normal: "Senden", beginner: "Senden" } },
        },
        blockList: {
          steps: { label: { normal: "Schritte", beginner: "Schritte" } },
          addStep: { label: { normal: "Schritt hinzufügen", beginner: "Schritt hinzufügen" } },
        },
        docs: {
          navigation: {
            previous: {
              label: {
                normal: "Zurück",
                beginner: "Zurück",
              },
            },
            next: {
              label: {
                normal: "Weiter",
                beginner: "Weiter",
              },
            },
          },
        },
        ring: {
          demo: {
            label: {
              normal: "Ring",
              beginner: "Ring",
            },
          },
        },
        iconSelector: {
          mode: {
            url: { label: { normal: "URL", beginner: "URL" } },
            shortcode: { label: { normal: "Kurzcode", beginner: "Kurzcode" } },
            math: { label: { normal: "Mathe / Typst", beginner: "Mathe / Typst" } },
            data: { label: { normal: "Daten-URL", beginner: "Daten-URL" } },
            emoji: { label: { normal: "Emoji", beginner: "Emoji" } },
            text: { label: { normal: "Text", beginner: "Text" } },
            vector: { label: { normal: "Katalog / SVG", beginner: "Katalog / SVG" } },
          },
        },
        stepper: {
          demo: {
            label: {
              normal: "Wert",
              beginner: "Wert",
            },
          },
        },
        engagement: {
          actions: {
            label: {
              normal: "Aktionen",
              beginner: "Schnellaktionen für den aktuellen Schritt",
            },
          },
          viewport: {
            label: {
              normal: "Ansicht",
              beginner: "Ansicht",
            },
          },
        },
        windowSearch: {
          title: {
            label: {
              normal: "Suche",
              beginner: "Suche",
            },
          },
          action: {
            label: {
              normal: "Aktion",
              beginner: "Aktion eingeben oder aus der Liste wählen",
            },
          },
          actionActive: {
            label: {
              normal: "Aktion oder Wert",
              beginner: "Aktion oder Zahl für den aktuellen Schritt",
            },
          },
          suggestions: {
            label: {
              normal: "Vorschläge",
              beginner: "Liste der passenden Aktionen öffnen",
            },
          },
          noMatches: {
            label: {
              normal: "Keine Treffer",
              beginner: "Keine passenden Aktionen",
            },
          },
        },
        flowSpotlight: {
          typeToAdd: { label: { normal: "Zum Hinzufügen tippen…", beginner: "Zum Hinzufügen tippen…" } },
          collapseSuggestions: { label: { normal: "Vorschläge einklappen", beginner: "Vorschläge einklappen" } },
          showAllSuggestions: { label: { normal: "Alle Vorschläge anzeigen", beginner: "Alle Vorschläge anzeigen" } },
        },
        nodeGraph: {
          fitGraph: { label: { normal: "Graph einpassen", beginner: "Ganzen Graph zeigen" } },
        },
        sync: {
          attach: { label: { normal: "Verbinden", beginner: "Verbinden" } },
          detach: { label: { normal: "Trennen", beginner: "Trennen" } },
        },
        ink: {
          link: { label: { normal: "Link", beginner: "Link" } },
          linkUrlPrompt: { label: { normal: "Link-URL", beginner: "Link-URL" } },
        },
        surfaceContextMenu: {
          architecture: { label: { normal: "Architekturmenü", beginner: "Architekturmenü" } },
          attraction: { label: { normal: "Anziehungsmenü", beginner: "Anziehungsmenü" } },
          block: { label: { normal: "Blockmenü", beginner: "Blockmenü" } },
          edge: { label: { normal: "Kantenmenü", beginner: "Kantenmenü" } },
          entry: { label: { normal: "Eintragsmenü", beginner: "Eintragsmenü" } },
          feature: { label: { normal: "Elementmenü", beginner: "Elementmenü" } },
          group: { label: { normal: "Gruppenmenü", beginner: "Gruppenmenü" } },
          handle: { label: { normal: "Griffmenü", beginner: "Griffmenü" } },
          layer: { label: { normal: "Ebenenmenü", beginner: "Ebenenmenü" } },
          object: { label: { normal: "Objektmenü", beginner: "Objektmenü" } },
          part: { label: { normal: "Teilmenü", beginner: "Teilmenü" } },
          path: { label: { normal: "Pfadmenü", beginner: "Pfadmenü" } },
          pixel: { label: { normal: "Pixelmenü", beginner: "Pixelmenü" } },
          position: { label: { normal: "Positionsmenü", beginner: "Positionsmenü" } },
          reference: { label: { normal: "Referenzmenü", beginner: "Referenzmenü" } },
          route: { label: { normal: "Routenmenü", beginner: "Routenmenü" } },
          slider: { label: { normal: "Reglermenü", beginner: "Reglermenü" } },
          vortex: { label: { normal: "Vortexmenü", beginner: "Vortexmenü" } },
          file: { label: { normal: "Dateimenü", beginner: "Dateimenü" } },
          workspace: { label: { normal: "Arbeitsbereichsmenü", beginner: "Arbeitsbereichsmenü" } },
          canvas: { label: { normal: "Canvas-Menü", beginner: "Canvas-Menü" } },
          scene: { label: { normal: "Szenenmenü", beginner: "Szenenmenü" } },
          placementSuggestions: { label: { normal: "Platzierungsvorschläge", beginner: "Platzierungsvorschläge" } },
          node: { label: { normal: "Knotenmenü", beginner: "Knotenmenü" } },
          flow: { label: { normal: "Flow-Menü", beginner: "Flow-Menü" } },
          row: { label: { normal: "Zeilenmenü", beginner: "Zeilenmenü" } },
          paint: { label: { normal: "Malmenü", beginner: "Malmenü" } },
          board: { label: { normal: "Board-Menü", beginner: "Board-Menü" } },
          ink: { label: { normal: "Tintenmenü", beginner: "Tintenmenü" } },
          history: { label: { normal: "Verlaufsmenü", beginner: "Verlaufsmenü" } },
          step: { label: { normal: "Schrittmenü", beginner: "Schrittmenü" } },
          diff: { label: { normal: "Vergleichsmenü", beginner: "Vergleichsmenü" } },
          event: { label: { normal: "Ereignismenü", beginner: "Ereignismenü" } },
          editor: { label: { normal: "Editormenü", beginner: "Editormenü" } },
          map: { label: { normal: "Kartenmenü", beginner: "Kartenmenü" } },
        },
        mutation: {
          level: {
            info: { label: { normal: "Info", beginner: "Info" } },
            warning: { label: { normal: "Warnung", beginner: "Warnung" } },
            error: { label: { normal: "Fehler", beginner: "Fehler" } },
            fatal: { label: { normal: "Kritisch", beginner: "Kritischer Fehler" } },
          },
          code: {
            targetMissing: { label: { normal: "Ziel fehlt", beginner: "Das Ziel dieser Änderung existiert nicht mehr." } },
            noOp: { label: { normal: "Keine Änderung", beginner: "Der Zustand war bereits so — nichts wurde geändert." } },
            partial: { label: { normal: "Teilweise angewendet", beginner: "Nur ein Teil der Änderung konnte angewendet werden." } },
            clamped: { label: { normal: "Begrenzt", beginner: "Ein Wert wurde auf den zulässigen Bereich begrenzt." } },
            duplicateId: { label: { normal: "ID bereits vergeben", beginner: "Es existiert bereits ein Element mit dieser ID." } },
            invariant: { label: { normal: "Ungültiger Zustand", beginner: "Diese Änderung würde einen ungültigen Zustand erzeugen." } },
            cascade: { label: { normal: "Folgeänderung", beginner: "Diese Änderung hat weitere Änderungen ausgelöst." } },
          },
          policy: {
            laissezFaire: {
              label: { label: { normal: "Laissez-faire", beginner: "Laissez-faire" } },
              description: { label: { normal: "Nimmt jede Änderung an, außer sie ist kritisch.", beginner: "Nimmt jede Änderung an, solange sie nicht kritisch ist." } },
            },
            normal: {
              label: { label: { normal: "Normal", beginner: "Normal" } },
              description: { label: { normal: "Lehnt fehlerhafte Änderungen ab, erlaubt Warnungen.", beginner: "Lehnt Änderungen mit Fehlern ab, lässt Warnungen aber zu." } },
            },
            vigilant: {
              label: { label: { normal: "Wachsam", beginner: "Wachsam" } },
              description: { label: { normal: "Lehnt bereits Änderungen mit Warnungen ab.", beginner: "Am strengsten: lehnt schon Änderungen mit Warnungen ab." } },
            },
            setting: {
              label: { label: { normal: "Merge-Richtlinie", beginner: "Merge-Richtlinie" } },
            },
          },
          rejected: {
            title: { label: { normal: "Änderung abgelehnt", beginner: "Änderung abgelehnt" } },
            body: { label: { normal: "Diese Änderung konnte nicht angewendet werden.", beginner: "Diese Änderung konnte nicht angewendet werden." } },
          },
        },
        conflict: {
          panel: { label: { normal: "Konflikte", beginner: "Konflikte" } },
          accept: { label: { normal: "Übernehmen", beginner: "Übernehmen" } },
          discard: { label: { normal: "Verwerfen", beginner: "Verwerfen" } },
          quarantined: { label: { normal: "Zurückgehalten", beginner: "Eingehende Änderungen werden zurückgehalten, bis du entscheidest." } },
          degraded: { label: { normal: "Beeinträchtigt", beginner: "Übernommen, aber mit Warnungen." } },
        },
        presence: {
          roster: { label: { normal: "Anwesende", beginner: "Anwesende" } },
          empty: { label: { normal: "Niemand sonst ist hier", beginner: "Niemand sonst ist hier" } },
          overflow: { label: { normal: "+{{count}} weitere", beginner: "+{{count}} weitere" } },
          role: {
            author: { label: { normal: "Bearbeitet", beginner: "Bearbeitet" } },
            spectator: { label: { normal: "Betrachtet", beginner: "Betrachtet" } },
          },
        },
      },
      settings: {
        layout: {
          desktop: {
            label: {
              normal: "Desktop-Layout",
              beginner: "Verwendet das Standard-Layout, optimiert für Maus und Tastatur.",
            },
          },
          tablet: {
            label: {
              normal: "Tablet-Layout",
              beginner: "Verwendet das Tablet-Layout mit größeren, touch-freundlichen Bedienelementen.",
            },
          },
          mobile: {
            label: {
              normal: "Mobil-Layout",
              beginner: "Verwendet das Mobil-Layout, automatisch aktiv auf kleinen Bildschirmen.",
            },
          },
        },
        driver: {
          select: { label: { normal: "Treiber", beginner: "Treiber" } },
          default: { label: { normal: "Standard", beginner: "Standard" } },
          compact: { label: { normal: "Kompakt", beginner: "Kompakt" } },
          labels: { label: { normal: "Beschriftungen", beginner: "Beschriftungen" } },
          labelsOption: {
            full: { label: { normal: "Voll", beginner: "Symbol und Beschriftung" } },
            icons: { label: { normal: "Nur Symbole", beginner: "Nur Symbole" } },
          },
          labelTier: { label: { normal: "Beschriftungsstufe", beginner: "Beschriftungsstufe" } },
          labelTierOption: {
            beginner: { label: { normal: "Anfänger", beginner: "Ausführliche Beschriftungen" } },
            normal: { label: { normal: "Normal", beginner: "Kurze Beschriftungen" } },
          },
          drag: { label: { normal: "Ziehen", beginner: "Ziehen" } },
          dragOption: {
            handle: { label: { normal: "Griff", beginner: "Eigener Ziehgriff" } },
            surface: { label: { normal: "Fläche", beginner: "Ganzes Element ziehbar" } },
          },
          chrome: { label: { normal: "Oberflächenanzeige", beginner: "Oberflächenanzeige" } },
          chromeOption: {
            always: { label: { normal: "Immer", beginner: "Immer sichtbar" } },
            hover: { label: { normal: "Bei Hover", beginner: "Nur bei Mauszeiger sichtbar" } },
          },
          gumball: { label: { normal: "Gumball-Anzeige", beginner: "Gumball-Anzeige" } },
          gumballOption: {
            always: { label: { normal: "Immer", beginner: "Immer sichtbar" } },
            hover: { label: { normal: "Bei Hover", beginner: "Nur bei Mauszeiger sichtbar" } },
          },
          tooltips: { label: { normal: "Tooltips", beginner: "Tooltips" } },
          tooltipsOption: {
            full: { label: { normal: "Voll", beginner: "Mit Handbuch- und Tutorial-Links" } },
            minimal: { label: { normal: "Minimal", beginner: "Nur Name und Tastenkürzel" } },
            none: { label: { normal: "Keine", beginner: "Keine Tooltips" } },
          },
          hotkeys: { label: { normal: "Tastenkürzel", beginner: "Tastenkürzel" } },
          hotkeysOption: {
            inline: { label: { normal: "Inline", beginner: "Am Steuerelement" } },
            tooltip: { label: { normal: "Tooltip", beginner: "Nur im Tooltip" } },
            none: { label: { normal: "Keine", beginner: "Ausgeblendet" } },
          },
          save: { label: { normal: "Speichern unter", beginner: "Speichern unter" } },
          savePlaceholder: { label: { normal: "Treibername", beginner: "Treibername" } },
          delete: { label: { normal: "Löschen", beginner: "Löschen" } },
          dirty: { label: { normal: "Nicht gespeichert", beginner: "Nicht gespeichert" } },
        },
        keybindings: {
          capture: { label: { normal: "Aufnehmen", beginner: "Aufnehmen" } },
          reset: { label: { normal: "Zurücksetzen", beginner: "Zurücksetzen" } },
          conflict: { label: { normal: "Belegt", beginner: "Bereits vergeben" } },
          pressKeys: { label: { normal: "Tasten drücken…", beginner: "Tasten drücken…" } },
        },
      },
      tooltip: {
        manual: {
          label: {
            normal: "Handbuch",
            beginner: "Handbuch",
          },
        },
        tutorial: {
          label: {
            normal: "Tutorial",
            beginner: "Tutorial",
          },
        },
      },
      introduction: {
        skip: { label: { normal: "Überspringen", beginner: "Überspringen" } },
        back: { label: { normal: "Zurück", beginner: "Zurück" } },
        next: { label: { normal: "Weiter", beginner: "Weiter" } },
        done: { label: { normal: "Fertig", beginner: "Fertig" } },
      },
      tutorial: {
        play: { label: { normal: "Abspielen", beginner: "Abspielen" } },
        pause: { label: { normal: "Pause", beginner: "Pause" } },
        stop: { label: { normal: "Tutorial beenden", beginner: "Tutorial beenden" } },
        rate: { label: { normal: "Geschwindigkeit", beginner: "Geschwindigkeit" } },
        mute: { label: { normal: "Ton aus", beginner: "Ton aus" } },
        captions: { label: { normal: "Untertitel", beginner: "Untertitel" } },
        record: { label: { normal: "Aufnehmen", beginner: "Aufnehmen" } },
        recording: { label: { normal: "Aufnahme läuft", beginner: "Aufnahme läuft" } },
        addChapter: { label: { normal: "Kapitel setzen", beginner: "Kapitel setzen" } },
        chapter: { label: { normal: "Kapitel", beginner: "Kapitel" } },
      },
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇩️🇪️ German Bundle

  // #region 🇬️🇧️ English Bundle
  en: {
    translation: {
      ui: {
        nav: {
          back: {
            label: {
              normal: "Go back",
              beginner: "Go back",
            },
          },
          forward: {
            label: {
              normal: "Go forward",
              beginner: "Go forward",
            },
          },
          up: {
            label: {
              normal: "Go up one level",
              beginner: "Go up one level",
            },
          },
        },
        search: {
          toggle: {
            label: {
              normal: "Search",
              beginner: "Search",
            },
          },
          close: {
            label: {
              normal: "Close search",
              beginner: "Close search",
            },
          },
          title: {
            label: {
              normal: "Search",
              beginner: "Search",
            },
          },
          description: {
            label: {
              normal: "Search for items",
              beginner: "Search for items",
            },
          },
          placeholder: {
            label: {
              normal: "Search...",
              beginner: "Search...",
            },
          },
          empty: {
            label: {
              normal: "No results found.",
              beginner: "No results found.",
            },
          },
          category: {
            panels: { label: { normal: "Panels", beginner: "Panels" } },
            windows: { label: { normal: "Windows", beginner: "Windows" } },
            catalogue: { label: { normal: "Catalogue", beginner: "Catalogue" } },
            // 🏠️ "Space" here is a deliberate, deferred duplicate of the host plugin's own manifest label
            // (`App::builder(S_PLAY_APP_ID, LocalizedLabel::native("Space", "Space"))`,
            // ✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs:869) — reading it from there via
            // `resolveManifestLabel(hostApp.label, …)` (the same pattern `appWindowLabel` already uses)
            // is the correct fix, EXCEPT `ShellHost/🟦️.tsx`'s `hostApp` lookup (line ~1132,
            // `manifest.apps.find(app => app.id === hostConfig?.hostAppId)`) is ALREADY always
            // `undefined`: `hostConfig.hostAppId` is the raw Cargo.toml `host = { shell = "studio" }`
            // alias, never the real dialect-derived `AppDefinition.id`
            // (`s.space.studio@1/*#editor`) — a pre-existing bug the same file's own w4-h comment
            // (lines 4112-4121) already documents and declines to fix. Wiring this label to
            // `hostApp?.label` today would render an EMPTY category header, not "Space" — a regression.
            // Fix plan (needs a new field, not a string-matching workaround): add
            // `AppDefinition.host_role: Option<HostRole>` (`Landing`/`Host`) in
            // `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (~3034), a `.host_role(...)` builder
            // method on `AppBuilder`/forwarded by `EditorBuilder`/`ViewerBuilder`
            // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`), set it in
            // `create_home_app()`/`create_space_app()`, regenerate the TS mirror, then have
            // `ShellHost/🟦️.tsx:1132-1133` match on `hostRole` instead of the broken id
            // comparison. See
            // `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/STUBS-AND-PLACEHOLDERS-COMPLETION/📓️hostapp-label-layering.md`.
            hostApp: { label: { normal: "Space", beginner: "Space" } },
            navigation: { label: { normal: "Navigation", beginner: "Navigation" } },
          },
        },
        palette: {
          undo: { label: { normal: "Undo", beginner: "Undo" } },
          redo: { label: { normal: "Redo", beginner: "Redo" } },
          goHome: { label: { normal: "Go Home", beginner: "Go Home" } },
          spawnPrefix: { label: { normal: "Spawn", beginner: "Spawn" } },
        },
        panel: {
          artifact: { label: { normal: "Artifact", beginner: "Artifact" } },
          catalogue: { label: { normal: "Catalogue", beginner: "Catalogue" } },
          inspection: { label: { normal: "Inspection", beginner: "Inspection" } },
          parameters: { label: { normal: "Parameters", beginner: "Parameters" } },
          artifactEmpty: { label: { normal: "—", beginner: "—" } },
          spawnedAppsSuffix: { label: { normal: "spawned app(s)", beginner: "spawned app(s)" } },
          sync: { label: { normal: "Sync", beginner: "Sync" } },
          actions: { label: { normal: "Actions", beginner: "Actions" } },
          history: { label: { normal: "History", beginner: "History" } },
        },
        tree: {
          drag: {
            sort: { label: { normal: "Reorder", beginner: "Reorder row" } },
            sortTarget: { label: { normal: "Click and hold left click to drag {{target}}", beginner: "Click and hold left click to drag {{target}}" } },
            transfer: { label: { normal: "Drag to window", beginner: "Drag into a window" } },
            transferTarget: { label: { normal: "Click and hold left click to drag {{target}}", beginner: "Click and hold left click to drag {{target}}" } },
          },
        },
        find: {
          toggle: {
            label: {
              normal: "Find",
              beginner: "Find in view",
            },
          },
          title: {
            label: {
              normal: "Find",
              beginner: "Find",
            },
          },
          description: {
            label: {
              normal: "Find items in this view",
              beginner: "Find items in this view",
            },
          },
          placeholder: {
            label: {
              normal: "Find...",
              beginner: "Find...",
            },
          },
          empty: {
            label: {
              normal: "No results found.",
              beginner: "No results found.",
            },
          },
        },
        fullscreen: {
          toggle: {
            label: {
              normal: "Fullscreen",
              beginner: "Fullscreen",
            },
          },
          exit: {
            label: {
              normal: "Exit Fullscreen",
              beginner: "Exit Fullscreen",
            },
          },
        },
        mobilePanel: {
          toggle: {
            label: {
              normal: "Panel",
              beginner: "Panel",
            },
          },
          app: {
            label: {
              normal: "App",
              beginner: "App",
            },
          },
        },
        panelToggle: {
          topLeft: {
            label: {
              normal: "Top Left",
              beginner: "Top Left",
            },
          },
          topRight: {
            label: {
              normal: "Top Right",
              beginner: "Top Right",
            },
          },
          bottomLeft: {
            label: {
              normal: "Bottom Left",
              beginner: "Bottom Left",
            },
          },
          bottomRight: {
            label: {
              normal: "Bottom Right",
              beginner: "Bottom Right",
            },
          },
          display: {
            label: {
              normal: "Display",
              beginner: "Display",
            },
          },
          command: {
            label: {
              normal: "Command",
              beginner: "Command",
            },
          },
          tool: {
            label: {
              normal: "Tool",
              beginner: "Tool",
            },
          },
          overview: {
            label: {
              normal: "Overview",
              beginner: "Overview",
            },
          },
          workbench: {
            label: {
              normal: "Workbench",
              beginner: "Workbench",
            },
          },
          details: {
            label: {
              normal: "Details",
              beginner: "Details",
            },
          },
          settings: {
            label: {
              normal: "Settings",
              beginner: "Settings",
            },
          },
          chat: {
            label: {
              normal: "Chat",
              beginner: "Chat",
            },
          },
          plugins: {
            label: {
              normal: "Plugins",
              beginner: "Plugins",
            },
          },
        },
        display: {
          tab: {
            windows: { label: { normal: "Windows", beginner: "Windows" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
          },
          saveLayout: { label: { normal: "Save layout", beginner: "Save layout" } },
          saveLayoutPlaceholder: { label: { normal: "Layout name", beginner: "Layout name" } },
          saveCurrentLayout: { label: { normal: "Save current layout", beginner: "Save current layout" } },
          deleteLayout: { label: { normal: "Delete", beginner: "Delete" } },
          emptyShell: {
            label: {
              normal: "Drag windows from Display in the navbar, or restore a saved layout.",
              beginner: "Drag windows from Display in the navbar, or restore a saved layout.",
            },
          },
          layouts: { label: { normal: "Layouts", beginner: "Layouts" } },
          saved: { label: { normal: "Saved", beginner: "Saved" } },
          unavailable: { label: { normal: "Display unavailable", beginner: "Display unavailable" } },
        },
        settings: {
          tab: {
            general: { label: { normal: "General", beginner: "General" } },
            driver: { label: { normal: "Driver", beginner: "Driver" } },
            app: { label: { normal: "App", beginner: "App" } },
            appearance: { label: { normal: "Appearance", beginner: "Appearance" } },
            layout: { label: { normal: "Layout", beginner: "Layout" } },
            language: { label: { normal: "Language", beginner: "Language" } },
            terminology: { label: { normal: "Terminology", beginner: "Terminology" } },
            theme: { label: { normal: "Theme", beginner: "Theme" } },
            keybindings: { label: { normal: "Hotkeys", beginner: "Hotkeys" } },
          },
          appearance: {
            light: { label: { normal: "Light", beginner: "Light" } },
            dark: { label: { normal: "Dark", beginner: "Dark" } },
            system: { label: { normal: "System", beginner: "System" } },
          },
          language: {
            en: { label: { normal: "English", beginner: "English" } },
            de: { label: { normal: "Deutsch", beginner: "Deutsch" } },
          },
          terminology: {
            native: { label: { normal: "Native", beginner: "Native" } },
            reuse: { label: { normal: "Reuse", beginner: "Reuse" } },
          },
          app: {
            name: { label: { normal: "Name", beginner: "Name" } },
            id: { label: { normal: "App id", beginner: "App id" } },
            controller: { label: { normal: "Controller", beginner: "Controller" } },
            plugin: { label: { normal: "Plugin", beginner: "Plugin" } },
          },
          theme: {
            select: { label: { normal: "Theme", beginner: "Theme" } },
            save: { label: { normal: "Save as", beginner: "Save as" } },
            savePlaceholder: { label: { normal: "Theme name", beginner: "Theme name" } },
            reset: { label: { normal: "Reset", beginner: "Reset" } },
            export: { label: { normal: "Export", beginner: "Export" } },
            import: { label: { normal: "Import", beginner: "Import" } },
            delete: { label: { normal: "Delete", beginner: "Delete" } },
            colors: { label: { normal: "Colors", beginner: "Colors" } },
            spacing: { label: { normal: "Spacing", beginner: "Spacing" } },
            fonts: { label: { normal: "Fonts", beginner: "Fonts" } },
            strokes: { label: { normal: "Strokes", beginner: "Strokes" } },
            radii: { label: { normal: "Radii", beginner: "Radii" } },
            opacities: { label: { normal: "Opacities", beginner: "Opacities" } },
            metrics: { label: { normal: "Metrics", beginner: "Metrics" } },
            appearances: { label: { normal: "Appearances", beginner: "Appearances" } },
            dirty: { label: { normal: "Unsaved", beginner: "Unsaved" } },
            appearance: {
              light: { label: { normal: "Light", beginner: "Light" } },
              dark: { label: { normal: "Dark", beginner: "Dark" } },
            },
            group: {
              board: { label: { normal: "Board", beginner: "Board" } },
              map: { label: { normal: "Map", beginner: "Map" } },
              canvas: { label: { normal: "Canvas", beginner: "Canvas" } },
              chrome: { label: { normal: "Chrome", beginner: "Chrome" } },
            },
          },
          unavailable: { label: { normal: "Settings unavailable", beginner: "Settings unavailable" } },
          resetDock: { label: { normal: "Reset panels", beginner: "Reset panels" } },
        },
        plugins: {
          status: {
            available: { label: { normal: "Available", beginner: "Available" } },
            installing: { label: { normal: "Installing…", beginner: "Installing…" } },
            loaded: { label: { normal: "Loaded", beginner: "Loaded" } },
            failed: { label: { normal: "Failed", beginner: "Failed" } },
            reloading: { label: { normal: "Reloading…", beginner: "Reloading…" } },
          },
          action: {
            install: { label: { normal: "Install", beginner: "Install" } },
            uninstall: { label: { normal: "Uninstall", beginner: "Uninstall" } },
            reload: { label: { normal: "Reload", beginner: "Reload" } },
          },
          waitingForHost: { label: { normal: "Waiting for host program…", beginner: "Waiting for host program…" } },
          unavailable: { label: { normal: "Plugins unavailable", beginner: "Plugins unavailable" } },
          source: { label: { normal: "Source", beginner: "Source" } },
        },
        command: {
          introduceApp: { label: { normal: "Introduce App", beginner: "Introduce App" } },
          playTutorial: { label: { normal: "Play Tutorial", beginner: "Play Tutorial" } },
          recordTutorial: { label: { normal: "Record Tutorial", beginner: "Record Tutorial" } },
          setAppearance: { label: { normal: "Set Appearance", beginner: "Set Appearance" } },
          setTheme: { label: { normal: "Set Theme", beginner: "Set Theme" } },
          setLayout: { label: { normal: "Set Layout", beginner: "Set Layout" } },
          setLocale: { label: { normal: "Set Locale", beginner: "Set Locale" } },
          setTerminology: { label: { normal: "Set Terminology", beginner: "Set Terminology" } },
          setDriver: { label: { normal: "Set Driver", beginner: "Set Driver" } },
        },
        shellCommand: {
          dockMove: { label: { normal: "Move Panel Tab", beginner: "Move Panel Tab" } },
          windowResize: { label: { normal: "Resize Window", beginner: "Resize Window" } },
          windowMove: { label: { normal: "Rearrange Windows", beginner: "Rearrange Windows" } },
          windowActivate: { label: { normal: "Activate Window", beginner: "Activate Window" } },
          windowClose: { label: { normal: "Close Window", beginner: "Close Window" } },
          windowSplit: { label: { normal: "Split Window", beginner: "Split Window" } },
          windowOpenInNewWindow: { label: { normal: "Open in New Window", beginner: "Open in New Window" } },
          panelToggle: { label: { normal: "Toggle Panel", beginner: "Toggle Panel" } },
          panelTab: { label: { normal: "Switch Panel Tab", beginner: "Switch Panel Tab" } },
        },
        ribbon: {
          group: {
            parent: {
              label: {
                normal: "Utility",
                beginner: "Utility",
              },
            },
          },
          parent: uiRibbonParentEn,
        },
        selection: {
          method: { label: { normal: "Method", beginner: "Method" } },
          mode: { label: { normal: "Mode", beginner: "Mode" } },
          rectangle: { label: { normal: "Rectangle", beginner: "Rectangle" } },
          lasso: { label: { normal: "Lasso", beginner: "Lasso" } },
          selective: { label: { normal: "Selective", beginner: "Selective" } },
          additive: { label: { normal: "Additive", beginner: "Additive" } },
          subtractive: { label: { normal: "Subtractive", beginner: "Subtractive" } },
          invertive: { label: { normal: "Invertive", beginner: "Invertive" } },
        },
        windowFault: {
          title: { label: { normal: "Window is not responding", beginner: "Window is not responding" } },
          abiMismatch: { label: { normal: "Plugin module does not match the host interface", beginner: "This plugin is out of date and no longer fits the program" } },
          interactiveCeiling: { label: { normal: "Plugin step overran the interactive time ceiling", beginner: "The plugin took too long for one step" } },
          clock: { label: { normal: "No monotonic clock available", beginner: "The system clock reading is unavailable" } },
          pluginInternal: { label: { normal: "Internal plugin runtime fault", beginner: "Something went wrong inside the plugin" } },
          installFailed: { label: { normal: "Plugin failed to install", beginner: "The plugin could not be loaded" } },
          unknown: { label: { normal: "Unknown fault cause", beginner: "The cause is unknown" } },
        },
        common: {
          mixedValues: {
            label: {
              normal: "Mixed",
              beginner: "Mixed",
            },
          },
          name: { label: { normal: "Name", beginner: "Name" } },
          save: { label: { normal: "Save", beginner: "Save" } },
          loading: { label: { normal: "Loading…", beginner: "Loading…" } },
          loadingPlugins: { label: { normal: "Loading plugins…", beginner: "Loading plugins…" } },
          renderError: { label: { normal: "Render error", beginner: "Render error" } },
          noPluginsLoaded: { label: { normal: "No plugins loaded", beginner: "No plugins loaded" } },
          missingWindow: { label: { normal: "Missing window", beginner: "Missing window" } },
          home: { label: { normal: "Home", beginner: "Home" } },
          backToWorkflow: { label: { normal: "Back to Workflow", beginner: "Back to Workflow" } },
          execute: { label: { normal: "Execute", beginner: "Execute" } },
          reset: { label: { normal: "Reset", beginner: "Reset" } },
          windowOptions: { label: { normal: "Window Options", beginner: "Window Options" } },
          focus: { label: { normal: "Focus", beginner: "Focus" } },
          unfocus: { label: { normal: "Unfocus", beginner: "Unfocus" } },
          example: { label: { normal: "Example", beginner: "Example" } },
          noExample: { label: { normal: "No example", beginner: "No example" } },
          loadingSurface: { label: { normal: "Loading surface…", beginner: "Loading surface…" } },
          unknownComponent: { label: { normal: "Unknown component", beginner: "Unknown component" } },
          select: { label: { normal: "Select", beginner: "Select" } },
          commandPalette: { label: { normal: "Command Palette", beginner: "Command Palette" } },
          searchForCommand: { label: { normal: "Search for a command to run…", beginner: "Search for a command to run…" } },
          find: { label: { normal: "Find…", beginner: "Find…" } },
          noData: { label: { normal: "No data", beginner: "No data" } },
          noFileSystemNodes: { label: { normal: "No file system nodes", beginner: "No file system nodes" } },
          selectTarget: { label: { normal: "Select target", beginner: "Select target" } },
          selectOption: { label: { normal: "Select option…", beginner: "Select option…" } },
          noOptionsFound: { label: { normal: "No options found.", beginner: "No options found." } },
          close: { label: { normal: "Close", beginner: "Close" } },
          newWindow: { label: { normal: "New Window", beginner: "New Window" } },
          minimize: { label: { normal: "Minimize", beginner: "Minimize" } },
          maximize: { label: { normal: "Maximize", beginner: "Maximize" } },
          action: { label: { normal: "Action", beginner: "Action" } },
          actions: { label: { normal: "Actions", beginner: "Actions" } },
          utilities: { label: { normal: "Utilities", beginner: "Utilities" } },
          retry: { label: { normal: "Retry", beginner: "Retry" } },
          somethingWentWrong: { label: { normal: "Something went wrong", beginner: "Something went wrong" } },
          doubleClickToEdit: { label: { normal: "Double-click to edit", beginner: "Double-click to edit" } },
          importFile: { label: { normal: "Import file…", beginner: "Import file…" } },
          clear: { label: { normal: "Clear", beginner: "Clear" } },
          collapse: { label: { normal: "Collapse", beginner: "Collapse" } },
          expand: { label: { normal: "Expand", beginner: "Expand" } },
          cancel: { label: { normal: "Cancel", beginner: "Cancel" } },
          error: { label: { normal: "Error", beginner: "Error" } },
        },
        window: {
          close: { label: { normal: "Close", beginner: "Close" } },
          focus: { label: { normal: "Focus", beginner: "Focus" } },
          unfocus: { label: { normal: "Unfocus", beginner: "Unfocus" } },
          newWindow: { label: { normal: "New Window", beginner: "New Window" } },
        },
        contextMenu: {
          more: { label: { normal: "More", beginner: "More" } },
          select: { label: { normal: "Select", beginner: "Select" } },
          deselect: { label: { normal: "Deselect", beginner: "Deselect" } },
          selectAll: { label: { normal: "Select all", beginner: "Select all" } },
          clearSelection: { label: { normal: "Clear selection", beginner: "Clear selection" } },
          selectSameKind: { label: { normal: "Select same kind", beginner: "Select same kind" } },
          duplicate: { label: { normal: "Duplicate", beginner: "Duplicate" } },
          delete: { label: { normal: "Delete", beginner: "Delete" } },
          zoomToSelection: { label: { normal: "Zoom to selection", beginner: "Zoom to selection" } },
          focusZoom: { label: { normal: "Focus / zoom to", beginner: "Focus / zoom to" } },
          openSource: { label: { normal: "Open source", beginner: "Open source" } },
          fitWorld: { label: { normal: "Fit world", beginner: "Fit world" } },
          cut: { label: { normal: "Cut", beginner: "Cut" } },
          copy: { label: { normal: "Copy", beginner: "Copy" } },
          paste: { label: { normal: "Paste", beginner: "Paste" } },
          rename: { label: { normal: "Rename", beginner: "Rename" } },
          formatDocument: { label: { normal: "Format document", beginner: "Format document" } },
          lintDocument: { label: { normal: "Lint document", beginner: "Lint document" } },
          suggestCompletions: { label: { normal: "Suggest completions", beginner: "Suggest completions" } },
          selectToken: { label: { normal: "Select token", beginner: "Select token" } },
          selectLine: { label: { normal: "Select line", beginner: "Select line" } },
          hide: { label: { normal: "Hide", beginner: "Hide" } },
          show: { label: { normal: "Show", beginner: "Show" } },
          lock: { label: { normal: "Lock", beginner: "Lock" } },
          unlock: { label: { normal: "Unlock", beginner: "Unlock" } },
        },
        host: {
          emptyScene: { label: { normal: "No scene", beginner: "No scene" } },
          preview: { label: { normal: "Preview", beginner: "Preview" } },
          sourceAvailable: { label: { normal: "Source available", beginner: "Source available" } },
          blockImage: { label: { normal: "Image", beginner: "Image" } },
          blockTable: { label: { normal: "Table", beginner: "Table" } },
          blockMath: { label: { normal: "Math", beginner: "Math" } },
          blockInk: { label: { normal: "Ink", beginner: "Ink" } },
          blockGroup: { label: { normal: "Group", beginner: "Group" } },
          blockText: { label: { normal: "Text", beginner: "Text" } },
          checkingPlacement: { label: { normal: "Checking collision-free placements…", beginner: "Checking collision-free placements…" } },
          noPlacement: { label: { normal: "No collision-free placement at this connector", beginner: "No collision-free placement at this connector" } },
          canvasUnavailable: { label: { normal: "Canvas unavailable", beginner: "Canvas unavailable" } },
          rendering: { label: { normal: "Rendering…", beginner: "Rendering…" } },
          documentPlaceholder: { label: { normal: "Artifact", beginner: "Artifact" } },
          languageDocument: { label: { normal: "{{language}} document", beginner: "{{language}} document" } },
          iconShot: { label: { normal: "Icon shot", beginner: "Icon shot" } },
          projection: { label: { normal: "Projection", beginner: "Projection" } },
          frameVisible: { label: { normal: "Frame visible", beginner: "Frame visible" } },
          perspective: { label: { normal: "Perspective", beginner: "Perspective" } },
          orthographic: { label: { normal: "Orthographic", beginner: "Orthographic" } },
        },
        chat: {
          readyFor: { label: { normal: "Chat is ready for {{title}}.", beginner: "Chat is ready for {{title}}." } },
          localOnly: { label: { normal: "Messages stay local in this panel until a connected assistant is added.", beginner: "Messages stay local in this panel until a connected assistant is added." } },
          instructions: { label: { normal: "Local chat for {{title}}. Use Enter to send and Shift+Enter for a new line.", beginner: "Local chat for {{title}}. Use Enter to send and Shift+Enter for a new line." } },
          placeholder: { label: { normal: "Write a message for {{title}}…", beginner: "Write a message for {{title}}…" } },
          savedLocally: { label: { normal: 'Saved locally: "{{preview}}"', beginner: 'Saved locally: "{{preview}}"' } },
          send: { label: { normal: "Send", beginner: "Send" } },
        },
        blockList: {
          steps: { label: { normal: "Steps", beginner: "Steps" } },
          addStep: { label: { normal: "Add Step", beginner: "Add Step" } },
        },
        docs: {
          navigation: {
            previous: {
              label: {
                normal: "Previous",
                beginner: "Previous",
              },
            },
            next: {
              label: {
                normal: "Next",
                beginner: "Next",
              },
            },
          },
        },
        ring: {
          demo: {
            label: {
              normal: "Ring",
              beginner: "Ring",
            },
          },
        },
        iconSelector: {
          mode: {
            url: { label: { normal: "URL", beginner: "URL" } },
            shortcode: { label: { normal: "Shortcode", beginner: "Shortcode" } },
            math: { label: { normal: "Math / Typst", beginner: "Math / Typst" } },
            data: { label: { normal: "Data URL", beginner: "Data URL" } },
            emoji: { label: { normal: "Emoji", beginner: "Emoji" } },
            text: { label: { normal: "Text", beginner: "Text" } },
            vector: { label: { normal: "Catalog / SVG", beginner: "Catalog / SVG" } },
          },
        },
        stepper: {
          demo: {
            label: {
              normal: "Value",
              beginner: "Value",
            },
          },
        },
        engagement: {
          actions: {
            label: {
              normal: "Actions",
              beginner: "Quick actions for the current step",
            },
          },
          viewport: {
            label: {
              normal: "Viewport",
              beginner: "Viewport",
            },
          },
        },
        windowSearch: {
          title: {
            label: {
              normal: "Search",
              beginner: "Search",
            },
          },
          action: {
            label: {
              normal: "Action",
              beginner: "Type an action or pick one from the list",
            },
          },
          actionActive: {
            label: {
              normal: "Action or value",
              beginner: "Action or number for the current step",
            },
          },
          suggestions: {
            label: {
              normal: "Suggestions",
              beginner: "Open the list of matching actions",
            },
          },
          noMatches: {
            label: {
              normal: "No matches",
              beginner: "No matching actions",
            },
          },
        },
        flowSpotlight: {
          typeToAdd: { label: { normal: "Type to add…", beginner: "Type to add…" } },
          collapseSuggestions: { label: { normal: "Collapse suggestions", beginner: "Collapse suggestions" } },
          showAllSuggestions: { label: { normal: "Show all suggestions", beginner: "Show all suggestions" } },
        },
        nodeGraph: {
          fitGraph: { label: { normal: "Fit graph", beginner: "Show the whole graph" } },
        },
        sync: {
          attach: { label: { normal: "Attach", beginner: "Attach" } },
          detach: { label: { normal: "Detach", beginner: "Detach" } },
        },
        ink: {
          link: { label: { normal: "Link", beginner: "Link" } },
          linkUrlPrompt: { label: { normal: "Link URL", beginner: "Link URL" } },
        },
        surfaceContextMenu: {
          architecture: { label: { normal: "Architecture Menu", beginner: "Architecture Menu" } },
          attraction: { label: { normal: "Attraction Menu", beginner: "Attraction Menu" } },
          block: { label: { normal: "Block Menu", beginner: "Block Menu" } },
          edge: { label: { normal: "Edge Menu", beginner: "Edge Menu" } },
          entry: { label: { normal: "Entry Menu", beginner: "Entry Menu" } },
          feature: { label: { normal: "Feature Menu", beginner: "Feature Menu" } },
          group: { label: { normal: "Group Menu", beginner: "Group Menu" } },
          handle: { label: { normal: "Handle Menu", beginner: "Handle Menu" } },
          layer: { label: { normal: "Layer Menu", beginner: "Layer Menu" } },
          object: { label: { normal: "Object Menu", beginner: "Object Menu" } },
          part: { label: { normal: "Part Menu", beginner: "Part Menu" } },
          path: { label: { normal: "Path Menu", beginner: "Path Menu" } },
          pixel: { label: { normal: "Pixel Menu", beginner: "Pixel Menu" } },
          position: { label: { normal: "Position Menu", beginner: "Position Menu" } },
          reference: { label: { normal: "Reference Menu", beginner: "Reference Menu" } },
          route: { label: { normal: "Route Menu", beginner: "Route Menu" } },
          slider: { label: { normal: "Slider Menu", beginner: "Slider Menu" } },
          vortex: { label: { normal: "Vortex Menu", beginner: "Vortex Menu" } },
          file: { label: { normal: "File Menu", beginner: "File Menu" } },
          workspace: { label: { normal: "Workspace Menu", beginner: "Workspace Menu" } },
          canvas: { label: { normal: "Canvas Menu", beginner: "Canvas Menu" } },
          scene: { label: { normal: "Scene Menu", beginner: "Scene Menu" } },
          placementSuggestions: { label: { normal: "Placement suggestions", beginner: "Placement suggestions" } },
          node: { label: { normal: "Node Menu", beginner: "Node Menu" } },
          flow: { label: { normal: "Flow Menu", beginner: "Flow Menu" } },
          row: { label: { normal: "Row Menu", beginner: "Row Menu" } },
          paint: { label: { normal: "Paint Menu", beginner: "Paint Menu" } },
          board: { label: { normal: "Board Menu", beginner: "Board Menu" } },
          ink: { label: { normal: "Ink Menu", beginner: "Ink Menu" } },
          history: { label: { normal: "History Menu", beginner: "History Menu" } },
          step: { label: { normal: "Step Menu", beginner: "Step Menu" } },
          diff: { label: { normal: "Diff Menu", beginner: "Diff Menu" } },
          event: { label: { normal: "Event Menu", beginner: "Event Menu" } },
          editor: { label: { normal: "Editor Menu", beginner: "Editor Menu" } },
          map: { label: { normal: "Map Menu", beginner: "Map Menu" } },
        },
        mutation: {
          level: {
            info: { label: { normal: "Info", beginner: "Info" } },
            warning: { label: { normal: "Warning", beginner: "Warning" } },
            error: { label: { normal: "Error", beginner: "Error" } },
            fatal: { label: { normal: "Fatal", beginner: "Fatal error" } },
          },
          code: {
            targetMissing: { label: { normal: "Target missing", beginner: "The target of this change no longer exists." } },
            noOp: { label: { normal: "No change", beginner: "Nothing changed — the state already matched." } },
            partial: { label: { normal: "Partially applied", beginner: "Only part of the change could be applied." } },
            clamped: { label: { normal: "Clamped", beginner: "A value was clamped to its valid range." } },
            duplicateId: { label: { normal: "Duplicate id", beginner: "An element with this id already exists." } },
            invariant: { label: { normal: "Invalid state", beginner: "This change would leave the document in an invalid state." } },
            cascade: { label: { normal: "Cascaded", beginner: "This change triggered further changes." } },
          },
          policy: {
            laissezFaire: {
              label: { label: { normal: "Laissez-faire", beginner: "Laissez-faire" } },
              description: { label: { normal: "Accepts every change unless it is fatal.", beginner: "Accepts every change as long as it isn't fatal." } },
            },
            normal: {
              label: { label: { normal: "Normal", beginner: "Normal" } },
              description: { label: { normal: "Rejects changes with errors, allows warnings.", beginner: "Rejects any change with an error, but allows warnings through." } },
            },
            vigilant: {
              label: { label: { normal: "Vigilant", beginner: "Vigilant" } },
              description: { label: { normal: "Rejects changes with warnings too.", beginner: "Strictest: rejects a change as soon as it carries a warning." } },
            },
            setting: {
              label: { label: { normal: "Merge policy", beginner: "Merge policy" } },
            },
          },
          rejected: {
            title: { label: { normal: "Change rejected", beginner: "Change rejected" } },
            body: { label: { normal: "This change could not be applied.", beginner: "This change could not be applied." } },
          },
        },
        conflict: {
          panel: { label: { normal: "Conflicts", beginner: "Conflicts" } },
          accept: { label: { normal: "Accept", beginner: "Accept" } },
          discard: { label: { normal: "Discard", beginner: "Discard" } },
          quarantined: { label: { normal: "Held back", beginner: "Incoming changes are held back until you decide." } },
          degraded: { label: { normal: "Degraded", beginner: "Applied, but with warnings." } },
        },
        presence: {
          roster: { label: { normal: "People here", beginner: "People here" } },
          empty: { label: { normal: "No one else is here", beginner: "No one else is here" } },
          overflow: { label: { normal: "+{{count}} more", beginner: "+{{count}} more" } },
          role: {
            author: { label: { normal: "Editing", beginner: "Editing" } },
            spectator: { label: { normal: "Viewing", beginner: "Viewing" } },
          },
        },
      },
      settings: {
        layout: {
          desktop: {
            label: {
              normal: "Desktop layout",
              beginner: "Use the standard layout optimized for mouse and keyboard.",
            },
          },
          tablet: {
            label: {
              normal: "Tablet layout",
              beginner: "Use the tablet layout with larger, touch-friendly controls.",
            },
          },
          mobile: {
            label: {
              normal: "Mobile layout",
              beginner: "Uses the mobile layout automatically on small screens.",
            },
          },
        },
        driver: {
          select: { label: { normal: "Driver", beginner: "Driver" } },
          default: { label: { normal: "Default", beginner: "Default" } },
          compact: { label: { normal: "Compact", beginner: "Compact" } },
          labels: { label: { normal: "Labels", beginner: "Labels" } },
          labelsOption: {
            full: { label: { normal: "Full", beginner: "Icon and label" } },
            icons: { label: { normal: "Icons only", beginner: "Icons only" } },
          },
          labelTier: { label: { normal: "Label Tier", beginner: "Label Tier" } },
          labelTierOption: {
            beginner: { label: { normal: "Beginner", beginner: "Verbose labels" } },
            normal: { label: { normal: "Normal", beginner: "Short labels" } },
          },
          drag: { label: { normal: "Drag", beginner: "Drag" } },
          dragOption: {
            handle: { label: { normal: "Handle", beginner: "Dedicated grip handle" } },
            surface: { label: { normal: "Surface", beginner: "Whole element draggable" } },
          },
          chrome: { label: { normal: "Chrome Reveal", beginner: "Chrome Reveal" } },
          chromeOption: {
            always: { label: { normal: "Always", beginner: "Always visible" } },
            hover: { label: { normal: "On Hover", beginner: "Visible only near the cursor" } },
          },
          gumball: { label: { normal: "Gumball Reveal", beginner: "Gumball Reveal" } },
          gumballOption: {
            always: { label: { normal: "Always", beginner: "Always visible" } },
            hover: { label: { normal: "On Hover", beginner: "Visible only near the cursor" } },
          },
          tooltips: { label: { normal: "Tooltips", beginner: "Tooltips" } },
          tooltipsOption: {
            full: { label: { normal: "Full", beginner: "With manual and tutorial links" } },
            minimal: { label: { normal: "Minimal", beginner: "Name and hotkey only" } },
            none: { label: { normal: "None", beginner: "No tooltips" } },
          },
          hotkeys: { label: { normal: "Hotkeys", beginner: "Hotkeys" } },
          hotkeysOption: {
            inline: { label: { normal: "Inline", beginner: "On the control" } },
            tooltip: { label: { normal: "Tooltip", beginner: "In tooltip only" } },
            none: { label: { normal: "None", beginner: "Hidden" } },
          },
          save: { label: { normal: "Save As", beginner: "Save As" } },
          savePlaceholder: { label: { normal: "Driver name", beginner: "Driver name" } },
          delete: { label: { normal: "Delete", beginner: "Delete" } },
          dirty: { label: { normal: "Unsaved", beginner: "Unsaved" } },
        },
        keybindings: {
          capture: { label: { normal: "Record", beginner: "Record" } },
          reset: { label: { normal: "Reset", beginner: "Reset" } },
          conflict: { label: { normal: "Conflict", beginner: "Already assigned" } },
          pressKeys: { label: { normal: "Press keys…", beginner: "Press keys…" } },
        },
      },
      tooltip: {
        manual: {
          label: {
            normal: "Manual",
            beginner: "Manual",
          },
        },
        tutorial: {
          label: {
            normal: "Tutorial",
            beginner: "Tutorial",
          },
        },
      },
      introduction: {
        skip: { label: { normal: "Skip", beginner: "Skip" } },
        back: { label: { normal: "Back", beginner: "Back" } },
        next: { label: { normal: "Next", beginner: "Next" } },
        done: { label: { normal: "Done", beginner: "Done" } },
      },
      tutorial: {
        play: { label: { normal: "Play", beginner: "Play" } },
        pause: { label: { normal: "Pause", beginner: "Pause" } },
        stop: { label: { normal: "Stop Tutorial", beginner: "Stop Tutorial" } },
        rate: { label: { normal: "Speed", beginner: "Speed" } },
        mute: { label: { normal: "Mute", beginner: "Mute" } },
        captions: { label: { normal: "Captions", beginner: "Captions" } },
        record: { label: { normal: "Record", beginner: "Record" } },
        recording: { label: { normal: "Recording", beginner: "Recording" } },
        addChapter: { label: { normal: "Add Chapter", beginner: "Add Chapter" } },
        chapter: { label: { normal: "Chapter", beginner: "Chapter" } },
      },
    } satisfies UiTranslationSchema,
  },
  // #endregion 🇬️🇧️ English Bundle
} satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>;

// #region 🔌️I18n Port
// i18n "port"/wiring glue: registration functions, locale resolvers, the i18next module augmentation, and the shared port instance.

export type UiTranslationLocaleCode = UiLocale;

export type UiTranslationBundlesInput = {
  readonly [L in UiLocale]: { readonly translation: Record<string, unknown> };
};

declare module "i18next" {
  interface CustomTypeOptions {
    defaultNS: "translation";
    resources: {
      readonly en: { readonly translation: UiTranslationSchema };
      readonly de: { readonly translation: UiTranslationSchema };
    };
  }
}

// UiRegisteredTranslationKey imported from core I18n above/with schema import

/** @emoji 🪁️ Merges additional locale bundles into the shared UI i18n instance, requiring every
 * {@link UiLocale} to register the exact same schema `S` (a compile error otherwise — the same
 * both-locales-or-nothing guarantee the domain-neutral chrome bundle gets from `satisfies
 * UiTranslationSchema`). Returns a caster from `S`'s own dot-path key union to {@link UiRegisteredTranslationKey}
 * — the only way callers should obtain a key for their registered strings; passing an unregistered
 * string does not type-check. */
export function registerUiTranslationBundles<S extends Record<string, unknown>>(bundles: { readonly [L in UiLocale]: { readonly translation: S } }): <K extends DeepUiTranslationKeys<S>>(key: K) => UiRegisteredTranslationKey {
  applyUiTranslationBundleTo(i18next, bundles);
  registeredUiTranslationBundles.push(bundles);
  for (const instance of liveShellI18nInstances) applyUiTranslationBundleTo(instance, bundles);
  return (key) => key as UiRegisteredTranslationKey;
}

// #region 🐚️ShellI18n
/** 🐚️ Every bundle ever registered — the chrome's own domain-neutral one plus every product's, in
 * registration order — replayed onto each new per-shell i18next instance at creation time so an
 * embedded shell has the same translations as the page-owning singleton from its very first render.
 * Seeded with `uiChromeTranslationBundles` itself since that one is loaded directly via `.init({resources})`
 * rather than through `registerUiTranslationBundles`. */
const registeredUiTranslationBundles: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }[] = [uiChromeTranslationBundles];

/** 🐚️ Every currently-mounted shell's own i18next instance — {@link registerUiTranslationBundles}
 * replays a late-registering bundle (e.g. a lazily-loaded product module importing after some shells
 * already mounted) into each of these too, not just the shared singleton. */
const liveShellI18nInstances = ephemeralSet<typeof i18next>("framework.modules.ui.packages.typescript.targets.react.index.tsx.liveShellI18nInstances");

function applyUiTranslationBundleTo(instance: typeof i18next, bundle: { readonly [L in UiLocale]: { readonly translation: Record<string, unknown> } }): void {
  for (const [language, resource] of Object.entries(bundle)) {
    instance.addResourceBundle(language, "translation", resource.translation, true, true);
  }
}

/** 🐚️ Creates and synchronously initializes a fresh i18next instance for one shell, pre-loaded with
 * every bundle registered so far — mirrors {@link initializeUiI18n}'s own synchronous init (`initImmediate:
 * false`) so an embedded shell never flashes untranslated chrome on its first paint either. `react-i18next`
 * resolves the *nearest* `I18nextProvider` ancestor via context, so wrapping a shell's subtree in one
 * (see `FrameworkOsShell`) is the only wiring `useUiTranslation`/`useLabel` call sites need — none of
 * their many call sites throughout this file change. */
export function createShellI18nInstance(initialLocale: UiLocale): typeof i18next {
  const instance = i18next.createInstance();
  instance.use(initReactI18next);
  void instance.init({
    resources: {},
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: initialLocale,
    showSupportNotice: false,
    returnObjects: true,
    initImmediate: false,
    interpolation: { escapeValue: false },
    react: { useSuspense: false, bindI18n: "languageChanged", bindI18nStore: "added removed" },
  });
  for (const bundle of registeredUiTranslationBundles) applyUiTranslationBundleTo(instance, bundle);
  liveShellI18nInstances.add(instance);
  return instance;
}

/** 🐚️ Releases a shell's i18next instance on unmount — stops it receiving future
 * {@link registerUiTranslationBundles} replays. */
export function disposeShellI18nInstance(instance: typeof i18next): void {
  liveShellI18nInstances.delete(instance);
}
// #endregion 🐚️ShellI18n

//#region 🗣️TsNativeTerminology
/** @emoji 🗣️ A `(locale) -> label-record` pair for one terminology id, mirroring the Rust `*_LABELS_{ID}_{LOCALE}` const pattern for TS-native products (e.g. compose, coda) that never cross the WASM plugin boundary and so have no `AppDefinition.terminologies`/`AppLabelsOverlay`. */
export type UiTerminologyLabelSet<Keys extends string> = Readonly<Record<UiLocale, Readonly<Record<Keys, string>>>>;

/** @emoji 🗣️ Builds a `(terminologyId, locale) -> labels` resolver from a set of terminology-keyed label tables, falling back to `native` for unknown/undeclared ids — the TS analog of the Rust `puzzle2d_labels`-style resolver. */
export function createTerminologyLabelResolver<Keys extends string>(sets: Readonly<Record<string, UiTerminologyLabelSet<Keys>>>): (terminologyId: string, locale: UiLocale) => Readonly<Record<Keys, string>> {
  return (terminologyId, locale) => (sets[terminologyId] ?? sets[UI_TERMINOLOGY_NATIVE])![locale];
}

const uiTerminologyChangeListeners = ephemeralSet<() => void>("framework.modules.ui.packages.typescript.targets.react.index.tsx.uiTerminologyChangeListeners");

/** @emoji 🗣️ React hook giving TS-native products (no Rust `AppDefinition`) read/write access to the shared `ui.chrome.terminology` contract — the same localStorage key the shell's Settings terminology dropdown drives — without depending on `os-shell` state or any Rust type. */
export function useUiTerminology(): { readonly terminology: string; readonly setTerminology: (id: string) => void } {
  const storage = shellScopeStorageOrBrowserFallback(useShellScopeOptional());
  const [terminology, setTerminologyState] = React.useState<string>(() => readStoredUiChromeTerminology(storage));
  React.useEffect(() => {
    const onStorage = (event: StorageEvent) => {
      if (event.key === UI_CHROME_TERMINOLOGY_STORAGE_KEY) setTerminologyState(readStoredUiChromeTerminology(storage));
    };
    const onLocalChange = () => setTerminologyState(readStoredUiChromeTerminology(storage));
    uiTerminologyChangeListeners.add(onLocalChange);
    if (typeof window !== "undefined") window.addEventListener("storage", onStorage);
    return () => {
      uiTerminologyChangeListeners.delete(onLocalChange);
      if (typeof window !== "undefined") window.removeEventListener("storage", onStorage);
    };
  }, [storage]);
  const setTerminology = React.useCallback(
    (id: string) => {
      writeStoredUiChromeTerminology(storage, id);
      for (const listener of uiTerminologyChangeListeners) listener();
    },
    [storage],
  );
  return { terminology, setTerminology };
}
//#endregion 🗣️TsNativeTerminology

function normalizeUiLocale(language?: string): UiTranslationLocaleCode {
  return language?.toLowerCase().startsWith("de") ? "de" : "en";
}

/** @emoji 🧭️ Maps a BCP47 tag (e.g. `navigator.language`, `"de-AT"`) onto a {@link ShellLocale};
 * defaults to `"en"`. Same rule the chrome's own locale detector uses — exposed so boot code
 * (renderer/demonstrator) can resolve a default before any brand lock is known. */
export const detectShellLocale = normalizeUiLocale as (language?: string) => ShellLocale;

function resolveRequestedUiLocale(): UiTranslationLocaleCode {
  // 🐚️ The legacy shared `uiI18n` singleton (kept only for callers not yet wrapped in a `ShellScopeProvider`)
  // is inherently page-global, so a plain browser-backed port is the correct (and only sensible) storage here.
  const storedLocale = readStoredUiChromeLocale(createBrowserStoragePort());
  if (storedLocale) return storedLocale;
  return normalizeUiLocale(i18next.resolvedLanguage || i18next.language || (typeof navigator !== "undefined" ? navigator.language : undefined));
}

function registerUiChromeTranslationBundles() {
  registerUiTranslationBundles(uiChromeTranslationBundles);
}

function createUiI18nPort(instance: typeof i18next): UiI18nPort {
  return {
    t: ((key, options) => instance.t(key as never, options as never)) as UiTranslateFn,
    changeLanguage: (locale) => instance.changeLanguage(locale),
    get language() {
      return instance.language;
    },
    get resolvedLanguage() {
      return instance.resolvedLanguage;
    },
    get isInitialized() {
      return instance.isInitialized;
    },
  };
}

function initializeUiI18n(): UiI18nPort {
  const requestedLocale = resolveRequestedUiLocale();

  if (i18next.isInitialized) {
    registerUiChromeTranslationBundles();
    if (i18next.language !== requestedLocale) {
      void i18next.changeLanguage(requestedLocale);
    }
    return createUiI18nPort(i18next);
  }

  i18next.use(initReactI18next);

  void i18next.init({
    resources: uiChromeTranslationBundles,
    fallbackLng: "en",
    supportedLngs: ["en", "de"],
    nonExplicitSupportedLngs: true,
    lng: requestedLocale,
    showSupportNotice: false,
    returnObjects: true,
    // 🚀️ Resources are bundled inline above (no backend fetch), so there is nothing to await —
    // forces synchronous readiness instead of deferring to a microtask, which is what let the
    // very first paint render with i18next still uninitialized (the English-chrome flash this
    // ticket fixes; see `initUiLocaleSync`).
    initImmediate: false,
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
      bindI18n: "languageChanged",
      bindI18nStore: "added removed",
    },
  });

  return createUiI18nPort(i18next);
}

/** @emoji 🪁️ Shared UI i18n port (domain-neutral bundles; extend via {@link registerUiTranslationBundles}). */
export const uiI18n = initializeUiI18n();

/** @emoji 🪁️ Sets the active UI locale on the shared i18n port (user-initiated, in-app switch —
 * for boot-time/brand-locked locale resolution, use {@link initUiLocaleSync} instead, which runs
 * before the first render rather than in a post-paint effect). */
export function setUiLocale(locale: UiLocale): Promise<unknown> {
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  return uiI18n.changeLanguage(locale);
}

/** @emoji 🚀️ Resolves the shell's locale synchronously, before the first React render — call this
 * at renderer/demonstrator boot (module scope or before `ReactDOM.createRoot(...).render(...)`),
 * never from a `useEffect`. A `useEffect`-based call runs after the first paint has already
 * committed, which is exactly how a German-locked brand could still flash English chrome
 * ("Skip"/"Back"/"Next"/"Done") on first load. Persists the locale (so a reload's
 * `resolveRequestedUiLocale` agrees) and sets `documentElement.lang` synchronously; also nudges the
 * already-initialized i18next instance in case this runs after `uiI18n`'s own module-load default
 * resolved differently. */
export function initUiLocaleSync(locale: ShellLocale): void {
  // 🐚️ Page-owning boot code only (renderer/demonstrator, before `createRoot(...).render(...)`) — a
  // plain browser-backed port is correct here, same as `resolveRequestedUiLocale`.
  writeStoredUiChromeLocale(createBrowserStoragePort(), locale);
  if (typeof document !== "undefined") document.documentElement.lang = locale;
  if (i18next.language !== locale) void i18next.changeLanguage(locale);
}

// #endregion 🔌️I18n Port

// #endregion 🪁️I18n Resources

/**
 * Hook binding a keyboard shortcut from the control registry, shell table, or a raw chord literal.
 **/
export function useActionHotkey(
  hotkeyOrControlId: string,
  callback: ControlKeybindingCallback,
  options?: ControlKeybindingOptions,
  dependencies?: ControlKeybindingDependencies,
  configuration?: {
    overrides?: Record<string, string> | undefined;
  },
) {
  const bindings = useUiKeybindingsByControlId();
  const finalHotkey = reactHostPort.useMemo(() => {
    const override = configuration?.overrides?.[hotkeyOrControlId];
    if (override) return override;
    const fromRegistry = resolveControlKeybindingRaw(hotkeyOrControlId, bindings) ?? SHELL_KEYBINDINGS[hotkeyOrControlId];
    if (fromRegistry) return fromRegistry;
    return hotkeyOrControlId;
  }, [bindings, configuration?.overrides, hotkeyOrControlId]);

  useHotkeys(finalHotkey, callback, options ?? {}, dependencies ?? []);
}

/** @emoji ⌨️ Chords for toggling each panel's fold/unfold state (derived from {@link SHELL_KEYBINDINGS}). */
export const PANEL_TOGGLE_HOTKEYS: Record<Anchor, string> = {
  "top-left": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["top-left"]],
  "top-middle": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["top-middle"]],
  "top-right": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["top-right"]],
  "right-middle": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["right-middle"]],
  "bottom-right": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["bottom-right"]],
  "bottom-middle": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["bottom-middle"]],
  "bottom-left": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["bottom-left"]],
  "left-middle": SHELL_KEYBINDINGS[SHELL_PANEL_ANCHOR_KEY_IDS["left-middle"]],
};

/**
 * ⌨️ Binds {@link PANEL_TOGGLE_HOTKEYS} for all eight anchors when a handler is provided.
 **/
export function usePanelChromeHotkeys(options: { readonly onToggle?: (anchor: Anchor) => void }): void {
  const { onToggle } = options;
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-left"], () => onToggle?.("top-left"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-middle"], () => onToggle?.("top-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["top-right"], () => onToggle?.("top-right"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["right-middle"], () => onToggle?.("right-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-right"], () => onToggle?.("bottom-right"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-middle"], () => onToggle?.("bottom-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["bottom-left"], () => onToggle?.("bottom-left"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
  useHotkeys(PANEL_TOGGLE_HOTKEYS["left-middle"], () => onToggle?.("left-middle"), { preventDefault: true, enabled: onToggle != null }, [onToggle]);
}

/**
 * Hook returning whether a CSS media query currently matches.
 **/
export function useMediaQuery(query: string, defaultValue = false): boolean {
  const getMatches = reactHostPort.useCallback(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return defaultValue;
    }

    return window.matchMedia(query).matches;
  }, [defaultValue, query]);

  const [matches, setMatches] = reactHostPort.useState<boolean>(getMatches);

  reactHostPort.useEffect(() => {
    if (typeof window === "undefined" || typeof window.matchMedia !== "function") {
      return undefined;
    }

    const mediaQueryList = window.matchMedia(query);
    const bindings = createDOMEventBinding();
    const handleChange = (event: MediaQueryListEvent) => setMatches(event.matches);
    setMatches(mediaQueryList.matches);
    bindings.listen(mediaQueryList, "change", handleChange);

    return () => {
      bindings.dispose();
    };
  }, [query]);

  return matches;
}

// #region 📱️UiMobile Context
const UiMobileContext = reactHostPort.createContext<boolean | undefined>(undefined);

/** @emoji 📱️ Broadcasts the shell's authoritative mobile flag to descendants (e.g. {@link Pane}, {@link Window}) that have no `mobile` prop of their own. */
export const UiMobileProvider: React.FC<{
  readonly mobile: boolean;
  readonly children: React.ReactNode;
}> = ({ mobile, children }) => <UiMobileContext.Provider value={mobile}>{children}</UiMobileContext.Provider>;

/** @emoji 📱️ Returns the nearest {@link UiMobileProvider} flag, falling back to {@link UI_MOBILE_MEDIA_QUERY} for standalone usage (Storybook, tests). */
export function useUiMobile(): boolean {
  const media = useMediaQuery(UI_MOBILE_MEDIA_QUERY);
  const ctx = reactHostPort.useContext(UiMobileContext);
  return ctx ?? media;
}
// #endregion 📱️UiMobile Context

/**
 * 3D point with x, y, z coordinates.
 **/
export interface Point {
  x: number;
  y: number;
  z: number;
}

/**
 * 3D direction vector with x, y, z components.
 **/
export interface Vector {
  x: number;
  y: number;
  z: number;
}

/**
 * 3D coordinate plane defined by an origin point and two axis vectors.
 **/
export interface Plane {
  origin: Point;
  xAxis: Vector;
  yAxis: Vector;
}

/**
 * 3D camera defined by position, forward direction, and up direction.
 **/
export interface Camera {
  position: Point;
  forward: Vector;
  up: Vector;
}

// #endregion 🎼️Utilities

// #region 🔊️Section Specificity
// Enum defining priority levels for section content ownership.
// Consumers MUST use these constants for section precedence.

/**
 * Priority enum for section content ownership across apps.
 **/
export enum SectionSpecificity {
  SKETCHPAD = 0,
  KIT = 10,
  QUALITY = 20,
  TYPE = 20,
  DESIGN = 20,
  DOCS = 20,
  SELECTION = 30,
}

// #endregion 🔊️Section Specificity

// #region 🔤️Interaction Context
// Global ghost mode hides open panel/pane chrome during interactions; navbar/footer panel toggles stay visible.

const PANEL_GHOST_MOVE_THRESHOLD_PX = 4;

/** @emoji 👻️ Global ghost session API (begin/end while dragging or editing). */
export interface PanelGhostValue {
  readonly active: boolean;
  readonly begin: (target: EventTarget | null) => void;
  readonly end: () => void;
}

const PanelGhostContext = reactHostPort.createContext<PanelGhostValue | undefined>(undefined);

/** @emoji 👻️ Returns the global ghost controller when inside {@link GhostProvider}. */
export const usePanelGhost = (): PanelGhostValue | undefined => reactHostPort.useContext(PanelGhostContext);

interface InteractionCommands {
  setActiveInteraction: (elementId?: string, interactionId?: string) => void;
}

const InteractionContext = reactHostPort.createContext<InteractionCommands | undefined>(undefined);
const ActiveInteractionContext = reactHostPort.createContext<string | undefined>(undefined);

/** @emoji 🔤️ External provider for interaction commands; prefer {@link GhostProvider} at layout root. */
export const InteractionProvider: React.FC<{
  commands?: InteractionCommands;
  activeInteraction?: string;
  children: React.ReactNode;
}> = ({ commands, activeInteraction, children }) => (
  <InteractionContext.Provider value={commands}>
    <ActiveInteractionContext.Provider value={activeInteraction}>{children}</ActiveInteractionContext.Provider>
  </InteractionContext.Provider>
);

export const useInteractionCommands = () => reactHostPort.useContext(InteractionContext);
const useActiveInteraction = () => reactHostPort.useContext(ActiveInteractionContext);

type GhostController = PanelGhostValue & {
  readonly commands: InteractionCommands;
  readonly activeInteraction?: string;
};

function findGhostRegionAncestor(target: Element): Element | null {
  let node: Element | null = target;
  while (node) {
    if (node.hasAttribute("data-ghost-region")) return node;
    node = node.parentElement;
  }
  return null;
}

/** @emoji 📐️ Pane/panel/mode edge resize is layout chrome, not a canvas ghost interaction. */
function isChromeResizeHandleTarget(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  const slotted = target.closest("[data-slot]");
  if (!slotted) return false;
  const slot = slotted.getAttribute("data-slot") ?? "";
  return slot === "pane-resize-handle" || slot === "panel-resize-handle" || slot === "resizable-handle" || slot === "resizable-corner" || slot.startsWith("window-measures-resize");
}

/** @emoji 👻️ Keeps automatic ghosting on interaction surfaces and direct tree rows, not nested UI controls. */
function shouldBeginAutomaticGhostInteraction(target: EventTarget | null): boolean {
  if (!(target instanceof Element)) return false;
  if (isChromeResizeHandleTarget(target)) return false;
  const region = findGhostRegionAncestor(target);
  const dimmed = target.closest("[data-dim]");
  if (!dimmed || (region && !region.contains(dimmed))) return true;
  if (!region) return false;
  return resolveHoverRow(target as HTMLElement, region as HTMLElement) === dimmed;
}

function useGhostController(): GhostController {
  const [active, setActive] = reactHostPort.useState(false);
  const [activeInteraction, setActiveInteractionState] = reactHostPort.useState<string | undefined>(undefined);
  const activeMarkedRefs = reactHostPort.useRef<HTMLElement[]>([]);
  const sessionActiveRef = reactHostPort.useRef(false);

  const clearActiveMarks = reactHostPort.useCallback(() => {
    for (const el of activeMarkedRefs.current) {
      el.removeAttribute("data-active-interaction");
      el.removeAttribute("data-active-ancestor");
    }
    activeMarkedRefs.current = [];
  }, []);

  const begin = reactHostPort.useCallback(
    (target: EventTarget | null) => {
      clearActiveMarks();
      sessionActiveRef.current = true;
      setActive(true);
      if (!target || !(target instanceof Element)) return;
      const region = findGhostRegionAncestor(target);
      if (!region) return;
      activeMarkedRefs.current = markGhostTreeInteraction(target, region);
    },
    [clearActiveMarks],
  );

  const end = reactHostPort.useCallback(() => {
    clearActiveMarks();
    sessionActiveRef.current = false;
    setActive(false);
    setActiveInteractionState(undefined);
  }, [clearActiveMarks]);

  const setActiveInteraction = reactHostPort.useCallback(
    (elementId?: string, interactionId?: string) => {
      if (interactionId) {
        setActiveInteractionState(interactionId);
        const el = elementId ? document.getElementById(elementId) : null;
        begin(el);
        return;
      }
      end();
    },
    [begin, end],
  );

  const commands = reactHostPort.useMemo<InteractionCommands>(() => ({ setActiveInteraction }), [setActiveInteraction]);

  const controllerRef = reactHostPort.useRef<GhostController | null>(null);
  controllerRef.current = { active, begin, end, commands, activeInteraction };

  reactHostPort.useEffect(() => {
    const pendingRef: { current: { readonly x: number; readonly y: number; readonly target: EventTarget | null } | null } = { current: null };
    const beganViaPointerDragRef = { current: false };
    const bindings = createDOMEventBinding();
    const capture = true;
    const onDown = (event: Event) => {
      const pointerEvent = event as PointerEvent;
      if (pointerEvent.button !== 0) return;
      pendingRef.current = { x: pointerEvent.clientX, y: pointerEvent.clientY, target: pointerEvent.target };
    };
    const onMove = (event: Event) => {
      const pointerEvent = event as PointerEvent;
      const pending = pendingRef.current;
      if (!pending || sessionActiveRef.current) return;
      const dx = pointerEvent.clientX - pending.x;
      const dy = pointerEvent.clientY - pending.y;
      if (dx * dx + dy * dy < PANEL_GHOST_MOVE_THRESHOLD_PX * PANEL_GHOST_MOVE_THRESHOLD_PX) return;
      if (!shouldBeginAutomaticGhostInteraction(pending.target)) {
        pendingRef.current = null;
        return;
      }
      controllerRef.current?.begin(pending.target);
      beganViaPointerDragRef.current = true;
      pendingRef.current = null;
    };
    const onUp = () => {
      pendingRef.current = null;
      if (beganViaPointerDragRef.current) {
        beganViaPointerDragRef.current = false;
        controllerRef.current?.end();
      }
    };
    bindings.listen(document, "pointerdown", onDown, capture);
    bindings.listen(document, "pointermove", onMove, capture);
    bindings.listen(document, "pointerup", onUp, capture);
    bindings.listen(document, "pointercancel", onUp, capture);
    return () => bindings.dispose();
  }, []);

  return reactHostPort.useMemo(() => ({ active, begin, end, commands, activeInteraction }), [active, activeInteraction, begin, commands, end]);
}

/** @emoji 👻️ Mounts global ghost detection and interaction context for layout + panels. */
export const GhostProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const ghost = useGhostController();
  reactHostPort.useEffect(() => {
    setPanelGhostSessionBridge({ begin: ghost.begin, end: ghost.end });
    return () => setPanelGhostSessionBridge(null);
  }, [ghost.begin, ghost.end]);
  const panelGhostValue = reactHostPort.useMemo<PanelGhostValue>(() => ({ active: ghost.active, begin: ghost.begin, end: ghost.end }), [ghost.active, ghost.begin, ghost.end]);
  return (
    <PanelGhostContext.Provider value={panelGhostValue}>
      <InteractionContext.Provider value={ghost.commands}>
        <ActiveInteractionContext.Provider value={ghost.activeInteraction}>{children}</ActiveInteractionContext.Provider>
      </InteractionContext.Provider>
    </PanelGhostContext.Provider>
  );
};

interface GhostRegionShellProps extends React.HTMLAttributes<HTMLDivElement> {
  children?: React.ReactNode;
  /** @emoji 🫳️ When true, the region root is click-through while ghosted (panels over canvas). */
  clickThroughWhenGhost?: boolean;
  /** @emoji 👻️ When false, this region stays undimmed during a global ghost session (navbar/footer panel toggles). Default true — open panels/panes hide. */
  sessionGhost?: boolean;
}

/** @emoji 👻️ Ghost region shell: dims {@link data-dim} children when {@link GhostProvider} is active and {@link sessionGhost} is true. */
export const GhostRegionShell = reactHostPort.forwardRef<HTMLDivElement, GhostRegionShellProps>(function GhostRegionShell({ children, className, style, clickThroughWhenGhost = false, sessionGhost = true, ...props }, ref) {
  const ghost = usePanelGhost();
  const ghostActive = Boolean(sessionGhost && ghost?.active);
  const mergedStyle = reactHostPort.useMemo(
    () => ({
      ...(style as React.CSSProperties | undefined),
      pointerEvents: clickThroughWhenGhost && ghostActive ? ("none" as const) : undefined,
    }),
    [clickThroughWhenGhost, ghostActive, style],
  );
  return (
    <div ref={ref} data-ghost-region data-ghost={ghostActive ? "true" : undefined} className={className} style={mergedStyle} {...props}>
      {children}
    </div>
  );
});

interface PanelGhostRootProps extends React.HTMLAttributes<HTMLDivElement> {
  children: React.ReactNode;
}

/** @emoji 👻️ Panel shell marked as a ghost region; dims when {@link GhostProvider} session is active. */
export const PanelGhostRoot = reactHostPort.forwardRef<HTMLDivElement, PanelGhostRootProps>(function PanelGhostRoot({ children, className, style, ...props }, ref) {
  const level = useLevel();
  return (
    <GhostRegionShell ref={ref} clickThroughWhenGhost data-level={level} data-elevation-root="" className={className} style={style} {...props}>
      {children}
    </GhostRegionShell>
  );
});

// #endregion 🔤️Interaction Context

// #region 🎓️Introduction
/** 🪟️ The glass overlay box recipe shared by the introduction info box and modal dialogs (see
 * `UIDialog`) — the two mechanisms are styled identically by construction, not by convention.
 * Border color is CSS-owned (`[data-slot="introduction-info-box"|"dialog-box"]`): normal at rest,
 * emphasized on `:hover` only — never `:focus-within`, so clicking a button does not leave the
 * border stuck after the pointer leaves. */
export const GLASS_OVERLAY_BOX_CLASS = cn("text-foreground pointer-events-auto fixed z-tutorial max-w-sm rounded-lg p-double shadow-lg");

type IntroductionRect = { readonly top: number; readonly left: number; readonly width: number; readonly height: number };

/** @emoji 🎓️ Converts a viewport (getBoundingClientRect) box into coordinates local to `host`. */
export function introductionRectRelativeToHost(rect: IntroductionRect, hostRect: IntroductionRect): IntroductionRect {
  return { top: rect.top - hostRect.top, left: rect.left - hostRect.left, width: rect.width, height: rect.height };
}

/** @emoji 🎓️ Viewport-pixel point → host-local point for absolute overlays inside a transformed shell. */
export function introductionPointRelativeToHost(point: { readonly x: number; readonly y: number }, hostRect: IntroductionRect): { readonly x: number; readonly y: number } {
  return { x: point.x - hostRect.left, y: point.y - hostRect.top };
}

/** @emoji 🎓️ Live-tracks the union DOM rect of an introduction step's `introduce` element(s) (via
 * {@link elementIdSelector}), stamping every match `data-introduced="true"` (pulsing the introduced border, see
 * `framework/ui/styling/js/🎨️ui.css`) for as long as they stay attached — cleared on unmount/selector change. A kind-level
 * window id that aliases every open instance (Top + Perspective) therefore pulses and anchors against the
 * union of all matches, not only the first. Waits for the element(s) to mount (a folded panel/utility bar the
 * shell is in the middle of revealing) instead of failing closed; a `null` selector or a never-found element
 * both resolve to `null`, which the caller treats as a `Screen`-style full veil. A `MutationObserver` on the
 * document re-attempts attachment on every DOM change until at least one mounts, then keeps observing so
 * late-arriving aliases (extra window instances) join the pulse set. */
function useIntroductionAnchorRect(selector: string | null): IntroductionRect | null {
  const [rect, setRect] = reactHostPort.useState<IntroductionRect | null>(null);
  // 🐚️ Falls back to `document`/`document.body` outside any shell (e.g. the mit-bestand demonstrator's
  // own standalone tour) — inside one, scopes the search+observe to that shell's own root so an id/alias
  // shared across two mounted shells only anchors within the one actually running this introduction.
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (!selector) {
      setRect(null);
      return;
    }
    const getSearchRoot = (): ParentNode | null => {
      if (shellScope) return shellScope.rootRef.current;
      return document;
    };
    const observeRoot: globalThis.Node = shellScope?.rootRef.current ?? document.body;
    let elements: Element[] = [];
    let resizeObserver: ResizeObserver | null = null;
    let reportedUnresolved = false;

    const unionRect = (matches: readonly Element[]): IntroductionRect | null => {
      if (matches.length === 0) return null;
      let top = Infinity;
      let left = Infinity;
      let right = -Infinity;
      let bottom = -Infinity;
      for (const match of matches) {
        const box = match.getBoundingClientRect();
        top = Math.min(top, box.top);
        left = Math.min(left, box.left);
        right = Math.max(right, box.right);
        bottom = Math.max(bottom, box.bottom);
      }
      return { top, left, width: right - left, height: bottom - top };
    };
    const measure = () => {
      const next = unionRect(elements);
      if (next) setRect(next);
    };
    const attach = () => {
      const searchRoot = getSearchRoot();
      if (!searchRoot) {
        if (elements.length > 0) {
          elements.forEach((el) => el.removeAttribute("data-introduced"));
          elements = [];
          resizeObserver?.disconnect();
          resizeObserver = null;
        }
        if (!reportedUnresolved) {
          reportedUnresolved = true;
          setRect(null);
        }
        return;
      }
      const candidates = [...searchRoot.querySelectorAll(selector)];
      if (candidates.length === 0) {
        if (elements.length > 0) {
          elements.forEach((el) => el.removeAttribute("data-introduced"));
          elements = [];
          resizeObserver?.disconnect();
          resizeObserver = null;
        }
        if (!reportedUnresolved) {
          reportedUnresolved = true;
          setRect(null);
        }
        return;
      }
      const same = candidates.length === elements.length && candidates.every((candidate, index) => candidate === elements[index]);
      if (same) return;
      const previous = new Set(elements);
      const next = new Set(candidates);
      previous.forEach((el) => {
        if (!next.has(el)) el.removeAttribute("data-introduced");
      });
      candidates.forEach((el) => el.setAttribute("data-introduced", "true"));
      elements = candidates;
      resizeObserver?.disconnect();
      resizeObserver = new ResizeObserver(measure);
      candidates.forEach((el) => resizeObserver?.observe(el));
      measure();
    };

    attach();
    const mutationObserver = new MutationObserver(attach);
    mutationObserver.observe(observeRoot, { childList: true, subtree: true });
    const onResize = () => measure();
    window.addEventListener("resize", onResize);
    return () => {
      elements.forEach((el) => el.removeAttribute("data-introduced"));
      resizeObserver?.disconnect();
      mutationObserver.disconnect();
      window.removeEventListener("resize", onResize);
      setRect(null);
    };
  }, [selector, shellScope]);

  return rect;
}

/** @emoji 🎓️ Elevates the chrome unit containing each of `ids` above the single fullscreen introduction
 * veil, by stamping `data-introduction-elevated` (see `framework/ui/styling/js/🎨️ui.css`) on its nearest
 * `[data-slot="mode-dock-stack"]` (full window silhouette: tabs + gap + controls + body) or else its
 * nearest `[data-elevation-root]` ancestor — the panel/window/navbar/footer that owns a real, root-level
 * stacking context (a deeply-nested target such as a tree row can't be raised on its own; CSS stacking
 * contexts trap it inside its panel/window regardless of its own z-index). An id with no enclosing
 * chrome unit elevates itself directly, gaining `position: relative` as a fallback if it was
 * `static` (restored on cleanup). Every DOM match of an id is elevated — a window-kind id aliased onto
 * every open instance (Top + Perspective via `data-element-alias`) therefore raises all of them, not
 * only the first `querySelector` hit. Waits for each id to mount (a folded utility bar/panel the shell is
 * still revealing) via the same `MutationObserver` re-attach pattern as `useIntroductionAnchorRect`, and
 * returns the ids that currently resolved so the caller can tell "still waiting" from "revealed" per id
 * (e.g. to decide whether the veil should block pointer events yet). All stamps and fallback positioning
 * are undone on id-list change and unmount. */
function useIntroductionElevation(ids: readonly string[]): ReadonlySet<string> {
  const [resolved, setResolved] = reactHostPort.useState<ReadonlySet<string>>(() => new Set());
  // 🐚️ See `useIntroductionAnchorRect`'s doc for why this is scoped to a shell's own root when one exists.
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (ids.length === 0) {
      setResolved(new Set());
      return;
    }
    const getSearchRoot = (): ParentNode | null => {
      if (shellScope) return shellScope.rootRef.current;
      return document;
    };
    const observeRoot: globalThis.Node = shellScope?.rootRef.current ?? document.body;
    let stampedRoots = new Set<Element>();
    const positionedByUs = new Set<Element>();

    const resolve = () => {
      const searchRoot = getSearchRoot();
      if (!searchRoot) return;
      const nextResolved = new Set<string>();
      const wantedRoots = new Set<Element>();
      for (const id of ids) {
        const targets = searchRoot.querySelectorAll(elementIdSelector(id));
        if (targets.length === 0) continue;
        nextResolved.add(id);
        targets.forEach((target) => wantedRoots.add(target.closest('[data-slot="mode-dock-stack"]') ?? target.closest("[data-elevation-root]") ?? target));
      }
      stampedRoots.forEach((root) => {
        if (wantedRoots.has(root)) return;
        root.removeAttribute("data-introduction-elevated");
        if (positionedByUs.has(root)) {
          (root as HTMLElement).style.position = "";
          positionedByUs.delete(root);
        }
      });
      wantedRoots.forEach((root) => {
        root.setAttribute("data-introduction-elevated", "true");
        if (!positionedByUs.has(root) && getComputedStyle(root as Element).position === "static") {
          (root as HTMLElement).style.position = "relative";
          positionedByUs.add(root);
        }
      });
      stampedRoots = wantedRoots;
      setResolved((prev) => (prev.size === nextResolved.size && [...nextResolved].every((id) => prev.has(id)) ? prev : nextResolved));
    };

    resolve();
    const mutationObserver = new MutationObserver(resolve);
    mutationObserver.observe(observeRoot, { childList: true, subtree: true });
    return () => {
      mutationObserver.disconnect();
      stampedRoots.forEach((root) => {
        root.removeAttribute("data-introduction-elevated");
        if (positionedByUs.has(root)) (root as HTMLElement).style.position = "";
      });
      setResolved(new Set());
    };
  }, [ids, shellScope]);

  return resolved;
}

type IntroductionInfoBoxPosition = { readonly top: number; readonly left: number };

const INTRODUCTION_INFO_BOX_GAP_PX = 16;

/** @emoji 📝️ Splits an introduction step body into visual paragraphs on blank lines so each
 * paragraph can emphasize independently on hover. */
export function splitIntroductionBodyParagraphs(body: string): readonly string[] {
  return body
    .split(/\n\n+/)
    .map((paragraph) => paragraph.trim())
    .filter((paragraph) => paragraph.length > 0);
}

/** @emoji 🧲️ Clamps an introduction info box to the viewport. Authored placement uses a visual inset;
 * direct manipulation uses zero so every outer edge can meet the corresponding viewport border. */
export function clampIntroductionInfoBoxPosition(
  position: IntroductionInfoBoxPosition,
  boxSize: { readonly width: number; readonly height: number },
  viewport: { readonly width: number; readonly height: number },
  inset: number,
): IntroductionInfoBoxPosition {
  return {
    top: Math.min(Math.max(position.top, inset), Math.max(inset, viewport.height - boxSize.height - inset)),
    left: Math.min(Math.max(position.left, inset), Math.max(inset, viewport.width - boxSize.width - inset)),
  };
}

/** @emoji 🎓️ Where the info box sits relative to its anchor. `auto` picks the side with the most free
 * viewport space; `center` (and any anchor-less step) centers the box in the viewport. */
export function resolveIntroductionPlacement(
  placement: IntroductionPlacement,
  anchorRect: IntroductionRect | null,
  boxSize: { readonly width: number; readonly height: number },
  viewport: { readonly width: number; readonly height: number },
): IntroductionInfoBoxPosition {
  const centered = { top: (viewport.height - boxSize.height) / 2, left: (viewport.width - boxSize.width) / 2 };
  if (!anchorRect || placement === "center") return centered;

  const gap = INTRODUCTION_INFO_BOX_GAP_PX;
  const clamp = (position: IntroductionInfoBoxPosition) => clampIntroductionInfoBoxPosition(position, boxSize, viewport, gap);
  const space = {
    top: anchorRect.top,
    bottom: viewport.height - (anchorRect.top + anchorRect.height),
    left: anchorRect.left,
    right: viewport.width - (anchorRect.left + anchorRect.width),
  } as const;
  const side = placement === "auto" ? (Object.keys(space) as (keyof typeof space)[]).sort((a, b) => space[b] - space[a])[0] : placement;

  switch (side) {
    case "top":
      return clamp({ top: anchorRect.top - boxSize.height - gap, left: anchorRect.left + anchorRect.width / 2 - boxSize.width / 2 });
    case "bottom":
      return clamp({ top: anchorRect.top + anchorRect.height + gap, left: anchorRect.left + anchorRect.width / 2 - boxSize.width / 2 });
    case "left":
      return clamp({ top: anchorRect.top + anchorRect.height / 2 - boxSize.height / 2, left: anchorRect.left - boxSize.width - gap });
    case "right":
      return clamp({ top: anchorRect.top + anchorRect.height / 2 - boxSize.height / 2, left: anchorRect.left + anchorRect.width + gap });
    default:
      return centered;
  }
}

//#region 🎬️DemonstrationProjectors
/** @emoji 🏷️ What a surface resolver returns for `IntroductionPoint.Entity`/`Curve`/`Domain`: a viewport
 * pixel anchor plus whatever richer geometry the surface can offer — `rect` for offset-within-bounds and
 * domain-to-track mapping, `polyline` (viewport px) for arc-length curve targeting, `domain` for mapping
 * a value onto `rect`. `visible: false` means "found but not currently resolvable" (off-camera, hidden,
 * zero-length geometry) — callers treat that exactly like `null`, never as an error. */
export type IntroductionResolvedGeometry = {
  readonly point: { readonly x: number; readonly y: number };
  readonly rect?: { readonly x: number; readonly y: number; readonly width: number; readonly height: number };
  readonly polyline?: readonly { readonly x: number; readonly y: number }[];
  readonly domain?: { readonly min: number; readonly max: number; readonly axis: "x" | "y" };
  readonly visible: boolean;
};

/** @emoji 🧭️ A surface's live-camera/live-entity capabilities for demonstration targeting, registered per
 * window element id by the surface that owns the data (a react-three-fiber `useThree` bridge, a WASM
 * canvas session, a DOM-rendered host). Every method may be omitted — a surface implements only the point
 * kinds it can genuinely resolve. All three are called every animation frame; implementations must be
 * cheap (cache parsed JSON keyed by the source string's identity, never re-parse a whole fixture per
 * frame). */
export type IntroductionSurfaceResolver = {
  /** 🧊️ 3D world position → viewport pixel through the surface's live camera. */
  readonly scenePoint?: (position: readonly [number, number, number]) => { readonly x: number; readonly y: number; readonly visible: boolean } | null;
  /** 🗺️ 2D world (camera x/y/zoom) position → viewport pixel. */
  readonly canvasPoint?: (x: number, y: number) => { readonly x: number; readonly y: number; readonly visible: boolean } | null;
  /** 🏷️ A live entity by domain + id (`"*"` = any representative) → its resolved geometry. */
  readonly entity?: (domain: string, entity: string) => IntroductionResolvedGeometry | null;
};

const introductionSurfaceResolvers = ephemeralMap<string, IntroductionSurfaceResolver>("framework.modules.ui.packages.typescript.targets.react.index.tsx.introductionSurfaceResolvers");

/** @emoji 🧭️ Registers the demonstration-targeting resolver for the surface shown by window element
 * `windowId` — call from the window's own host component (has the live camera/session/DOM refs),
 * unregister on unmount via the returned disposer. Multiple concurrently open instances of the same
 * window kind (split panes) last-write-wins; acceptable for a single demonstration target. */
export function registerIntroductionSurfaceResolver(windowId: string, resolver: IntroductionSurfaceResolver): () => void {
  introductionSurfaceResolvers.set(windowId, resolver);
  return () => {
    if (introductionSurfaceResolvers.get(windowId) === resolver) introductionSurfaceResolvers.delete(windowId);
  };
}

/** @emoji 🧊️ Pure NDC (`[-1, 1]`, y-up) → viewport-pixel conversion shared by every 3D scene resolver. */
export function ndcToViewportPoint(ndc: { readonly x: number; readonly y: number }, rect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): { readonly x: number; readonly y: number } {
  return { x: rect.left + ((ndc.x + 1) / 2) * rect.width, y: rect.top + ((1 - ndc.y) / 2) * rect.height };
}
//#endregion 🎬️DemonstrationProjectors

//#region 🎬️DemonstrationResolve
type IntroductionResolvedPoint = { readonly x: number; readonly y: number };

/** @emoji 🪡️ A point at `t` (0–1, clamped) along `points` by arc length — linearly interpolates between
 * the two points straddling `t`'s cumulative distance. Degenerate inputs (0 or 1 points, zero-length
 * polyline) return the first point (or the origin) for every `t`, never throw. */
export function polylinePointAt(points: readonly { readonly x: number; readonly y: number }[], t: number): { readonly x: number; readonly y: number } {
  if (points.length === 0) return { x: 0, y: 0 };
  if (points.length === 1) return points[0];
  const segmentLengths: number[] = [];
  let totalLength = 0;
  for (let i = 1; i < points.length; i++) {
    const length = Math.hypot(points[i].x - points[i - 1].x, points[i].y - points[i - 1].y);
    segmentLengths.push(length);
    totalLength += length;
  }
  if (totalLength === 0) return points[0];
  const targetDistance = Math.min(1, Math.max(0, t)) * totalLength;
  let accumulated = 0;
  for (let i = 0; i < segmentLengths.length; i++) {
    const segmentLength = segmentLengths[i];
    if (accumulated + segmentLength >= targetDistance || i === segmentLengths.length - 1) {
      const segmentT = segmentLength === 0 ? 0 : (targetDistance - accumulated) / segmentLength;
      return {
        x: points[i].x + (points[i + 1].x - points[i].x) * segmentT,
        y: points[i].y + (points[i + 1].y - points[i].y) * segmentT,
      };
    }
    accumulated += segmentLength;
  }
  return points[points.length - 1];
}

/** @emoji 🪡️ Samples a canvas-layer path (`move`/`line`/`quad`/`cubic` segments, in whatever local
 * coordinate space the caller's points are already expressed in — apply layer transforms before calling)
 * into a flat polyline via de Casteljau evaluation — the one parametric-curve evaluator this codebase has,
 * needed because `IntroductionPoint.Curve` resolves through arc-length interpolation over a polyline, not
 * a true parametric curve. `arc`/`close` segments are not sampled (curve targeting degrades to the
 * straight gap between their neighbors); rare enough for a demonstration target to not warrant full SVG
 * arc-to-bezier conversion. */
export function sampleBezierSegments(
  segments: readonly {
    readonly kind?: string;
    readonly to?: readonly [number, number];
    readonly ctrl?: readonly [number, number];
    readonly ctrl1?: readonly [number, number];
    readonly ctrl2?: readonly [number, number];
  }[],
  samplesPerCurve = 16,
): readonly { readonly x: number; readonly y: number }[] {
  const points: { x: number; y: number }[] = [];
  let current = { x: 0, y: 0 };
  for (const segment of segments) {
    if ((segment.kind === "move" || segment.kind === "line") && segment.to) {
      current = { x: segment.to[0], y: segment.to[1] };
      points.push(current);
    } else if (segment.kind === "quad" && segment.ctrl && segment.to) {
      const ctrl = { x: segment.ctrl[0], y: segment.ctrl[1] };
      const to = { x: segment.to[0], y: segment.to[1] };
      for (let i = 1; i <= samplesPerCurve; i++) {
        const t = i / samplesPerCurve;
        const oneMinusT = 1 - t;
        points.push({
          x: oneMinusT * oneMinusT * current.x + 2 * oneMinusT * t * ctrl.x + t * t * to.x,
          y: oneMinusT * oneMinusT * current.y + 2 * oneMinusT * t * ctrl.y + t * t * to.y,
        });
      }
      current = to;
    } else if (segment.kind === "cubic" && segment.ctrl1 && segment.ctrl2 && segment.to) {
      const ctrl1 = { x: segment.ctrl1[0], y: segment.ctrl1[1] };
      const ctrl2 = { x: segment.ctrl2[0], y: segment.ctrl2[1] };
      const to = { x: segment.to[0], y: segment.to[1] };
      for (let i = 1; i <= samplesPerCurve; i++) {
        const t = i / samplesPerCurve;
        const oneMinusT = 1 - t;
        points.push({
          x: oneMinusT ** 3 * current.x + 3 * oneMinusT * oneMinusT * t * ctrl1.x + 3 * oneMinusT * t * t * ctrl2.x + t ** 3 * to.x,
          y: oneMinusT ** 3 * current.y + 3 * oneMinusT * oneMinusT * t * ctrl1.y + 3 * oneMinusT * t * t * ctrl2.y + t ** 3 * to.y,
        });
      }
      current = to;
    }
  }
  return points;
}

/** @emoji 📌️ Resolves an `IntroductionPoint` to a live viewport pixel. Called every animation frame by a
 * playing demonstration — re-resolving (rather than caching) keeps a drag path glued to a target that's
 * still moving (a resizing panel, an orbiting 3D camera, a panning 2D canvas). `null` means "not
 * resolvable yet" (element not mounted, no surface resolver registered, entity not found/hidden, or a 3D
 * point off-camera) — callers wait and retry next frame rather than treating it as an error. */
export function resolveIntroductionPoint(point: IntroductionPoint, root: ParentNode | null = document): IntroductionResolvedPoint | null {
  if (!root) return null;
  switch (point.kind) {
    case "element": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      const [offsetX, offsetY] = point.offset ?? [0.5, 0.5];
      return { x: rect.left + offsetX * rect.width, y: rect.top + offsetY * rect.height };
    }
    case "screen":
      return { x: point.x, y: point.y };
    case "screenNormalized":
      return { x: point.x * window.innerWidth, y: point.y * window.innerHeight };
    case "window": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      return { x: rect.left + point.x, y: rect.top + point.y };
    }
    case "windowNormalized": {
      const element = root.querySelector(elementIdSelector(point.id));
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      return { x: rect.left + point.x * rect.width, y: rect.top + point.y * rect.height };
    }
    case "scene": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const projected = resolver?.scenePoint?.(point.position);
      if (!projected || !projected.visible) return null;
      return { x: projected.x, y: projected.y };
    }
    case "canvas": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const projected = resolver?.canvasPoint?.(point.x, point.y);
      if (!projected || !projected.visible) return null;
      return { x: projected.x, y: projected.y };
    }
    case "entity": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible) return null;
      if (!geometry.rect) return geometry.point;
      const [offsetX, offsetY] = point.offset ?? [0.5, 0.5];
      return { x: geometry.rect.x + offsetX * geometry.rect.width, y: geometry.rect.y + offsetY * geometry.rect.height };
    }
    case "curve": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible || !geometry.polyline || geometry.polyline.length === 0) return null;
      return polylinePointAt(geometry.polyline, point.t);
    }
    case "domain": {
      const resolver = introductionSurfaceResolvers.get(point.id);
      const geometry = resolver?.entity?.(point.domain, point.entity);
      if (!geometry || !geometry.visible || !geometry.rect || !geometry.domain) return null;
      const { min, max, axis } = geometry.domain;
      const t = max === min ? 0 : (Math.min(max, Math.max(min, point.value)) - min) / (max - min);
      const rect = geometry.rect;
      // 🎚️ x-axis tracks read left→right with increasing value; y-axis tracks read bottom→top (screen y
      // decreases upward), matching how a vertical slider's "up" reads as "more".
      return axis === "x" ? { x: rect.x + t * rect.width, y: rect.y + rect.height / 2 } : { x: rect.x + rect.width / 2, y: rect.y + (1 - t) * rect.height };
    }
    default:
      return null;
  }
}
//#endregion 🎬️DemonstrationResolve

//#region 🎬️DemonstrationIdle
const INTRODUCTION_DEMO_IDLE_THRESHOLD_MS = 1600;

type IntroductionPointerPosition = { readonly x: number; readonly y: number };

type IntroductionPointerIdleState = {
  readonly idle: boolean;
  /** 🎬️ The real pointer's last observed viewport position (`null` until it has moved at least once) —
   * lets a demonstration open by traveling from where the user's actual cursor is resting, instead of
   * materializing out of nowhere near its target. Stable ref identity; read `.current`, don't watch it. */
  readonly lastPositionRef: React.RefObject<IntroductionPointerPosition | null>;
};

/** @emoji 💤️ Tracks whether the user's real pointer has been still for `thresholdMs` — the gate a
 * demonstration plays behind — and where it last was. Any `pointerdown`/`wheel`/`keydown`, or a
 * `pointermove` with different coordinates or non-zero movement deltas, resets the timer and flips back
 * to not-idle immediately. Same-coordinate, zero-delta moves are ignored because browsers re-fire them
 * when the DOM mutates under a stationary cursor. `onActivity` runs synchronously inside the native
 * event handler so the visual disappears before React commits the idle-state update. Starts not-idle:
 * a demonstration should only appear once the user has first settled, not the instant a step mounts
 * mid-motion. */
function useIntroductionPointerIdle(enabled: boolean, thresholdMs: number = INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, onActivity?: () => void): IntroductionPointerIdleState {
  const [idle, setIdle] = reactHostPort.useState(false);
  const lastPositionRef = reactHostPort.useRef<IntroductionPointerPosition | null>(null);
  const onActivityRef = reactHostPort.useRef(onActivity);
  onActivityRef.current = onActivity;

  reactHostPort.useEffect(() => {
    if (!enabled) {
      setIdle(false);
      return;
    }
    let timer: ReturnType<typeof setTimeout> | null = null;

    const settle = () => {
      timer = setTimeout(() => setIdle(true), thresholdMs);
    };
    const unsettle = () => {
      onActivityRef.current?.();
      setIdle(false);
      if (timer) clearTimeout(timer);
      settle();
    };
    const onPointerMove = (event: PointerEvent) => {
      const previous = lastPositionRef.current;
      const coordinatesChanged = !previous || event.clientX !== previous.x || event.clientY !== previous.y;
      const movementChanged = event.movementX !== 0 || event.movementY !== 0;
      if (!coordinatesChanged && !movementChanged) return;
      lastPositionRef.current = { x: event.clientX, y: event.clientY };
      unsettle();
    };
    const onOtherInput = () => unsettle();

    window.addEventListener("pointermove", onPointerMove, true);
    window.addEventListener("pointerdown", onOtherInput, true);
    window.addEventListener("wheel", onOtherInput, true);
    window.addEventListener("keydown", onOtherInput, true);
    settle();
    return () => {
      if (timer) clearTimeout(timer);
      window.removeEventListener("pointermove", onPointerMove, true);
      window.removeEventListener("pointerdown", onOtherInput, true);
      window.removeEventListener("wheel", onOtherInput, true);
      window.removeEventListener("keydown", onOtherInput, true);
    };
  }, [enabled, thresholdMs]);

  return { idle, lastPositionRef };
}
//#endregion 🎬️DemonstrationIdle

//#region 🎬️DemonstrationOverlay
const INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR: Record<IntroductionGesture["kind"], IntroductionCursor> = {
  leftClick: "pointer",
  rightClick: "pointer",
  doubleClick: "pointer",
  drag: "grab",
  scroll: "default",
  orbit: "move",
};

type IntroductionDemoPointerButton = "left" | "middle" | "right" | "wheel";

type IntroductionDemoFeedbackKind = "leftClick" | "rightClick" | "doubleClick" | "scroll" | "dragLeft" | "dragMiddle" | "dragRight" | "orbit";

type IntroductionDemoVisual = {
  readonly button: IntroductionDemoPointerButton;
  readonly modifiers: readonly IntroductionKeyModifier[];
  readonly feedback: IntroductionDemoFeedbackKind;
  readonly showDoubleChip: boolean;
};

/** @emoji 🖱️ Resolves the mini-mouse highlight, modifier chips, and press/trail family for a gesture. */
function introductionDemoResolveVisual(gesture: IntroductionGesture): IntroductionDemoVisual {
  switch (gesture.kind) {
    case "leftClick":
      return { button: "left", modifiers: [], feedback: "leftClick", showDoubleChip: false };
    case "rightClick":
      return { button: "right", modifiers: [], feedback: "rightClick", showDoubleChip: false };
    case "doubleClick":
      return { button: "left", modifiers: [], feedback: "doubleClick", showDoubleChip: true };
    case "scroll":
      return { button: "wheel", modifiers: [], feedback: "scroll", showDoubleChip: false };
    case "drag": {
      const button = gesture.button ?? "left";
      const feedback = button === "middle" ? "dragMiddle" : button === "right" ? "dragRight" : "dragLeft";
      return { button, modifiers: gesture.modifiers ?? [], feedback, showDoubleChip: false };
    }
    case "orbit":
      return {
        button: gesture.button ?? "right",
        modifiers: gesture.modifiers ?? ["alt"],
        feedback: "orbit",
        showDoubleChip: false,
      };
  }
}

function introductionDemoRippleClass(feedback: IntroductionDemoFeedbackKind, double = false): string {
  const tone = feedback === "rightClick" || feedback === "dragRight" || feedback === "orbit" ? "right" : feedback === "dragMiddle" ? "middle" : "left";
  const base = `introduction-demo-ripple introduction-demo-ripple--${tone}`;
  return double ? `${base} introduction-demo-ripple--double` : base;
}

/** @emoji 🌐️ A point along the quadratic-bezier arc from `from` to `to`, bulged perpendicular to the
 * straight line between them — an `orbit` gesture reads as a curved rotation around a pivot, visually
 * distinct from `drag`'s straight-line pan/reposition. */
function introductionDemoArcPoint(from: IntroductionResolvedPoint, to: IntroductionResolvedPoint, t: number): IntroductionResolvedPoint {
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const distance = Math.hypot(dx, dy) || 1;
  const bulge = Math.min(64, distance * 0.35);
  const controlX = (from.x + to.x) / 2 + (-dy / distance) * bulge;
  const controlY = (from.y + to.y) / 2 + (dx / distance) * bulge;
  const oneMinusT = 1 - t;
  return {
    x: oneMinusT * oneMinusT * from.x + 2 * oneMinusT * t * controlX + t * t * to.x,
    y: oneMinusT * oneMinusT * from.y + 2 * oneMinusT * t * controlY + t * t * to.y,
  };
}

const INTRODUCTION_DEMO_PHASE_MS = {
  appear: 250,
  travel: 600,
  press: 180,
  dragMove: 900,
  release: 180,
  linger: 500,
  fadeOut: 250,
  pause: 900,
} as const;

type IntroductionDemoPhase = keyof typeof INTRODUCTION_DEMO_PHASE_MS;

const INTRODUCTION_DEMO_NEXT_PHASE: Record<IntroductionDemoPhase, IntroductionDemoPhase> = {
  appear: "travel",
  travel: "press",
  press: "dragMove",
  dragMove: "release",
  release: "linger",
  linger: "fadeOut",
  fadeOut: "pause",
  pause: "appear",
};

function introductionDemoEaseInOutCubic(t: number): number {
  return t < 0.5 ? 4 * t * t * t : 1 - (-2 * t + 2) ** 3 / 2;
}

function introductionDemoLerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

const INTRODUCTION_DEMO_APPEAR_OFFSET = { x: -32, y: -32 } as const;

/** @emoji 🎬️ Ghost-cursor gesture demonstration(s) for an introduction step, mounted by `UIIntroduction`
 * whenever it has one or more effective demonstrations. Plays them in order, one full gesture-loop each,
 * then wraps back to the first — e.g. a viewport step showing zoom, then pan, then orbit, repeating.
 * Plays only while {@link useIntroductionPointerIdle} is true — any real pointer movement hides it and
 * restores the real cursor instantly (`data-introduction-demonstrating` cleared, see `🎨️ui.css`); going
 * idle again restarts the sequence from its first demonstration's `appear` phase. Never dispatches real
 * pointer events — purely visual, driven by an imperative rAF loop (no per-frame React state) that
 * re-resolves its `IntroductionPoint` endpoints every frame via {@link resolveIntroductionPoint} so it
 * stays glued to a moving/orbiting target. Renders nothing under `prefers-reduced-motion: reduce` — the
 * info box's "perform to continue" text remains the fallback. */
const IntroductionDemonstrationOverlay: React.FC<{ readonly demonstrations: readonly IntroductionDemonstration[] }> = ({ demonstrations }) => {
  const overlayRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const ghostRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const calloutRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const trailPathRef = reactHostPort.useRef<SVGPathElement | null>(null);
  const rippleHostRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  // 🐚️ Falls back to `document.documentElement` outside any shell (e.g. the mit-bestand demonstrator's
  // own standalone onboarding tour) — inside one, scopes the cursor-hide to that shell's own root so one
  // shell's demonstration never mutes the cursor over another shell (`.semio-scope[...]` in `🎨️ui.css`).
  const shellScope = useShellScopeOptional();
  const demoHost = shellScope?.portalLayerRef.current ?? null;
  const demoPositionClass = demoHost ? "absolute" : "fixed";
  const demonstratingRoot = reactHostPort.useCallback((): Element | null => (shellScope ? shellScope.rootRef.current : document.documentElement), [shellScope]);
  const hide = reactHostPort.useCallback(() => {
    demonstratingRoot()?.removeAttribute("data-introduction-demonstrating");
    if (overlayRef.current) overlayRef.current.style.visibility = "hidden";
  }, [demonstratingRoot]);
  const { idle, lastPositionRef } = useIntroductionPointerIdle(true, INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, hide);
  const mouseClipId = reactHostPort.useId().replace(/:/g, "");
  const reducedMotion = reactHostPort.useMemo(() => typeof window !== "undefined" && window.matchMedia?.("(prefers-reduced-motion: reduce)")?.matches === true, []);

  reactHostPort.useEffect(() => {
    if (reducedMotion || !idle || demonstrations.length === 0) {
      hide();
      return;
    }
    if (overlayRef.current) overlayRef.current.style.visibility = "visible";
    demonstratingRoot()?.setAttribute("data-introduction-demonstrating", "true");
    const origin = lastPositionRef.current;

    const setGlyph = (glyph: IntroductionCursor) => {
      const ghost = ghostRef.current;
      if (ghost) ghost.style.backgroundImage = `var(--cursor-ghost-${glyph})`;
    };
    const toOverlayPoint = (x: number, y: number) => {
      if (!demoHost) return { x, y };
      const hostRect = demoHost.getBoundingClientRect();
      return introductionPointRelativeToHost({ x, y }, hostRect);
    };
    const setPosition = (x: number, y: number) => {
      const ghost = ghostRef.current;
      if (!ghost) return;
      const local = toOverlayPoint(x, y);
      ghost.style.transform = `translate3d(${local.x}px, ${local.y}px, 0)`;
    };
    const setOpacity = (opacity: number) => {
      const ghost = ghostRef.current;
      if (ghost) ghost.style.opacity = String(opacity);
      const callout = calloutRef.current;
      if (callout) callout.style.opacity = String(opacity);
    };
    const setCallout = (x: number, y: number, visual: IntroductionDemoVisual, pressed: boolean, scrollDirection?: number) => {
      const callout = calloutRef.current;
      if (!callout) return;
      const local = toOverlayPoint(x, y);
      callout.style.transform = `translate3d(${local.x + 28}px, ${local.y + 8}px, 0)`;
      callout.dataset.button = visual.button;
      callout.dataset.feedback = visual.feedback;
      if (visual.modifiers.length > 0) callout.setAttribute("data-modifiers", visual.modifiers.join(" "));
      else callout.removeAttribute("data-modifiers");
      if (visual.showDoubleChip) callout.setAttribute("data-double", "true");
      else callout.removeAttribute("data-double");
      if (pressed) callout.setAttribute("data-pressed", "true");
      else callout.removeAttribute("data-pressed");
      if (scrollDirection !== undefined) callout.dataset.scrollDirection = scrollDirection >= 0 ? "down" : "up";
      else callout.removeAttribute("data-scroll-direction");
    };
    const clearTrail = () => {
      const path = trailPathRef.current;
      if (!path) return;
      path.removeAttribute("d");
      path.removeAttribute("class");
    };
    const updateTrail = (from: IntroductionResolvedPoint, to: IntroductionResolvedPoint, orbit: boolean, feedback: IntroductionDemoFeedbackKind) => {
      const path = trailPathRef.current;
      if (!path) return;
      const localFrom = toOverlayPoint(from.x, from.y);
      const localTo = toOverlayPoint(to.x, to.y);
      if (orbit) {
        const dx = localTo.x - localFrom.x;
        const dy = localTo.y - localFrom.y;
        const distance = Math.hypot(dx, dy) || 1;
        const bulge = Math.min(64, distance * 0.35);
        const controlX = (localFrom.x + localTo.x) / 2 + (-dy / distance) * bulge;
        const controlY = (localFrom.y + localTo.y) / 2 + (dx / distance) * bulge;
        path.setAttribute("d", `M ${localFrom.x} ${localFrom.y} Q ${controlX} ${controlY} ${localTo.x} ${localTo.y}`);
      } else {
        path.setAttribute("d", `M ${localFrom.x} ${localFrom.y} L ${localTo.x} ${localTo.y}`);
      }
      path.setAttribute("class", `introduction-demo-trail-path introduction-demo-trail-path--${feedback}`);
      const length = path.getTotalLength();
      path.style.setProperty("--introduction-demo-trail-length", String(length));
    };
    const spawnRipple = (x: number, y: number, feedback: IntroductionDemoFeedbackKind, double = false) => {
      const host = rippleHostRef.current;
      if (!host || feedback === "scroll") return;
      const local = toOverlayPoint(x, y);
      const ripple = document.createElement("div");
      ripple.className = `${introductionDemoRippleClass(feedback, double)} pointer-events-none ${demoPositionClass} rounded-full`;
      ripple.style.left = `${local.x - 12}px`;
      ripple.style.top = `${local.y - 12}px`;
      ripple.style.width = "24px";
      ripple.style.height = "24px";
      host.appendChild(ripple);
      ripple.addEventListener("animationend", () => ripple.remove());
    };
    const spawnZoomRing = (x: number, y: number, zoomIn: boolean) => {
      const host = rippleHostRef.current;
      if (!host) return;
      const local = toOverlayPoint(x, y);
      const ring = document.createElement("div");
      ring.className = `introduction-demo-zoom-ring introduction-demo-zoom-ring--${zoomIn ? "in" : "out"} pointer-events-none ${demoPositionClass}`;
      ring.style.left = `${local.x}px`;
      ring.style.top = `${local.y}px`;
      host.appendChild(ring);
      ring.addEventListener("animationend", () => ring.remove());
    };

    let cancelled = false;
    let rafId = 0;
    let demoIndex = 0;
    let phase: IntroductionDemoPhase = "appear";
    let phaseStart = performance.now();
    let rippleSpawned = false;
    let secondRippleSpawned = false;
    let thirdZoomSpawned = false;

    const tick = (now: number) => {
      if (cancelled) return;
      const demonstration = demonstrations[demoIndex];
      const gesture = demonstration.gesture;
      const visual = introductionDemoResolveVisual(gesture);
      const isDragLike = gesture.kind === "drag" || gesture.kind === "orbit";
      const isScroll = gesture.kind === "scroll";
      const hoverCursor = demonstration.cursor ?? INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR[gesture.kind];
      const dragCursor = demonstration.cursor ?? (gesture.kind === "orbit" ? "move" : "grabbing");
      const targets: { readonly start: IntroductionPoint; readonly end: IntroductionPoint | null } = isDragLike ? { start: gesture.from, end: gesture.to } : { start: gesture.at, end: null };

      const start = resolveIntroductionPoint(targets.start, demonstratingRoot());
      const end = targets.end ? resolveIntroductionPoint(targets.end, demonstratingRoot()) : null;
      if (!start || (targets.end && !end)) {
        rafId = requestAnimationFrame(tick);
        return;
      }

      // 🎬️ Scroll/zoom needs a longer dwell than a click press — rings and wheel-roll read as zoom only
      // when they have time to expand/contract instead of flashing like a middle-button click.
      const duration = isScroll && phase === "press" ? 900 : INTRODUCTION_DEMO_PHASE_MS[phase];
      const t = Math.min(1, (now - phaseStart) / duration);
      const eased = introductionDemoEaseInOutCubic(t);
      const anchor = origin ?? { x: start.x + INTRODUCTION_DEMO_APPEAR_OFFSET.x, y: start.y + INTRODUCTION_DEMO_APPEAR_OFFSET.y };
      const pressed = !isScroll && (phase === "press" || phase === "dragMove");

      switch (phase) {
        case "appear":
          setGlyph("default");
          setOpacity(t);
          setPosition(anchor.x, anchor.y);
          setCallout(anchor.x, anchor.y, visual, false);
          clearTrail();
          break;
        case "travel":
          setOpacity(1);
          setPosition(introductionDemoLerp(anchor.x, start.x, eased), introductionDemoLerp(anchor.y, start.y, eased));
          setCallout(introductionDemoLerp(anchor.x, start.x, eased), introductionDemoLerp(anchor.y, start.y, eased), visual, false);
          if (t > 0.7) setGlyph(hoverCursor);
          clearTrail();
          break;
        case "press":
          setGlyph(hoverCursor);
          if (isScroll) {
            const zoomIn = gesture.deltaY < 0;
            const direction = zoomIn ? -1 : 1;
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, false, direction);
            if (!rippleSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              rippleSpawned = true;
            }
            if (t > 0.33 && !secondRippleSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              secondRippleSpawned = true;
            }
            if (t > 0.66 && !thirdZoomSpawned) {
              spawnZoomRing(start.x, start.y, zoomIn);
              thirdZoomSpawned = true;
            }
          } else {
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, true);
            if (!rippleSpawned) {
              spawnRipple(start.x, start.y, visual.feedback);
              rippleSpawned = true;
            }
            if (gesture.kind === "doubleClick" && t > 0.5 && !secondRippleSpawned) {
              spawnRipple(start.x, start.y, visual.feedback, true);
              secondRippleSpawned = true;
            }
          }
          clearTrail();
          break;
        case "dragMove": {
          setGlyph(dragCursor);
          if (end) {
            const point = gesture.kind === "orbit" ? introductionDemoArcPoint(start, end, eased) : { x: introductionDemoLerp(start.x, end.x, eased), y: introductionDemoLerp(start.y, end.y, eased) };
            setPosition(point.x, point.y);
            setCallout(point.x, point.y, visual, true);
            updateTrail(start, point, gesture.kind === "orbit", visual.feedback);
          } else {
            setPosition(start.x, start.y);
            setCallout(start.x, start.y, visual, true);
            clearTrail();
          }
          break;
        }
        case "release": {
          const point = end ?? start;
          setGlyph(hoverCursor);
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, false);
          clearTrail();
          break;
        }
        case "linger": {
          const point = end ?? start;
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, false);
          clearTrail();
          break;
        }
        case "fadeOut": {
          const point = end ?? start;
          setOpacity(1 - t);
          setPosition(point.x, point.y);
          setCallout(point.x, point.y, visual, pressed);
          clearTrail();
          break;
        }
        case "pause":
          setOpacity(0);
          clearTrail();
          break;
      }

      if (t >= 1) {
        phaseStart = now;
        rippleSpawned = false;
        secondRippleSpawned = false;
        thirdZoomSpawned = false;
        if (phase === "pause") demoIndex = (demoIndex + 1) % demonstrations.length;
        phase = phase === "press" && !isDragLike ? "release" : INTRODUCTION_DEMO_NEXT_PHASE[phase];
      }
      rafId = requestAnimationFrame(tick);
    };

    rafId = requestAnimationFrame(tick);
    return () => {
      cancelled = true;
      cancelAnimationFrame(rafId);
      hide();
    };
  }, [demonstratingRoot, demonstrations, hide, idle, reducedMotion, demoHost, demoPositionClass]);

  if (reducedMotion) return null;

  return (
    <div ref={overlayRef} data-slot="introduction-demonstration" className={cn("pointer-events-none inset-0", demoPositionClass)} style={{ visibility: "hidden", zIndex: "calc(var(--z-tutorial) + 2)" }}>
      <svg className={cn("pointer-events-none inset-0 h-full w-full overflow-visible", demoPositionClass)} aria-hidden="true">
        <path ref={trailPathRef} />
      </svg>
      <div
        ref={ghostRef}
        data-slot="introduction-demonstration-cursor"
        className={cn("pointer-events-none h-6 w-6 bg-contain bg-no-repeat opacity-0", demoPositionClass)}
        style={{ backgroundImage: "var(--cursor-ghost-default)", willChange: "transform, opacity" }}
      />
      <div ref={calloutRef} data-slot="introduction-demonstration-callout" className={cn("introduction-demo-callout pointer-events-none opacity-0 text-foreground", demoPositionClass)} data-button="left" data-feedback="leftClick">
        <svg className="introduction-demo-mouse" viewBox="0 0 48 72" aria-hidden="true">
          <defs>
            <clipPath id={mouseClipId}>
              <path d="M24 4C12.954 4 4 12.954 4 24v24c0 11.046 8.954 20 20 20s20-8.954 20-20V24C44 12.954 35.046 4 24 4Z" />
            </clipPath>
          </defs>
          <g clipPath={`url(#${mouseClipId})`}>
            <path className="introduction-demo-mouse-button" data-part="left" d="M24 4C12.954 4 4 12.954 4 24V28H24Z" />
            <path className="introduction-demo-mouse-button" data-part="right" d="M24 4C35.046 4 44 12.954 44 24V28H24Z" />
            <path className="introduction-demo-mouse-button" data-part="middle" d="M21 12H27V22H21Z" />
          </g>
          <path className="introduction-demo-mouse-body" d="M24 4C12.954 4 4 12.954 4 24v24c0 11.046 8.954 20 20 20s20-8.954 20-20V24C44 12.954 35.046 4 24 4Z" />
          <path className="introduction-demo-mouse-divider" d="M24 4V28" />
          <path className="introduction-demo-mouse-seam" d="M8 28H40" />
          <path className="introduction-demo-mouse-wheel" d="M24 14v10" />
        </svg>
        <div className="introduction-demo-callout-chips">
          <kbd className="introduction-demo-callout-chip" data-chip="zoom-in">
            +
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="zoom-out">
            −
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="double">
            2×
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="alt">
            Alt
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="shift">
            Shift
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="control">
            Ctrl
          </kbd>
          <kbd className="introduction-demo-callout-chip" data-chip="meta">
            ⌘️
          </kbd>
        </div>
      </div>
      <div ref={rippleHostRef} className={cn("pointer-events-none inset-0", demoPositionClass)} />
    </div>
  );
};

/** @emoji 🎬️ The default demonstration for a purely informational step (`interactions.length === 0`, the
 * Next/Done button is the only way to continue) — clicking `ui.introduction.next`, the stable id every
 * such button renders under regardless of its Next/Done label. Used by `UIIntroduction` whenever the step
 * doesn't declare its own `demonstration`, so authors never have to spell this out per step. */
const INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION: IntroductionDemonstration = { gesture: { kind: "leftClick", at: { kind: "element", id: "ui.introduction.next" } } };
//#endregion 🎬️DemonstrationOverlay

export type UIIntroductionProps = {
  readonly introduction: IntroductionDefinition;
  readonly stepIndex: number;
  /** ✅️ Indices into the active step's `interactions` that are done — drives the checklist rows. */
  readonly completedInteractionIndices?: readonly number[];
  readonly onStepIndexChange: (index: number) => void;
  readonly onDismiss: (completed: boolean) => void;
};

/** @emoji 🗺️ Resolves manifest-localized copy at the UI boundary while retaining support for already-resolved TS-native strings. */
export function resolveUiLocalizedText(value: unknown, terminology: string, locale: UiLocale): string {
  if (typeof value === "string") return value;
  if (value === null || typeof value !== "object") return "";
  const matrix = value as Readonly<Record<string, unknown>>;
  const terminologyCells = matrix[terminology] ?? matrix[UI_TERMINOLOGY_NATIVE] ?? matrix.reuse;
  if (terminologyCells === null || typeof terminologyCells !== "object") return "";
  const localeCells = terminologyCells as Readonly<Record<string, unknown>>;
  const resolved = localeCells[locale] ?? localeCells.en ?? Object.values(localeCells).find((cell) => typeof cell === "string");
  return typeof resolved === "string" ? resolved : "";
}

/** @emoji 📐️ One row of an introduction step's {@link IntroductionLogo}s, all sharing one computed height
 * so the row fills its full width edge-to-edge with no logo dominating over another — the height isn't
 * guessed: it's solved from the row's measured width and each logo's own natural aspect ratio (`width /
 * sum-of-aspect-ratios`, accounting for the `gap-double` between logos), so it's exactly right for
 * whatever logos a step declares, never distorting any of them. */
function IntroductionLogoRow({ logos }: { readonly logos: readonly IntroductionLogo[] }) {
  const rowRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [rowWidth, setRowWidth] = reactHostPort.useState(0);
  const [gapPx, setGapPx] = reactHostPort.useState(0);
  const [aspectRatios, setAspectRatios] = reactHostPort.useState<readonly (number | null)[]>(() => logos.map(() => null));

  reactHostPort.useEffect(() => {
    const el = rowRef.current;
    if (!el) return;
    const measure = () => {
      setRowWidth(el.getBoundingClientRect().width);
      setGapPx(parseFloat(getComputedStyle(el).columnGap) || 0);
    };
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  const onLogoLoad = reactHostPort.useCallback((index: number, event: React.SyntheticEvent<HTMLImageElement>) => {
    const img = event.currentTarget;
    if (!img.naturalWidth || !img.naturalHeight) return;
    setAspectRatios((prev) => (prev[index] != null ? prev : prev.map((ratio, i) => (i === index ? img.naturalWidth / img.naturalHeight : ratio))));
  }, []);

  const aspectRatioSum = aspectRatios.reduce<number | null>((sum, ratio) => (sum == null || ratio == null ? null : sum + ratio), 0);
  const height = rowWidth > 0 && aspectRatioSum != null && aspectRatioSum > 0 ? (rowWidth - gapPx * (logos.length - 1)) / aspectRatioSum : undefined;

  return (
    <div ref={rowRef} className="flex w-full items-center gap-double" style={{ visibility: height == null ? "hidden" : "visible" }}>
      {logos.map((logo, index) => {
        const style = height != null ? { height } : { height: 0, width: 0 };
        return (
          <span key={index} className="inline-flex items-center">
            {logo.href ? (
              <a href={logo.href} target="_blank" rel="noopener noreferrer" className="inline-flex items-center transition-opacity hover:opacity-80">
                <img src={logo.src} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className={cn("w-auto object-contain", logo.darkSrc && "dark:hidden")} style={style} />
                {logo.darkSrc && <img src={logo.darkSrc} alt={logo.alt} className="hidden w-auto object-contain dark:block" style={style} />}
              </a>
            ) : (
              <>
                <img src={logo.src} alt={logo.alt} onLoad={(event) => onLogoLoad(index, event)} className={cn("w-auto object-contain", logo.darkSrc && "dark:hidden")} style={style} />
                {logo.darkSrc && <img src={logo.darkSrc} alt={logo.alt} className="hidden w-auto object-contain dark:block" style={style} />}
              </>
            )}
          </span>
        );
      })}
    </div>
  );
}

/** @emoji 🎓️ Full-screen first-run walkthrough: a single fullscreen glass veil covers the screen, the
 * current step's `introduce`/`show` elements elevate above it (see `useIntroductionElevation`) and stay
 * crisp and interactive — `introduce` additionally pulses the introduced border on the precise element —
 * and an info box explains it (header {@link DragHandle} between title and step count repositions the box
 * for the current step; the box silhouette starts with the introduced highlight + thickness pulse, then
 * follows the shared surface-active lifecycle — click activates (primary stroke), click outside returns
 * to normal and never re-enters the introduced pulse for this step; each blank-line body paragraph
 * emphasizes only while the pointer is on that paragraph — see `[data-slot="introduction-body-paragraph"]:hover`
 * in `🎨️ui.css`).
 * Renders the declarative `IntroductionDefinition`/`IntroductionStepDefinition` contract. Every step
 * plays a ghost-cursor `IntroductionDemonstrationOverlay`: the step's own declared `demonstration` if
 * it has one, otherwise — for a purely informational step whose only way forward is the Next/Done
 * button — an automatic "click Next" demonstration (see `INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION`),
 * so authors never have to spell that one out per step. */
export const UIIntroduction: React.FC<UIIntroductionProps> = ({ introduction, stepIndex, completedInteractionIndices, onStepIndexChange, onDismiss }) => {
  const introductionShellScope = useShellScopeOptional();
  const { terminology } = useUiTerminology();
  const locale = detectShellLocale(introductionShellScope?.i18n.resolvedLanguage ?? uiI18n.resolvedLanguage);
  const step: IntroductionStepDefinition | undefined = introduction.steps[stepIndex];
  const introduceSelector = step?.introduce ? elementIdSelector(step.introduce) : null;
  const introduceRect = useIntroductionAnchorRect(introduceSelector);
  const elevationIds = reactHostPort.useMemo((): readonly string[] => [...(step?.introduce ? [step.introduce] : []), ...(step?.show ?? [])], [step]);
  const elevated = useIntroductionElevation(elevationIds);
  // 🎬️ A step that declares no `demonstrations` still gets one automatically when Next/Done is its only
  // way forward — see `INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION`. Memoized on `step` (not recomputed
  // fresh every render) so `IntroductionDemonstrationOverlay`'s effect — keyed on this array — doesn't
  // restart the demonstration loop on every unrelated re-render (viewport resize, box measure, …).
  const effectiveDemonstrations = reactHostPort.useMemo((): readonly IntroductionDemonstration[] => {
    if (!step) return [];
    // 🎬️ Hand-written brands may omit `demonstrations` (Rust `#[serde(default)]`); treat missing as empty.
    const demonstrations = step.demonstrations ?? [];
    if (demonstrations.length === 0) return (step.interactions ?? []).length === 0 ? [INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION] : [];
    const interactions = step.interactions ?? [];
    if (interactions.length === 0) return demonstrations;
    const completed = completedInteractionIndices ?? [];
    return demonstrations.filter((_, index) => index >= interactions.length || !completed.includes(index));
  }, [step, completedInteractionIndices]);
  const [viewport, setViewport] = reactHostPort.useState(() => ({ width: window.innerWidth, height: window.innerHeight }));
  const [boxSize, setBoxSize] = reactHostPort.useState({ width: 320, height: 160 });
  const boxRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(boxRef);
  // 🎯️ Once the step chrome is activated the introduced pulse is gone for this step — active while
  // selected, then normal after a background click (same lifecycle as panels/panes/windows).
  const [introductionActivated, setIntroductionActivated] = reactHostPort.useState(false);
  // 🫳️ User-dragged absolute position — `null` keeps auto/`placement` until the top-center handle moves the box;
  // reset on every step so each card starts at its authored placement again.
  const [dragPosition, setDragPosition] = reactHostPort.useState<IntroductionInfoBoxPosition | null>(null);
  const [dragging, setDragging] = reactHostPort.useState(false);
  const dragPositionRef = reactHostPort.useRef(dragPosition);
  dragPositionRef.current = dragPosition;
  const dragLayoutRef = reactHostPort.useRef({ placement: { top: 0, left: 0 }, boxSize, viewport });
  const dragSessionRef = reactHostPort.useRef<{ startTop: number; startLeft: number; pointerX: number; pointerY: number } | null>(null);

  reactHostPort.useEffect(() => {
    if (surfaceActive) setIntroductionActivated(true);
  }, [surfaceActive]);

  const shellScope = useShellScopeOptional();
  const shellActive = useIsActiveShellRoot(shellScope?.rootRef ?? NULL_SHELL_ROOT_REF);
  const overlayHost = shellScope?.portalLayerRef.current ?? null;
  const overlayPositioned = Boolean(overlayHost);

  const measureOverlayViewport = reactHostPort.useCallback(() => {
    if (overlayHost) {
      setViewport({ width: overlayHost.clientWidth, height: overlayHost.clientHeight });
      return;
    }
    setViewport({ width: window.innerWidth, height: window.innerHeight });
  }, [overlayHost]);

  reactHostPort.useEffect(() => {
    // Stamp the shell root (or documentElement for the landing-page tour outside any ShellScope) so
    // elevated-chrome CSS stays scoped to the introducing shell instead of every co-hosted shell.
    const root = shellScope?.rootRef.current ?? document.documentElement;
    root.setAttribute("data-introduction-active", "true");
    return () => root.removeAttribute("data-introduction-active");
  }, [shellScope]);

  reactHostPort.useEffect(() => {
    measureOverlayViewport();
    const onResize = () => measureOverlayViewport();
    window.addEventListener("resize", onResize);
    let observer: ResizeObserver | null = null;
    if (overlayHost && typeof ResizeObserver !== "undefined") {
      observer = new ResizeObserver(measureOverlayViewport);
      observer.observe(overlayHost);
    }
    return () => {
      window.removeEventListener("resize", onResize);
      observer?.disconnect();
    };
  }, [measureOverlayViewport, overlayHost]);

  reactHostPort.useEffect(() => {
    const el = boxRef.current;
    if (!el) return;
    const measure = () => setBoxSize({ width: el.offsetWidth, height: el.offsetHeight });
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(el);
    return () => observer.disconnect();
  }, [stepIndex]);

  reactHostPort.useEffect(() => {
    setDragPosition(null);
    setDragging(false);
    dragSessionRef.current = null;
    setIntroductionActivated(false);
  }, [stepIndex]);

  const skipLabel = useLabel("introduction.skip");
  const backLabel = useLabel("introduction.back");
  const nextLabel = useLabel("introduction.next");
  const doneLabel = useLabel("introduction.done");

  const isLast = stepIndex >= introduction.steps.length - 1;
  const skip = reactHostPort.useCallback(() => onDismiss(false), [onDismiss]);
  const next = reactHostPort.useCallback(() => {
    if (isLast) onDismiss(true);
    else onStepIndexChange(stepIndex + 1);
  }, [isLast, onDismiss, onStepIndexChange, stepIndex]);
  const back = reactHostPort.useCallback(() => onStepIndexChange(Math.max(0, stepIndex - 1)), [onStepIndexChange, stepIndex]);
  const interactions = step?.interactions ?? [];
  const advanceByButton = interactions.length === 0;

  // 🎉️ Celebrates a checklist row's own label text (conic-gradient ring, see `[data-celebrated="true"]`
  // in `🎨️ui.css`) the instant its interaction flips from pending to done — on top of whatever app element
  // `interaction.celebrate`/`step.introduce` celebrates, so the completed line item itself visibly glows.
  const interactionLabelRefs = reactHostPort.useRef<Map<number, HTMLSpanElement>>(new Map());
  const prevCompletedRef = reactHostPort.useRef<{ stepIndex: number; indices: readonly number[] }>({ stepIndex, indices: [] });
  reactHostPort.useEffect(() => {
    const prev = prevCompletedRef.current;
    const current = completedInteractionIndices ?? [];
    if (prev.stepIndex === stepIndex) {
      for (const index of current) {
        if (!prev.indices.includes(index)) {
          const label = interactionLabelRefs.current.get(index);
          if (label) celebrateElement(label);
        }
      }
    }
    prevCompletedRef.current = { stepIndex, indices: current };
  }, [stepIndex, completedInteractionIndices]);

  const dragPointerProps = usePointerDrag<HTMLSpanElement>({
    onStart: (event) => {
      event.preventDefault();
      event.stopPropagation();
      const start = dragPositionRef.current ?? dragLayoutRef.current.placement;
      dragSessionRef.current = { startTop: start.top, startLeft: start.left, pointerX: event.clientX, pointerY: event.clientY };
      setDragging(true);
    },
    onMove: (event) => {
      const session = dragSessionRef.current;
      if (!session) return;
      const { boxSize: size, viewport: vp } = dragLayoutRef.current;
      setDragPosition(
        clampIntroductionInfoBoxPosition(
          {
            top: session.startTop + (event.clientY - session.pointerY),
            left: session.startLeft + (event.clientX - session.pointerX),
          },
          size,
          vp,
          0,
        ),
      );
    },
    onEnd: () => {
      dragSessionRef.current = null;
      setDragging(false);
    },
    onCancel: () => {
      dragSessionRef.current = null;
      setDragging(false);
    },
  });

  useControlKeybinding("ui.introduction.skip", skip, { enableOnFormTags: true, enabled: shellActive }, [skip, shellActive]);
  useControlKeybinding("ui.introduction.next", () => advanceByButton && next(), { enableOnFormTags: true, enabled: shellActive && advanceByButton }, [advanceByButton, next, shellActive]);
  useControlKeybinding("ui.introduction.back", back, { enableOnFormTags: true, enabled: shellActive }, [back, shellActive]);

  if (!step) return null;

  const stepTitle = resolveUiLocalizedText(step.title, terminology, locale);
  const bodyParagraphs = splitIntroductionBodyParagraphs(resolveUiLocalizedText(step.body, terminology, locale));
  // When the overlay is absolutely positioned inside the shell portal layer (multi-shell hosts that
  // CSS-transform their shells, e.g. the demonstrator grid), placement must use host-local coords —
  // getBoundingClientRect is viewport-space and would otherwise land the box on the wrong pane.
  const hostRect = overlayHost?.getBoundingClientRect() ?? null;
  const placementAnchor = introduceRect && hostRect ? introductionRectRelativeToHost(introduceRect, hostRect) : introduceRect;
  const boxPosition = resolveIntroductionPlacement(step.placement, placementAnchor, boxSize, viewport);
  dragLayoutRef.current = { placement: boxPosition, boxSize, viewport };
  const position = dragPosition ?? boxPosition;
  const overlayPositionClass = overlayPositioned ? "absolute" : "fixed";
  // 🎓️ A targeted element that hasn't mounted yet (a folded utility bar/panel the shell is still revealing)
  // must not trap the user behind an opaque-to-clicks veil — only screen-style steps (`introduce == null`)
  // and steps whose target did resolve block pointer events; an unresolved `introduce` lets clicks through
  // so the user can reveal it themselves. A resolved `show` target also counts (e.g. a drop window while
  // the drag source mounts).
  const veilBlocksPointer = step.introduce == null || elevated.has(step.introduce) || (step.show ?? []).some((id) => elevated.has(id));

  const overlay = (
    <>
      <div data-level="dialog" className={cn(veilClass, "z-tutorial inset-0", overlayPositionClass, veilBlocksPointer ? "pointer-events-auto" : "pointer-events-none")} />
      <WindowChrome
        ref={boxRef}
        stackSlot="introduction-info-box"
        stackDataAttrs={dragging ? { "data-dragging": "true" } : undefined}
        stackBindProps={surfaceActiveProps}
        active={surfaceActive}
        borderKind={introductionActivated ? undefined : "introduced"}
        level="dialog"
        className={cn("pointer-events-auto z-tutorial w-fit max-w-[calc(100vw-2rem)] bg-transparent", overlayPositionClass)}
        style={{ top: position.top, left: position.left, zIndex: "calc(var(--z-tutorial) + 2)" }}
        titleChips={
          <div data-hover-scope data-slot="introduction-info-box-chip" className={cn(windowChromeTitleChipClass, "!h-auto max-w-none shrink overflow-visible whitespace-normal")}>
            <span data-slot="introduction-info-box-title" className="min-w-0 flex-1 break-words whitespace-normal text-sm leading-normal font-medium">
              {stepTitle}
            </span>
            <div data-slot="introduction-info-box-drag" className="flex shrink-0 items-center justify-center">
              <DragHandle {...dragPointerProps} emphasized={dragging} />
            </div>
          </div>
        }
        close={{
          id: "ui.introduction.skip",
          slot: "introduction-close",
          icon: <CloseIcon className="size-small" />,
          label: skipLabel,
          onClick: skip,
        }}
        body={
          <div data-slot="introduction-info-box-content" className="max-w-sm">
            {bodyParagraphs.length > 0 && (
              <div data-slot="introduction-body" className="mb-double flex flex-col gap-double">
                {bodyParagraphs.map((paragraph, index) => (
                  <p key={index} data-slot="introduction-body-paragraph" className="whitespace-pre-line text-xs text-muted-foreground">
                    {paragraph}
                  </p>
                ))}
              </div>
            )}
            {step.logos && step.logos.length > 0 && (
              <div className="mb-double flex flex-col gap-double">
                {[step.logos.slice(0, -1), step.logos.slice(-1)]
                  .filter((row) => row.length > 0)
                  .map((row, rowIndex) => (
                    <IntroductionLogoRow key={rowIndex} logos={row} />
                  ))}
              </div>
            )}
            {interactions.length > 0 && (
              <ul data-slot="introduction-interactions" className="mb-double flex flex-col gap-half text-xs">
                {interactions.map((interaction, index) => {
                  const done = completedInteractionIndices?.includes(index) ?? false;
                  return (
                    <li key={index} data-completed={done || undefined} className={cn("flex items-center gap-single", done ? "text-foreground" : "text-muted-foreground")}>
                      <Icon icon={done ? "check" : "circle"} size="small" />
                      {step.ordered && <span data-slot="introduction-interaction-index">{index + 1}.</span>}
                      <span
                        ref={(el) => {
                          if (el) interactionLabelRefs.current.set(index, el);
                          else interactionLabelRefs.current.delete(index);
                        }}
                        data-slot="introduction-interaction-label"
                        className="inline-flex items-center"
                      >
                        {interaction.label}
                      </span>
                    </li>
                  );
                })}
              </ul>
            )}
          </div>
        }
        footerCenterChips={
          <div data-slot="introduction-step-chip" className={windowChromeTitleChipClass}>
            <span className="px-single text-xs text-muted-foreground transition-colors group-hover:text-emphasized">
              {stepIndex + 1} / {introduction.steps.length}
            </span>
          </div>
        }
        footerLeftChips={stepIndex > 0 ? <Button id="ui.introduction.back" variant="ghost" icon="chevron-left" text={backLabel} onClick={back} /> : undefined}
        footerRightChips={advanceByButton ? <Button id="ui.introduction.next" icon={isLast ? "check" : "chevron-right"} text={isLast ? doneLabel : nextLabel} onClick={next} /> : undefined}
        bodyClassName="p-double shadow-lg"
      />
      {effectiveDemonstrations.length > 0 && <IntroductionDemonstrationOverlay key={step.id} demonstrations={effectiveDemonstrations} />}
    </>
  );

  if (overlayHost && typeof document !== "undefined") {
    return createPortal(overlay, overlayHost);
  }
  return overlay;
};
// #endregion 🎓️Introduction

// #region 🎥️Tutorial
//#region 🎬️TutorialEngine
/** @emoji ⏱️ Formats a millisecond offset as `mm:ss` (floored to the second, never negative). */
export function formatTutorialTime(ms: number): string {
  const totalSeconds = Math.max(0, Math.floor(ms / 1000));
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${String(seconds).padStart(2, "0")}`;
}

/** @emoji ✂️ Every cue whose `[at, at + durationMs)` window covers `atMs` — shared by narration/video/gesture track lookups. */
export function tutorialCuesBetween<T extends { readonly at: number; readonly durationMs: number }>(cues: readonly T[], atMs: number): readonly T[] {
  return cues.filter((cue) => atMs >= cue.at && atMs < cue.at + cue.durationMs);
}

function tutorialEaseInOut(t: number): number {
  return t < 0.5 ? 2 * t * t : 1 - (-2 * t + 2) ** 2 / 2;
}

function tutorialLerp3(a: readonly [number, number, number], b: readonly [number, number, number], t: number): [number, number, number] {
  return [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t];
}

/** @emoji 🎥️ TS port of Rust `interpolate_tutorial_camera` — same semantics (log-space zoom, ease-in-out/linear/hold curves, mismatched-kind snap). Keep in lockstep with `framework/core/rs/lib.rs`'s `//#region 🔖️TutorialEngine`. */
export function interpolateTutorialCamera(prev: TutorialCameraKeyframe, next: TutorialCameraKeyframe, atMs: number): TutorialCameraState {
  const span = Math.max(next.at - prev.at, 1);
  const raw = Math.min(1, Math.max(0, (atMs - prev.at) / span));
  const t = next.easing === "linear" ? raw : next.easing === "hold" ? (raw >= 1 ? 1 : 0) : tutorialEaseInOut(raw);
  const prevCamera = prev.camera;
  const nextCamera = next.camera;
  if (prevCamera.kind === "orbit" && nextCamera.kind === "orbit") {
    const fov = prevCamera.fov != null && nextCamera.fov != null ? prevCamera.fov + (nextCamera.fov - prevCamera.fov) * t : (prevCamera.fov ?? nextCamera.fov);
    return { kind: "orbit", position: tutorialLerp3(prevCamera.position, nextCamera.position, t), target: tutorialLerp3(prevCamera.target, nextCamera.target, t), up: tutorialLerp3(prevCamera.up, nextCamera.up, t), fov };
  }
  if (prevCamera.kind === "canvas" && nextCamera.kind === "canvas") {
    return {
      kind: "canvas",
      x: prevCamera.x + (nextCamera.x - prevCamera.x) * t,
      y: prevCamera.y + (nextCamera.y - prevCamera.y) * t,
      zoom: Math.exp(Math.log(prevCamera.zoom) + (Math.log(nextCamera.zoom) - Math.log(prevCamera.zoom)) * t),
    };
  }
  return t < 0.5 ? prevCamera : nextCamera;
}

/** @emoji 🎥️ TS port of Rust `tutorial_camera_at`. */
export function tutorialCameraAt(def: TutorialDefinition, windowId: string, atMs: number): TutorialCameraState | undefined {
  const keyframes = [...def.base.cameras, ...def.tracks.camera].filter((keyframe) => keyframe.windowId === windowId);
  const first = keyframes[0];
  if (!first) return undefined;
  if (atMs <= first.at) return first.camera;
  for (let index = 0; index < keyframes.length - 1; index++) {
    const prev = keyframes[index];
    const next = keyframes[index + 1];
    if (atMs <= next.at) return interpolateTutorialCamera(prev, next, atMs);
  }
  return keyframes[keyframes.length - 1].camera;
}

/** @emoji 🩹️ TS port of Rust `apply_tutorial_ui_change` — immutable (returns a new snapshot) rather than in-place, since TS callers never hold a live `&mut`. */
export function applyTutorialUiChange(state: TutorialUiSnapshot, change: TutorialUiChange): TutorialUiSnapshot {
  switch (change.kind) {
    case "activeMode":
      return { ...state, activeModeId: change.id };
    case "focusedWindow":
      return { ...state, focusedWindowId: change.id };
    case "activeUtility": {
      const next = { ...state.activeUtilityByWindowId };
      if (change.utilityId) next[change.windowId] = change.utilityId;
      else delete next[change.windowId];
      return { ...state, activeUtilityByWindowId: next };
    }
    case "activeTool":
      return { ...state, activeToolId: change.id };
    case "layout":
      return { ...state, layout: change.layout };
    case "panelTab": {
      const next = { ...state.activePanelTabByGroup };
      if (change.tabId) next[change.group] = change.tabId;
      else delete next[change.group];
      return { ...state, activePanelTabByGroup: next };
    }
    case "panelState":
      return { ...state, panelJson: change.panelJson };
    case "selection":
      return { ...state, interactionSelection: { ...state.interactionSelection, [change.domainId]: { granularity: change.granularity, ids: [...change.ids] } } };
    case "dialog":
      return { ...state, openDialogId: change.id };
    case "treeExpansion": {
      const has = state.expandedTreeIds.includes(change.id);
      if (change.expanded) return has ? state : { ...state, expandedTreeIds: [...state.expandedTreeIds, change.id] };
      return has ? { ...state, expandedTreeIds: state.expandedTreeIds.filter((id) => id !== change.id) } : state;
    }
    case "commandPanel":
      return { ...state, commandPanelOpen: change.open };
    default:
      return state;
  }
}

/** @emoji 🧮️ TS port of Rust `compose_tutorial_ui`. */
export function composeTutorialUi(def: TutorialDefinition, atMs: number): TutorialUiSnapshot {
  let state = def.base.ui;
  let deltas: TutorialUiChange[] = [];
  for (const keyframe of def.tracks.ui) {
    if (keyframe.at > atMs) break;
    if (keyframe.sample.kind === "snapshot") {
      state = keyframe.sample.state;
      deltas = [];
    } else {
      deltas = [...deltas, ...keyframe.sample.changes];
    }
  }
  for (const change of deltas) state = applyTutorialUiChange(state, change);
  return state;
}

/** @emoji ✂️ TS mirror of Rust `TutorialSlice` — see `tutorialSlice` below. */
export type TutorialSlice = {
  readonly forward: boolean;
  readonly events: readonly TutorialEvent[];
  readonly document: readonly TutorialArtifactEvent[];
  readonly uiChanges: readonly TutorialUiChange[];
};

/** @emoji ✂️ TS port of Rust `tutorial_slice` — same directionality contract (forward: oldest→newest;
 * backward: newest→oldest so `Edit.backwards` unwind in the right order). Never spans a
 * `TutorialUiSample::Snapshot` boundary correctly on its own — a seek/scrub must call
 * {@link composeTutorialUi} wholesale instead, exactly like the Rust doc comment warns. */
export function tutorialSlice(def: TutorialDefinition, fromMs: number, toMs: number): TutorialSlice {
  const forward = toMs >= fromMs;
  const lo = Math.min(fromMs, toMs);
  const hi = Math.max(fromMs, toMs);
  const inRange = (at: number) => at > lo && at <= hi;
  let events = def.tracks.events.filter((event) => inRange(event.at));
  let document = def.tracks.document.filter((event) => inRange(event.at));
  let uiChanges: TutorialUiChange[] = [];
  for (const keyframe of def.tracks.ui) {
    if (!inRange(keyframe.at)) continue;
    if (keyframe.sample.kind === "delta") uiChanges = [...uiChanges, ...keyframe.sample.changes];
  }
  if (!forward) {
    events = [...events].reverse();
    document = [...document].reverse();
    uiChanges = [...uiChanges].reverse();
  }
  return { forward, events, document, uiChanges };
}

/** @emoji ✅️ TS port of Rust `validate_tutorial` — light structural sanity check shared by the recorder before download; does not validate referenced action/command/element ids (no `AppDefinition` in scope here). Returns the first error found, or `null`. */
export function validateTutorial(def: TutorialDefinition): string | null {
  const sortedByAt = <T,>(label: string, items: readonly T[], at: (item: T) => number): string | null => {
    let last: number | null = null;
    for (const item of items) {
      const value = at(item);
      if (value > def.durationMs) return `tutorial track \`${label}\` has an entry at ${value}ms beyond durationMs ${def.durationMs}`;
      if (last != null && value < last) return `tutorial track \`${label}\` is not sorted ascending by \`at\` (${last}ms then ${value}ms)`;
      last = value;
    }
    return null;
  };
  const checks: readonly (() => string | null)[] = [
    () => sortedByAt("chapters", def.chapters, (c) => c.at),
    () => sortedByAt("narration", def.tracks.narration, (c) => c.at),
    () => sortedByAt("video", def.tracks.video, (c) => c.at),
    () => sortedByAt("events", def.tracks.events, (e) => e.at),
    () => sortedByAt("ui", def.tracks.ui, (k) => k.at),
    () => sortedByAt("document", def.tracks.document, (e) => e.at),
    () => sortedByAt("camera", def.tracks.camera, (k) => k.at),
    () => sortedByAt("gestures", def.tracks.gestures, (c) => c.at),
  ];
  for (const check of checks) {
    const error = check();
    if (error) return error;
  }
  const chapterIds = new Set<string>();
  for (const chapter of def.chapters) {
    if (chapterIds.has(chapter.id)) return `duplicate tutorial chapter id \`${chapter.id}\``;
    chapterIds.add(chapter.id);
  }
  const cueIds = new Set<string>();
  for (const cue of def.tracks.narration) {
    if (cueIds.has(cue.id)) return `duplicate tutorial narration cue id \`${cue.id}\``;
    cueIds.add(cue.id);
  }
  for (const camera of def.base.cameras) {
    if (camera.at !== 0) return `tutorial base camera keyframe for window \`${camera.windowId}\` must have at == 0`;
  }
  return null;
}
//#endregion 🎬️TutorialEngine

//#region 🎬️TutorialClock
/** @emoji ⏱️ The minimal read+subscribe surface {@link TutorialBar}/{@link TutorialCaptions}/{@link TutorialVideoOverlay}/{@link TutorialGhostPointer} need — lets every per-frame time consumer self-subscribe via `useSyncExternalStore` instead of the whole shell re-rendering on every tick. */
export type TutorialClockPort = {
  readonly getTimeMs: () => number;
  readonly subscribe: (callback: () => void) => () => void;
};

/** @emoji ⏱️ Full imperative control surface for a `TutorialClockPort` — owned by the shell orchestration (director), read by the UI kit. */
export type TutorialClock = TutorialClockPort & {
  readonly play: () => void;
  readonly pause: () => void;
  readonly seek: (ms: number) => void;
  readonly setRate: (rate: number) => void;
  readonly setDurationMs: (durationMs: number) => void;
  readonly isPlaying: () => boolean;
  readonly getRate: () => number;
  readonly dispose: () => void;
};

/** @emoji ⏱️ Creates a tiny external store driven by `requestAnimationFrame`: `t += dtWallClock * rate` while playing, auto-pausing at `durationMs`. Not a React hook itself — subscribe via {@link useTutorialClock} (or any `useSyncExternalStore`) for the reactive read. */
export function createTutorialClock(durationMs: number): TutorialClock {
  let tMs = 0;
  let rate = 1;
  let playing = false;
  let duration = Math.max(0, durationMs);
  let lastFrameAt: number | null = null;
  let rafHandle: number | null = null;
  const subscribers = new Set<() => void>();
  const notify = () => subscribers.forEach((callback) => callback());
  const tick = (now: number) => {
    if (!playing) {
      rafHandle = null;
      return;
    }
    const dt = lastFrameAt == null ? 0 : now - lastFrameAt;
    lastFrameAt = now;
    tMs = Math.min(duration, Math.max(0, tMs + dt * rate));
    if (tMs >= duration) playing = false;
    notify();
    rafHandle = playing ? requestAnimationFrame(tick) : null;
  };
  return {
    getTimeMs: () => tMs,
    subscribe: (callback) => {
      subscribers.add(callback);
      return () => subscribers.delete(callback);
    },
    play: () => {
      if (playing || duration <= 0) return;
      playing = true;
      lastFrameAt = null;
      if (rafHandle == null) rafHandle = requestAnimationFrame(tick);
      notify();
    },
    pause: () => {
      if (!playing) return;
      playing = false;
      if (rafHandle != null) {
        cancelAnimationFrame(rafHandle);
        rafHandle = null;
      }
      notify();
    },
    seek: (ms) => {
      tMs = Math.min(duration, Math.max(0, ms));
      lastFrameAt = playing ? performance.now() : null;
      notify();
    },
    setRate: (nextRate) => {
      rate = nextRate;
      notify();
    },
    setDurationMs: (nextDuration) => {
      duration = Math.max(0, nextDuration);
      tMs = Math.min(tMs, duration);
      notify();
    },
    isPlaying: () => playing,
    getRate: () => rate,
    dispose: () => {
      if (rafHandle != null) cancelAnimationFrame(rafHandle);
      subscribers.clear();
    },
  };
}

/** @emoji ⏱️ Subscribes the calling component to a {@link TutorialClockPort}'s per-frame time — only this component re-renders on tick, never the whole shell. */
export function useTutorialClock(clock: TutorialClockPort): number {
  return reactHostPort.useSyncExternalStore(clock.subscribe, clock.getTimeMs, clock.getTimeMs);
}
//#endregion 🎬️TutorialClock

//#region 🎬️TutorialCameraDriver
/** @emoji 🎥️ A live 3D/2D window's imperative camera bridge for tutorial playback — modeled exactly on {@link registerIntroductionSurfaceResolver}. `get` reads the surface's current live pose (for deviation-then-play convergence); `set` writes a pose during playback/seek. */
export type TutorialCameraDriver = {
  readonly get: () => TutorialCameraState | null;
  readonly set: (camera: TutorialCameraState) => void;
};

const tutorialCameraDrivers = ephemeralMap<string, TutorialCameraDriver>("framework.modules.ui.packages.typescript.targets.react.index.tsx.tutorialCameraDrivers");

/** @emoji 🎥️ Registers the tutorial camera driver for window instance `windowId` — call from the window's own host component (e.g. `World3dHost`), unregister on unmount via the returned disposer. */
export function registerTutorialCameraDriver(windowId: string, driver: TutorialCameraDriver): () => void {
  tutorialCameraDrivers.set(windowId, driver);
  return () => {
    if (tutorialCameraDrivers.get(windowId) === driver) tutorialCameraDrivers.delete(windowId);
  };
}

/** @emoji 🎥️ Looks up a registered {@link TutorialCameraDriver}, or `undefined` if that window hasn't mounted/registered one yet. */
export function getTutorialCameraDriver(windowId: string): TutorialCameraDriver | undefined {
  return tutorialCameraDrivers.get(windowId);
}
//#endregion 🎬️TutorialCameraDriver

//#region 🎬️TutorialBar
const TUTORIAL_RATES = [0.5, 1, 1.5, 2] as const;

/** @emoji ⏭️ Cycles through the fixed rate ladder (0.5→1→1.5→2→0.5→…). */
function nextTutorialRate(rate: number): number {
  const index = TUTORIAL_RATES.indexOf(rate as (typeof TUTORIAL_RATES)[number]);
  return TUTORIAL_RATES[(index === -1 ? 0 : index + 1) % TUTORIAL_RATES.length];
}

export type TutorialChapterMarker = { readonly id: string; readonly title: string; readonly atMs: number };

export type TutorialBarProps = {
  readonly title: string;
  readonly durationMs: number;
  readonly playing: boolean;
  readonly rate: number;
  readonly muted: boolean;
  readonly captionsOn: boolean;
  readonly recording: boolean;
  readonly recordAvailable: boolean;
  readonly chapters: readonly TutorialChapterMarker[];
  readonly clock: TutorialClockPort;
  readonly onPlayPause: () => void;
  readonly onStop: () => void;
  readonly onSeek: (ms: number) => void;
  readonly onRateChange: (rate: number) => void;
  readonly onMutedChange: (muted: boolean) => void;
  readonly onCaptionsChange: (captionsOn: boolean) => void;
  readonly onRecordToggle: () => void;
  readonly onAddChapter: () => void;
};

/** @emoji 🎥️ Navbar-style timeline/controls row shown as {@link LayoutProps.subnavbar} whenever a tutorial is active — markup mirrors `Navbar`/`Footer` (`id="ui.tutorial.bar"`, `data-slot="tutorial-bar"`, `data-ui-reveal-region`, `data-elevation-root`). Only this component subscribes to the per-frame `clock` (see {@link useTutorialClock}), so the 60fps scrubber/time readout never re-renders the rest of the shell. */
export const TutorialBar: React.FC<TutorialBarProps> = ({
  title,
  durationMs,
  playing,
  rate,
  muted,
  captionsOn,
  recording,
  recordAvailable,
  chapters,
  clock,
  onPlayPause,
  onStop,
  onSeek,
  onRateChange,
  onMutedChange,
  onCaptionsChange,
  onRecordToggle,
  onAddChapter,
}) => {
  const parent = useSurface();
  const paints = shellFloorPaints(parent);
  const bgClass = shellFloorFillClass(parent);
  const timeMs = useTutorialClock(clock);
  const playLabel = useLabel("tutorial.play");
  const pauseLabel = useLabel("tutorial.pause");
  const stopLabel = useLabel("tutorial.stop");
  const muteLabel = useLabel("tutorial.mute");
  const captionsLabel = useLabel("tutorial.captions");
  const recordLabel = useLabel("tutorial.record");
  const recordingLabel = useLabel("tutorial.recording");
  const addChapterLabel = useLabel("tutorial.addChapter");

  const body = (
    <>
      <div className="p-single flex gap-single items-center min-w-0 h-full">
        <Button id="ui.tutorial.play" icon={playing ? "pause" : "play"} text={playing ? pauseLabel : playLabel} onClick={onPlayPause} />
        <Button id="ui.tutorial.stop" icon="square" text={stopLabel} onClick={onStop} />
        <span id="ui.tutorial.time" data-slot="tutorial-time" className="shrink-0 whitespace-nowrap px-single text-xs tabular-nums text-muted-foreground">
          {formatTutorialTime(timeMs)} / {formatTutorialTime(durationMs)}
        </span>
        <div className="relative min-w-0 flex-1 px-single">
          <Slider id="ui.tutorial.scrubber" min={0} max={Math.max(durationMs, 1)} value={[timeMs]} onValueChange={(values) => onSeek(values[0] ?? 0)} showValue={false} />
          {chapters.map((chapter) => (
            <button
              key={chapter.id}
              id={`ui.tutorial.chapter.${chapter.id}`}
              type="button"
              data-slot="tutorial-chapter-tick"
              title={chapter.title}
              className="pointer-events-auto absolute top-1/2 size-1 -translate-x-1/2 -translate-y-1/2 rounded-full bg-foreground/50 hover:bg-foreground"
              style={{ left: `${durationMs > 0 ? (chapter.atMs / durationMs) * 100 : 0}%` }}
              onClick={() => onSeek(chapter.atMs)}
            />
          ))}
        </div>
        <Button id="ui.tutorial.rate" icon="bar-chart-3" text={`${rate}x`} onClick={() => onRateChange(nextTutorialRate(rate))} />
        <Button id="ui.tutorial.mute" icon="x" text={muteLabel} aria-pressed={muted} onClick={() => onMutedChange(!muted)} />
        <Button id="ui.tutorial.captions" icon="message-square" text={captionsLabel} aria-pressed={captionsOn} onClick={() => onCaptionsChange(!captionsOn)} />
        {recordAvailable && (
          <>
            <Button id="ui.tutorial.record" icon="circle-dot" text={recordLabel} aria-pressed={recording} onClick={onRecordToggle} />
            {recording && (
              <span id="ui.tutorial.recordingIndicator" data-slot="tutorial-recording-indicator" className="whitespace-nowrap px-single text-xs text-destructive">
                {recordingLabel}
              </span>
            )}
          </>
        )}
        <Button id="ui.tutorial.addChapter" icon="plus" text={addChapterLabel} onClick={onAddChapter} />
        <span className="min-w-0 flex-1 truncate px-single text-right text-xs text-muted-foreground">{title}</span>
      </div>
    </>
  );

  return (
    <nav id="ui.tutorial.bar" data-slot="tutorial-bar" data-level="base" data-ui-reveal-region="tutorial-bar" data-elevation-root="" className={cn("relative h-large z-navbar border-t border-border", bgClass)}>
      {paints ? (
        <SurfaceScope level="base" fill="surface">
          {body}
        </SurfaceScope>
      ) : (
        body
      )}
    </nav>
  );
};
//#endregion 🎬️TutorialBar

//#region 🎬️TutorialCaptions
export type TutorialCaptionsProps = {
  readonly text: string | null;
  readonly visible: boolean;
};

/** @emoji 💬️ Bottom-center glass strip showing the active narration cue's caption text — hidden entirely when `visible` is false (captions toggled off) or no cue currently covers the playhead. Reuses {@link GLASS_OVERLAY_BOX_CLASS}'s panel glass tier, positioned bottom-center instead of anchored. */
export const TutorialCaptions: React.FC<TutorialCaptionsProps> = ({ text, visible }) => {
  if (!visible || !text) return null;
  return (
    <Surface id="ui.tutorial.captions.text" data-slot="tutorial-captions" level="dialog" fill="glass" className={cn(GLASS_OVERLAY_BOX_CLASS, "bottom-double left-1/2 max-w-2xl -translate-x-1/2 text-center text-sm")}>
      {text}
    </Surface>
  );
};
//#endregion 🎬️TutorialCaptions

//#region 🎬️TutorialVideoOverlay
export type TutorialVideoOverlayProps = {
  readonly src: string | null;
  readonly rect: TutorialOverlayRect;
  readonly muted: boolean;
  readonly playing: boolean;
  readonly rate: number;
  /** ⏱️ Offset (ms) into THIS cue's own source — the overlay resyncs `video.currentTime` to this whenever it drifts. */
  readonly localTimeMs: number;
};

/** @emoji 📹️ Fixed `<video>` positioned via the active cue's normalized {@link TutorialOverlayRect}, resynced to the playhead every render (hard reseek past a 300ms drift, matching the plan's tolerance) rather than played independently. */
export const TutorialVideoOverlay: React.FC<TutorialVideoOverlayProps> = ({ src, rect, muted, playing, rate, localTimeMs }) => {
  const videoRef = reactHostPort.useRef<HTMLVideoElement | null>(null);
  reactHostPort.useEffect(() => {
    const video = videoRef.current;
    if (!video) return;
    video.muted = muted;
    video.playbackRate = rate;
    const targetSeconds = localTimeMs / 1000;
    if (Math.abs(video.currentTime - targetSeconds) > 0.3) video.currentTime = targetSeconds;
    if (playing) void video.play().catch(() => {});
    else video.pause();
  }, [muted, rate, playing, localTimeMs]);
  if (!src) return null;
  return (
    <video
      id="ui.tutorial.video"
      ref={videoRef}
      data-slot="tutorial-video-overlay"
      src={src}
      className="pointer-events-none fixed z-tutorial rounded-lg object-cover shadow-lg"
      style={{ left: `${rect.x * 100}%`, top: `${rect.y * 100}%`, width: `${rect.width * 100}%`, height: `${rect.height * 100}%` }}
      muted={muted}
      playsInline
    />
  );
};
//#endregion 🎬️TutorialVideoOverlay

//#region 🎬️TutorialGhostPointer
export type TutorialGhostPointerProps = {
  /** 🎬️ The gesture cue currently covering the playhead, or `null` between cues. */
  readonly cue: TutorialGestureCue | null;
  /** 📈️ 0–1 progress through `cue`'s own `durationMs`, driven by the tutorial playhead (never the ghost pointer's own clock, unlike `IntroductionDemonstrationOverlay`). */
  readonly progress: number;
};

/** @emoji 👻️ Tutorial-playback ghost cursor — a lean, parallel implementation of `IntroductionDemonstrationOverlay`'s point resolution (reuses {@link resolveIntroductionPoint}/{@link introductionDemoArcPoint}/{@link introductionDemoResolveVisual} verbatim rather than re-deriving them) driven by the tutorial playhead's `progress` instead of its own internal rAF phase machine — click-family/scroll gestures render statically at their point, `drag` lerps linearly from→to, `orbit` bulges along the same quadratic arc the introduction overlay uses. */
export const TutorialGhostPointer: React.FC<TutorialGhostPointerProps> = ({ cue, progress }) => {
  const [point, setPoint] = reactHostPort.useState<{ readonly x: number; readonly y: number } | null>(null);
  // 🐚️ See `IntroductionDemonstrationOverlay`'s doc for why this falls back to `document.documentElement`.
  const shellScope = useShellScopeOptional();

  reactHostPort.useEffect(() => {
    if (!cue) {
      setPoint(null);
      return;
    }
    const root: Element = shellScope?.rootRef.current ?? document.documentElement;
    const gesture = cue.gesture;
    let next: { readonly x: number; readonly y: number } | null = null;
    switch (gesture.kind) {
      case "leftClick":
      case "rightClick":
      case "doubleClick":
      case "scroll":
        next = resolveIntroductionPoint(gesture.at, root);
        break;
      case "drag": {
        const from = resolveIntroductionPoint(gesture.from, root);
        const to = resolveIntroductionPoint(gesture.to, root);
        next = from && to ? { x: from.x + (to.x - from.x) * progress, y: from.y + (to.y - from.y) * progress } : (from ?? to ?? null);
        break;
      }
      case "orbit": {
        const from = resolveIntroductionPoint(gesture.from, root);
        const to = resolveIntroductionPoint(gesture.to, root);
        next = from && to ? introductionDemoArcPoint(from, to, progress) : (from ?? to ?? null);
        break;
      }
      default:
        next = null;
    }
    setPoint(next);
  }, [cue, progress, shellScope]);

  if (!cue || !point) return null;
  const glyph = cue.cursor ?? INTRODUCTION_DEMO_GESTURE_DEFAULT_CURSOR[cue.gesture.kind];
  return (
    <div
      id="ui.tutorial.ghostPointer"
      data-slot="tutorial-ghost-pointer"
      aria-hidden
      className="pointer-events-none fixed z-tutorial size-double -translate-x-1/2 -translate-y-1/2 bg-contain bg-no-repeat opacity-90"
      style={{ left: point.x, top: point.y, backgroundImage: `var(--cursor-ghost-${glyph})` }}
    />
  );
};
//#endregion 🎬️TutorialGhostPointer
// #endregion 🎥️Tutorial

// #region 🗨️Dialog
import { UIDialog, type UIDialogProps, type UIDialogFieldBinding } from "../../../../🧱️elements/📨️UIDialog/🟦️.tsx";
export { UIDialog, type UIDialogProps, type UIDialogFieldBinding };
// #endregion 🗨️Dialog

// #region 🎈️Level Context
/** @emoji 📚️ Semantic UI depth layer for background/glass/z-index tokens (base=0 .. menu=5, formula-derived — see contract at .🧬semio/🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt). */
import {
  type Level,
  LEVELS,
  LevelProvider,
  useLevel,
  getLevelZClass,
  type SurfaceFill,
  surfaceFillClass,
  type SurfaceScopeValue,
  SurfaceScope,
  useSurface,
  type SurfaceProps,
  Surface,
  isSurfaceActiveBackgroundPointer,
  type SurfaceActiveBindProps,
  useSurfaceActive,
  setSurfaceActiveRoot,
} from "../../../../🧱️elements/🌈️Surface/🟦️.tsx";
export {
  type Level,
  LEVELS,
  LevelProvider,
  useLevel,
  getLevelZClass,
  type SurfaceFill,
  surfaceFillClass,
  type SurfaceScopeValue,
  SurfaceScope,
  useSurface,
  type SurfaceProps,
  Surface,
  isSurfaceActiveBackgroundPointer,
  type SurfaceActiveBindProps,
  useSurfaceActive,
  setSurfaceActiveRoot,
};

/** @emoji 📏️ Emphasized shell stroke for active/selected chrome accents. */
export const borderEmphasizedClass = "!border-emphasized";

/** @emoji 📏️ Emphasized chrome frame (`border` + {@link borderEmphasizedClass}). */
export const borderEmphasizedFrameClass = `box-border border border-solid ${borderEmphasizedClass}`;

/** @emoji 📏️ Emphasized navbar bottom edge. */
export const borderEmphasizedBottomClass = `border-b ${borderEmphasizedClass}`;

/** @emoji 📏️ Emphasized footer top edge. */
export const borderEmphasizedTopClass = `border-t ${borderEmphasizedClass}`;

/** @emoji 📏️ Subtle normal stroke for controls, windows, dividers, and in-chrome separators. */

/** @emoji 📏️ Normal chrome frame (`border` + {@link borderNormalClass}); panel/pane hosts prefer {@link shellChromeFrameLayerClass} + CSS parent-hover. */
export const borderNormalFrameClass = `box-border border border-solid ${borderNormalClass}`;

/** @emoji 📏️ Normal bottom edge utility for in-chrome dividers (not shell navbar — navbar uses a CSS `::after` stroke). */

/** @emoji 📏️ Normal top edge utility for in-chrome dividers (not shell footer — footer uses a CSS `::before` stroke). */
export const borderNormalTopClass = `border-t ${borderNormalClass}`;

/** @emoji 📏️ Active window chrome line when that stack is globally active. */
export const activeLineClass = "border-active-base";

/** @emoji 🪟️ Shell outline — normal gray frame; emphasized while the pointer is inside the parent `[data-slot="panel"]` / `[data-slot="pane"]`. */
export const shellChromeBorderClass = borderNormalFrameClass;

/** @emoji 🪟️ Parent stroke on top of fill and content (transparent center; ghost-dimmed with the open panel/pane). Border color is CSS-only so parent `:hover` can emphasize it like the window silhouette. */
export const shellChromeFrameLayerClass = "pointer-events-none absolute inset-0 z-30 box-border bg-transparent";

/** @emoji 🪟️ Frosted floating menu/popover surface for technology renderer overlays — menu tier; host element must also carry `data-level="menu"` (Radix-portal-style consumers stamp their own content root). */
export const floatingMenuSurfaceClass = cn(glassClass, "overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass);

// #region 📋️MenuItem
import { MenuItem, menuItemClassName, type MenuItemProps } from "../../../../🧱️elements/📋️MenuItem/🟦️.tsx";
export { MenuItem, menuItemClassName, type MenuItemProps };

/** @emoji 🪟️ Action row inside {@link floatingMenuSurfaceClass}. */
export const floatingMenuItemClass = menuItemClassName;
// #endregion 📋️MenuItem

/** @emoji 🪟️ Frosted editor aside chrome for technology renderers — pane-level chrome; host element must also carry `data-level="pane"`. */
export const floatingPaneAsideClass = cn("relative flex shrink-0 flex-col gap-single overflow-auto p-double text-element z-[2]", shellChromeBorderClass, glassClass);

/** @emoji 🪟️ Frosted compact ribbon chrome (projection switch, align controls) — window-level chrome; host element must also carry `data-level="window"`. */
export const floatingRibbonSurfaceClass = cn("overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass, glassClass);

/** @emoji 🪟️ Frosted inline field/action shell inside editor asides — menu tier (fallback); host element must also carry `data-level="menu"`. */
export const floatingFieldSurfaceClass = cn(glassClass, "relative overflow-visible rounded-md border", borderNormalClass);

/** @emoji 🪟️ Golden-window host root for technology canvases inside {@link ProductShell} — base level (the floor windows float on); host element must also carry `data-level="base"`. */
export const canvasHostRootClass = cn("relative flex h-full min-h-0 w-full min-w-0 flex-col text-element font-sans", surfaceClass);

/** @emoji 🪟️ Full-viewport standalone editor shell (outside golden windows) — base level. */
export const editorShellRootClass = cn("text-element flex h-screen min-h-0 w-full flex-row font-sans", surfaceClass);

/** @emoji 🏷️ Toggle chip for layer/filter controls in technology renderers. */
export const floatingTagClass = cn("inline-flex items-center gap-half rounded-full border px-half py-0.5 text-xs text-element", borderNormalClass);

export const floatingTagOnClass = "bg-accent text-accent-foreground";
export const floatingTagOffClass = "bg-transparent text-muted-foreground";

/** @emoji 🪟️ Canvas viewport surface inside a host root. */
export const canvasViewportClass = cn("relative h-full min-h-0 w-full min-w-0 outline-none", surfaceClass);

/** @emoji 📑️ Panel tab strip scroll row — `ui-scrollbar-hidden` preserves fixed control height when overflowing tabs scroll horizontally. */
const panelTabBarScrollClass = "ui-scrollbar-hidden relative z-40 flex min-w-0 items-stretch shrink-0 overflow-x-auto overscroll-x-contain scroll-px-single";

/** @emoji 📑️ Panel tab strip base — `w-full` spans chrome/mobile dividers across the row; floating panel caps omit it so the U-gap opens after the last tab. */
const panelTabBarBaseClass = cn(panelTabBarScrollClass, "w-full");

/** @emoji 📑️ Panel tab strip with its divider on the content-facing side. */
export const panelTabBarClass = cn(panelTabBarBaseClass, borderNormalBottomClass);

/** @emoji 📑️ Panel tab icon slot — defers dimensions to the tab icon (12px). */

/** @emoji 📑️ Panel tab label beside the icon. */

/** @emoji 📏️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome silhouette. */
/** @emoji 📏️ Normal logical-end divider between sibling panel-tab toggles; the last toggle defers its outer edge to the hosting chrome silhouette. */
export const panelTabButtonDividerClass = "border-e border-solid !border-normal last:border-e-0";

/** @emoji 📑️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
/** @emoji 📑️ Panel tab button with icon, mandatory name, and a normal divider between sibling toggles. */
export const panelTabButtonClass = cn(
  "inline-flex min-h-0 shrink-0 items-center gap-tiny bg-transparent p-0",
  panelTabButtonDividerClass,
  "cursor-pointer whitespace-nowrap text-xs leading-none text-element transition-colors",
  "outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base",
  "[&[data-active=true]:not(:first-child)]:border-s [&[data-active=true]:not(:first-child)]:-ms-px",
  hoverExcludingHandleBgFillClass,
  hoverExcludingHandleTextEmphasizedClass,
);

/** @emoji 📑️ Floating panel tab strip inside {@link WindowChrome} — collapsed tabs defer every outer edge to the silhouette; expanded tabs restore the normal content-facing edge that separates their toggles from the panel body. */
export function panelAnchorTabBarClass(direction: "up" | "down", expanded = false): string {
  return cn(panelTabBarScrollClass, "h-medium", expanded && (direction === "up" ? borderNormalTopClass : borderNormalBottomClass));
}

/** @emoji 📑️ Panel tab button padding. */
/** @emoji 📑️ Panel tab button padding. */
export const panelAnchorTabButtonClass = cn(panelTabButtonClass, "px-tiny");

// #region 🫳️DragAffordance
import { DragHandle, HANDLE_HOVER_SCOPE_ATTR } from "../../../../🧱️elements/🧱️DragHandle/🟦️.tsx";
export { DragHandle, HANDLE_HOVER_SCOPE_ATTR };
// #endregion 🫳️DragAffordance

/** @emoji 📑️ Shared panel/mobile panel tab bar variant. */
/** @emoji 📑️ `"chrome"` is a host alias for `"panel"` — folded chrome-hosted bars render via {@link WindowChrome} chipOnly, not a separate visual variant. */
// #region 📑️PanelTabBar
import {
  reconcileActivePath,
  singleTreeLeaf,
  panelTabChildren,
  findPanelTabNode,
  findPanelTabPath,
  resolvePanelBranchBodyLeaf,
  progressPanelTabSelection,
  usePanelTabSelection,
  dockSkeletonOf,
  dockSkeletonsEqual,
  applyDockSkeleton,
  PanelTabBar,
  type PanelTabBarVariant,
  type PanelTreeUnit,
  type PanelTabLeaf,
  type PanelTabBranch,
  type PanelTabNode,
  type PanelTabSelectionResult,
  type PanelTabSelectionOptions,
  type PanelDock,
  type PanelTabBarProps,
} from "../../../../🧱️elements/🧭️PanelTabBar/🟦️.tsx";
export {
  reconcileActivePath,
  singleTreeLeaf,
  panelTabChildren,
  findPanelTabNode,
  findPanelTabPath,
  resolvePanelBranchBodyLeaf,
  progressPanelTabSelection,
  usePanelTabSelection,
  dockSkeletonOf,
  dockSkeletonsEqual,
  applyDockSkeleton,
  PanelTabBar,
  type PanelTabBarVariant,
  type PanelTreeUnit,
  type PanelTabLeaf,
  type PanelTabBranch,
  type PanelTabNode,
  type PanelTabSelectionResult,
  type PanelTabSelectionOptions,
  type PanelDock,
  type PanelTabBarProps,
};
// #endregion 📑️PanelTabBar

export const ANCHORS = ["top-left", "top-middle", "top-right", "right-middle", "bottom-right", "bottom-middle", "bottom-left", "left-middle"] as const;

/** @emoji 🧭️ One of the eight anchors a panel or pane can grow from. */
export type Anchor = (typeof ANCHORS)[number];

/** @emoji 🧭️ `"top"`/`"middle"`/`"bottom"` row of an {@link Anchor} — `left-middle`/`right-middle` sit in the middle row. */
export function anchorVertical(anchor: Anchor): "top" | "middle" | "bottom" {
  if (anchor.startsWith("top")) return "top";
  if (anchor.startsWith("bottom")) return "bottom";
  return "middle";
}

/** @emoji 🧭️ `"left"`/`"middle"`/`"right"` column of an {@link Anchor} — `top-middle`/`bottom-middle` sit in the middle column. */
export function anchorHorizontal(anchor: Anchor): "left" | "middle" | "right" {
  switch (anchor) {
    case "top-left":
    case "bottom-left":
    case "left-middle":
      return "left";
    case "top-right":
    case "bottom-right":
    case "right-middle":
      return "right";
    default:
      return "middle";
  }
}

// #region 🧭️Flow Context
/** @emoji 🧭️ Horizontal reading direction — `"rtl"` mirrors icon/label order and rides on native CSS `dir`. */
import { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow } from "../../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
export { type FlowInline, type FlowBlock, type Flow, FlowProvider, useFlow };

/** @emoji 🧭️ The mirrored {@link Flow} a {@link Panel} or {@link Pane} grows into — right anchors flip inline, bottom anchors flip block; middle anchors (row or column) never mirror. */
export function flowFromAnchor(anchor: Anchor): Flow {
  return { inline: anchorHorizontal(anchor) === "right" ? "rtl" : "ltr", block: anchorVertical(anchor) === "bottom" ? "up" : "down" };
}

/** @emoji 🧭️ Edge insets for an {@link Anchor} plus responsive width/height clamps to its containing region — corners inset on both axes, an edge-middle anchor centers along its middle axis via a translate. Shared by {@link Panel} and {@link Pane} so both float from identical math. */
export function anchorPositionStyle(anchor: Anchor): React.CSSProperties {
  const horizontal = anchorHorizontal(anchor);
  const vertical = anchorVertical(anchor);
  const style: React.CSSProperties = {
    maxWidth: "calc(100% - (var(--spacing-single) * 2))",
    maxHeight: "calc(100% - (var(--spacing-single) * 2))",
  };
  if (horizontal === "middle") {
    style.left = "50%";
    style.transform = "translateX(-50%)";
  } else {
    style[horizontal] = "var(--spacing-single)";
  }
  if (vertical === "middle") {
    style.top = "50%";
    style.transform = style.transform ? `${style.transform} translateY(-50%)` : "translateY(-50%)";
  } else {
    style[vertical] = "var(--spacing-single)";
  }
  return style;
}

/**
 * @emoji 🧭️ Open chrome-hosted {@link Panel} position — pulls the window-chrome cap into the navbar/footer
 * band where {@link PanelChromeTabBar} parked the folded toggles, so opening unfolds in place instead of
 * jumping the chips into the canvas. Navbar/footer are `h-large` with centered `h-medium` items; the offset
 * aligns the panel's `min-h-medium` cap to that centered row. Max height grows by the same overhang so the
 * body still fills the canvas region.
 * @see {@link anchorPositionStyle}
 */
export function chromeHostedOpenPanelPositionStyle(anchor: Anchor): React.CSSProperties {
  const style = anchorPositionStyle(anchor);
  const vertical = anchorVertical(anchor);
  // 📍️ From the middle-region edge back to the centered h-medium row inside h-large shell chrome.
  const intoShellChrome = "calc(-1 * (var(--size-large) + var(--size-medium)) / 2)";
  const maxHeightWithChrome = "calc(100% + (var(--size-large) + var(--size-medium)) / 2)";
  if (vertical === "top") {
    style.top = intoShellChrome;
    style.maxHeight = maxHeightWithChrome;
  } else if (vertical === "bottom") {
    style.bottom = intoShellChrome;
    style.maxHeight = maxHeightWithChrome;
  }
  return style;
}

/** @emoji 🧭️ Chevron {@link IconName} that points toward where a fold's content is — the opposite state's chevron, mirrored for {@link FlowInline} `"rtl"`. `pointsOut` is `true` when the fold is collapsed (chevron points toward the hidden content) and `false` when expanded (chevron points back at the fold). */
export function flowChevronIconName(inline: FlowInline, pointsOut: boolean): IconName {
  const right = inline === "rtl" ? pointsOut : !pointsOut;
  return right ? "chevron-right" : "chevron-left";
}
// #endregion 🧭️Flow Context

/** @emoji 🌱️ One draggable tree granule inside a leaf tab — dockable between leaf tabs, rendered as its own collapsible section. */
// #region 🧲️PanelDock
// Composable drag-and-drop: tabs dock between all eight anchors (pointer-capture drag, mirrors 🧭️ModeDockDrag);
// tree units dock between leaf tabs (native HTML5 drag-and-drop, mirrors the window-template palette-drag session).

//#region 🔀️Transforms

/** @emoji 🔀️ True if `id` is `node.id` itself or belongs to one of its descendants. */
export function isPanelTabInSubtree(node: PanelTabNode, id: string): boolean {
  if (node.id === id) return true;
  return node.kind === "branch" && node.children.some((child) => isPanelTabInSubtree(child, id));
}

/** @emoji 🔀️ Locates a tab anywhere in `dock`, returning its home anchor and the node itself. */
export function findPanelTabInDock(dock: PanelDock, id: string): { readonly anchor: Anchor; readonly node: PanelTabNode } | null {
  for (const anchor of ANCHORS) {
    const path = findPanelTabPath(dock.anchors[anchor], id);
    if (!path) continue;
    const node = findPanelTabNode(dock.anchors[anchor], path);
    if (node) return { anchor, node };
  }
  return null;
}

function collectPanelTabSubtreeIds(node: PanelTabNode, into: Set<string>): void {
  into.add(node.id);
  if (node.kind === "branch") node.children.forEach((child) => collectPanelTabSubtreeIds(child, into));
}

/** @emoji 🔀️ Reassigns `order` to match array position — call after any transform that reorders a sibling array so the existing order-based sort renders the new arrangement. */
export function normalizePanelTabOrder(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  return tabs.map((tab, index) => (tab.order === index ? tab : { ...tab, order: index }));
}

/** @emoji 🔀️ Recursively drops branch tabs left with zero children. The complementary half of "deliberately emptied" — keeping such branches out of {@link applyDockSkeleton}'s auto-append — is handled there by its subtree-mention check, so an emptied branch never resurfaces with default children restored. */
export function pruneEmptyPanelBranches(tabs: readonly PanelTabNode[]): readonly PanelTabNode[] {
  const pruned = tabs.map((tab) => (tab.kind === "branch" ? { ...tab, children: pruneEmptyPanelBranches(tab.children) } : tab)).filter((tab) => tab.kind !== "branch" || tab.children.length > 0);
  return pruned.length === tabs.length && pruned.every((tab, index) => tab === tabs[index]) ? tabs : pruned;
}

/** @emoji 🔀️ Removes `id` wherever it lives in `tabs` (searching recursively into branches), pruning any ancestor branch left empty by the removal. Removal-first: callers compute insertion indices against this result, never the pre-removal tree. */
function removePanelTabFromSiblings(tabs: readonly PanelTabNode[], id: string): { readonly tabs: readonly PanelTabNode[]; readonly removed: PanelTabNode | null } {
  const directIndex = tabs.findIndex((tab) => tab.id === id);
  if (directIndex >= 0) {
    const removed = tabs[directIndex]!;
    return { tabs: normalizePanelTabOrder([...tabs.slice(0, directIndex), ...tabs.slice(directIndex + 1)]), removed };
  }
  for (let index = 0; index < tabs.length; index++) {
    const tab = tabs[index]!;
    if (tab.kind !== "branch") continue;
    const { tabs: nextChildren, removed } = removePanelTabFromSiblings(tab.children, id);
    if (!removed) continue;
    const next = nextChildren.length > 0 ? [...tabs.slice(0, index), { ...tab, children: nextChildren }, ...tabs.slice(index + 1)] : [...tabs.slice(0, index), ...tabs.slice(index + 1)];
    return { tabs: normalizePanelTabOrder(next), removed };
  }
  return { tabs, removed: null };
}

function insertPanelTabAtPath(tabs: readonly PanelTabNode[], parentPath: readonly string[], node: PanelTabNode, index: number): readonly PanelTabNode[] {
  if (parentPath.length === 0) {
    const clampedIndex = Math.max(0, Math.min(index, tabs.length));
    return normalizePanelTabOrder([...tabs.slice(0, clampedIndex), node, ...tabs.slice(clampedIndex)]);
  }
  const [headId, ...restPath] = parentPath;
  const headIndex = tabs.findIndex((tab) => tab.id === headId);
  if (headIndex < 0) return tabs;
  const head = tabs[headIndex]!;
  if (head.kind !== "branch") return tabs;
  const next = [...tabs];
  next[headIndex] = { ...head, children: insertPanelTabAtPath(head.children, restPath, node, index) };
  return next;
}

function appendPanelTabAsChild(tabs: readonly PanelTabNode[], parentId: string, node: PanelTabNode): readonly PanelTabNode[] {
  return tabs.map((tab) => {
    if (tab.id === parentId && tab.kind === "branch") return { ...tab, children: normalizePanelTabOrder([...tab.children, node]) };
    if (tab.kind === "branch") return { ...tab, children: appendPanelTabAsChild(tab.children, parentId, node) };
    return tab;
  });
}

/** @emoji 🎯️ Where a dragged tab lands: `"insert"` places it among `parentPath`'s children at `index` (root when `parentPath` is empty); `"child"` appends it as the last child of the branch tab `parentId` (leaf targets never promote to branches in v1). */
export type PanelTabDockTarget = { readonly kind: "insert"; readonly anchor: Anchor; readonly parentPath: readonly string[]; readonly index: number } | { readonly kind: "child"; readonly anchor: Anchor; readonly parentId: string };

/** @emoji 🎯️ A completed tab drag: move `tabId` (found via {@link findPanelTabInDock}, not `fromAnchor` alone) to `target`. */
export interface PanelTabDockMove {
  readonly tabId: string;
  readonly fromAnchor: Anchor;
  readonly target: PanelTabDockTarget;
}

/**
 * 🎯️ Pure move transform: removes the dragged tab's subtree from wherever it lives and reinserts it at `target`.
 * Dropping a subtree into itself (as a child of one of its own descendants, or among the children of one) is a
 * no-operation returning the exact same {@link PanelDock} reference. Visibility is never touched here — the shell unfolds
 * a folded target anchor on drop.
 **/
export function moveTabInDock(dock: PanelDock, move: PanelTabDockMove): PanelDock {
  const located = findPanelTabInDock(dock, move.tabId);
  if (!located) return dock;
  const { anchor: fromAnchor, node } = located;
  const { target } = move;
  if (target.kind === "child" && isPanelTabInSubtree(node, target.parentId)) return dock;
  if (target.kind === "insert" && target.parentPath.some((id) => isPanelTabInSubtree(node, id))) return dock;

  const { tabs: sourceTabs, removed } = removePanelTabFromSiblings(dock.anchors[fromAnchor], move.tabId);
  if (!removed) return dock;

  const anchors: Record<Anchor, readonly PanelTabNode[]> = { ...dock.anchors, [fromAnchor]: sourceTabs };
  anchors[target.anchor] = target.kind === "child" ? appendPanelTabAsChild(anchors[target.anchor], target.parentId, removed) : insertPanelTabAtPath(anchors[target.anchor], target.parentPath, removed, target.index);
  return { anchors };
}

function replacePanelTabNode(tabs: readonly PanelTabNode[], id: string, nextNode: PanelTabNode): readonly PanelTabNode[] {
  return tabs.map((tab) => {
    if (tab.id === id) return nextNode;
    if (tab.kind === "branch") return { ...tab, children: replacePanelTabNode(tab.children, id, nextNode) };
    return tab;
  });
}

function replacePanelTabInDock(dock: PanelDock, id: string, nextNode: PanelTabNode): PanelDock {
  const located = findPanelTabInDock(dock, id);
  if (!located) return dock;
  const anchors = { ...dock.anchors };
  anchors[located.anchor] = replacePanelTabNode(dock.anchors[located.anchor], id, nextNode);
  return { anchors };
}

/** @emoji 🎯️ Where a dragged tree unit lands: `tabId`'s (a leaf's) unit list, at `index`. */
export interface PanelTreeUnitDockTarget {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly index: number;
}

/** @emoji 🎯️ A completed tree-unit drag from leaf `fromTabId` to `target`. */
export interface PanelTreeUnitDockMove {
  readonly unitId: string;
  readonly fromTabId: string;
  readonly target: PanelTreeUnitDockTarget;
}

/** @emoji 🎯️ Pure move transform for tree units — reorders within a leaf's own unit list, or moves a unit from one leaf tab's list into another's. */
export function moveTreeUnitInDock(dock: PanelDock, move: PanelTreeUnitDockMove): PanelDock {
  const from = findPanelTabInDock(dock, move.fromTabId);
  if (!from || from.node.kind !== "leaf") return dock;
  const fromNode = from.node;
  const unit = fromNode.trees.find((candidate) => candidate.id === move.unitId);
  if (!unit) return dock;

  if (move.fromTabId === move.target.tabId) {
    const withoutUnit = fromNode.trees.filter((candidate) => candidate.id !== move.unitId);
    const clampedIndex = Math.max(0, Math.min(move.target.index, withoutUnit.length));
    const nextUnits = normalizePanelTreeUnitOrder([...withoutUnit.slice(0, clampedIndex), unit, ...withoutUnit.slice(clampedIndex)]);
    if (nextUnits.length === fromNode.trees.length && nextUnits.every((candidate, index) => candidate === fromNode.trees[index])) return dock;
    return replacePanelTabInDock(dock, move.fromTabId, { ...fromNode, trees: nextUnits });
  }

  const to = findPanelTabInDock(dock, move.target.tabId);
  if (!to || to.node.kind !== "leaf") return dock;
  const toNode = to.node;
  const fromNextUnits = normalizePanelTreeUnitOrder(fromNode.trees.filter((candidate) => candidate.id !== move.unitId));
  const clampedIndex = Math.max(0, Math.min(move.target.index, toNode.trees.length));
  const toNextUnits = normalizePanelTreeUnitOrder([...toNode.trees.slice(0, clampedIndex), unit, ...toNode.trees.slice(clampedIndex)]);
  const afterSource = replacePanelTabInDock(dock, move.fromTabId, { ...fromNode, trees: fromNextUnits });
  return replacePanelTabInDock(afterSource, move.target.tabId, { ...toNode, trees: toNextUnits });
}

function normalizePanelTreeUnitOrder(units: readonly PanelTreeUnit[]): readonly PanelTreeUnit[] {
  return units.map((unit, index) => (unit.order === index ? unit : { ...unit, order: index }));
}

//#endregion 🔀️Transforms

//#region 🎯️HitTesting

/** @emoji 🎯️ A registered {@link PanelTabRow} drop surface — v1's drop surfaces are the base row of every anchor plus the rows along each anchor's current active path (branches not on the active path aren't visible, so aren't registered). */
export interface PanelTabRowDropTarget {
  readonly anchor: Anchor;
  readonly parentPath: readonly string[];
  readonly rowElement: HTMLElement;
}

function panelDockRowTabButtons(rowElement: HTMLElement, excludedIds: ReadonlySet<string>): HTMLElement[] {
  return [...rowElement.querySelectorAll<HTMLElement>("[data-tab-id]")].filter((element) => !excludedIds.has(element.dataset.tabId ?? ""));
}

/**
 * 🎯️ Resolves a tab-drag drop target from registered rows. A row hit lands on a branch button's 30–70% center
 * band as `"child"` (nest into it), elsewhere as a midpoint `"insert"`. {@link PanelTabRow} preserves declared
 * left-to-right order at every anchor, so physical and model insertion order always agree.
 **/
export function computeTabDockDropZone(pointerX: number, pointerY: number, rows: readonly PanelTabRowDropTarget[], excludedIds: ReadonlySet<string>): PanelTabDockTarget | null {
  for (const row of rows) {
    const rect = row.rowElement.getBoundingClientRect();
    if (pointerX < rect.left || pointerX > rect.right || pointerY < rect.top || pointerY > rect.bottom) continue;
    const buttons = panelDockRowTabButtons(row.rowElement, excludedIds);
    for (let index = 0; index < buttons.length; index++) {
      const button = buttons[index]!;
      const buttonRect = button.getBoundingClientRect();
      if (pointerX < buttonRect.left || pointerX > buttonRect.right) continue;
      const fraction = buttonRect.width > 0 ? (pointerX - buttonRect.left) / buttonRect.width : 0.5;
      if (button.dataset.tabKind === "branch" && fraction > 0.3 && fraction < 0.7) {
        return { kind: "child", anchor: row.anchor, parentId: button.dataset.tabId! };
      }
      return { kind: "insert", anchor: row.anchor, parentPath: row.parentPath, index: index + (fraction >= 0.5 ? 1 : 0) };
    }
    return { kind: "insert", anchor: row.anchor, parentPath: row.parentPath, index: buttons.length };
  }
  return null;
}

//#endregion 🎯️HitTesting

//#region 🪟️DragPreview

const DOCK_DRAG_CURSOR_OFFSET_X = 8;
const DOCK_DRAG_CURSOR_OFFSET_Y = 10;

/** @emoji 🪟️ Floating label following the cursor while dragging a panel tab or tree unit. */
const DockDragChip: React.FC<{ readonly label: string; readonly x: number; readonly y: number }> = ({ label, x, y }) => (
  <div
    data-slot="dock-drag-chip"
    data-level="window"
    className={cn("pointer-events-none fixed z-tutorial flex max-w-[12rem] shrink-0 items-center gap-half rounded-sm border px-single py-half text-xs text-element shadow-md select-none", surfaceClass, borderNormalClass)}
    style={{ left: x + DOCK_DRAG_CURSOR_OFFSET_X, top: y + DOCK_DRAG_CURSOR_OFFSET_Y }}
  >
    <span className="truncate">{label}</span>
  </div>
);

//#endregion 🪟️DragPreview

//#region 🌱️TreeUnitDrag

/** @emoji 🌱️ `dataTransfer` MIME identifying a tree-unit drag between leaf tabs. */
export const PANEL_TREE_UNIT_MIME = "application/x-semio-panel-tree-unit";

/** @emoji ↕️ `dataTransfer` MIME identifying a tree-section reorder drag within one {@link Tree}. */
export const TREE_SECTION_REORDER_MIME = "application/x-semio-tree-section-reorder";

export interface PanelTreeUnitDragSession {
  readonly tabId: string;
  readonly unitId: string;
  readonly label: string;
}

const activePanelTreeUnitDragSession = ephemeralBox<PanelTreeUnitDragSession | null>("framework.modules.ui.packages.typescript.targets.react.index.tsx.activePanelTreeUnitDragSession", null);
const panelTreeUnitDragListeners = ephemeralSet<() => void>("framework.modules.ui.packages.typescript.targets.react.index.tsx.panelTreeUnitDragListeners");

/** @emoji 🌱️ Records the active tree-unit drag until drop or dragend. */
export function beginPanelTreeUnitDrag(session: PanelTreeUnitDragSession): void {
  activePanelTreeUnitDragSession.current = session;
  panelGhostSessionBridge?.begin(null);
  panelTreeUnitDragListeners.forEach((listener) => listener());
}

/** @emoji 🌱️ Clears the active tree-unit drag session. */
export function endPanelTreeUnitDrag(): void {
  activePanelTreeUnitDragSession.current = null;
  panelGhostSessionBridge?.end();
  panelTreeUnitDragListeners.forEach((listener) => listener());
}

/** @emoji 🌱️ Returns the in-flight tree-unit drag, if any. */
export function readActivePanelTreeUnitDrag(): PanelTreeUnitDragSession | null {
  return activePanelTreeUnitDragSession.current;
}

/** @emoji 🌱️ True while a tree-unit drag is in flight — re-renders drop-zone consumers. */
export function usePanelTreeUnitDragActive(): boolean {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      panelTreeUnitDragListeners.add(listener);
      return () => panelTreeUnitDragListeners.delete(listener);
    },
    () => activePanelTreeUnitDragSession.current !== null,
    () => false,
  );
}

//#endregion 🌱️TreeUnitDrag

//#region 🎛️Provider

interface PanelDockDragState {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly pointerId: number;
  readonly label: string;
  readonly x: number;
  readonly y: number;
}

interface PanelDockPendingDrag {
  readonly anchor: Anchor;
  readonly tabId: string;
  readonly pointerId: number;
  readonly label: string;
  readonly startX: number;
  readonly startY: number;
}

/** @emoji 🎛️ Business-free ui↔shell contract shared by every {@link Panel} under one {@link PanelDockProvider}. */
export interface PanelDockContextValue {
  readonly dragTabId: string | null;
  readonly draggedSubtreeIds: ReadonlySet<string> | null;
  readonly dropTarget: PanelTabDockTarget | null;
  readonly startTabDrag: (anchor: Anchor, tabId: string, label: string, event: React.PointerEvent<HTMLElement>) => void;
  readonly registerTabRowDropTarget: (anchor: Anchor, parentPath: readonly string[], element: HTMLElement | null) => void;
  readonly onTreeUnitDockDrop: (move: PanelTreeUnitDockMove) => void;
}

const PanelDockContext = reactHostPort.createContext<PanelDockContextValue | null>(null);

/** @emoji 🎛️ The enclosing {@link PanelDockProvider} contract, or `null` outside one, including Layout's private mobile panel. */
export function usePanelDockContext(): PanelDockContextValue | null {
  return reactHostPort.useContext(PanelDockContext);
}

/** @emoji 🎛️ Props for {@link PanelDockProvider}. */
export interface PanelDockProviderProps {
  readonly dock: PanelDock;
  readonly onTabDockDrop: (move: PanelTabDockMove) => void;
  readonly onTreeUnitDockDrop: (move: PanelTreeUnitDockMove) => void;
  readonly children: React.ReactNode;
}

/** @emoji 🎛️ Wraps a layout's panels, wiring pointer-capture tab dragging (mirrors {@link Mode}'s window-tab drag) across all of them. Tree-unit drags are native HTML5 DnD and don't need this provider — see {@link beginPanelTreeUnitDrag}. */
export const PanelDockProvider: React.FC<PanelDockProviderProps> = ({ dock, onTabDockDrop, onTreeUnitDockDrop, children }) => {
  const panelGhost = usePanelGhost();
  const [pendingDrag, setPendingDrag] = reactHostPort.useState<PanelDockPendingDrag | null>(null);
  const [dragState, setDragState] = reactHostPort.useState<PanelDockDragState | null>(null);
  const [dropTarget, setDropTarget] = reactHostPort.useState<PanelTabDockTarget | null>(null);
  const dropTargetRef = reactHostPort.useRef<PanelTabDockTarget | null>(null);
  const rowsRef = reactHostPort.useRef(new Map<string, PanelTabRowDropTarget>());
  const excludedIdsRef = reactHostPort.useRef<ReadonlySet<string>>(new Set());
  const dockRef = reactHostPort.useRef(dock);
  dockRef.current = dock;

  const registerTabRowDropTarget = reactHostPort.useCallback((anchor: Anchor, parentPath: readonly string[], element: HTMLElement | null) => {
    const key = `${anchor}:${parentPath.join("/")}`;
    if (!element) {
      rowsRef.current.delete(key);
      return;
    }
    rowsRef.current.set(key, { anchor, parentPath, rowElement: element });
  }, []);

  const refreshDropTarget = reactHostPort.useCallback((x: number, y: number) => {
    const zone = computeTabDockDropZone(x, y, [...rowsRef.current.values()], excludedIdsRef.current);
    dropTargetRef.current = zone;
    setDropTarget(zone);
  }, []);

  const startTabDrag = reactHostPort.useCallback((anchor: Anchor, tabId: string, label: string, event: React.PointerEvent<HTMLElement>) => {
    if (event.button !== 0) return;
    setPendingDrag({ anchor, tabId, pointerId: event.pointerId, label, startX: event.clientX, startY: event.clientY });
  }, []);

  reactHostPort.useEffect(() => {
    if (!pendingDrag && !dragState) return;
    const handleMove = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (pendingDrag && !dragState) {
        const distance = Math.hypot(event.clientX - pendingDrag.startX, event.clientY - pendingDrag.startY);
        if (distance < 6) return;
        const located = findPanelTabInDock(dockRef.current, pendingDrag.tabId);
        const subtreeIds = new Set<string>();
        if (located) collectPanelTabSubtreeIds(located.node, subtreeIds);
        excludedIdsRef.current = subtreeIds;
        panelGhost?.begin(null);
        setDragState({ anchor: pendingDrag.anchor, tabId: pendingDrag.tabId, pointerId: pendingDrag.pointerId, label: pendingDrag.label, x: event.clientX, y: event.clientY });
        setPendingDrag(null);
        refreshDropTarget(event.clientX, event.clientY);
        return;
      }
      if (!dragState) return;
      setDragState((prev) => (prev ? { ...prev, x: event.clientX, y: event.clientY } : prev));
      refreshDropTarget(event.clientX, event.clientY);
    };
    const handleUp = (event: PointerEvent) => {
      const activePointerId = dragState?.pointerId ?? pendingDrag?.pointerId;
      if (activePointerId === undefined || event.pointerId !== activePointerId) return;
      if (dragState && dropTargetRef.current) {
        onTabDockDrop({ tabId: dragState.tabId, fromAnchor: dragState.anchor, target: dropTargetRef.current });
      }
      panelGhost?.end();
      setDragState(null);
      setPendingDrag(null);
      dropTargetRef.current = null;
      setDropTarget(null);
    };
    document.addEventListener("pointermove", handleMove);
    document.addEventListener("pointerup", handleUp);
    return () => {
      document.removeEventListener("pointermove", handleMove);
      document.removeEventListener("pointerup", handleUp);
    };
  }, [pendingDrag, dragState, onTabDockDrop, panelGhost, refreshDropTarget]);

  reactHostPort.useEffect(() => {
    if (!dragState) return;
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      event.preventDefault();
      panelGhost?.end();
      setDragState(null);
      setPendingDrag(null);
      dropTargetRef.current = null;
      setDropTarget(null);
    };
    document.addEventListener("keydown", handleKeyDown, true);
    return () => document.removeEventListener("keydown", handleKeyDown, true);
  }, [dragState, panelGhost]);

  const contextValue = reactHostPort.useMemo<PanelDockContextValue>(
    () => ({ dragTabId: dragState?.tabId ?? null, draggedSubtreeIds: dragState ? excludedIdsRef.current : null, dropTarget, startTabDrag, registerTabRowDropTarget, onTreeUnitDockDrop }),
    [dragState, dropTarget, startTabDrag, registerTabRowDropTarget, onTreeUnitDockDrop],
  );

  return (
    <PanelDockContext.Provider value={contextValue}>
      {children}
      {dragState ? <DockDragChip label={dragState.label} x={dragState.x} y={dragState.y} /> : null}
    </PanelDockContext.Provider>
  );
};

//#endregion 🎛️Provider

// #endregion 🧲️PanelDock

//#region 🎛️PanelChromeTabBar

/** @emoji 🎛️ Props for {@link PanelChromeTabBar}: the anchor's own tab-selection state (see {@link usePanelTabSelection}) plus which anchor it drives. */
export interface PanelChromeTabBarProps extends PanelTabSelectionOptions {
  readonly anchor: Anchor;
  readonly className?: string;
}

/**
 * 🎛️ Hosts an anchor's root tab row inline in the navbar/footer chrome while the floating panel is
 * folded — the SAME {@link PanelTabBar} mechanism (progressive reveal, drill-down memory, drag-and-drop)
 * as a panel-hosted bar, just placed elsewhere. When the panel is open (`visible`), the floating
 * {@link Panel}'s {@link WindowChrome} owns the tab chips; this host keeps a width-matched placeholder so
 * trailing navbar controls (fullscreen) do not reflow. Renders nothing at rest once `tabs` is empty; becomes
 * a drop target only while a dock drag is in flight, mirroring {@link PanelEmptyDockZone}'s empty-anchor
 * behavior. Pair with `Panel`'s `tabBarHost="chrome"` for the matching anchor, and pass the SAME controlled
 * selection props to both — chrome hosting requires controlled state so the two hosts never fork.
 **/
export const PanelChromeTabBar: React.FC<PanelChromeTabBarProps> = ({ anchor, className = "", ...selection }) => {
  const dock = usePanelDockContext();
  const { resolvedPath, handlePathChange } = usePanelTabSelection(selection);
  const { tabs, visible } = selection;
  const shellRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [parkedWidth, setParkedWidth] = reactHostPort.useState(0);

  reactHostPort.useLayoutEffect(() => {
    if (visible || tabs.length === 0) return;
    const shell = shellRef.current;
    if (!shell) return;
    const width = shell.getBoundingClientRect().width;
    if (width > 0) setParkedWidth(width);
  }, [visible, tabs, resolvedPath]);

  if (visible) {
    if (tabs.length === 0) return null;
    return (
      <LevelProvider level="panel">
        <GhostRegionShell
          ref={shellRef}
          sessionGhost={false}
          data-level="panel"
          data-slot="panel-chrome-tab-bar"
          data-anchor={anchor}
          data-panel-chrome-tab-bar-placeholder="true"
          aria-hidden="true"
          className={cn("pointer-events-none flex shrink-0 items-center", className)}
          style={parkedWidth > 0 ? { width: parkedWidth, minWidth: parkedWidth } : undefined}
        />
      </LevelProvider>
    );
  }

  if (tabs.length === 0) {
    if (!dock?.dragTabId) return null;
    return (
      <LevelProvider level="panel">
        <GhostRegionShell ref={shellRef} sessionGhost={false} data-level="panel" data-slot="panel-chrome-tab-bar" data-anchor={anchor} className={cn("flex shrink-0 items-center", className)}>
          <PanelEmptyDockZone anchor={anchor} />
        </GhostRegionShell>
      </LevelProvider>
    );
  }

  return (
    <LevelProvider level="panel">
      <GhostRegionShell ref={shellRef} sessionGhost={false} data-level="panel" data-slot="panel-chrome-tab-bar" data-anchor={anchor} className={cn("flex shrink-0 items-center", className)}>
        <WindowChrome
          chipOnly
          level="panel"
          stackSlot="window-chrome-stack"
          titleChips={<PanelTabBar variant="panel" anchor={anchor} tabs={tabs} activePath={resolvedPath} onActivePathChange={handlePathChange} maxRows={1} direction={flowFromAnchor(anchor).block} showActiveColor={visible} />}
        />
      </GhostRegionShell>
    </LevelProvider>
  );
};

//#endregion 🎛️PanelChromeTabBar

/** @emoji 📑️ Mobile panel tab strip height. */
export const mobilePanelTabBarClass = cn(panelTabBarClass, "h-large");

/** @emoji 📑️ Mobile panel tab button padding. */
export const mobilePanelTabButtonClass = cn(panelTabButtonClass, "px-single");

/** @emoji ↔ Accent stroke on the panel resize edge while hovered or dragging. */
export function panelResizeEdgeAccentClass(resizeSide: "left" | "right", active: boolean): string | undefined {
  if (!active) return undefined;
  switch (resizeSide) {
    case "left":
      return "border-l-accent";
    case "right":
      return "border-r-accent";
  }
}

/** @emoji 🪟️ All border effects the silhouette SVG can paint. */
export const WINDOW_SILHOUETTE_BORDER_KINDS = ["celebrated", "introduced", "loading", "waiting", "active", "normal"] as const;

/** @emoji 🪟️ Which border effect the dock-stack silhouette overlay should paint. */
export type WindowSilhouetteBorderKind = (typeof WINDOW_SILHOUETTE_BORDER_KINDS)[number];

/** @emoji 🪟️ Whether an introduced stamp is the window chrome body itself (kind/instance scroll surface or
 * `[data-slot="window"]`), not a nested utility/action/tree row inside the pane. Window silhouette pulse
 * and the stack SVG border must follow only these stamps — introducing `transform` must pulse the utility
 * toggle, not the enclosing Top/Perspective silhouette. */
export function isWindowChromeIntroducedTarget(el: Element): boolean {
  if (el.getAttribute("data-slot") === "window") return true;
  const ids = [el.getAttribute("id") ?? "", ...(el.getAttribute("data-element-alias") ?? "").split(/\s+/)].filter(Boolean);
  for (const id of ids) {
    if (!id.startsWith("framework.window.")) continue;
    const rest = id.slice("framework.window.".length);
    if (!rest.includes(".")) return true;
  }
  return false;
}

/** @emoji 🪟️ Resolves silhouette border kind from the active window + stack active flag.
 * Introduction stamps `data-introduced` on the window kind id target — often the inner scroll surface
 * (`framework.window.{kind}`), not `[data-slot="window"]` itself — so window-chrome descendants count.
 * Nested introduce targets (utilities, actions) must not promote the window silhouette. `celebrated`
 * (from `celebrateElements()`) is checked FIRST: it follows an introduced stamp being cleared on the
 * same target, and completion feedback must win during any overlap. */
export function resolveWindowSilhouetteBorderKind(windowEl: Element | null, stackActive = false): WindowSilhouetteBorderKind {
  if (windowEl?.getAttribute("data-celebrated") === "true") return "celebrated";
  if (windowEl) {
    for (const el of windowEl.querySelectorAll('[data-celebrated="true"]')) {
      if (isWindowChromeIntroducedTarget(el)) return "celebrated";
    }
  }
  if (windowEl?.getAttribute("data-introduced") === "true") return "introduced";
  if (windowEl) {
    for (const el of windowEl.querySelectorAll('[data-introduced="true"]')) {
      if (isWindowChromeIntroducedTarget(el)) return "introduced";
    }
  }
  const className = windowEl && typeof windowEl.className === "string" ? windowEl.className : "";
  if (/(?:^|\s)border-loading(?:-active|-element)?(?:\s|$)/.test(className)) return "loading";
  if (/(?:^|\s)border-waiting(?:-active|-element)?(?:\s|$)/.test(className)) return "waiting";
  return stackActive ? "active" : "normal";
}

/** @emoji 🪟️ Maps a silhouette border kind to stroke classes and color tokens. */
export function windowSilhouetteBorderPaint(kind: WindowSilhouetteBorderKind): { readonly className: string; readonly stroke: string } {
  switch (kind) {
    case "celebrated":
      return { className: "window-silhouette-border window-silhouette-border-celebrated-mask", stroke: "white" };
    case "introduced":
      return { className: "window-silhouette-border window-silhouette-border-introduced", stroke: "var(--introduced-border-color, var(--color-secondary))" };
    case "loading":
      return { className: "window-silhouette-border window-silhouette-border-loading", stroke: "var(--loading-border-color, var(--border-normal-color))" };
    case "waiting":
      return { className: "window-silhouette-border window-silhouette-border-waiting", stroke: "var(--waiting-border-color, var(--border-normal-color))" };
    case "active":
      return { className: "window-silhouette-border window-silhouette-border-active", stroke: "var(--active-base)" };
    case "normal":
      return { className: "window-silhouette-border window-silhouette-border-normal", stroke: "var(--border-normal-color)" };
  }
}

const WINDOW_CHROME_GAP_SELECTOR = '[data-slot="window-chrome-gap"], [data-slot="mode-dock-tab-gap"]';
const WINDOW_CHROME_CAP_SELECTOR = '[data-slot="window-chrome-cap"], [data-slot="mode-dock-tabbar"]';

/** @emoji 🪟️ Whether a stack-local rect sits on the given silhouette dock edge. */
function windowSilhouetteRectOnDock(stackRect: DOMRect, rect: DOMRect, dock: "top" | "bottom"): boolean {
  return dock === "top" ? rect.top - stackRect.top <= WINDOW_SILHOUETTE_CHIP_EPSILON : stackRect.bottom - rect.bottom <= WINDOW_SILHOUETTE_CHIP_EPSILON;
}

/** @emoji 🪟️ Whether `element` belongs to `stack`'s own chrome — nested pane/panel `[data-window-silhouette]` hosts (e.g. projection) keep their chips out of the enclosing window outline so the window bottom stays rectangular while those panes overlay like window options. */
function windowSilhouetteOwnsElement(stack: HTMLElement, element: Element): boolean {
  const owner = element.closest("[data-window-silhouette]");
  return owner === null || owner === stack;
}

/** @emoji 🪟️ Reads live silhouette metrics from painted chip spans grouped by `data-dock` (works for RTL caps and bottom-docked panels). Nested silhouette chips are ignored — see {@link windowSilhouetteOwnsElement}. */
export function measureWindowSilhouetteMetrics(stack: HTMLElement): WindowSilhouetteMetrics | null {
  const stackRect = stack.getBoundingClientRect();
  const width = stackRect.width;
  const height = stackRect.height;
  if (width <= 0 || height <= 0) return null;
  const measureEdge = (dock: "top" | "bottom"): WindowSilhouetteEdge => {
    const chips: WindowSilhouetteChip[] = [];
    let depth = 0;
    for (const chip of stack.querySelectorAll<HTMLElement>(`[data-window-silhouette-chip][data-dock="${dock}"]`)) {
      if (!windowSilhouetteOwnsElement(stack, chip)) continue;
      const rect = chip.getBoundingClientRect();
      if (rect.width <= WINDOW_SILHOUETTE_CHIP_EPSILON || rect.height <= WINDOW_SILHOUETTE_CHIP_EPSILON) continue;
      chips.push({ left: rect.left - stackRect.left, right: rect.right - stackRect.left });
      depth = Math.max(depth, rect.height);
    }
    for (const gap of stack.querySelectorAll<HTMLElement>(WINDOW_CHROME_GAP_SELECTOR)) {
      if (!windowSilhouetteOwnsElement(stack, gap)) continue;
      const gapRect = gap.getBoundingClientRect();
      if (gapRect.height > WINDOW_SILHOUETTE_CHIP_EPSILON && windowSilhouetteRectOnDock(stackRect, gapRect, dock)) depth = Math.max(depth, gapRect.height);
    }
    for (const cap of stack.querySelectorAll<HTMLElement>(WINDOW_CHROME_CAP_SELECTOR)) {
      if (!windowSilhouetteOwnsElement(stack, cap)) continue;
      const capRect = cap.getBoundingClientRect();
      if (capRect.height > WINDOW_SILHOUETTE_CHIP_EPSILON && windowSilhouetteRectOnDock(stackRect, capRect, dock)) depth = Math.max(depth, capRect.height);
    }
    return { depth, chips: normalizeWindowSilhouetteChips(chips, 0, width) };
  };
  return { width, height, top: measureEdge("top"), bottom: measureEdge("bottom") };
}

/** @emoji 📐️ Coalesced owned-chip measurement shared by silhouette content, glass, border, and hit clipping. */
export function useWindowSilhouetteGeometry(stack: HTMLElement | null, enabled = true): WindowSilhouetteGeometry {
  const [geometry, setGeometry] = reactHostPort.useState<WindowSilhouetteGeometry>(() => createWindowSilhouetteGeometry(null));
  reactHostPort.useLayoutEffect(() => {
    if (!stack || !enabled) return;
    let frame = 0;
    const commit = () => {
      frame = 0;
      const next = createWindowSilhouetteGeometry(measureWindowSilhouetteMetrics(stack));
      setGeometry((previous) =>
        previous.state === next.state && previous.contentClipPath === next.contentClipPath && previous.borderPath === next.borderPath && previous.metrics.width === next.metrics.width && previous.metrics.height === next.metrics.height
          ? previous
          : next,
      );
    };
    const schedule = () => {
      if (frame) return;
      if (typeof requestAnimationFrame === "function") frame = requestAnimationFrame(commit);
      else commit();
    };
    const targetSelector = '[data-window-silhouette-chip], [data-slot="window-chrome-cap"], [data-slot="mode-dock-tabbar"]';
    const resizeObserver = typeof ResizeObserver === "undefined" ? null : new ResizeObserver(schedule);
    const refreshResizeTargets = () => {
      resizeObserver?.disconnect();
      resizeObserver?.observe(stack);
      for (const element of stack.querySelectorAll<HTMLElement>(targetSelector)) {
        if (windowSilhouetteOwnsElement(stack, element)) resizeObserver?.observe(element);
      }
    };
    const containsGeometryTarget = (node: globalThis.Node): boolean => node instanceof Element && (node.matches(targetSelector) || node.querySelector(targetSelector) !== null);
    const mutationObserver =
      typeof MutationObserver === "undefined"
        ? null
        : new MutationObserver((records) => {
            const changed = records.some((record) =>
              record.type === "attributes"
                ? (record.target === stack && record.attributeName === "data-silhouette-remeasure") || (record.target instanceof Element && (record.target.matches(targetSelector) || record.target.closest(targetSelector) !== null))
                : [...record.addedNodes, ...record.removedNodes].some(containsGeometryTarget),
            );
            if (!changed) return;
            refreshResizeTargets();
            schedule();
          });
    commit();
    refreshResizeTargets();
    mutationObserver?.observe(stack, { attributes: true, attributeFilter: ["data-dock", "data-silhouette-remeasure", "data-slot", "data-window-silhouette-chip"], childList: true, subtree: true });
    return () => {
      if (frame && typeof cancelAnimationFrame === "function") cancelAnimationFrame(frame);
      resizeObserver?.disconnect();
      mutationObserver?.disconnect();
    };
  }, [enabled, stack]);
  return geometry;
}

/** @emoji 📏️ Tab/gap/controls cells stay transparent; glass lives on chip (+ controls) cells only so the U-gap punches through to the base floor. Borders owned by {@link ModeDockStackSilhouetteBorder}. */
export const windowCapFrameClass = "relative z-[2] border-0 bg-transparent";

/** @emoji 🪟️ Gap cutout stays clear — never glass — so the base/canvas floor shows through the U-notch. */
export const windowGapFrameClass = "border-0 bg-transparent";

/** @emoji 📏️ Body fill only — outer stroke is the stack silhouette SVG (tabs + cutout + controls + body); base level (mode body / floor windows float on) — host element must also carry `data-level="base"`. */
export const windowBodyFrameClass = cn("relative border-0", surfaceClass);

/** @emoji 📐️ Grid tracks for multi-tab active chrome: one column per tab, then flex gap, then controls. */
export interface ModeDockChromeGrid {
  readonly templateColumns: string;
  readonly activeCol: number;
  readonly gapCol: number;
  readonly controlsCol: number;
  readonly bodyColumnSpan: string;
  readonly activeTabIndex: number;
  readonly tabCol: (tabIndex: number) => number;
}

/** @emoji 📐️ Computes {@link ModeDockChromeGrid} column indices for a tab stack. */
export function modeDockChromeGridPlacement(tabs: readonly { id: string; title: string }[], activeId: string | undefined): ModeDockChromeGrid {
  const activeTabIndex = Math.max(
    0,
    tabs.findIndex((tab) => tab.id === activeId),
  );
  const gapCol = tabs.length + 1;
  const controlsCol = tabs.length + 2;
  const activeCol = activeTabIndex + 1;
  const templateParts = [...tabs.map(() => "max-content"), "minmax(0, 1fr)", "max-content"];
  return {
    templateColumns: templateParts.join(" "),
    activeCol,
    gapCol,
    controlsCol,
    bodyColumnSpan: `${activeCol} / ${gapCol + 1}`,
    activeTabIndex,
    tabCol: (tabIndex) => tabIndex + 1,
  };
}

/** @emoji 📏️ Inactive sibling tab — normal pill resting on the U-frame baseline; transparent so it
 * shows the chip-cell glass beneath it rather than a second opaque fill (matches {@link panelWindowInactiveTabClass}'s rule for the panel variant). */
export const modeDockInactiveTabClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** @emoji 📏️ Inactive tab before gap — inner divider only; outer stroke owned by the silhouette SVG. */
export const modeDockInactiveTabBeforeGapClass = cn(`relative z-30 box-border min-h-medium shrink-0`, "bg-transparent");

/** @emoji 📑️ Inactive panel tab inside {@link WindowChrome} — no opaque fill; the chip-cap glass is the panel boundary and must meet the body without a seam. */
export const panelWindowInactiveTabClass = "relative z-30 box-border min-h-medium h-full shrink-0 bg-transparent";

/** @emoji 🪟️ Icon + title cluster inside a mode-dock tab — standard gap between glyph and label. */
export const modeDockTabLabelClassName = "flex min-w-0 flex-1 items-center gap-single overflow-hidden";

/** @emoji 🪟️ Default mode-dock tab label — element gray; emphasize on hover/active only. */

/** @emoji 🪧️ Static shell title (navbar app label, pane headings) — element gray at rest. */
export const shellChromeTitleClassName = "truncate text-sm font-medium text-element";

/** @emoji 🪧️ Uppercase shell section title — element gray at rest. */
export const shellChromeSectionTitleClassName = "text-2xs font-semibold uppercase tracking-wide text-element";

/** @emoji 📏️ Globally active dock tab — primary fill + emphasized label. */
export const modeDockActiveTabFillClass = interactiveActiveFillClass;

/** @emoji 📏️ Stack-active tab fill — outline owned by the stack silhouette SVG; `border-0` must win over {@link interactiveActiveFillClass}'s border color utility. */
export const modeDockActiveTabClass = cn("relative z-20 box-border min-h-medium shrink-0 border-0", modeDockActiveTabFillClass);

/** @emoji 📏️ Maximize/controls glass cell — host stamps {@link glassClass}; fill must not span the U-gap. */
export const windowControlsCapClass = "pointer-events-auto relative z-[2] flex shrink-0 items-stretch border-0 bg-transparent text-element";

/** @emoji 📏️ Multi-tab controls cap — chip glass only; U-gap stays a clear punch-through. */
export const windowControlsCapActiveSplitClass = "relative flex shrink-0 items-stretch border-0 bg-transparent text-element";

//#region 🪟️WindowChrome

/** @emoji 🪟️ Optional right-cap control on {@link WindowChrome} (enlarge / close). */
export interface WindowChromeControlAction {
  readonly id: string;
  readonly slot: string;
  readonly icon: React.ReactNode;
  readonly label: string;
  readonly onClick: () => void;
}

/** @emoji 🪟️ Title chip in the window-chrome cap row (name + optional drag) — transparent and
 * borderless so the painted chip-cap cell shows through and the silhouette remains the sole outline. */
export const windowChromeTitleChipClass = cn(modeDockTabClassName, "relative z-30 box-border min-h-medium shrink-0 border-0 bg-transparent");

export interface WindowChromeProps {
  readonly active?: boolean;
  readonly chipOnly?: boolean;
  readonly className?: string;
  readonly stackClassName?: string;
  readonly bodyClassName?: string;
  readonly bodySurfaceClassName?: string;
  readonly bodySurfaceLevel?: Level;
  /** 🎈️ Stamps `data-level={level}` on the chrome stack and wraps its content in a {@link LevelProvider}; cap/controls/body all render {@link glassClass} so one level is one appearance. */
  readonly level?: Level;
  readonly style?: React.CSSProperties;
  readonly stackRef?: React.Ref<HTMLDivElement>;
  readonly capRef?: React.Ref<HTMLDivElement>;
  readonly bodyRef?: React.Ref<HTMLDivElement>;
  readonly stackSlot?: string;
  readonly bodySlot?: string;
  readonly bodyStyle?: React.CSSProperties;
  readonly titleChips?: React.ReactNode;
  /** @emoji 🧭️ Optional top-right chip content rendered ahead of enlarge/close in the controls cell. */
  readonly capRightChips?: React.ReactNode;
  readonly body?: React.ReactNode;
  readonly enlarge?: WindowChromeControlAction;
  readonly close?: WindowChromeControlAction;
  readonly gapProps?: React.HTMLAttributes<HTMLDivElement>;
  readonly footerLeftChips?: React.ReactNode;
  readonly footerCenterChips?: React.ReactNode;
  readonly footerRightChips?: React.ReactNode;
  readonly footerGapProps?: React.HTMLAttributes<HTMLDivElement>;
  readonly footerRef?: React.Ref<HTMLDivElement>;
  readonly introduceTarget?: Element | null;
  /** 🎓️ Force silhouette border kind (e.g. introduction steps pulse like `data-introduced` until activated). */
  readonly borderKind?: WindowSilhouetteBorderKind;
  readonly stackBindProps?: SurfaceActiveBindProps;
  readonly stackDataAttrs?: Record<string, string | undefined>;
  /** @emoji 🧭️ Which silhouette edge the cap row docks to — `"bottom"` for panels that grow upward from a bottom anchor. */
  readonly capDock?: "top" | "bottom";
  /** @emoji ↔ Inline layout overrides for the cap row (e.g. chrome-hosted trailing navbar reserve). */
  readonly capRowStyle?: React.CSSProperties;
  readonly capSlot?: string;
  readonly chipSlot?: string;
  readonly controlsSlot?: string;
  readonly silhouetteSlot?: string;
}

/** @emoji 🪟️ SVG overlay that paints the U-cutout silhouette for any window-chrome stack. */
export const WindowChromeSilhouetteBorder: React.FC<{
  readonly stack: HTMLElement | null;
  readonly geometry?: WindowSilhouetteGeometry;
  readonly active?: boolean;
  readonly introduceTarget?: Element | null;
  readonly borderKind?: WindowSilhouetteBorderKind;
  readonly silhouetteSlot?: string;
}> = ({ stack, geometry, active = false, introduceTarget, borderKind, silhouetteSlot = "window-chrome-silhouette-border" }) => {
  const [epoch, setEpoch] = reactHostPort.useState(0);
  const celebrateMaskId = `window-silhouette-celebrate-${reactHostPort.useId().replace(/:/g, "")}`;
  const observedGeometry = useWindowSilhouetteGeometry(stack, geometry === undefined);
  const resolvedGeometry = geometry ?? observedGeometry;

  reactHostPort.useLayoutEffect(() => {
    if (!stack) return;
    const bump = () => setEpoch((value) => value + 1);
    bump();
    const mutationObserver = new MutationObserver(bump);
    mutationObserver.observe(stack, { attributes: true, attributeFilter: ["class", "data-celebrated", "data-introduced"], subtree: true });
    return () => {
      mutationObserver.disconnect();
    };
  }, [stack]);

  const windowEl = stack?.querySelector('[data-slot="window"]') ?? introduceTarget ?? stack;
  const kind = resolveWindowSilhouetteBorderKind(windowEl, active);
  const resolvedKind =
    stack && [...stack.querySelectorAll('[data-celebrated="true"]')].some(isWindowChromeIntroducedTarget)
      ? "celebrated"
      : borderKind
        ? borderKind
        : stack && [...stack.querySelectorAll('[data-introduced="true"]')].some(isWindowChromeIntroducedTarget)
          ? "introduced"
          : kind;
  const metrics = resolvedGeometry.metrics;
  void epoch;

  if (resolvedGeometry.state === "pending") {
    return <div data-slot={silhouetteSlot} data-window-silhouette-border data-kind={resolvedKind} data-pending="" data-dim="" aria-hidden className="pointer-events-none absolute inset-0 z-[40] overflow-visible" />;
  }
  const path = resolvedGeometry.borderPath;
  const paint = windowSilhouetteBorderPaint(resolvedKind);
  if (resolvedKind === "celebrated") {
    return (
      <svg
        data-slot={silhouetteSlot}
        data-window-silhouette-border
        data-kind={resolvedKind}
        data-dim=""
        className="pointer-events-none absolute inset-0 z-[40] overflow-visible"
        width={metrics.width}
        height={metrics.height}
        viewBox={`0 0 ${metrics.width} ${metrics.height}`}
        aria-hidden
      >
        <defs>
          <mask id={celebrateMaskId} maskUnits="userSpaceOnUse" x={0} y={0} width={metrics.width} height={metrics.height}>
            <rect x={0} y={0} width={metrics.width} height={metrics.height} fill="black" />
            <path d={path} fill="none" stroke={paint.stroke} strokeLinejoin="miter" vectorEffect="non-scaling-stroke" className={paint.className} />
          </mask>
        </defs>
        <foreignObject x={0} y={0} width={metrics.width} height={metrics.height} mask={`url(#${celebrateMaskId})`}>
          <div className="window-silhouette-border-celebrated-fill" style={{ width: "100%", height: "100%" }} />
        </foreignObject>
      </svg>
    );
  }
  return (
    <svg
      data-slot={silhouetteSlot}
      data-window-silhouette-border
      data-kind={resolvedKind}
      data-dim=""
      className="pointer-events-none absolute inset-0 z-[40] overflow-visible"
      width={metrics.width}
      height={metrics.height}
      viewBox={`0 0 ${metrics.width} ${metrics.height}`}
      aria-hidden
    >
      <path d={path} fill="none" stroke={paint.stroke} strokeLinejoin="miter" vectorEffect="non-scaling-stroke" className={paint.className} />
    </svg>
  );
};

/** @emoji 🪟️ Shared U-cutout window chrome: left title chip(s), open gap, optional enlarge/close, continuous body border.
 * Cap glass lives only on the chip (+ controls) cells — never the full cap row — so the U-gap stays transparent
 * and shows whatever sits behind the stack (veil, canvas, page). Do not paint an absolute inset fill. */
export const WindowChrome = reactHostPort.forwardRef<HTMLDivElement, WindowChromeProps>(
  (
    {
      active = false,
      chipOnly = false,
      className,
      stackClassName,
      bodyClassName,
      bodySurfaceClassName,
      bodySurfaceLevel,
      level,
      style,
      stackRef,
      capRef,
      bodyRef,
      stackSlot = "window-chrome-stack",
      bodySlot = "window-chrome-body",
      bodyStyle,
      titleChips,
      capRightChips,
      body,
      enlarge,
      close,
      gapProps,
      footerLeftChips,
      footerCenterChips,
      footerRightChips,
      footerGapProps,
      footerRef,
      introduceTarget,
      borderKind,
      stackBindProps,
      stackDataAttrs,
      capDock = "top",
      capRowStyle,
      capSlot = "window-chrome-cap",
      chipSlot = "window-chrome-chip-cap",
      controlsSlot = "window-chrome-controls",
      silhouetteSlot = "window-chrome-silhouette-border",
    },
    ref,
  ) => {
    const [stackEl, setStackEl] = reactHostPort.useState<HTMLDivElement | null>(null);
    const setStackRef = reactHostPort.useCallback(
      (element: HTMLDivElement | null) => {
        setStackEl(element);
        if (typeof ref === "function") ref(element);
        else if (ref) (ref as React.MutableRefObject<HTMLDivElement | null>).current = element;
        if (typeof stackRef === "function") stackRef(element);
        else if (stackRef) (stackRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
      },
      [ref, stackRef],
    );

    const chipSurfaceClass = cn(windowCapFrameClass, glassClass);
    const bodySurfaceClass = cn("pointer-events-none absolute inset-x-0 z-0 border-0", bodySurfaceClassName ?? glassClass);
    const bodyContentClass = "window-silhouette-content-plane relative border-0";
    const controlsSurfaceClass = cn(windowControlsCapClass, glassClass);
    const geometry = useWindowSilhouetteGeometry(stackEl);
    const silhouetteVars = {
      "--window-silhouette-top-clearance": `${geometry.safeClearances.top}px`,
      "--window-silhouette-bottom-clearance": `${geometry.safeClearances.bottom}px`,
    } as React.CSSProperties;
    const contentStyle = {
      ...bodyStyle,
      clipPath: geometry.contentClipPath,
      WebkitClipPath: geometry.contentClipPath,
    } as React.CSSProperties;
    // 🪟️ The stack element itself stamps `data-level` (below), so this only needs to open the
    // SurfaceScope (fill="glass" — every cell above already renders it) for descendants to see via useSurface().
    const wrapLevel = (node: React.ReactNode): React.ReactNode =>
      level ? (
        <SurfaceScope level={level} fill="glass">
          {node}
        </SurfaceScope>
      ) : (
        node
      );

    if (chipOnly) {
      return wrapLevel(
        <div ref={setStackRef} data-slot={stackSlot} data-window-silhouette data-level={level} className={cn("relative inline-flex min-w-0 bg-transparent", className, stackClassName)} style={style} {...stackDataAttrs}>
          <WindowChromeSilhouetteBorder stack={stackEl} geometry={geometry} active={active} borderKind={borderKind} silhouetteSlot={silhouetteSlot} />
          {titleChips ? (
            <div data-slot={chipSlot} data-window-silhouette-chip data-dock={capDock} data-ui-reveal-region="window-cap" data-dim className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {titleChips}
            </div>
          ) : null}
        </div>,
      );
    }

    const { className: gapClassName, ...gapRest } = gapProps ?? {};
    const { className: footerGapClassName, ...footerGapRest } = footerGapProps ?? {};
    const hasFooter = Boolean(footerLeftChips || footerCenterChips || footerRightChips);
    const hasFooterGap = Boolean(footerLeftChips && footerRightChips && !footerCenterChips);
    const footerGapClass = cn("pointer-events-none relative min-h-0 min-w-0 bg-transparent", windowGapFrameClass, footerGapClassName);
    const footerChipClass = cn("relative flex min-h-medium min-w-0 shrink-0 items-stretch", chipSurfaceClass);
    return wrapLevel(
      <div
        ref={setStackRef}
        data-slot={stackSlot}
        data-window-silhouette
        data-level={level}
        data-active={active ? "true" : undefined}
        className={cn("relative flex min-h-0 min-w-0 flex-col overflow-visible bg-transparent text-foreground", capDock === "bottom" && "flex-col-reverse", stackClassName, className)}
        style={{ ...style, ...silhouetteVars }}
        {...stackBindProps}
        {...stackDataAttrs}
      >
        <WindowChromeSilhouetteBorder stack={stackEl} geometry={geometry} active={active} introduceTarget={introduceTarget} borderKind={borderKind} silhouetteSlot={silhouetteSlot} />
        <div ref={capRef} data-slot={capSlot} data-ui-reveal-region="window-cap" data-dim className="relative z-[2] flex w-full min-w-0 shrink-0 items-stretch bg-transparent" style={capRowStyle}>
          {titleChips ? (
            <div data-slot={chipSlot} data-window-silhouette-chip data-dock={capDock} className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
              {titleChips}
            </div>
          ) : null}
          <div data-slot="window-chrome-gap" data-window-silhouette-gap aria-hidden {...gapRest} className={cn("pointer-events-none relative min-h-medium min-w-0 flex-1 bg-transparent", windowGapFrameClass, gapClassName)} />
          {capRightChips || enlarge || close ? (
            <div data-slot={controlsSlot} data-window-silhouette-chip data-dock={capDock} className={cn("relative z-[2] flex shrink-0 items-stretch", controlsSurfaceClass)}>
              {capRightChips}
              {enlarge ? (
                <button
                  type="button"
                  id={enlarge.id}
                  data-slot={enlarge.slot}
                  className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
                  onClick={enlarge.onClick}
                >
                  {enlarge.icon}
                  <span className="text-tiny whitespace-nowrap">{enlarge.label}</span>
                </button>
              ) : null}
              {close ? (
                <button
                  type="button"
                  id={close.id}
                  data-slot={close.slot}
                  className={cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent transition-colors px-single gap-single text-element", interactiveHoverClass)}
                  onClick={close.onClick}
                >
                  {close.icon}
                  <span className="text-tiny whitespace-nowrap">{close.label}</span>
                </button>
              ) : null}
            </div>
          ) : null}
        </div>
        {geometry.bodyRegion ? (
          <div
            data-slot="window-chrome-body-surface"
            data-level={bodySurfaceLevel ?? level}
            aria-hidden
            className={bodySurfaceClass}
            style={{ top: geometry.bodyRegion.y, bottom: geometry.metrics.height - geometry.bodyRegion.y - geometry.bodyRegion.height }}
          />
        ) : null}
        <div
          ref={bodyRef}
          data-slot={bodySlot}
          data-level={bodySurfaceLevel ?? level}
          data-window-silhouette-content
          data-silhouette-state={geometry.state}
          data-dim
          className={cn("z-[1] min-h-0 flex-1", bodyContentClass, bodyClassName)}
          style={contentStyle}
        >
          {body}
        </div>
        {hasFooter ? (
          footerCenterChips ? (
            <div ref={footerRef} data-slot="window-chrome-footer" data-dim className="relative z-[2] grid w-full min-w-0 shrink-0 grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] items-end bg-transparent">
              <div data-slot="window-chrome-footer-gap-left" data-window-silhouette-gap {...footerGapRest} aria-hidden={footerLeftChips ? undefined : true} className={cn(footerGapClass, "justify-self-start")}>
                {footerLeftChips ? (
                  <div data-slot="window-chrome-footer-left" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                    {footerLeftChips}
                  </div>
                ) : null}
              </div>
              <div data-slot="window-chrome-footer-center" className={cn(footerGapClass, "flex justify-center justify-self-center")}>
                <div data-slot="window-chrome-footer-center-chip" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                  {footerCenterChips}
                </div>
              </div>
              <div data-slot="window-chrome-footer-gap-right" data-window-silhouette-gap aria-hidden={footerRightChips ? undefined : true} className={cn(footerGapClass, "justify-self-end")}>
                {footerRightChips ? (
                  <div data-slot="window-chrome-footer-right" data-window-silhouette-chip data-dock="bottom" className={cn("pointer-events-auto", footerChipClass)}>
                    {footerRightChips}
                  </div>
                ) : null}
              </div>
            </div>
          ) : (
            <div
              ref={footerRef}
              data-slot="window-chrome-footer"
              data-dim
              className={cn("relative z-[2] flex w-full min-w-0 shrink-0 items-stretch bg-transparent", footerLeftChips && !footerRightChips && "justify-start", footerRightChips && !footerLeftChips && "justify-end")}
            >
              {footerLeftChips ? (
                <div data-slot="window-chrome-footer-left" data-window-silhouette-chip data-dock="bottom" className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
                  {footerLeftChips}
                </div>
              ) : null}
              {hasFooterGap ? <div data-slot="window-chrome-footer-gap" data-window-silhouette-gap aria-hidden {...footerGapRest} className={cn(footerGapClass, "flex-1")} /> : null}
              {footerRightChips ? (
                <div data-slot="window-chrome-footer-right" data-window-silhouette-chip data-dock="bottom" className={cn("relative flex min-h-medium min-w-0 shrink items-stretch", chipSurfaceClass)}>
                  {footerRightChips}
                </div>
              ) : null}
            </div>
          )
        ) : null}
      </div>,
    );
  },
);
WindowChrome.displayName = "WindowChrome";

/** @emoji 🪟️ Context menu with U-cutout chrome — title chip only, no enlarge/close; gap punches through. Forwards its ref to the outer window-chrome stack so callers (e.g. {@link ContextMenuController}'s on-screen clamp) can measure/adjust the rendered surface. */
export const ContextMenuChrome = reactHostPort.forwardRef<HTMLDivElement, { readonly title: string; readonly icon: IconSource; readonly children: React.ReactNode; readonly className?: string; readonly style?: React.CSSProperties }>(
  ({ title, icon, children, className, style }, ref) => {
    return (
      <WindowChrome
        ref={ref}
        active={true}
        level="menu"
        stackSlot="context-menu-content"
        className={cn("z-menu w-auto min-w-[10rem] max-h-layout-command overflow-y-auto", className)}
        style={style}
        titleChips={
          <div data-slot="context-menu-title-chip" className={cn(windowChromeTitleChipClass, "flex min-w-0 items-center gap-single")}>
            <Icon icon={icon} size="small" className="shrink-0" />
            <span className="truncate">{title}</span>
          </div>
        }
        body={children}
        bodyClassName="pointer-events-auto p-single"
      />
    );
  },
);
ContextMenuChrome.displayName = "ContextMenuChrome";

//#endregion 🪟️WindowChrome

/** @emoji 🪟️ Window chrome icon button — element gray by default, emphasize on hover. */
export const windowChromeControlButtonClass = cn("flex size-medium items-center justify-center border-0 bg-transparent transition-colors", interactiveHoverClass);

/** @emoji 📐️ Default unfolded width of window panes and the options rail (token-derived; matches panel default 300px). */
export const windowMeasuresDefaultWidthPx = domSizePx("layoutPanelRailUiSpacing");

/** @emoji 📐️ Minimum unfolded width of the window options rail (token-derived). */
export const windowMeasuresMinWidthPx = domSizePx("layoutPanelMinUiSpacing");

/** @emoji 📐️ Maximum unfolded width of the window options rail (token-derived). */
export const windowMeasuresMaxWidthPx = domSizePx("layoutPanelMaxUiSpacing");

/** @emoji 📐️ Max width cap for window engagement (token-derived). */
export const windowEngagementMaxWidthPx = domSizePx("layoutEngagementMaxUiSpacing");

/** @emoji 📐️ Merged top-left Actions body beside the engagement chrome toggle: the active engagement's status/control (when present) stacked above the categorized ad-hoc actions tree; scrolls once content exceeds the window body. */
export const windowEngagementBodyClass = "flex min-h-medium min-w-0 max-h-full flex-auto flex-col gap-half overflow-y-auto px-single";

/** @emoji 📐️ Utility row beside the utility bar chrome toggle — a single utility keeps the chrome's height, but the active utility's options tree (stacked above it) can grow taller; its inline `maxHeight` (see {@link useWindowUtilityBarMaxHeightPx}) caps it just below the top-anchored chrome and this scrolls the overflow instead of painting past that line. */
export const utilityBarBodyClass = "flex min-h-medium min-w-0 flex-auto items-center gap-single overflow-x-auto overflow-y-auto px-single";

import {
  windowChromeScrollClearanceVar,
  windowContentDeadLineVar,
  windowContentDeadLineScrollClass,
  readWindowChromeScrollClearancePx,
  measureWindowChromeScrollClearancePx,
  isWindowContentDeadLineHost,
  readWindowContentDeadLinePx,
  readScrollerContentOverflows,
  useWindowContentDeadLineScroll,
} from "../../../../🧱️elements/🚧️WindowContentDeadLine/🟦️.tsx";
export {
  windowChromeScrollClearanceVar,
  windowContentDeadLineVar,
  windowContentDeadLineScrollClass,
  readWindowChromeScrollClearancePx,
  measureWindowChromeScrollClearancePx,
  isWindowContentDeadLineHost,
  readWindowContentDeadLinePx,
  readScrollerContentOverflows,
  useWindowContentDeadLineScroll,
};

/** @emoji 🚧️ Block offset that clears a window's floating chrome control row — the single rule for
 * anything a window's CONTENT anchors to a top corner (a scene overlay button, a status chip, the folded
 * engagement's quick-action rail). Reads the live clearance the enclosing {@link Window} publishes
 * ({@link windowChromeScrollClearanceVar}, measured off the mounted engagement/search/measures overlays)
 * and falls back to the chrome row's own token height, so content can never paint over a pane toggle. */
export const windowChromeClearedTopOffset = `calc(var(${windowChromeScrollClearanceVar}, calc(var(--size-medium) + var(--spacing-single))) + var(--spacing-single))`;

/** @emoji 🏝️ Full-bleed scroll surface for chrome-aware window bodies (writer hosts, forms, tables). */
export const ChromeAwareWindowScrollSurface = reactHostPort.forwardRef<HTMLDivElement, React.ComponentPropsWithoutRef<"div">>(({ className, children, ...props }, ref) => {
  const scrollerRef = reactHostPort.useRef<HTMLDivElement | null>(null);
  const setScrollerRef = reactHostPort.useCallback(
    (node: HTMLDivElement | null) => {
      scrollerRef.current = node;
      if (typeof ref === "function") ref(node);
      else if (ref) ref.current = node;
    },
    [ref],
  );
  useWindowContentDeadLineScroll(scrollerRef);
  return (
    <div ref={setScrollerRef} data-slot="window-dead-line-scroll" className={cn("min-h-0 min-w-0 overflow-auto", windowContentDeadLineScrollClass, className)} {...props}>
      {children}
    </div>
  );
});
ChromeAwareWindowScrollSurface.displayName = "ChromeAwareWindowScrollSurface";

/** @emoji 📐️ Labelled icon action in window rail chrome bars (options + action). */
export const windowRailChromeLabelActionClass = cn("flex h-medium w-auto items-center justify-center border-0 bg-transparent text-element px-single gap-single", interactiveHoverClass);

/** @emoji 🪟️ Pane chrome toggle — same layout as {@link panelAnchorTabButtonClass}: leading semantic icon, label, trailing {@link DragHandle}. */

/** @emoji 🪟️ Built-in window pane icons — fixed semantic affordances (never fold-direction chevrons). */
export const WINDOW_PANE_MEASURES_ICON = "settings-2" as const satisfies IconName;
export const WINDOW_PANE_ACTIONS_ICON = "play" as const satisfies IconName;
export const WINDOW_PANE_SEARCH_ICON = "search" as const satisfies IconName;
export const WINDOW_PANE_UTILITIES_ICON = "hammer" as const satisfies IconName;

/** @emoji 📐️ Measure tree body: grows with content, scrolls once the stack hits the window bottom. */
export const windowMeasuresBodyClass = "flex min-h-0 min-w-0 flex-auto flex-col overflow-y-auto overscroll-contain p-tiny";

/** @emoji 📐️ Vertical rhythm between top-level measure groups in the rail. */
export const windowMeasuresStackInnerClass = "flex w-full min-w-0 flex-col gap-tiny";

/** @emoji 📐️ Single measure tile in the window rail — transparent so the {@link WindowChrome} body glass shows through, never a second glass layer of its own. */
export const windowMeasureTileClass = cn("pointer-events-auto select-none bg-transparent w-full min-w-0 shrink-0 rounded-sm border", `${borderElementClass}/40`, "px-tiny py-tiny");

/** @emoji 📐️ Optional measure caption above a control. */
export const windowMeasureLabelClass = "text-muted-foreground mb-tiny block min-w-0 truncate text-2xs font-medium leading-none";

/** @emoji 📐️ Measure section title without a heavy chrome box. */
export const windowMeasureSectionClass = "text-muted-foreground w-full truncate px-single py-tiny text-center text-2xs font-medium uppercase tracking-wide";

/** @emoji 📐️ Constrains measure controls to the rail width. */
export const windowMeasureControlClass = "w-full min-w-0 max-w-full";

/** @emoji 🌳️ Compact disclosure header for a nested measure group. */
export const windowMeasureGroupHeaderClass = "pointer-events-auto flex h-small w-full min-w-0 shrink-0 cursor-pointer select-none items-center gap-tiny rounded-sm px-tiny py-0 text-element hover:bg-hover-interactive-fill hover:text-emphasized";

/** @emoji 🌳️ Indented children under a measure group (minimal chrome). */
export const windowMeasureGroupChildrenClass = "pointer-events-none flex w-full min-w-0 flex-col gap-0 border-s ps-tiny ms-tiny pb-0 pt-0";

/** @emoji 🌳️ Nested measure leaf without an outer tile border (indent only). */
export const windowMeasureTileNestedClass = "pointer-events-auto select-none w-full min-w-0 shrink-0 px-0 py-0";

/** @emoji 📐️ Toggle sized to fill the measure tree row (active fill spans full width). */
export const windowMeasureToggleClass =
  "!w-full min-w-0 max-w-full [&_[data-slot=toggle-group-item]]:!flex-1 [&_[data-slot=toggle-group-item]]:min-w-0 [&_[data-slot=toggle-group-item]]:max-w-full [&_[data-slot=toggle-group-item]]:!aspect-auto [&_[data-slot=toggle-group-item]]:!shrink [&_[data-slot=inline-label]]:min-w-0 [&_[data-slot=inline-label]]:truncate";

/** @emoji 📐️ Dense toggle row for nested measure groups (shorter control chrome). */
export const windowMeasureToggleCompactClass =
  "[&_[data-slot=toggle-group]]:h-small [&_[data-slot=toggle-group-item]]:min-h-0 [&_[data-slot=toggle-group-item]]:py-tiny [&_[data-slot=toggle-group-item]]:px-single [&_[data-slot=inline-label]]:!text-tiny";

/** @emoji 🌳️ Typography for measure tree group headers. */

/** @emoji 🌳️ Typography for measure tree leaf labels. */

// #endregion 🎈️Level Context

// #region 🐹️Element
import { type ElementProps } from "../../../../🔨️modules/🆔️element-identity/🟦️.ts";
export type { ElementProps };

//#region 🧭️ElementState
/** @emoji 🧭️ The shared, compile-time-enforced state model every rendered UI element carries — explicit
 * re-export from `@semio-tech/ui-styling` (this package must not leak types from outside the codebase).
 * `state`/`status`/`hover`/`selected` mirror the Rust `UiState`/`UiStatus`/`UiPresence` model in `ui_wgpu`
 * (see `framework/ui/wgpu/rs/lib.rs`'s 🔖️Presence region) byte-for-byte. */
export type { UiState, UiStatus, UiElementState, ElementFillKind };
export interface UiElementStateProps {
  /** @default "normal" */
  state?: UiState;
  /** @default "idle" */
  status?: UiStatus;
  /** Authored render-hovered flag; composes with (never replaces) live CSS `:hover`. @default false */
  hover?: boolean;
  selected?: boolean;
}

/** @emoji 🧭️ Resolves `props` against the shared defaults and returns everything a component needs to
 * apply the model: whether it must render `null` (`state === "hidden"`), the `data-ui-*` attribute
 * spread for CSS-driven components, and the fill-kind for 3D/canvas components that can't use CSS. */
export function useElementState(props: UiElementStateProps): {
  state: UiElementState;
  hidden: boolean;
  attrs: ReturnType<typeof elementStateAttributes>;
  fillKind: ElementFillKind | null;
} {
  const state = resolveElementState(props);
  return { state, hidden: state.state === "hidden", attrs: elementStateAttributes(state), fillKind: resolveElementFillKind(state) };
}

/** @emoji 🎉️ Default lifetime of a transient celebration stamp — two burst cycles of --celebrate-border-duration. */
export const CELEBRATE_STAMP_DURATION_MS = 2400;

/** @emoji 🎉️ Imperatively stamps `data-celebrated="true"` on `target` for `durationMs`, then removes it —
 * the transient counterpart of an authored `state: "celebrating"`. Unmanaged by React (like the
 * introduction engine's `data-introduced` stamp) so re-renders can't clobber it. Returns a cancel
 * function that un-stamps immediately. */
export function celebrateElement(target: Element, durationMs = CELEBRATE_STAMP_DURATION_MS): () => void {
  target.setAttribute("data-celebrated", "true");
  const clear = () => target.removeAttribute("data-celebrated");
  const timer = window.setTimeout(clear, durationMs);
  return () => {
    window.clearTimeout(timer);
    clear();
  };
}

/** @emoji 🎉️ Imperatively stamps `data-celebrated="true"` on every match of `selector` for `durationMs`,
 * then removes it — the selector form of {@link celebrateElement}. Returns a cancel that un-stamps
 * every match immediately. `root` scopes the search (e.g. a shell's own root) — omitted, searches the
 * whole document as before; matters because element ids/aliases are not guaranteed unique across
 * several mounted shells, and an unscoped search can stamp another shell's element by mistake. */
export function celebrateElements(selector: string, durationMs = CELEBRATE_STAMP_DURATION_MS, root: ParentNode = document): () => void {
  const cancels = [...root.querySelectorAll(selector)].map((el) => celebrateElement(el, durationMs));
  return () => cancels.forEach((cancel) => cancel());
}

/** @emoji 🎉️ Imperatively stamps `data-celebrated="true"` on every mounted UI element id (and every
 * element carrying a valid `data-element-alias`) for `durationMs` — the tour-finale counterpart of
 * {@link celebrateElements}. Skips the introduction chrome itself (`ui.introduction.*`) so the
 * dismiss unmount doesn't race the stamp. Returns a cancel that un-stamps every match immediately.
 * `root` scopes the search — see {@link celebrateElements}'s doc for why this matters across shells. */
export function celebrateAllElements(durationMs = CELEBRATE_STAMP_DURATION_MS, root: ParentNode = document): () => void {
  const targets = new Set<Element>();
  for (const el of root.querySelectorAll("[id]")) {
    if (isElementId(el.id) && !el.id.startsWith("ui.introduction.")) targets.add(el);
  }
  for (const el of root.querySelectorAll("[data-element-alias]")) {
    const aliases = (el.getAttribute("data-element-alias") ?? "").split(/\s+/).filter(Boolean);
    if (aliases.some((alias) => isElementId(alias) && !alias.startsWith("ui.introduction."))) targets.add(el);
  }
  const cancels = [...targets].map((el) => celebrateElement(el, durationMs));
  return () => cancels.forEach((cancel) => cancel());
}
//#endregion 🧭️ElementState

//#region 🆔️ElementId
import { ELEMENT_ID_PATTERN, isElementId, elementIdSegment, childElementId, assertElementId, elementIdSelector, useFirstDraggableElementAlias } from "../../../../🧱️elements/🆔️ElementId/🟦️.tsx";
export { ELEMENT_ID_PATTERN, isElementId, elementIdSegment, childElementId, assertElementId, elementIdSelector, useFirstDraggableElementAlias };
//#endregion 🆔️ElementId

// #endregion 🐹️Element

// #region 🪆️Command
import { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "../../../../🧱️elements/⌨️Command/🟦️.tsx";
export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };
// #endregion 🪆️Command

// #region 🎛️CommandPanel
// The footer command palette used to be its own bespoke component; commands now render as category leaf
// tabs under one expandable Command branch of the real `Panel` at anchor="bottom-middle" (see
// `buildCommandCategoryTabs`/`FRAMEWORK_CATEGORY_COMMAND_ID` in framework/os/renderer/js/react/index.tsx) —
// no ui-react component needed here anymore.
// #endregion 🎛️CommandPanel

// #region 🎮️Footer
import { Footer, type FooterProps } from "../../../../🧱️elements/🔚️Footer/🟦️.tsx";
export { Footer, type FooterProps };
// #endregion 🎮️Footer

// #region 🪨️Layout
import { Layout, type LayoutMobilePanelProps, type LayoutProps } from "../../../../🧱️elements/📐️Layout/🟦️.tsx";
export { Layout, type LayoutMobilePanelProps, type LayoutProps };
// #endregion 🪨️Layout

// #region 🌐️Popover
import {
  Popover,
  PopoverAnchor,
  PopoverContent,
  PopoverTrigger,
  resolvePopoverPlacement,
  type PopoverAlign,
  type PopoverAnchorProps,
  type PopoverContentProps,
  type PopoverPreventableEvent,
  type PopoverProps,
  type PopoverSide,
  type PopoverTriggerProps,
} from "../../../../🧱️elements/🗨️Popover/🟦️.tsx";
export {
  Popover,
  PopoverAnchor,
  PopoverContent,
  PopoverTrigger,
  resolvePopoverPlacement,
  type PopoverAlign,
  type PopoverAnchorProps,
  type PopoverContentProps,
  type PopoverPreventableEvent,
  type PopoverProps,
  type PopoverSide,
  type PopoverTriggerProps,
};
// #endregion 🌐️Popover

// #region 🌥️Base Components
// #region 🏷️Label
import { Label, useLabel, useIdLabel, useControlAccessibleLabel, useControlInlineText, useControlTooltipText, resolveTranslationLabel, useUiTranslation } from "../../../../🧱️elements/🏷️Label/🟦️.tsx";
export { Label, useLabel, useIdLabel, useControlAccessibleLabel, useControlInlineText, useControlTooltipText, resolveTranslationLabel, useUiTranslation };
export type { ControlTooltipTextOptions } from "../../../../🧱️elements/🏷️Label/🟦️.tsx";
// #endregion 🏷️Label

// #endregion 🌥️Base Components

// #region 🏷️Display Components
// Read-only display wrappers for tooltips and callouts.
// Consumers MUST pass valid config objects.

// #region 📣️Aside
// Callout boxes for notes, tips, cautions, and dangers.
// Consumers MUST specify a valid kind prop.

/**
 * Props interface for the Aside callout component.
 **/
export interface AsideProps {
  kind?: "note" | "tip" | "caution" | "danger";
  title?: UiLabel;
  children: React.ReactNode;
}

/**
 * iconMap holds the data fields for a iconMap record.
 **/
const iconMap = {
  note: InfoIcon,
  tip: LightbulbIcon,
  caution: TriangleAlertIcon,
  danger: AlertCircleIcon,
};

/**
 * colorMap holds the data fields for a colorMap record.
 **/
const colorMap = {
  note: "border-info-border bg-info-bg text-info-foreground",
  tip: "border-success-border bg-success-bg text-success-foreground",
  caution: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

/**
 * Callout component rendering note, tip, caution, or danger boxes.
 **/
export const Aside: React.FC<AsideProps> = ({ kind = "note", title, children }) => {
  const Icon = iconMap[kind];
  const colorClass = colorMap[kind];

  return (
    <aside className={`my-small p-single border ${colorClass}`}>
      <div className="flex items-start gap-single">
        <Icon className="size-small mt-0.5 flex-shrink-0" />
        <div className="flex-1">
          {title && <div className="font-semibold mb-1">{title}</div>}
          <div>{children}</div>
        </div>
      </div>
    </aside>
  );
};

// #endregion 📣️Aside

// #region 📻️TableAvatar
import { TableAvatar, type TableAvatarProps } from "../../../../🧱️elements/📻️TableAvatar/🟦️.tsx";
export { TableAvatar, type TableAvatarProps };
// #endregion 📻️TableAvatar

// #region 👥️PresenceBar
import { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole } from "../../../../🧱️elements/👥️PresenceBar/🟦️.tsx";
export { PresenceBar, presenceColor, presenceCssVar, PRESENCE_BAR_DEFAULT_MAX, type PresenceAppearance, type PresenceBarProps, type PresenceHsl, type PresencePeer, type PresenceRole };
// #endregion 👥️PresenceBar

// #region 🎹️Spinner
// Animated loading spinner in small, medium, or large sizes.
// Consumers MUST choose an appropriate size for the context.

/**
 * Props interface for the Spinner component.
 **/
export interface SpinnerProps {
  size?: "small" | "medium" | "large";
  className?: string;
}

/**
 * Animated SVG loading spinner.
 **/
export const Spinner: React.FC<SpinnerProps> = ({ size = "medium", className = "" }) => {
  const sizeClass = size === "small" ? "size-small" : size === "large" ? "size-large" : "size-medium";
  return (
    <svg className={`animate-spin ${sizeClass} ${className}`} xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
    </svg>
  );
};

// #endregion 🎹️Spinner

// #region 🔗️RouteLink
// Anchor that intercepts same-origin, protocol-less clicks for client-side navigation via history.pushState.
// Consumers MUST rely on the owned route-target parser; external/absolute hrefs always fall through to a plain anchor.

type OwnedRouteTarget = { kind: "internal"; href: string };

/** @emoji 🧭️ Parses the closed same-document route grammar without normalizing its path, query, or fragment. */
function parseOwnedRouteTarget(href: unknown): OwnedRouteTarget | null {
  if (typeof href !== "string" || href.length === 0 || /[\s\u0000-\u001f\u007f\\]/u.test(href)) return null;
  if (/^[a-z][a-z0-9+.-]*:/i.test(href) || href.startsWith("//")) return null;
  try {
    if (new URL(href, "https://owned-route.invalid/current").origin !== "https://owned-route.invalid") return null;
  } catch {
    return null;
  }
  return { kind: "internal", href };
}

/** @emoji 🚦️ Performs the one owned browser-history command and publishes one matching navigation signal. */
function navigateOwnedRoute(target: OwnedRouteTarget): { navigated: boolean } {
  if (typeof window === "undefined" || typeof window.history?.pushState !== "function" || typeof window.PopStateEvent !== "function") return { navigated: false };
  let event: PopStateEvent;
  try {
    event = new window.PopStateEvent("popstate");
    window.history.pushState(null, "", target.href);
  } catch {
    return { navigated: false };
  }
  window.dispatchEvent(event);
  return { navigated: true };
}

/** @emoji 🔗️ Anchor that delegates primary internal clicks to the owned navigation command while preserving native behavior for every other target or gesture. */
export function RouteLink({ href, target, download, onClick, ...props }: React.AnchorHTMLAttributes<HTMLAnchorElement>): React.ReactElement {
  const handleClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
    onClick?.(event);
    if (event.defaultPrevented || download !== undefined || (target && target !== "_self")) return;
    if (event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
    const route = parseOwnedRouteTarget(href);
    if (!route || !navigateOwnedRoute(route).navigated) return;
    event.preventDefault();
  };
  return <a href={href} target={target} download={download} onClick={handleClick} data-slot="route-link" {...props} />;
}

// #endregion 🔗️RouteLink

// #region 🎍️NotFound
// 404-style placeholder with icon, title, and back navigation.
// Consumers MUST provide a title for the error.

/**
 * Props interface for the NotFound component.
 **/
export interface NotFoundProps {
  title: string;
  description?: string;
  parentPath?: string;
  parentLabel?: string;
  icon?: React.ReactNode;
}

/**
 * Not-found placeholder page with navigation link.
 **/
export const NotFound: React.FC<NotFoundProps> = ({ title, description, parentPath, parentLabel, icon }) => {
  const goBackLabel = useLabel("ui.nav.back");
  const parentTarget = parseOwnedRouteTarget(parentPath);
  return (
    <div className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-muted-foreground">{icon || <AlertCircleIcon className="size-huge" />}</div>
      <h1 className="text-xl font-semibold">{title}</h1>
      {description && <p className="text-muted-foreground max-w-md">{description}</p>}
      {parentTarget && (
        <button type="button" onClick={() => navigateOwnedRoute(parentTarget)} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <ChevronLeftIcon className="size-small" />
          <span>{parentLabel || goBackLabel}</span>
        </button>
      )}
    </div>
  );
};

// #endregion 🎍️NotFound

// #region 🚦️StatusSurface
// Tone-based status-surface kit: badges, status chips, stat tiles, and empty/error placeholders.
// Consumers MUST use StatusTone (never hardcoded palette colors) for tone-driven styling.

/** @emoji 🚦️ Semantic tone shared by {@link Badge}, {@link StatusChip}, and {@link StatCard} deltas. */
export type StatusTone = "neutral" | "info" | "success" | "warning" | "danger";

/**
 * STATUS_TONE_SURFACE_CLASS holds the border/background/foreground classes for a StatusTone.
 **/
const STATUS_TONE_SURFACE_CLASS: Record<StatusTone, string> = {
  neutral: "bg-muted text-muted-foreground",
  info: "border-info-border bg-info-bg text-info-foreground",
  success: "border-success-border bg-success-bg text-success-foreground",
  warning: "border-warning-border bg-warning-bg text-warning-foreground",
  danger: "border-destructive-border bg-destructive-bg text-destructive-foreground",
};

/**
 * STATUS_TONE_DOT_CLASS holds the solid dot background class for a StatusTone.
 **/
const STATUS_TONE_DOT_CLASS: Record<StatusTone, string> = {
  neutral: "bg-muted-foreground",
  info: "bg-info-border",
  success: "bg-success-border",
  warning: "bg-warning-border",
  danger: "bg-destructive-border",
};

/**
 * Props interface for the Badge component.
 **/
export interface BadgeProps {
  id?: string;
  tone?: StatusTone;
  text: string;
  icon?: React.ReactNode;
}

/** @emoji 🏷️ Small tone-colored pill for inline status labels. */
export function Badge({ id, tone = "neutral", text, icon }: BadgeProps): React.ReactElement {
  return (
    <span id={id} data-slot="badge" className={cn("inline-flex items-center gap-half rounded-full border px-half py-0.5 text-xs font-medium", STATUS_TONE_SURFACE_CLASS[tone])}>
      {icon ? <span className="inline-flex shrink-0 items-center justify-center size-tiny">{icon}</span> : null}
      <span className="truncate">{text}</span>
    </span>
  );
}

/**
 * Props interface for the StatusChip component.
 **/
export interface StatusChipProps {
  id?: string;
  status: "ok" | "busy" | "error" | "offline";
  label: string;
}

/**
 * STATUS_CHIP_TONE maps a StatusChip status to its StatusTone.
 **/
const STATUS_CHIP_TONE: Record<StatusChipProps["status"], StatusTone> = {
  ok: "success",
  busy: "warning",
  error: "danger",
  offline: "neutral",
};

/** @emoji 🟢️ Inline dot-and-label indicator for live status (connection, task, presence). */
export function StatusChip({ id, status, label }: StatusChipProps): React.ReactElement {
  const tone = STATUS_CHIP_TONE[status];
  return (
    <span id={id} data-slot="status-chip" data-status={status} className="inline-flex items-center gap-half text-xs text-muted-foreground">
      <span className={cn("size-tiny shrink-0 rounded-full", STATUS_TONE_DOT_CLASS[tone])} aria-hidden />
      <span className="truncate">{label}</span>
    </span>
  );
}

/**
 * Props interface for the StatCard component.
 **/
export interface StatCardProps {
  id?: string;
  label: string;
  value: string;
  delta?: { value: string; tone: StatusTone };
  icon?: React.ReactNode;
}

/** @emoji 📊️ Labelled metric tile with an optional tone-colored delta badge. */
export function StatCard({ id, label, value, delta, icon }: StatCardProps): React.ReactElement {
  return (
    <div id={id} data-slot="stat-card" className="flex flex-col gap-single border p-single">
      <div className="flex items-center gap-single text-muted-foreground">
        {icon ? <span className="inline-flex shrink-0 items-center justify-center size-small">{icon}</span> : null}
        <span className="truncate text-xs">{label}</span>
      </div>
      <div className="flex items-baseline gap-single">
        <span className="text-xl font-semibold text-foreground">{value}</span>
        {delta ? <Badge tone={delta.tone} text={delta.value} /> : null}
      </div>
    </div>
  );
}

/**
 * Props interface for the EmptyState component.
 **/
export interface EmptyStateProps {
  id?: string;
  icon?: React.ReactNode;
  title: string;
  description?: string;
  action?: { label: string; onClick: () => void };
}

/** @emoji 📭️ Centered placeholder for lists or panels with no content yet. */
export function EmptyState({ id, icon, title, description, action }: EmptyStateProps): React.ReactElement {
  return (
    <div id={id} data-slot="empty-state" className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-muted-foreground">{icon || <BoxIcon className="size-huge" />}</div>
      <h2 className="text-xl font-semibold">{title}</h2>
      {description && <p className="text-muted-foreground max-w-md">{description}</p>}
      {action && (
        <button type="button" onClick={action.onClick} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <span>{action.label}</span>
        </button>
      )}
    </div>
  );
}

/**
 * Props interface for the ErrorView component.
 **/
export interface ErrorViewProps {
  id?: string;
  title?: UiLabel;
  message: string;
  onRetry?: () => void;
}

/** @emoji 🚨️ Centered error placeholder with an optional retry action. */
export function ErrorView({ id, title, message, onRetry }: ErrorViewProps): React.ReactElement {
  const somethingWentWrongLabel = useLabel("ui.common.somethingWentWrong");
  const retryLabel = useLabel("ui.common.retry");
  return (
    <div id={id} data-slot="error-view" className="flex flex-col items-center justify-center h-full gap-medium p-large text-center">
      <div className="flex items-center justify-center size-huge text-destructive-foreground">
        <AlertCircleIcon className="size-huge" />
      </div>
      <h2 className="text-xl font-semibold">{title || somethingWentWrongLabel}</h2>
      <p className="text-muted-foreground max-w-md">{message}</p>
      {onRetry && (
        <button type="button" onClick={onRetry} className="flex items-center gap-single text-sm text-primary hover:underline cursor-pointer mt-small">
          <span>{retryLabel}</span>
        </button>
      )}
    </div>
  );
}

// #endregion 🚦️StatusSurface

//#region 🧭️UiElementRegistry
/** @emoji 🧭️ Compile-time checklist of chrome components that must accept the shared {@link UiElementStateProps} axes. */
export const UI_ELEMENT_REGISTRY = ["Window", "Panel", "Canvas", "Button", "Slider", "TreeItem", "Action"] as const;
//#endregion 🧭️UiElementRegistry

// #region 🦴️Skeletons
import { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind } from "../../../../🧱️elements/🦴️Skeletons/🟦️.tsx";
export { skeletonPulseClass, SkeletonBlock, elementSkeleton, WindowBodySkeleton, PanelTreeSkeleton, CanvasSkeleton, type ElementSkeletonKind };
// #endregion 🦴️Skeletons

// #region 🖲️Section
// Collapsible section container with heading and specificity.
// Consumers MUST provide a heading string.

/**
 * Props interface for the Section component.
 **/
export interface SectionProps {
  id?: string;
  title?: UiLabel;
  children: React.ReactNode;
  className?: string;
}

/**
 **/
const Section: React.FC<SectionProps> = ({ id, title, children, className = "" }) => {
  return (
    <section id={id} className={`mb-8 ${className}`} aria-labelledby={id && title ? `${id}.title` : undefined}>
      {title && (
        <h2 className="text-2xl font-semibold mb-4" id={id ? `${id}.title` : undefined}>
          {title}
        </h2>
      )}
      <div>{children}</div>
    </section>
  );
};

export { Section };

// #endregion 🖲️Section

// #region 🏷️Field
import { Field, type FieldProps } from "../../../../🧱️elements/📝️Field/🟦️.tsx";
export { Field, type FieldProps };
// #endregion 🏷️Field

// #endregion 🏷️Display Components

// #region 🛒️Input Components

// #region 🌩️ActionGroup
import { Action, ActionDropdown, ActionGroup, ActionGroupItem, actionGroupItemVariants, type ActionDropdownOption, type ActionDropdownProps, type ActionProps } from "../../../../🧱️elements/⚡️ActionGroup/🟦️.tsx";
export { Action, ActionDropdown, ActionGroup, ActionGroupItem, actionGroupItemVariants, type ActionDropdownOption, type ActionDropdownProps, type ActionProps };
// #endregion 🌩️ActionGroup

// #region 🌩️ButtonGroup
import { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants } from "../../../../🧱️elements/🔳️ButtonGroup/🟦️.tsx";
export { ButtonGroup, ButtonGroupItem, buttonGroupItemVariants };
// #endregion 🌩️ButtonGroup

// #region 🌩️Button
import { Button, type ButtonProps } from "../../../../🧱️elements/🔘️Button/🟦️.tsx";
export { Button, type ButtonProps };
// #endregion 🌩️Button

// #region 🧾️Form
import { Form, type FormProps } from "../../../../🧱️elements/🧾️Form/🟦️.tsx";
export { Form, type FormProps };
// #endregion 🧾️Form

// #region ☑️Checkbox
import { Checkbox, type CheckboxProps, type CheckboxState } from "../../../../🧱️elements/☑️Checkbox/🟦️.tsx";
export { Checkbox, type CheckboxProps, type CheckboxState };
// #endregion ☑️Checkbox

// #region 🩺️Input
import { Input, CollapsedFieldDisplay, fitCollapsedFieldText, resolveCollapsedFieldDisplayState, COLLAPSED_FIELD_ELLIPSIS, formatNumber } from "../../../../🧱️elements/✏️Input/🟦️.tsx";
export { Input, CollapsedFieldDisplay, fitCollapsedFieldText, resolveCollapsedFieldDisplayState, COLLAPSED_FIELD_ELLIPSIS, formatNumber };
// #endregion 🩺️Input

// #region 🔎️Select
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  resolveSelectPlacement,
  type SelectAlign,
  type SelectContentProps,
  type SelectGroupProps,
  type SelectItemProps,
  type SelectLabelProps,
  type SelectPosition,
  type SelectPreventableEvent,
  type SelectProps,
  type SelectScrollButtonProps,
  type SelectSeparatorProps,
  type SelectSide,
  type SelectTriggerProps,
  type SelectValueProps,
} from "../../../../🧱️elements/🔽️Select/🟦️.tsx";
export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectScrollDownButton,
  SelectScrollUpButton,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  resolveSelectPlacement,
  type SelectAlign,
  type SelectContentProps,
  type SelectGroupProps,
  type SelectItemProps,
  type SelectLabelProps,
  type SelectPosition,
  type SelectPreventableEvent,
  type SelectProps,
  type SelectScrollButtonProps,
  type SelectSeparatorProps,
  type SelectSide,
  type SelectTriggerProps,
  type SelectValueProps,
};
// #endregion 🔎️Select

// #region 🏩️Slider
import {
  Slider,
  sliderValuesMatch,
  resolveSliderDraftClear,
  clampSliderValuesToReady,
  normalizeSliderRange,
  normalizeSliderValues,
  type SliderDirection,
  type SliderOrientation,
  type SliderProps,
  type SliderRange,
  type SliderValue,
} from "../../../../🧱️elements/🎚️Slider/🟦️.tsx";
export { Slider, sliderValuesMatch, resolveSliderDraftClear, clampSliderValuesToReady, normalizeSliderRange, normalizeSliderValues, type SliderDirection, type SliderOrientation, type SliderProps, type SliderRange, type SliderValue };
// #endregion 🏩️Slider

// #region 🏬️Stepper
import { Stepper } from "../../../../🧱️elements/🪜️Stepper/🟦️.tsx";
export { Stepper };
// #endregion 🏬️Stepper

// #region 🎏️Textarea
import { Textarea } from "../../../../🧱️elements/🔤️Textarea/🟦️.tsx";
export { Textarea };
// #endregion 🎏️Textarea

// #region 🗡️Toggle
import { Toggle, type ToggleItem, type ToggleProps } from "../../../../🧱️elements/🔀️Toggle/🟦️.tsx";
export { Toggle };
export type { ToggleItem, ToggleProps };
// #endregion 🗡️Toggle

// #region 🧩️ToggleGroup
import {
  ToggleGroup,
  ToggleGroupItem,
  type ToggleGroupProps,
  type ToggleGroupSingleProps,
  type ToggleGroupMultipleProps,
  type ToggleGroupItemProps,
  type ToggleGroupOrientation,
  type ToggleGroupDirection,
} from "../../../../🧱️elements/🎛️ToggleGroup/🟦️.tsx";
export { ToggleGroup, ToggleGroupItem };
export type { ToggleGroupProps, ToggleGroupSingleProps, ToggleGroupMultipleProps, ToggleGroupItemProps, ToggleGroupOrientation, ToggleGroupDirection };
// #endregion 🧩️ToggleGroup

// #region 🧫️Ring
import { Ring, type RingOrbData, type RingProps } from "../../../../🧱️elements/⭕️Ring/🟦️.tsx";
export { Ring };
export type { RingOrbData, RingProps };
// #endregion 🧫️Ring

// #endregion 🛒️Input Components

// #region 🗼️Aggregation Components

// #region 🖥️Collapsible
import { Collapsible, CollapsibleContent, CollapsibleTrigger, type CollapsibleContentProps, type CollapsibleProps, type CollapsibleTriggerProps } from "../../../../🧱️elements/↕️Collapsible/🟦️.tsx";
export { Collapsible, CollapsibleContent, CollapsibleTrigger };
export type { CollapsibleContentProps, CollapsibleProps, CollapsibleTriggerProps };
// #endregion 🖥️Collapsible

// #region 🧸️Dialog
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
  type DialogCloseProps,
  type DialogContentProps,
  type DialogDescriptionProps,
  type DialogOverlayProps,
  type DialogPortalProps,
  type DialogPreventableEvent,
  type DialogProps,
  type DialogTitleProps,
  type DialogTriggerProps,
} from "../../../../🧱️elements/💬️Dialog/🟦️.tsx";
export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
  type DialogCloseProps,
  type DialogContentProps,
  type DialogDescriptionProps,
  type DialogOverlayProps,
  type DialogPortalProps,
  type DialogPreventableEvent,
  type DialogProps,
  type DialogTitleProps,
  type DialogTriggerProps,
};
// #endregion 🧸️Dialog

// #region 🪬️Resizable
import {
  RESIZABLE_HIT_TARGET_MIN_FINE_PX,
  RESIZABLE_HIT_TARGET_MIN_COARSE_PX,
  RESIZABLE_CORNER_GRAB_PX,
  type ResizableJoinEdgeSide,
  type ResizableJoinCornerSpec,
  type ResizableJoinCornerResizeHandler,
  resizableJoinCornerPlacementStyle,
  readResizableJoinCornerSpec,
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "../../../../🧱️elements/↔️Resizable/🟦️.tsx";
export {
  RESIZABLE_HIT_TARGET_MIN_FINE_PX,
  RESIZABLE_HIT_TARGET_MIN_COARSE_PX,
  RESIZABLE_CORNER_GRAB_PX,
  type ResizableJoinEdgeSide,
  type ResizableJoinCornerSpec,
  type ResizableJoinCornerResizeHandler,
  resizableJoinCornerPlacementStyle,
  readResizableJoinCornerSpec,
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
};
// #endregion 🪬️Resizable

// #region 🎮️Scrollable
import { Scrollable } from "../../../../🧱️elements/📜️Scrollable/🟦️.tsx";
export { Scrollable };
// #endregion 🎮️Scrollable

// #region 🖥️Fullscreen

const DOCUMENT_FULLSCREEN_CHANGE_EVENTS = ["fullscreenchange", "webkitfullscreenchange"] as const;

type FullscreenDocument = Document & {
  webkitFullscreenElement?: Element | null;
  webkitExitFullscreen?: () => Promise<void> | void;
};

type FullscreenHTMLElement = HTMLElement & {
  webkitRequestFullscreen?: () => Promise<void> | void;
};

/** @emoji 🖥️ Whether the browser document is in fullscreen mode. */
export function readDocumentFullscreenActive(doc: Document = document): boolean {
  const typed = doc as FullscreenDocument;
  return !!(typed.fullscreenElement ?? typed.webkitFullscreenElement);
}

/** @emoji 🖥️ Enter or exit browser fullscreen. `root` is the element requesting fullscreen — a shell's
 * own root (e.g. `ShellScope.rootRef.current`) so going fullscreen from within one embedded shell fills
 * the screen with just that shell's content, not the whole page (other mounted shells included);
 * omitted, defaults to `document.documentElement` (the single-shell-per-page case, unchanged). The
 * Fullscreen API only ever has one fullscreen element at a time regardless of which root requested it. */
export async function toggleDocumentFullscreen(root: Element = document.documentElement): Promise<void> {
  const typedRoot = root as FullscreenHTMLElement;
  const doc = root.ownerDocument;
  const typedDoc = doc as FullscreenDocument;
  if (readDocumentFullscreenActive(doc)) {
    if (typedDoc.exitFullscreen) await typedDoc.exitFullscreen();
    else typedDoc.webkitExitFullscreen?.();
    return;
  }
  if (typedRoot.requestFullscreen) await typedRoot.requestFullscreen();
  else typedRoot.webkitRequestFullscreen?.();
}

/** @emoji 🖥️ Tracks browser fullscreen state for shell chrome. `root` scopes which element requests
 * fullscreen — see {@link toggleDocumentFullscreen}'s doc. */
export function useDocumentFullscreen(root?: Element): { isFullscreen: boolean; toggle: () => void } {
  const [isFullscreen, setIsFullscreen] = reactHostPort.useState(() => (typeof document !== "undefined" ? readDocumentFullscreenActive() : false));

  reactHostPort.useEffect(() => {
    if (typeof document === "undefined") return;
    const sync = () => setIsFullscreen(readDocumentFullscreenActive());
    sync();
    for (const event of DOCUMENT_FULLSCREEN_CHANGE_EVENTS) {
      document.addEventListener(event, sync);
    }
    return () => {
      for (const event of DOCUMENT_FULLSCREEN_CHANGE_EVENTS) {
        document.removeEventListener(event, sync);
      }
    };
  }, []);

  const toggle = reactHostPort.useCallback(() => {
    void toggleDocumentFullscreen(root).catch((error) => console.error("Fullscreen request was rejected", error));
  }, [root]);

  return { isFullscreen, toggle };
}

// 🐚️ Keyed by shell root (falling back to `document.documentElement` outside any shell) so two navbars
// of different widths — one per shell — never overwrite each other's measured reserve.
const shellNavbarTrailingEndWidthByRoot = ephemeralMap<HTMLElement, number>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellNavbarTrailingEndWidthByRoot");
const shellNavbarTrailingEndWidthListenersByRoot = ephemeralMap<HTMLElement, Set<() => void>>("framework.modules.ui.packages.typescript.targets.react.index.tsx.shellNavbarTrailingEndWidthListenersByRoot");

function publishShellNavbarTrailingEndWidthPx(root: HTMLElement | undefined, width: number): void {
  const key = resolveElementsSurfaceChromeRoot(root);
  if (!key || width < 0 || width === shellNavbarTrailingEndWidthByRoot.get(key)) return;
  shellNavbarTrailingEndWidthByRoot.set(key, width);
  for (const listener of shellNavbarTrailingEndWidthListenersByRoot.get(key) ?? []) listener();
}

/** @emoji ↔ Measured trailing navbar chrome width (fullscreen toggle footprint) for this shell. */
export function useShellNavbarTrailingEndWidthPx(root?: HTMLElement): number {
  const key = resolveElementsSurfaceChromeRoot(root);
  return reactHostPort.useSyncExternalStore(
    (onStoreChange) => {
      if (!key) return () => {};
      let listeners = shellNavbarTrailingEndWidthListenersByRoot.get(key);
      if (!listeners) {
        listeners = new Set();
        shellNavbarTrailingEndWidthListenersByRoot.set(key, listeners);
      }
      listeners.add(onStoreChange);
      return () => listeners.delete(onStoreChange);
    },
    () => (key ? (shellNavbarTrailingEndWidthByRoot.get(key) ?? 0) : 0),
    () => 0,
  );
}

/** @emoji ↔ Inline cap-row reserve that clears trailing navbar controls for chrome-hosted right panels. */
export function shellNavbarTrailingEndReserveStyle(widthPx: number): React.CSSProperties | undefined {
  if (widthPx <= 0) return undefined;
  return { paddingInlineStart: `${widthPx + uiSpacingPx(1)}px` };
}

function NavbarFullscreenToggle({ onToggle }: { readonly onToggle?: () => void }) {
  const shellScope = useShellScopeOptional();
  const { isFullscreen, toggle } = useDocumentFullscreen(shellScope?.rootRef.current ?? undefined);
  const enterLabel = useLabel("ui.fullscreen.toggle");
  const exitLabel = useLabel("ui.fullscreen.exit");
  return (
    <Toggle
      id="ui.fullscreen.toggle"
      text={isFullscreen ? exitLabel : enterLabel}
      pressed={isFullscreen}
      onPressedChange={onToggle ?? toggle}
      icon={isFullscreen ? <Minimize2Icon className="size-small" /> : <Maximize2Icon className="size-small" />}
    />
  );
}

/** @emoji 🖥️ Navbar trailing slot for fullscreen — parks width so labels do not collapse when panels open. */
export function NavbarTrailingFullscreenSlot({ onToggle }: { readonly onToggle?: () => void } = {}) {
  const shellScope = useShellScopeOptional();
  // 🐚️ Resolved at render time (not read from `shellScope.rootRef` inside the effect): the ref object's
  // identity never changes, so a dep array holding the ref itself would never re-fire this effect once
  // the root attaches after this component's own mount commit (shell roots attach bottom-up, after
  // deeply-nested descendants like this one). Reading `.current` here and depending on the resolved
  // element instead picks up the populated root on the guaranteed re-render `FrameworkOsShell` triggers
  // once its own root ref callback fires.
  const root = shellScope?.rootRef.current ?? undefined;
  const shellRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [parkedMinWidth, setParkedMinWidth] = reactHostPort.useState(0);

  reactHostPort.useLayoutEffect(() => {
    const shell = shellRef.current;
    if (!shell) return;
    const measure = () => {
      const width = shell.getBoundingClientRect().width;
      if (width > 0) {
        setParkedMinWidth(width);
        publishShellNavbarTrailingEndWidthPx(root, width);
      }
    };
    measure();
    const resizeObserver = typeof ResizeObserver !== "undefined" ? new ResizeObserver(measure) : null;
    resizeObserver?.observe(shell);
    return () => resizeObserver?.disconnect();
  }, [root]);

  return (
    <div ref={shellRef} key="fullscreenToggle" data-slot="navbar-fullscreen-toggle" className="ms-auto flex h-medium shrink-0 min-w-fit items-center" style={parkedMinWidth > 0 ? { minWidth: parkedMinWidth } : undefined}>
      <NavbarFullscreenToggle onToggle={onToggle} />
    </div>
  );
}

// #endregion 🖥️Fullscreen

// #region 🩺️Navbar
import { Navbar, type NavbarItem, type NavbarProps, SemioLogo, ShellBrandLogo, navbarFillItem } from "../../../../🧱️elements/🔝️Navbar/🟦️.tsx";
export { Navbar, type NavbarItem, type NavbarProps, SemioLogo, ShellBrandLogo, navbarFillItem };
// #endregion 🩺️Navbar

// #region 🧪️NavbarExampleSelect
import { NavbarExampleSelect, type NavbarExampleOption, type NavbarExampleSelectProps } from "../../../../🧱️elements/🧪️NavbarExampleSelect/🟦️.tsx";
export { NavbarExampleSelect, type NavbarExampleOption, type NavbarExampleSelectProps };
// #endregion 🧪️NavbarExampleSelect

// #region 🪟️DesktopTitlebar
// Draggable Electron-style window title bar with minimize/maximize/close controls.
// Consumers MUST omit `controls` on non-Electron hosts; no control buttons render without it.

/** @emoji 🪟️ Configuration for {@link DesktopTitlebar} window controls. */
export interface DesktopTitlebarProps {
  title: string;
  controls?: { minimize(): void; maximize(): void; close(): void };
  children?: React.ReactNode;
}

/** @emoji 🪟️ Draggable title bar row with a title, extra chrome, and window controls. @see https://www.electronjs.org/docs/latest/tutorial/custom-title-bar */
export function DesktopTitlebar({ title, controls, children }: DesktopTitlebarProps): React.ReactElement {
  const minimizeLabel = useLabel("ui.common.minimize");
  const maximizeLabel = useLabel("ui.common.maximize");
  const closeLabel = useLabel("ui.common.close");
  return (
    <div data-slot="desktop-titlebar" className={cn(borderNormalBottomClass, "flex h-large shrink-0 items-center gap-single px-single")} style={{ WebkitAppRegion: "drag" } as React.CSSProperties}>
      <span className="truncate text-sm font-semibold">{title}</span>
      {children}
      {controls ? (
        <div className="ms-auto flex items-center gap-tiny" style={{ WebkitAppRegion: "no-drag" } as React.CSSProperties}>
          <button type="button" onClick={controls.minimize} aria-label={minimizeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-hover-window hover:text-foreground">
            <Icon icon="minus" size="tiny" />
          </button>
          <button type="button" onClick={controls.maximize} aria-label={maximizeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-hover-window hover:text-foreground">
            <Icon icon="square" size="tiny" />
          </button>
          <button type="button" onClick={controls.close} aria-label={closeLabel} className="cursor-pointer rounded-sm p-single text-muted-foreground transition-colors hover:bg-destructive-bg hover:text-destructive-foreground">
            <Icon icon="x" size="tiny" />
          </button>
        </div>
      ) : null}
    </div>
  );
}

// #endregion 🪟️DesktopTitlebar

// #region 🏷️Tabs
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  type TabsProps,
  type TabsListProps,
  type TabsTriggerProps,
  type TabsContentProps,
  type TabsOrientation,
  type TabsDirection,
  type TabsActivationMode,
} from "../../../../🧱️elements/📑️Tabs/🟦️.tsx";
export { Tabs, TabsContent, TabsList, TabsTrigger };
export type { TabsProps, TabsListProps, TabsTriggerProps, TabsContentProps, TabsOrientation, TabsDirection, TabsActivationMode };
// #endregion 🏷️Tabs

// #region 🖼️IconSelector
import { IconSelector, type IconSelectorProps } from "../../../../🧱️elements/🎴️IconSelector/🟦️.tsx";
export { IconSelector, type IconSelectorProps };
// #endregion 🖼️IconSelector

// #region 📜️Tree
import {
  BasicChatPanel,
  CATALOGUE_DRAG_MIME,
  Catalogue,
  ControlTree,
  FileTree,
  HelperRow,
  PropertyValueColumnContext,
  SortableTreeItems,
  Tree,
  TreeAlignedRow,
  TreeCheckbox,
  TreeContent,
  TreeContext,
  TreeItem,
  TreeItemCollapsibleState,
  TreeItems,
  TreeRow,
  TreeRowAlignmentContext,
  TreeSection,
  TreeStateProvider,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
  WindowPaneChromeToggle,
  buildControlTree,
  catalogueTreeDragController,
  createTreeHighlightStore,
  createTreeSelectionStore,
  defaultControlRenderer,
  deriveTreeDragRoles,
  detailPanelHeaderLineCenterPx,
  detailPanelIndentLen,
  detailPanelIndentPx,
  detailPanelPropertyControlClassName,
  detailPanelPropertyInlineGapPx,
  detailPanelPropertyRowClassName,
  detailPanelPropertyStackedToInlineHysteresisPx,
  getActiveCatalogueDragPayload,
  getTreeItemOrderedIds,
  getTreeNextSelectionState,
  getTreeSiblingGapPx,
  isTreeReorderDragEvent,
  markGhostTreeInteraction,
  mergeTreeRowContextMenu,
  mergeTreeSectionOrder,
  normalizeTreeSelectedIds,
  resolveHoverRow,
  resolveTreeDropPosition,
  shouldDispatchTreeRowPointerLeave,
  syncTreeSelectionPath,
  treeCompactSiblingGapPx,
  treeFoldChevronIcon,
  treeHeaderMainClassName,
  treeHeaderRowClassName,
  treeInspectorInnerRowClassName,
  treeItemLabelStyle,
  treeItemSecondaryTextClassName,
  treeReorderDragController,
  treeRowChromeClasses,
  treeRowChromeContentFillClasses,
  treeRowChromeShellClasses,
  treeRowDragPayloadAttributes,
  uiSpacingLen,
  useTreeReorder,
  useTreeState,
  type CatalogueItem,
  type CatalogueProps,
  type ControlDef,
  type ControlTreeClassNames,
  type ControlTreeFolderSettings,
  type ControlTreeProps,
  type FileTreeNode,
  type TreeActionPlacement,
  type TreeCheckboxAction,
  type TreeCheckboxProps,
  type TreeDataActivationContext,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDirection,
  type TreeDragAndDropController,
  type TreeDragRole,
  type TreeDropPosition,
  type TreeHeaderAction,
  type TreePointerPaletteDragController,
  type TreeReorderControllerOptions,
  type TreeReorderMove,
  type TreeSectionAction,
  type TreeSelectionMode,
  type UseTreeReorderResult,
  type WindowMeasureTreeGroupProps,
  type WindowMeasureTreeLeafProps,
  type WindowPaneChromeToggleProps,
} from "../../../../🧱️elements/🌳️Tree/🟦️.tsx";
export {
  BasicChatPanel,
  CATALOGUE_DRAG_MIME,
  Catalogue,
  ControlTree,
  FileTree,
  HelperRow,
  PropertyValueColumnContext,
  SortableTreeItems,
  Tree,
  TreeAlignedRow,
  TreeCheckbox,
  TreeContent,
  TreeContext,
  TreeItem,
  TreeItemCollapsibleState,
  TreeItems,
  TreeRow,
  TreeRowAlignmentContext,
  TreeSection,
  TreeStateProvider,
  WindowMeasureTreeGroup,
  WindowMeasureTreeLeaf,
  WindowMeasuresTree,
  WindowPaneChromeToggle,
  buildControlTree,
  catalogueTreeDragController,
  createTreeHighlightStore,
  createTreeSelectionStore,
  defaultControlRenderer,
  deriveTreeDragRoles,
  detailPanelHeaderLineCenterPx,
  detailPanelIndentLen,
  detailPanelIndentPx,
  detailPanelPropertyControlClassName,
  detailPanelPropertyInlineGapPx,
  detailPanelPropertyRowClassName,
  detailPanelPropertyStackedToInlineHysteresisPx,
  getActiveCatalogueDragPayload,
  getTreeItemOrderedIds,
  getTreeNextSelectionState,
  getTreeSiblingGapPx,
  isTreeReorderDragEvent,
  markGhostTreeInteraction,
  mergeTreeRowContextMenu,
  mergeTreeSectionOrder,
  normalizeTreeSelectedIds,
  resolveHoverRow,
  resolveTreeDropPosition,
  shouldDispatchTreeRowPointerLeave,
  syncTreeSelectionPath,
  treeCompactSiblingGapPx,
  treeFoldChevronIcon,
  treeHeaderMainClassName,
  treeHeaderRowClassName,
  treeInspectorInnerRowClassName,
  treeItemLabelStyle,
  treeItemSecondaryTextClassName,
  treeReorderDragController,
  treeRowChromeClasses,
  treeRowChromeContentFillClasses,
  treeRowChromeShellClasses,
  treeRowDragPayloadAttributes,
  uiSpacingLen,
  useTreeReorder,
  useTreeState,
};
export type {
  CatalogueItem,
  CatalogueProps,
  ControlDef,
  ControlTreeClassNames,
  ControlTreeFolderSettings,
  ControlTreeProps,
  FileTreeNode,
  TreeActionPlacement,
  TreeCheckboxAction,
  TreeCheckboxProps,
  TreeDataActivationContext,
  TreeDataItem,
  TreeDataSection,
  TreeDirection,
  TreeDragAndDropController,
  TreeDragRole,
  TreeDropPosition,
  TreeHeaderAction,
  TreePointerPaletteDragController,
  TreeReorderControllerOptions,
  TreeReorderMove,
  TreeSectionAction,
  TreeSelectionMode,
  UseTreeReorderResult,
  WindowMeasureTreeGroupProps,
  WindowMeasureTreeLeafProps,
  WindowPaneChromeToggleProps,
};
// #endregion 📜️Tree

// #endregion 🗼️Aggregation Components

// #region 📷️Panel Components

// #region 🧭️Panel
import {
  Panel,
  type PanelProps,
  type TreePanelConfig,
  type TreePanelDefinition,
  type TreePanelSource,
  staticTreePanelDefinition,
  usePointerDrag,
  useNativeDragAndDrop,
  PanelTreeUnitsPane,
  PanelEmptyDockZone,
} from "../../../../🧱️elements/🖼️Panel/🟦️.tsx";
export { Panel, staticTreePanelDefinition, usePointerDrag, useNativeDragAndDrop, PanelTreeUnitsPane, PanelEmptyDockZone };
export type { PanelProps, TreePanelConfig, TreePanelDefinition, TreePanelSource };
// #endregion 🧭️Panel

// #region 🪟️Pane
// Window-level floating chrome (measures, engagement, search, utility bar, and any app-contributed floating
// control) — anchored and draggable across the same eight anchors a Panel docks to (see 🧭️Panel above), so panels
// and panes share one positioning mechanism end to end. Naming follows the repo convention (see ticket
// `26/07/15/RENAME-WINDOW-PANELS-TO-PANES-AND-CORNER-PANELS-TO-PANELS`): "panel" is the shell-edge dock, "pane" is
// chrome floating inside one window.

/** @emoji 🧭️ Nearest anchor to a pointer position within a host's bounding rect — a 3×3 zone grid over the box.
 * There is no center anchor, so the dead-center zone resolves to whichever edge-middle the pointer has drifted
 * closer to (compares distance from the vertical vs. horizontal midline). */
export function nearestAnchor(pointerX: number, pointerY: number, hostRect: { readonly left: number; readonly top: number; readonly width: number; readonly height: number }): Anchor {
  const x = hostRect.width > 0 ? (pointerX - hostRect.left) / hostRect.width : 0.5;
  const y = hostRect.height > 0 ? (pointerY - hostRect.top) / hostRect.height : 0.5;
  const col: "left" | "middle" | "right" = x < 1 / 3 ? "left" : x > 2 / 3 ? "right" : "middle";
  const row: "top" | "middle" | "bottom" = y < 1 / 3 ? "top" : y > 2 / 3 ? "bottom" : "middle";
  if (row !== "middle" && col !== "middle") return `${row}-${col}` as Anchor;
  if (row !== "middle") return `${row}-middle` as Anchor;
  if (col !== "middle") return `${col}-middle` as Anchor;
  const distanceFromVerticalMidline = Math.abs(x - 0.5);
  const distanceFromHorizontalMidline = Math.abs(y - 0.5);
  if (distanceFromVerticalMidline >= distanceFromHorizontalMidline) return x < 0.5 ? "left-middle" : "right-middle";
  return y < 0.5 ? "top-middle" : "bottom-middle";
}

interface PaneHostContextValue {
  /** @emoji 🎯️ Imperative bounds source for drag math (read on demand, well after mount — never during another component's render). */
  readonly containerRef: React.RefObject<HTMLDivElement | null>;
  /** @emoji 🌱️ Reactive mirror of the same node, `null` until mount — {@link usePaneSlot} portals need a render-time value, and a ref's `.current` isn't populated yet during the same pass a child first renders alongside its ref owner. */
  readonly container: HTMLDivElement | null;
}

const PaneHostContext = reactHostPort.createContext<PaneHostContextValue | undefined>(undefined);

/** @emoji 🪟️ The nearest {@link PaneHost}, or `undefined` outside one — {@link Pane} drag-to-reanchor and {@link usePaneSlot} both need it. */
function usePaneHostContext(): PaneHostContextValue | undefined {
  return reactHostPort.useContext(PaneHostContext);
}

/**
 * PaneHostProps holds the data fields for a PaneHostProps record.
 **/
export interface PaneHostProps {
  readonly className?: string;
  readonly children?: React.ReactNode;
}

/** @emoji 🪟️ Bounds box for floating panes: mount one inside a window body (or any floating-chrome host) so every
 * {@link Pane} inside — direct JSX children or portaled in via {@link usePaneSlot} — shares one anchor coordinate
 * space to drag between and one DOM parent to measure drag drops against.
 *
 * Children live as siblings of the portal mount (not inside the `pointer-events-none` overlay) so window canvas
 * content keeps normal hit-testing while still receiving {@link PaneHostContext}. */
export const PaneHost: React.FC<PaneHostProps> = ({ className, children }) => {
  const containerRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [container, setContainer] = reactHostPort.useState<HTMLDivElement | null>(null);
  const setRef = reactHostPort.useCallback((element: HTMLDivElement | null) => {
    containerRef.current = element;
    setContainer(element);
  }, []);
  const contextValue = reactHostPort.useMemo((): PaneHostContextValue => ({ containerRef, container }), [container]);
  return (
    <PaneHostContext.Provider value={contextValue}>
      <div data-slot="pane-host-root" className={cn("relative h-full min-h-0 min-w-0 w-full", className)}>
        {children}
        <div ref={setRef} data-slot="pane-host" className="pointer-events-none absolute inset-0" />
      </div>
    </PaneHostContext.Provider>
  );
};

/** @emoji 🪟️ Portals `pane` into the nearest {@link PaneHost} — lets a component deep inside a window's canvas
 * (e.g. a per-tool floating control) contribute a draggable pane without threading it through the window's props.
 * Renders nothing outside a `PaneHost`. */
export function usePaneSlot(pane: React.ReactElement): React.ReactPortal | null {
  const host = usePaneHostContext();
  return host?.container ? (createPortal(pane, host.container) as React.ReactPortal) : null;
}

/** @emoji ↔ Pane resize handle — same sign convention as {@link PanelResizeHandle}: dragging the right-side handle right (or the left-side handle left) grows the pane. Middle anchors pass `deltaFactor={2}` so each edge contributes half the visual grow. */
function PaneResizeHandle({
  side,
  size,
  minSize,
  maxSize,
  onSizeChange,
  onActiveChange,
  deltaFactor = 1,
}: {
  readonly side: "left" | "right";
  readonly size: number;
  readonly minSize: number;
  readonly maxSize: number;
  readonly onSizeChange: (size: number) => void;
  readonly onActiveChange?: (active: boolean) => void;
  readonly deltaFactor?: number;
}) {
  const startRef = reactHostPort.useRef<{ pointerX: number; size: number } | null>(null);
  const pointerProps = usePointerDrag<HTMLDivElement>({
    onStart: (event) => {
      event.preventDefault();
      startRef.current = { pointerX: event.clientX, size };
      onActiveChange?.(true);
    },
    onMove: (event) => {
      const start = startRef.current;
      if (!start) return;
      const next = start.size + (side === "right" ? 1 : -1) * deltaFactor * (event.clientX - start.pointerX);
      if (next >= minSize && next <= maxSize) onSizeChange(next);
    },
    onEnd: () => {
      startRef.current = null;
      onActiveChange?.(false);
    },
    onCancel: () => {
      startRef.current = null;
      onActiveChange?.(false);
    },
  });
  return <div data-slot="pane-resize-handle" className={`absolute top-0 bottom-0 z-20 ${side === "left" ? "left-0" : "right-0"} w-single cursor-ew-resize`} {...pointerProps} />;
}

/**
 * Props interface for the Pane component.
 **/
export interface PaneProps {
  /** @emoji 🆔️ The pane container's own DOM id — rendered on the overlay root, and the stem every derived chrome id (`<id>.pane.fold`, `<id>.pane.fold-control`) falls back to. */
  readonly id: string;
  readonly anchor: Anchor;
  /** @emoji 🧭️ Fires while dragging the pane's handle, once per anchor crossed — omit to make the pane fixed (drag handle still renders as a pure affordance, matching panel toggles). */
  readonly onAnchorChange?: (anchor: Anchor) => void;
  readonly folded?: boolean;
  readonly onFoldToggle?: () => void;
  /** @emoji 🖼️ Fixed semantic icon for the pane chrome toggle — never a fold-direction chevron. */
  readonly icon: IconName;
  readonly label?: UiLabel;
  readonly size?: number;
  readonly onSizeChange?: (size: number) => void;
  readonly minSize?: number;
  readonly maxSize?: number;
  readonly resizable?: boolean;
  readonly enlarge?: WindowChromeControlAction;
  /** @emoji ⛶️ Fill the {@link PaneHost} (window options focus/unfocus). */
  readonly expanded?: boolean;
  readonly overlaySlot?: string;
  readonly overlayRef?: React.Ref<HTMLDivElement>;
  readonly stackSlot?: string;
  readonly bodySlot?: string;
  readonly bodyClassName?: string;
  readonly bodyStyle?: React.CSSProperties;
  readonly stackClassName?: string;
  readonly stackDataAttrs?: Record<string, string | undefined>;
  readonly toggleId?: string;
  readonly foldControlId?: string;
  readonly toggleDisabled?: boolean;
  readonly dimWhenOpen?: boolean;
  readonly onResizeActiveChange?: (active: boolean) => void;
  /** @emoji 🗂️ Stacking order among panes sharing one anchor — lower first, in this anchor's flow direction. */
  readonly order?: number;
  readonly zIndex?: 10 | 20 | 30 | 40;
  readonly className?: string;
  readonly children?: React.ReactNode;
}

const PANE_DEFAULT_SIZE = 300;
const PANE_DEFAULT_MIN_SIZE = 200;
const PANE_DEFAULT_MAX_SIZE = 600;

/** @emoji 🪟️ One floating pane inside a {@link PaneHost} — anchored via the exact same {@link anchorPositionStyle}/
 * {@link flowFromAnchor} math as {@link Panel}, foldable to a chip, optionally width-resizable (inner edge for
 * corners / side-middle; both edges for top/bottom-middle, matching panel grow), and (given `onAnchorChange`)
 * draggable by its handle to any of the eight anchors via {@link nearestAnchor} — dropped anywhere inside the
 * enclosing {@link PaneHost}, not just on the target slot, since the box is the only drop target there is.
 * Chrome matches panel toggles: semantic {@link icon} + label + trailing {@link DragHandle}. */
export const Pane: React.FC<PaneProps> = ({
  id,
  anchor,
  onAnchorChange,
  folded = true,
  onFoldToggle,
  icon,
  label,
  size = PANE_DEFAULT_SIZE,
  onSizeChange,
  minSize = PANE_DEFAULT_MIN_SIZE,
  maxSize = PANE_DEFAULT_MAX_SIZE,
  resizable: resizableProp,
  enlarge,
  expanded = false,
  overlaySlot,
  overlayRef,
  stackSlot = "window-chrome-stack",
  bodySlot = "pane-body",
  bodyClassName = "overflow-y-auto p-single",
  bodyStyle,
  stackClassName,
  stackDataAttrs,
  toggleId,
  foldControlId,
  toggleDisabled,
  dimWhenOpen = false,
  onResizeActiveChange,
  order = 0,
  zIndex,
  className = "",
  children,
}) => {
  const host = usePaneHostContext();
  const mobile = useUiMobile();
  const collapseLabel = useLabel("ui.common.collapse");
  const paneRootRef = reactHostPort.useRef<HTMLDivElement>(null);
  const setPaneRootRef = reactHostPort.useCallback(
    (element: HTMLDivElement | null) => {
      paneRootRef.current = element;
      if (typeof overlayRef === "function") overlayRef(element);
      else if (overlayRef) (overlayRef as React.MutableRefObject<HTMLDivElement | null>).current = element;
    },
    [overlayRef],
  );
  const [surfaceActive, surfaceActiveProps] = useSurfaceActive(paneRootRef);
  const effectiveFolded = folded;
  const flow = flowFromAnchor(anchor);
  const horizontal = anchorHorizontal(anchor);
  const [dragging, setDragging] = reactHostPort.useState(false);
  const lastAnchorRef = reactHostPort.useRef(anchor);
  lastAnchorRef.current = anchor;
  const resizable = resizableProp ?? Boolean(onSizeChange);
  const stopPointerPropagation = reactHostPort.useCallback((event: React.PointerEvent<HTMLDivElement>) => event.stopPropagation(), []);

  const dragPointerProps = usePointerDrag<HTMLSpanElement>({
    onStart: () => {
      if (!onAnchorChange) return;
      setDragging(true);
    },
    onMove: (event) => {
      if (!onAnchorChange) return;
      const hostRect = host?.containerRef.current?.getBoundingClientRect();
      if (!hostRect) return;
      const next = nearestAnchor(event.clientX, event.clientY, hostRect);
      if (next !== lastAnchorRef.current) {
        lastAnchorRef.current = next;
        onAnchorChange(next);
      }
    },
    onEnd: () => setDragging(false),
    onCancel: () => setDragging(false),
  });

  const positionStyle: React.CSSProperties = expanded
    ? { position: "absolute", inset: 0, zIndex: zIndex, width: "100%", height: "100%", maxWidth: "100%", maxHeight: "100%" }
    : {
        ...anchorPositionStyle(anchor),
        order,
        ...(zIndex !== undefined ? { zIndex } : {}),
        width: !mobile && !effectiveFolded ? `${size}px` : undefined,
        maxWidth: !mobile && !effectiveFolded ? `min(100% - (var(--spacing-single) * 2), ${size}px)` : undefined,
      };
  const paneFoldControl =
    !effectiveFolded && onFoldToggle
      ? {
          id: foldControlId ?? childElementId(id, "pane", "fold-control"),
          slot: "pane-fold",
          icon: <CloseIcon className="size-small" />,
          label: collapseLabel,
          onClick: onFoldToggle,
        }
      : undefined;
  const resizeSides: readonly ("left" | "right")[] = horizontal === "middle" ? ["left", "right"] : [horizontal === "left" ? "right" : "left"];
  const resizeDeltaFactor = horizontal === "middle" ? 2 : 1;
  const chromeToggleId = toggleId ?? childElementId(id, "pane", "fold");

  return (
    <LevelProvider level="pane">
      <div
        ref={setPaneRootRef}
        id={id}
        {...surfaceActiveProps}
        data-slot={overlaySlot ?? "pane"}
        data-level="pane"
        data-anchor={anchor}
        data-folded={effectiveFolded ? "true" : undefined}
        data-expanded={expanded ? "true" : undefined}
        data-dragging={dragging ? "true" : undefined}
        {...(dimWhenOpen ? { "data-dim": true } : {})}
        dir={flow.inline === "rtl" ? "rtl" : undefined}
        onPointerDown={stopPointerPropagation}
        onPointerMove={stopPointerPropagation}
        onPointerUp={stopPointerPropagation}
        onPointerCancel={stopPointerPropagation}
        className={cn(
          "pointer-events-auto absolute flex min-h-0 min-w-0 box-border overflow-visible",
          getLevelZClass("pane"),
          expanded && "z-panel",
          flow.block === "up" ? "flex-col-reverse" : "flex-col",
          horizontal === "middle" ? "items-center" : "items-start",
          effectiveFolded && "w-fit",
          !effectiveFolded && !expanded && "w-full max-w-full",
          expanded && "h-full max-h-full",
          className,
        )}
        style={positionStyle}
      >
        <FlowProvider inline={flow.inline} block={flow.block}>
          <WindowChrome
            stackSlot={stackSlot}
            chipOnly={effectiveFolded}
            active={!effectiveFolded && surfaceActive}
            level="pane"
            capDock={flow.block === "up" ? "bottom" : "top"}
            stackClassName={cn("bg-transparent", !effectiveFolded && "w-full min-h-0 flex-1", expanded && "h-full max-h-full min-h-0", stackClassName)}
            bodyClassName={bodyClassName}
            bodySlot={bodySlot}
            bodyStyle={bodyStyle}
            enlarge={!effectiveFolded ? enlarge : undefined}
            close={paneFoldControl}
            stackDataAttrs={stackDataAttrs}
            titleChips={
              <WindowPaneChromeToggle
                id={chromeToggleId}
                icon={icon}
                label={label ?? id}
                disabled={toggleDisabled}
                onClick={onFoldToggle}
                dragPointerProps={mobile || !onAnchorChange ? undefined : dragPointerProps}
                showDragHandle={!mobile}
                emphasized={dragging}
              />
            }
            body={!effectiveFolded ? children : undefined}
          />
          {resizable && !mobile && !effectiveFolded && !expanded && onSizeChange
            ? resizeSides.map((side) => <PaneResizeHandle key={side} side={side} size={size} minSize={minSize} maxSize={maxSize} onSizeChange={onSizeChange} onActiveChange={onResizeActiveChange} deltaFactor={resizeDeltaFactor} />)
            : null}
        </FlowProvider>
      </div>
    </LevelProvider>
  );
};

// #endregion 🪟️Pane

// #endregion 📷️Panel Components

// #region 🩻️Ribbon Components

import { RibbonZone, RibbonDivider, RibbonGroup, RibbonItem } from "../../../../🧱️elements/🎀️Ribbon/🟦️.tsx";
export { RibbonZone, RibbonDivider, RibbonGroup, RibbonItem };

// #region 🎀️Ribbon
import { Ribbon, type RibbonDirection, type RibbonRow, type RibbonProps } from "../../../../🧱️elements/🎀️Ribbon/🟦️.tsx";
export { Ribbon };
export type { RibbonDirection, RibbonRow, RibbonProps };
// #endregion 🎀️Ribbon

// #endregion 🩻️Ribbon Components

// #region 🧭️Shell

export interface EngagementOption {
  id: string;
  label?: UiLabel;
  icon: ControlIcon;
  pressed?: boolean;
  disabled?: boolean;
  onPress?: () => void;
}

export interface EngagementStatus {
  id: string;
  content: React.ReactNode;
}

/** @emoji 🔘️ One discrete option on an engagement {@link EngagementRingControl}. */
export interface EngagementRingOption {
  id: string;
  label: string;
  disabled?: boolean;
}

/** @emoji 🎚️ Engagement range slider for numeric values (height, distance, …). */
export interface EngagementSliderControl {
  kind: "slider";
  id?: string;
  label?: UiLabel;
  value: number;
  min: number;
  max: number;
  /** @emoji 🎚️ Absolute preloaded/ready extent on the fixed `[min, max]` range. */
  ready?: number;
  step?: number;
  unit?: string;
  disabled?: boolean;
  onChange?: (value: number) => void;
  onCommit?: (value: number) => void;
}

/** @emoji 🔢️ Engagement stepper for numeric values without fixed upper bound. */
export interface EngagementStepperControl {
  kind: "stepper";
  id?: string;
  label?: UiLabel;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  unit?: string;
  disabled?: boolean;
  onChange?: (value: number) => void;
  onCommit?: (value: number) => void;
}

/** @emoji 🧫️ Engagement ring (radial dial) for angles or discrete option selection. */
export interface EngagementRingControl {
  kind: "ring";
  id?: string;
  label?: UiLabel;
  value?: string;
  options: readonly EngagementRingOption[];
  disabled?: boolean;
  onSelect?: (id: string) => void;
}

/** @emoji 🔘️ One discrete option on an engagement {@link EngagementToggleGroupControl}. */
export interface EngagementToggleGroupOption {
  id: string;
  label: string;
  icon?: IconName;
  disabled?: boolean;
}

/** @emoji 🔘️ Engagement toggle button group for small unordered enums. */
export interface EngagementToggleGroupControl {
  kind: "toggleGroup";
  id?: string;
  label?: UiLabel;
  value?: string;
  options: readonly EngagementToggleGroupOption[];
  disabled?: boolean;
  onSelect?: (id: string) => void;
}

/** @emoji 🔽️ One item on an engagement {@link EngagementSelectControl}. */
export interface EngagementSelectItem {
  id: string;
  value: string;
  label: string;
}

/** @emoji 🔽️ Engagement select dropdown for large enums. */
export interface EngagementSelectControl {
  kind: "select";
  id?: string;
  label?: UiLabel;
  value?: string;
  placeholder?: UiLabel;
  items: readonly EngagementSelectItem[];
  disabled?: boolean;
  onChange?: (value: string) => void;
}

/** @emoji 🎛️ Optional engagement UI control for the active action step. */
export type EngagementControl = EngagementSliderControl | EngagementStepperControl | EngagementRingControl | EngagementToggleGroupControl | EngagementSelectControl;

/** @emoji 🏷️ i18n keys for window engagement chrome (`ui.engagement.*` in {@link uiChromeTranslationBundles}). */
export const UI_ENGAGEMENT = {
  actions: "ui.engagement.actions",
} as const satisfies Record<string, UiTranslationKey>;

/** @emoji 🏷️ Default English copy for window engagement chrome (matches `ui.engagement.*` en bundle). */
export const ENGAGEMENT_USER = {
  actionsAria: "Actions",
} as const;

/** @emoji 🏷️ i18n keys for the window search pane (`ui.windowSearch.*` in {@link uiChromeTranslationBundles}; distinct from the `ui.search.*` command palette). */
export const UI_WINDOW_SEARCH = {
  title: "ui.windowSearch.title",
  action: "ui.windowSearch.action",
  actionActive: "ui.windowSearch.actionActive",
  suggestions: "ui.windowSearch.suggestions",
  noMatches: "ui.windowSearch.noMatches",
} as const satisfies Record<string, UiTranslationKey>;

/** @emoji 🏷️ Default English copy for the window search pane (matches `ui.windowSearch.*` en bundle) — for standalone REPL surfaces that build a {@link SearchSpec} outside {@link Search}'s own `useLabel` resolution. */
export const WINDOW_SEARCH_USER = {
  actionPlaceholder: "Action",
  actionPlaceholderActive: "Action or value",
  suggestionsAria: "Suggestions",
  noMatches: "No matches",
} as const;

/** @emoji ⌨️ Normalizes engagement action text: no separators, PascalCase tokens (`set height` → `SetHeight`, `box` → `Box`), preserving decimal points inside numbers (`3.5` stays `3.5`, not `35`). */
export function normalizeEngagementActionText(text: string): string {
  const decimalMarker = "\u0001";
  const withProtectedDecimals = text.replace(/(\d)\.(\d)/g, `$1${decimalMarker}$2`);
  const words = withProtectedDecimals
    .replace(new RegExp(`[^a-zA-Z0-9${decimalMarker}]+`, "g"), " ")
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .flatMap((word) => word.split(/(?=[A-Z])/))
    .filter(Boolean)
    .map((word) => word.split(decimalMarker).join("."));
  return words.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()).join("");
}

/** @emoji ⚖️ True when two engagement action tokens match after {@link normalizeEngagementActionText} (case-insensitive). */
export function engagementActionTokenEquals(a: string, b: string): boolean {
  return normalizeEngagementActionText(a).toLowerCase() === normalizeEngagementActionText(b).toLowerCase();
}

// #region 🔎️WindowSearch

/** @emoji ⌨️ One typed action line for a window's {@link SearchSpec}. */
export interface SearchInput {
  id?: string;
  value?: string;
  placeholder?: UiLabel;
  onChange?: (value: string) => void;
  onSubmit?: (value: string) => void;
  /** @emoji 🔁️ Restarts the last finalized engagement when Space is pressed with an empty action. */
  onRepeatLast?: () => void;
  /** @emoji ⎋️ Cancels the active engagement session (Escape), e.g. abort interaction or clear action. */
  onAbort?: () => void;
  disabled?: boolean;
}

/** @emoji 🔎️ One autocomplete row for {@link SearchSpec.possibles} (interaction, transition, …). */
export interface SearchPossible {
  id: string;
  label: string;
  detail?: string;
  onSelect?: () => void;
}

function searchPossibleRankScore(query: string, item: SearchPossible): number {
  const ql = normalizeEngagementActionText(query).toLowerCase();
  if (!ql) return -1;
  const label = normalizeEngagementActionText(item.label).toLowerCase();
  const detail = (item.detail ?? "").toLowerCase();
  const id = item.id.toLowerCase();
  if (label.startsWith(ql)) return 3000 - label.length;
  if (detail.startsWith(ql)) return 2000 - detail.length;
  if (id.startsWith(ql)) return 1000 - id.length;
  const haystack = `${label} ${detail} ${id}`;
  if (haystack.includes(ql)) return 500;
  return -1;
}

/** @emoji 🎯️ Resolves a pointer event target to an element for search suggestion hit-testing. */
function searchSuggestionPointerTarget(event: Pick<PointerEvent, "target">): Element | null {
  const target = event.target;
  if (target instanceof Element) return target;
  if (target instanceof Text) return target.parentElement;
  return null;
}

/** @emoji 🎯️ True when a pointer event targets a search suggestion action row. */
export function isSearchSuggestionActionTarget(event: Pick<PointerEvent, "target">): boolean {
  return Boolean(searchSuggestionPointerTarget(event)?.closest('[data-slot="command-item"]'));
}

/** @emoji 🔎️ Filters {@link SearchPossible} rows by label, detail, and id for the window search action line. */
export function filterSearchPossibles(query: string, items: readonly SearchPossible[]): SearchPossible[] {
  const trimmed = normalizeEngagementActionText(query).toLowerCase();
  if (!trimmed) return [...items];
  return items
    .map((item) => ({ item, score: searchPossibleRankScore(query, item) }))
    .filter((row) => row.score >= 0)
    .sort((a, b) => b.score - a.score)
    .map((row) => row.item);
}

/** @emoji ⌨️ Inline completion segments for one {@link SearchPossible} using label casing for the matched name prefix. */
export interface SearchInlineCompletion {
  readonly prefix: string;
  readonly suffix: string;
}

/** @emoji ⌨️ Returns PascalCase inline completion when query prefix-matches the possible's name, detail, or id. */
export function searchInlineCompletion(query: string, item: SearchPossible | undefined): SearchInlineCompletion | null {
  if (!query.trim() || !item) return null;
  const q = query;
  const ql = q.toLowerCase();
  const label = normalizeEngagementActionText(item.label);
  let best: SearchInlineCompletion | null = null;
  const consider = (matched: boolean) => {
    if (!matched || !label.toLowerCase().startsWith(ql)) return;
    const prefix = label.slice(0, q.length);
    const suffix = label.slice(q.length);
    if (!suffix.length) return;
    if (!best || suffix.length > best.suffix.length) best = { prefix, suffix };
  };
  consider(label.toLowerCase().startsWith(ql));
  consider(Boolean(item.detail?.toLowerCase().startsWith(ql)));
  consider(item.id.toLowerCase().startsWith(ql));
  return best;
}

/** @emoji ⌨️ Inline completion suffix for one {@link SearchPossible} (longest prefix match on label, detail, or id). */
export function searchCompletionSuffix(query: string, item: SearchPossible | undefined): string {
  return searchInlineCompletion(query, item)?.suffix ?? "";
}

/** @emoji ⌨️ First non-empty inline completion across ranked {@link SearchPossible} matches. */
export function searchActiveInlineCompletion(query: string, matches: readonly SearchPossible[], index: number): SearchInlineCompletion | null {
  if (!query.trim() || !matches.length) return null;
  const order = [matches[Math.min(index, matches.length - 1)]!, ...matches];
  const seen = new Set<SearchPossible>();
  for (const item of order) {
    if (seen.has(item)) continue;
    seen.add(item);
    const completion = searchInlineCompletion(query, item);
    if (completion) return completion;
  }
  return null;
}

/** @emoji ⌨️ First non-empty inline completion suffix across ranked {@link SearchPossible} matches. */
export function searchActiveCompletionSuffix(query: string, matches: readonly SearchPossible[], index: number): string {
  return searchActiveInlineCompletion(query, matches, index)?.suffix ?? "";
}

/** @emoji 🔎️ Renders a possible name with the query prefix emphasized using label casing (e.g. **B**ox). */
export function searchHighlightedLabel(label: string, query: string, detail?: string): React.ReactNode {
  const displayLabel = normalizeEngagementActionText(label);
  const trimmed = normalizeEngagementActionText(query);
  if (!trimmed) return displayLabel;
  const ql = trimmed.toLowerCase();
  const ll = displayLabel.toLowerCase();
  let start = ll.startsWith(ql) ? 0 : -1;
  if (start < 0 && detail?.toLowerCase().startsWith(ql)) start = ll.indexOf(ql) >= 0 ? ll.indexOf(ql) : ll.indexOf(ql[0] ?? "");
  if (start < 0) return displayLabel;
  const end = start + trimmed.length;
  return (
    <>
      {start > 0 ? <span>{displayLabel.slice(0, start)}</span> : null}
      <span className="font-semibold text-foreground">{displayLabel.slice(start, end)}</span>
      <span>{displayLabel.slice(end)}</span>
    </>
  );
}

/** @emoji 🚫️ React props that disable native browser affordances on editable UI controls. */

/** @emoji 🚫️ Applies {@link uiFormControlBrowserDefaultProps} to a live form control (idempotent). */
export function applyUiFormControlBrowserDefaults(element: HTMLInputElement | HTMLTextAreaElement): void {
  if (element.dataset.uiBrowserDefaults === "true") return;
  const kind = element instanceof HTMLInputElement ? (element.type || "text").toLowerCase() : "textarea";
  if (kind === "file" || kind === "checkbox" || kind === "radio" || kind === "hidden" || kind === "range" || kind === "color") return;
  element.autocomplete = "off";
  element.spellcheck = false;
  element.autocapitalize = "off";
  element.setAttribute("autocorrect", "off");
  element.setAttribute("data-1p-ignore", "");
  element.setAttribute("data-lpignore", "true");
  element.dataset.uiBrowserDefaults = "true";
}

/** @emoji ⌨️ True when the event target should receive typed characters (skip engagement routing and global REPL capture). */
export function isUiTypingTarget(t: EventTarget | null): boolean {
  if (!(t instanceof HTMLElement)) return false;
  if (t instanceof HTMLTextAreaElement || t instanceof HTMLSelectElement) return true;
  if (t.isContentEditable) return true;
  if (t.closest('[role="textbox"]')) return true;
  if (t instanceof HTMLInputElement) {
    const kind = (t.type || "text").toLowerCase();
    return kind !== "button" && kind !== "checkbox" && kind !== "radio" && kind !== "file" && kind !== "range" && kind !== "color";
  }
  if (t.closest('[data-slot="input-root"], [data-slot="textarea-root"], [data-collapsed="true"], [data-slot="command-input"], [data-slot="select-trigger"], [data-slot="select-content"]')) {
    return true;
  }
  return Boolean(t.closest('[data-slot="search"] input, [data-slot="search"] textarea'));
}

/** @emoji 🚫️ Capture-phase listeners: native context menu off everywhere; Tab focus traversal off outside {@link isUiTypingTarget}; form-control browser defaults on focus. */
export function installElementsSurfaceBrowserDefaultSuppression(bindings: ReturnType<typeof createDOMEventBinding>): void {
  if (typeof document === "undefined") return;
  const onContextMenu = (event: Event): void => {
    event.preventDefault();
  };
  const onKeyDown = (event: KeyboardEvent): void => {
    if (event.key !== "Tab") return;
    if (isUiTypingTarget(event.target)) return;
    const active = document.activeElement;
    if (active instanceof HTMLElement && isUiTypingTarget(active)) return;
    event.preventDefault();
  };
  const onFocusIn = (event: FocusEvent): void => {
    const target = event.target;
    if (target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement) {
      applyUiFormControlBrowserDefaults(target);
    }
  };
  bindings.listen(document, "contextmenu", onContextMenu as EventListener, true);
  bindings.listen(document, "keydown", onKeyDown as EventListener, true);
  bindings.listen(document, "focusin", onFocusIn as EventListener, true);
}

/** @emoji ⌨️ True when the event target is already the active window search action field. */
export function isWindowSearchTypingTarget(t: EventTarget | null): boolean {
  if (!(t instanceof HTMLElement)) return false;
  return Boolean(t.closest('[data-slot="window"][data-active="true"] [data-slot="search"][data-active="true"] [data-slot="input"], [data-slot="window"][data-active="true"] [data-slot="search"][data-active="true"] textarea'));
}

/** @emoji ⌨️ True when printable keys should route to the active window search action (skip other text fields). */
export function shouldRouteKeysToWindowSearch(t: EventTarget | null): boolean {
  if (isWindowSearchTypingTarget(t)) return false;
  const searchField = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const active = document.activeElement;
  if (searchField && (active === searchField || searchField.contains(active))) return false;
  if (active instanceof HTMLElement && isUiTypingTarget(active) && !active.closest('[data-slot="search"]')) return false;
  if (isUiTypingTarget(t)) return false;
  return true;
}

/** @emoji ⌨️ Returns the window search action input, optionally requiring {@link SearchProps.active}. */
export function queryWindowSearchInput(activeOnly = false): HTMLInputElement | null {
  const searchActive = activeOnly ? '[data-active="true"]' : "";
  return document.querySelector<HTMLInputElement>(`[data-slot="window"][data-active="true"] [data-slot="search"]${searchActive} [data-slot="input"]`);
}

/** @emoji ⌨️ Focuses the action input in the active window search overlay, if present. */
export function focusActiveSearchInput(): boolean {
  const active = document.activeElement;
  if (active instanceof HTMLElement && active.closest('[data-slot="engagement-control"]')) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  if (!field || field.disabled) return false;
  field.focus({ preventScroll: true });
  return true;
}

/** @emoji ✍️ One local edit of a controlled search line: the text the user typed plus the published value
 * that stood when the edit began. */
export interface SearchLineEdit {
  readonly text: string;
  readonly base: string;
}

/**
 * @emoji ✍️ Which line a controlled window search field shows. The published value is a program's
 * property, and a program that stores the line without republishing it (or that answers a full round trip
 * later) used to make the field unwritable: every keystroke dispatched `onChange` and the field snapped
 * straight back to the stale `value`, so every submit carried an empty line. The local edit therefore
 * LEADS, and the published value only wins once it moved away from what stood when the edit began — which
 * is exactly the case where the program authored the line itself (a completed submit, an abort, a
 * program-side rewrite) and the user's draft is stale instead.
 */
export function searchControlledLineV1(published: string, edit: SearchLineEdit | null): string {
  if (!edit) return published;
  return published === edit.base ? edit.text : published;
}

/** @emoji ✅️ True when Space/Enter should pick the active filtered {@link SearchPossible} instead of submitting the raw draft. */
export function shouldActivateSearchPossibleOnConfirm(draft: string, showPossiblesList: boolean, filteredCount: number): boolean {
  if (!filteredCount) return false;
  return showPossiblesList || Boolean(draft.trim());
}

/** @emoji ␣️ Applies Space on a window search action line (step submit vs repeat-last when idle). */
export function applySearchSpaceAction(input: SearchInput, draft: string, sessionActive: boolean): boolean {
  if (input.disabled) return false;
  if (!draft.trim()) {
    if (sessionActive) {
      if (!input.onSubmit) return false;
      input.onSubmit(draft);
      return true;
    }
    if (!input.onRepeatLast) return false;
    input.onRepeatLast();
    return true;
  }
  if (!input.onSubmit) return false;
  input.onSubmit(draft);
  return true;
}

/** @emoji 🔁️ Routes Space outside the search field: idle empty → {@link SearchInput.onRepeatLast}; session or typed draft → {@link SearchInput.onSubmit}. */
export function routeWindowSearchSpace(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "defaultPrevented" | "isComposing" | "target">): boolean {
  const input = search?.input;
  if (!input || event.defaultPrevented || event.isComposing) return false;
  if (event.key !== " " || event.ctrlKey || event.metaKey || event.altKey) return false;
  if (!shouldRouteKeysToWindowSearch(event.target)) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const draft = normalizeEngagementActionText(input.value ?? field?.value ?? "");
  return applySearchSpaceAction(input, draft, Boolean(search?.sessionActive));
}

/** @emoji ⌨️ Routes a printable key to the active window search action when focus is elsewhere in the window. */
export function routeWindowSearchKeydown(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "altKey" | "defaultPrevented" | "isComposing" | "target">): boolean {
  const input = search?.input;
  if (!input || input.disabled || event.defaultPrevented || event.isComposing) return false;
  if (!shouldRouteKeysToWindowSearch(event.target)) return false;
  if (event.key === " ") return false;
  const printable = event.key.length === 1 && !event.ctrlKey && !event.metaKey && !event.altKey;
  if (!printable) return false;
  const field = queryWindowSearchInput(true) ?? queryWindowSearchInput(false);
  const next = normalizeEngagementActionText(`${input.value ?? field?.value ?? ""}${event.key}`);
  input.onChange?.(next);
  return true;
}

/** @emoji ⎋️ Routes Escape to {@link SearchInput.onAbort} when window search chrome is active (skips other typing targets). */
export function routeWindowSearchEscape(search: SearchSpec | undefined, event: Pick<KeyboardEvent, "key" | "defaultPrevented" | "isComposing" | "target">, zone: { readonly chromeVisible: boolean; readonly actionActive: boolean }): boolean {
  if (event.key !== "Escape" || event.defaultPrevented || event.isComposing) return false;
  const onAbort = search?.input?.onAbort;
  if (!onAbort) return false;
  if (!search?.sessionActive && !zone.chromeVisible && !zone.actionActive) return false;
  if (isUiTypingTarget(event.target) && !isWindowSearchTypingTarget(event.target)) return false;
  const focused = document.activeElement;
  if (focused instanceof HTMLElement && isUiTypingTarget(focused) && !focused.closest('[data-slot="search"]')) return false;
  onAbort();
  return true;
}

/** @emoji 🔎️ Floating top-middle window search payload: typed action input and autocomplete possibles. */
export interface SearchSpec {
  /** @emoji 🎯️ Ongoing engagement: chrome stays visible; action input accepts step values. */
  sessionActive?: boolean;
  input?: SearchInput;
  possibles?: SearchPossible[];
}

export interface SearchProps extends SearchSpec {
  className?: string;
  /** @emoji 🎯️ When true, focuses the action input whenever this search pane belongs to the globally active window. */
  active?: boolean;
}

// #endregion 🔎️WindowSearch

/** @emoji 💬️ Floating window engagement payload with options, status, and controls. */
export interface EngagementSpec {
  /** @emoji 🎯️ Ongoing engagement: chrome stays visible; {@link options} are step transitions. */
  sessionActive?: boolean;
  options?: EngagementOption[];
  /** @emoji 🎛️ Optional slider, stepper, or ring control for the active step. */
  control?: EngagementControl;
  /** @emoji 🎛️ Optional additional controls rendered below the primary control. */
  controls?: readonly EngagementControl[];
  status?: EngagementStatus[];
}

export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";

export interface WindowLayoutWindowNode {
  kind: "window";
  id: string;
  title?: UiLabel;
  size?: number;
  corner?: WindowStackCorner;
}

export interface WindowLayoutStackNode {
  kind: "stack";
  size?: number;
  activeId?: string;
  children: readonly WindowLayoutWindowNode[];
}

export interface WindowLayoutAxisNode {
  kind: "row" | "column";
  size?: number;
  children: readonly (WindowLayoutAxisNode | WindowLayoutStackNode)[];
}

/** @emoji 🪟️ Recursive resizable window layout tree for {@link Mode}. */
export type WindowLayoutNode = WindowLayoutAxisNode | WindowLayoutStackNode | WindowLayoutWindowNode;

/** @emoji 🪟️ Builds an even horizontal split layout for the given window ids. */
export function createEvenWindowLayout(windowIds: readonly string[]): WindowLayoutNode {
  if (windowIds.length === 0) return { kind: "stack", children: [] };
  if (windowIds.length === 1) return { kind: "stack", children: [{ kind: "window", id: windowIds[0]! }] };
  return {
    kind: "row",
    children: windowIds.map((id) => ({
      kind: "stack" as const,
      children: [{ kind: "window" as const, id }],
    })),
  };
}

export interface EngagementProps extends EngagementSpec {
  className?: string;
}

function engagementControlLabel(control: EngagementControl): string | undefined {
  if (!control.label) return undefined;
  if (control.kind === "ring" || control.kind === "toggleGroup" || control.kind === "select" || !control.unit) return control.label;
  return `${control.label} (${control.unit})`;
}

function engagementControlIsNumeric(control: EngagementControl): control is EngagementSliderControl | EngagementStepperControl {
  return control.kind === "slider" || control.kind === "stepper";
}

/** @emoji 🎛️ Renders one engagement {@link EngagementControl} using Slider, Stepper, Ring, toggle group, or Select. */
function EngagementControlView({ control }: { readonly control: EngagementControl }): React.ReactElement | null {
  const label = engagementControlLabel(control);
  const selectLabel = useLabel("ui.common.select");
  const lastNumericRef = reactHostPort.useRef(engagementControlIsNumeric(control) ? control.value : 0);
  reactHostPort.useEffect(() => {
    if (engagementControlIsNumeric(control)) lastNumericRef.current = control.value;
  }, [control]);
  if (control.kind === "slider") {
    return (
      <div data-slot="engagement-control" data-control-kind="slider" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Slider
          id={control.id ?? "engagement-control.slider"}
          value={[control.value]}
          min={control.min}
          max={control.max}
          ready={control.ready}
          step={control.step}
          disabled={control.disabled}
          onValueChange={(values) => {
            const next = values[0];
            if (next === undefined) return;
            lastNumericRef.current = next;
            control.onChange?.(next);
          }}
          onPointerUp={() => control.onCommit?.(lastNumericRef.current)}
        />
      </div>
    );
  }
  if (control.kind === "stepper") {
    return (
      <div data-slot="engagement-control" data-control-kind="stepper" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Stepper
          id={control.id ?? "engagement-control.stepper"}
          value={control.value}
          min={control.min}
          max={control.max}
          step={control.step}
          onChange={(value) => {
            lastNumericRef.current = value;
            control.onChange?.(value);
          }}
          onPointerUp={() => control.onCommit?.(lastNumericRef.current)}
        />
      </div>
    );
  }
  if (control.kind === "toggleGroup") {
    if (!control.options.length) return null;
    return (
      <div data-slot="engagement-control" data-control-kind="toggleGroup" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <ButtonGroup id={control.id} detailPanelWidthMode="fill">
          {control.options.map((option) => (
            <ButtonGroupItem
              key={option.id}
              id={option.id}
              text={option.label}
              icon={option.icon ?? "circle-dot"}
              className={cn(control.value === option.id && interactiveActiveFillClass)}
              disabled={option.disabled || control.disabled}
              onClick={() => control.onSelect?.(option.id)}
            />
          ))}
        </ButtonGroup>
      </div>
    );
  }
  if (control.kind === "select") {
    if (!control.items.length) return null;
    return (
      <div data-slot="engagement-control" data-control-kind="select" className="flex min-w-0 flex-col gap-half px-half">
        {label ? <span className="text-element text-xs">{label}</span> : null}
        <Select id={control.id ?? "engagement-control.select"} value={control.value} onValueChange={(value) => control.onChange?.(value)} disabled={control.disabled}>
          <SelectTrigger id={control.id} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={control.placeholder ?? selectLabel} />
          </SelectTrigger>
          <SelectContent>
            {control.items.map((item) => (
              <SelectItem key={item.id} value={item.value}>
                {item.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
    );
  }
  if (!control.options.length) return null;
  const orbs = control.options.map((option, index) => ({
    id: option.id,
    t: control.options.length > 1 ? index / control.options.length : 0,
    disabled: option.disabled || control.disabled,
    selected: control.value === option.id,
  }));
  return (
    <div data-slot="engagement-control" data-control-kind="ring" className="flex min-w-0 flex-col items-center gap-half px-half">
      {label ? <span className="text-element text-xs">{label}</span> : null}
      <Ring id={control.id ?? "engagement-control.ring"} orbs={orbs} onOrbSelect={(orbId) => control.onSelect?.(orbId)} />
    </div>
  );
}

// #region 🔎️WindowSearch

/** @emoji 🔎️ Floating top-middle window search pane: action input with optional right chevron for possibles. */
const Search: React.FC<SearchProps> = ({ sessionActive = false, input, possibles, className = "", active = false }) => {
  const { inline } = useFlow();
  const actionPlaceholderLabel = useLabel(UI_WINDOW_SEARCH.action);
  const actionActivePlaceholderLabel = useLabel(UI_WINDOW_SEARCH.actionActive);
  const suggestionsAriaLabel = useLabel(UI_WINDOW_SEARCH.suggestions);
  const noMatchesLabel = useLabel(UI_WINDOW_SEARCH.noMatches);
  const [uncontrolledDraft, setUncontrolledDraft] = reactHostPort.useState("");
  const [controlledEdit, setControlledEdit] = reactHostPort.useState<SearchLineEdit | null>(null);
  const isControlledInput = !!input?.onChange;
  const publishedLine = normalizeEngagementActionText(input?.value ?? "");
  const draft = isControlledInput ? searchControlledLineV1(publishedLine, controlledEdit) : normalizeEngagementActionText(uncontrolledDraft);
  const actionPlaceholder = input?.placeholder ?? (sessionActive ? actionActivePlaceholderLabel : actionPlaceholderLabel);
  const [possiblesExpanded, setPossiblesExpanded] = reactHostPort.useState(false);
  const [activePossibleIndex, setActivePossibleIndex] = reactHostPort.useState(0);
  const searchRef = reactHostPort.useRef<HTMLDivElement>(null);
  const filteredPossibles = reactHostPort.useMemo(() => filterSearchPossibles(draft, possibles ?? []), [draft, possibles]);

  reactHostPort.useEffect(() => {
    setActivePossibleIndex((index) => (filteredPossibles.length ? Math.min(index, filteredPossibles.length - 1) : 0));
  }, [filteredPossibles.length, draft]);

  reactHostPort.useEffect(() => {
    setPossiblesExpanded(false);
  }, [possibles]);

  const hasInput = !!input;
  const hasPossibles = !!possibles?.length;
  const showPossiblesList = hasPossibles && possiblesExpanded && filteredPossibles.length > 0;
  const inlineCompletion = reactHostPort.useMemo(() => (showPossiblesList ? null : searchActiveInlineCompletion(draft, filteredPossibles, activePossibleIndex)), [activePossibleIndex, draft, filteredPossibles, showPossiblesList]);

  const applyDraft = reactHostPort.useCallback(
    (value: string) => {
      const normalized = normalizeEngagementActionText(value);
      if (!isControlledInput) {
        setUncontrolledDraft(normalized);
        return;
      }
      setControlledEdit((previous) => ({ text: normalized, base: previous?.base ?? publishedLine }));
      input?.onChange?.(normalized);
    },
    [input, isControlledInput, publishedLine],
  );
  /** 🧹️ Hands the line back to the program on confirm or abort: the published value leads again from
   * here, so a command line the program empties (`engagement_submit` clears `engagement_input`) clears,
   * and a field the program keeps a standing value in keeps it. Never writes a value of its own — that
   * would leave the field empty forever against a program that publishes one. */
  const releaseDraft = reactHostPort.useCallback(() => {
    setControlledEdit(null);
    setUncontrolledDraft("");
  }, []);

  const pickingPossibleIdRef = reactHostPort.useRef<string | null>(null);
  const selectPossible = reactHostPort.useCallback(
    (item: SearchPossible) => {
      if (pickingPossibleIdRef.current === item.id) return;
      pickingPossibleIdRef.current = item.id;
      item.onSelect?.();
      applyDraft("");
      setPossiblesExpanded(false);
      setActivePossibleIndex(0);
      queueMicrotask(() => {
        if (pickingPossibleIdRef.current === item.id) pickingPossibleIdRef.current = null;
      });
    },
    [applyDraft],
  );

  const activatePossible = reactHostPort.useCallback((): boolean => {
    if (!filteredPossibles.length) return false;
    selectPossible(filteredPossibles[activePossibleIndex] ?? filteredPossibles[0]!);
    return true;
  }, [activePossibleIndex, filteredPossibles, selectPossible]);

  const wasActiveRef = reactHostPort.useRef(false);
  reactHostPort.useEffect(() => {
    const becameActive = active && !wasActiveRef.current;
    wasActiveRef.current = active;
    if (!becameActive || !hasInput || input?.disabled) return;
    const focused = document.activeElement;
    if (focused instanceof HTMLElement && isUiTypingTarget(focused) && !focused.closest('[data-slot="search"]')) return;
    const field = searchRef.current?.querySelector<HTMLInputElement>('[data-slot="input"]');
    field?.focus({ preventScroll: true });
  }, [active, hasInput, input?.disabled, input?.id]);

  if (!hasInput) return null;

  return (
    <LevelProvider level="pane">
      <div
        ref={searchRef}
        data-slot="search"
        data-active={active ? "true" : undefined}
        data-session-active={sessionActive ? "true" : undefined}
        data-possibles-open={showPossiblesList ? "true" : undefined}
        className={cn("pointer-events-auto flex w-full min-w-0 max-w-full flex-col gap-half", sessionActive && "bg-active-base ring-accent/35 rounded-sm ring-1 shadow-sm", className)}
      >
        <Popover
          open={showPossiblesList}
          onOpenChange={(open) => {
            if (!open) setPossiblesExpanded(false);
          }}
        >
          <PopoverAnchor asChild>
            <div data-slot="search-row" className="flex w-full min-w-0 items-stretch gap-half">
              <div data-slot="search-input" className="relative grid min-w-0 flex-1 [&_[data-slot=input-root]]:col-start-1 [&_[data-slot=input-root]]:row-start-1 [&_[data-slot=input-root]]:min-w-0">
                <Input
                  id={!input!.id || input!.id === "search-input" || isInternalChromeControlId(input!.id) ? UI_WINDOW_SEARCH.action : input!.id}
                  className="relative z-[1] min-w-0 flex-1 bg-transparent"
                  value={draft}
                  tabIndex={active ? 0 : -1}
                  onChange={(event) => applyDraft(event.target.value)}
                  onKeyDown={(event) => {
                    if (event.key === "Escape") {
                      if (showPossiblesList) {
                        event.preventDefault();
                        setPossiblesExpanded(false);
                        return;
                      }
                      if (input!.onAbort) {
                        event.preventDefault();
                        event.stopPropagation();
                        input!.onAbort();
                        releaseDraft();
                      }
                      return;
                    }
                    if (event.key === "Tab" && !showPossiblesList && inlineCompletion) {
                      event.preventDefault();
                      applyDraft(inlineCompletion.prefix + inlineCompletion.suffix);
                      return;
                    }
                    if (event.key === "ArrowDown" && filteredPossibles.length) {
                      event.preventDefault();
                      setActivePossibleIndex((index) => (index + 1) % filteredPossibles.length);
                      return;
                    }
                    if (event.key === "ArrowUp" && filteredPossibles.length) {
                      event.preventDefault();
                      setActivePossibleIndex((index) => (index - 1 + filteredPossibles.length) % filteredPossibles.length);
                      return;
                    }
                    if (event.key === " " && !event.ctrlKey && !event.metaKey && !event.altKey) {
                      event.preventDefault();
                      if (shouldActivateSearchPossibleOnConfirm(draft, showPossiblesList, filteredPossibles.length) && activatePossible()) return;
                      if (applySearchSpaceAction(input!, draft, sessionActive)) releaseDraft();
                      return;
                    }
                    if (event.key === "Enter") {
                      event.preventDefault();
                      if (shouldActivateSearchPossibleOnConfirm(draft, showPossiblesList, filteredPossibles.length) && activatePossible()) return;
                      input!.onSubmit?.(draft);
                      releaseDraft();
                    }
                  }}
                  placeholder={actionPlaceholder}
                  disabled={input!.disabled}
                  aria-label={actionPlaceholder}
                />
                {inlineCompletion ? (
                  <div aria-hidden data-slot="search-inline-completion" className="text-foreground pointer-events-none col-start-1 row-start-1 flex h-medium min-w-0 items-center overflow-hidden p-single text-sm md:text-sm">
                    <span className="relative inline-flex min-w-0 truncate">
                      <span className="truncate text-transparent">{draft}</span>
                      <span className="absolute inset-0 truncate font-semibold text-foreground">{inlineCompletion.prefix}</span>
                    </span>
                    <span data-slot="search-inline-suffix" className="truncate text-muted-foreground">
                      {inlineCompletion.suffix}
                    </span>
                  </div>
                ) : null}
              </div>
              {hasPossibles ? (
                <Action
                  id={UI_WINDOW_SEARCH.suggestions}
                  aria-expanded={possiblesExpanded}
                  aria-label={suggestionsAriaLabel}
                  data-slot="search-possibles-toggle"
                  icon={possiblesExpanded ? <ChevronDownIcon className="size-small" /> : inline === "rtl" ? <ChevronLeftIcon className="size-small" /> : <ChevronRightIcon className="size-small" />}
                  onClick={() => setPossiblesExpanded((open) => !open)}
                />
              ) : null}
            </div>
          </PopoverAnchor>
          {hasPossibles ? (
            <PopoverContent data-slot="search-autocomplete" className="w-[min(100vw-1rem,28rem)] p-0" align="end" onOpenAutoFocus={(event) => event.preventDefault()}>
              <Command shouldFilter={false}>
                <CommandList>
                  {filteredPossibles.length ? (
                    <CommandGroup>
                      {filteredPossibles.map((item, index) => (
                        <CommandItem
                          key={item.id}
                          value={item.id}
                          data-active={index === activePossibleIndex ? "true" : undefined}
                          className={cn(index === activePossibleIndex && interactiveActiveFillClass)}
                          onPointerDown={(event) => {
                            event.preventDefault();
                            event.stopPropagation();
                            selectPossible(item);
                          }}
                          onMouseEnter={() => setActivePossibleIndex(index)}
                          onSelect={() => selectPossible(item)}
                        >
                          <span className="truncate">{searchHighlightedLabel(item.label, draft, item.detail)}</span>
                          {item.detail ? <span className="ms-auto truncate text-xs text-muted-foreground">{item.detail}</span> : null}
                        </CommandItem>
                      ))}
                    </CommandGroup>
                  ) : (
                    <CommandEmpty>{noMatchesLabel}</CommandEmpty>
                  )}
                </CommandList>
              </Command>
            </PopoverContent>
          ) : null}
        </Popover>
      </div>
    </LevelProvider>
  );
};

export { Search };

// #endregion 🔎️WindowSearch

/** @emoji 💬️ Top-aligned engagement: status heading, optional control, and option buttons. */
const Engagement: React.FC<EngagementProps> = ({ sessionActive = false, options, control, controls, status, className = "" }) => {
  const stepOptionsAriaLabel = useLabel(UI_ENGAGEMENT.actions);
  const primaryStepStatus = sessionActive ? status?.find((row) => row.id === "engagement-step") : undefined;
  const secondaryStatus = sessionActive ? status?.filter((row) => row.id !== "engagement-step") : status;

  const hasOptions = !!options?.length;
  const hasControl = !!control || !!controls?.length;
  const hasStatus = !!status?.length;

  if (!hasOptions && !hasControl && !hasStatus) return null;

  return (
    <LevelProvider level="pane">
      <div
        data-slot="engagement"
        data-session-active={sessionActive ? "true" : undefined}
        className={cn("pointer-events-auto flex w-full min-w-0 max-w-full flex-col gap-half", sessionActive && "bg-active-base ring-accent/35 rounded-sm ring-1 shadow-sm", className)}
      >
        {primaryStepStatus ? (
          <div data-slot="engagement-step-heading" className="text-foreground px-half text-sm font-medium leading-tight">
            {primaryStepStatus.content}
          </div>
        ) : null}
        {control ? <EngagementControlView control={control} /> : null}
        {controls?.map((row) => (
          <EngagementControlView key={row.id ?? row.label ?? row.kind} control={row} />
        ))}
        {secondaryStatus?.length ? (
          <div data-slot="engagement-status" className="flex flex-wrap items-center justify-center gap-single text-xs text-muted-foreground">
            {secondaryStatus.map((item) => (
              <span key={item.id} data-slot="engagement-status-item">
                {item.content}
              </span>
            ))}
          </div>
        ) : null}
        {hasOptions ? (
          <div
            data-slot="engagement-options"
            data-step-options={sessionActive ? "true" : undefined}
            className="flex flex-wrap items-center justify-center gap-half"
            role="group"
            aria-label={sessionActive ? stepOptionsAriaLabel || ENGAGEMENT_USER.actionsAria : ENGAGEMENT_USER.actionsAria}
          >
            <ButtonGroup id={UI_ENGAGEMENT.actions}>
              {options!.map((option) => {
                const actionLabel = normalizeEngagementActionText(option.label ?? "");
                const optionControlId = isInternalChromeControlId(option.id) ? undefined : option.id;
                return (
                  <ButtonGroupItem key={option.id} id={optionControlId} aria-label={actionLabel} icon={option.icon} text={actionLabel} className={cn(option.pressed && interactiveActiveFillClass)} onClick={option.onPress} disabled={option.disabled} />
                );
              })}
            </ButtonGroup>
          </div>
        ) : null}
      </div>
    </LevelProvider>
  );
};

export { Engagement };

// #endregion 🧭️Shell

// #region 🔍️Window Components

// #region 🌊️Window
import { Window, type WindowConfig } from "../../../../🧱️elements/🪟️Window/🟦️.tsx";
export { Window, type WindowConfig };
// #endregion 🌊️Window

// #region 🧫️Diagram
export { applyNodeChanges, Background, BackgroundVariant, BaseEdge, getBezierPath, Handle, Position, ReactFlow, ReactFlowProvider, SelectionMode, useInternalNode, useReactFlow, useStoreApi, ViewportPortal };
export type { Connection, ConnectionLineComponentProps, Edge, EdgeProps, EdgeTypes, MiniMapNodeProps, FlowNode as Node, NodeProps, NodeTypes, OnSelectionChangeParams, ReactFlowInstance, Connection as RFConnection };
import {
  DIAGRAM_UNIT,
  type DiagramLayoutDirection,
  type DiagramLayoutOptions,
  createDiagramForceSimulation,
  type DiagramForceConfig,
  defaultDiagramForceConfig,
  type DiagramProps,
  Diagram,
  useDiagramLayout,
  DiagramSkeleton,
} from "../../../../🧱️elements/🕸️Diagram/🟦️.tsx";
export { DIAGRAM_UNIT, type DiagramLayoutDirection, type DiagramLayoutOptions, type DiagramForceConfig, defaultDiagramForceConfig, type DiagramProps, Diagram, useDiagramLayout, DiagramSkeleton };
// #endregion 🧫️Diagram

// #region 📍️Scene
import {
  sceneFrameControlRef,
  SCENE_GIZMO_LABELS,
  GUMBALL_PLANE_OFFSET,
  GUMBALL_PLANE_SIZE,
  GUMBALL_RING_RADIUS,
  GUMBALL_PREVIEW_MIN_EXTENT,
  GUMBALL_PREVIEW_RING_RADIUS,
  GUMBALL_PREVIEW_DISK_RADIUS,
  gumballHandleRaycast,
  gumballRaycastOwnedAtClientPoint,
  gumballApplyHandleVisualMaterial,
  type GumballVec3,
  planeFromPointAndDirection,
  getPlanePosition,
  hasValidPlane,
  isGeometryFocusable,
  Geometry,
  resolveSceneGizmoSnapTarget,
  resolveSceneGizmoViewportPlacement,
  GUMBALL_PLANE_HANDLES,
  DEFAULT_GUMBALL_CONFIG,
  resolveGumballConfig,
  gumballHandleAllowedByPlane,
  gumballHandleGroupEnabled,
  gumballHandleEnabled,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  gumballPoseFromObject3D,
  applyGumballPose,
  gumballRayAxisParameter,
  gumballEyeFromPivot,
  gumballAxisDragPlaneNormal,
  gumballProjectRayOntoAxis,
  gumballRayPlanePoint,
  gumballAxisRotateAngle,
  gumballAxisScaleFactor,
  gumballSnapScalar,
  GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP,
  GUMBALL_DEFAULT_SHIFT_SCALE_SNAP,
  gumballEffectiveSnapValue,
  gumballResolveDragSnaps,
  gumballScalePlaneAxisIndices,
  gumballPlaneScaleCorner,
  gumballPlaneScaleFactors,
  gumballScaleAxisOffset,
  gumballPreviewWorldExtent,
  resolveGumballVisualPalette,
  gumballHandleVisualState,
  gumballResolveHandleVisual,
  gumballRayFromNdc,
  gumballPointerConsumesCanvasEventRef,
  gumballKindFromRaycastObject,
  gumballRaycastAtClientPoint,
  gumballRaycastKindAtClientPoint,
  UnifiedGumball,
  Scene,
  SceneSkeleton,
  type SceneGeometry,
  type TransformableGeometry,
  type PlaneTransformDelta,
  type OnPlaneUpdate,
  type OnMultiPlaneUpdate,
  type GumballPose,
  type GumballHandleKind,
  type GumballPlaneId,
  type GumballConfig,
  type GumballVisualPalette,
  type GumballHandleVisualState,
  type UnifiedGumballProps,
} from "../../../../🧱️elements/🎬️Scene/🟦️.tsx";
export {
  sceneFrameControlRef,
  SCENE_GIZMO_LABELS,
  GUMBALL_PLANE_OFFSET,
  GUMBALL_PLANE_SIZE,
  GUMBALL_RING_RADIUS,
  GUMBALL_PREVIEW_MIN_EXTENT,
  GUMBALL_PREVIEW_RING_RADIUS,
  GUMBALL_PREVIEW_DISK_RADIUS,
  gumballHandleRaycast,
  gumballRaycastOwnedAtClientPoint,
  gumballApplyHandleVisualMaterial,
  type GumballVec3,
  planeFromPointAndDirection,
  getPlanePosition,
  hasValidPlane,
  isGeometryFocusable,
  Geometry,
  resolveSceneGizmoSnapTarget,
  resolveSceneGizmoViewportPlacement,
  GUMBALL_PLANE_HANDLES,
  DEFAULT_GUMBALL_CONFIG,
  resolveGumballConfig,
  gumballHandleAllowedByPlane,
  gumballHandleGroupEnabled,
  gumballHandleEnabled,
  gumballConfigVisible,
  gumballHandleKindToTransformMode,
  gumballPoseFromObject3D,
  applyGumballPose,
  gumballRayAxisParameter,
  gumballEyeFromPivot,
  gumballAxisDragPlaneNormal,
  gumballProjectRayOntoAxis,
  gumballRayPlanePoint,
  gumballAxisRotateAngle,
  gumballAxisScaleFactor,
  gumballSnapScalar,
  GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP,
  GUMBALL_DEFAULT_SHIFT_SCALE_SNAP,
  gumballEffectiveSnapValue,
  gumballResolveDragSnaps,
  gumballScalePlaneAxisIndices,
  gumballPlaneScaleCorner,
  gumballPlaneScaleFactors,
  gumballScaleAxisOffset,
  gumballPreviewWorldExtent,
  resolveGumballVisualPalette,
  gumballHandleVisualState,
  gumballResolveHandleVisual,
  gumballRayFromNdc,
  gumballPointerConsumesCanvasEventRef,
  gumballKindFromRaycastObject,
  gumballRaycastAtClientPoint,
  gumballRaycastKindAtClientPoint,
  UnifiedGumball,
  Scene,
  SceneSkeleton,
  type SceneGeometry,
  type TransformableGeometry,
  type PlaneTransformDelta,
  type OnPlaneUpdate,
  type OnMultiPlaneUpdate,
  type GumballPose,
  type GumballHandleKind,
  type GumballPlaneId,
  type GumballConfig,
  type GumballVisualPalette,
  type GumballHandleVisualState,
  type UnifiedGumballProps,
};
// #endregion 📍️Scene

// #region 🛎️Table
import { type SortDirection, type TableColumn, type HierarchicalRowData, type DragDropConfig, type TableProps, Table, type TableSkeletonProps, TableSkeleton } from "../../../../🧱️elements/📊️Table/🟦️.tsx";
export { type SortDirection, type TableColumn, type HierarchicalRowData, type DragDropConfig, type TableProps, Table, type TableSkeletonProps, TableSkeleton };
// #endregion 🛎️Table

// #region 🗄️HistoryTable
import { type HistoryColumnAuthor, type HistoryColumn, type HistoryTableProps, HistoryTable } from "../../../../🧱️elements/🕰️HistoryTable/🟦️.tsx";
export { type HistoryColumnAuthor, type HistoryColumn, type HistoryTableProps, HistoryTable };
// #endregion 🗄️HistoryTable

// #region 📁️VirtualFileSystem
import {
  VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
  getVirtualFileSystemOrderedRowIds,
  normalizeVirtualFileSystemSelectedRowIds,
  getVirtualFileSystemNextSelectionState,
  resolveVirtualFileSystemFileNodeKind,
  resolveVirtualFileSystemDescriptorKind,
  resolveVirtualFileSystemDescriptorBinding,
  buildVirtualFileSystemDescriptorValues,
  formatVirtualFileSystemTime,
  renderVirtualFileSystemDescriptorCell,
  buildVirtualFileSystemDescriptorColumns,
  resolveVirtualFileSystemSchemaIcon,
  virtualFileSystemKindIcon,
  isVirtualFileSystemRemoteIcon,
  buildVirtualFileSystemVisibleRows,
  VirtualFileSystem,
  type DescriptorKind,
  type FileNodeDescriptor,
  type FileNodeKind,
  type FileNodeDescriptorValue,
  type VirtualFileSystemSchema,
  type FileNode,
  type VirtualFileSystemNode,
  type VirtualFileSystemRow,
  type VirtualFileSystemProps,
} from "../../../../🧱️elements/⚙️VirtualFileSystem/🟦️.tsx";
export {
  VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
  getVirtualFileSystemOrderedRowIds,
  normalizeVirtualFileSystemSelectedRowIds,
  getVirtualFileSystemNextSelectionState,
  resolveVirtualFileSystemFileNodeKind,
  resolveVirtualFileSystemDescriptorKind,
  resolveVirtualFileSystemDescriptorBinding,
  buildVirtualFileSystemDescriptorValues,
  formatVirtualFileSystemTime,
  renderVirtualFileSystemDescriptorCell,
  buildVirtualFileSystemDescriptorColumns,
  resolveVirtualFileSystemSchemaIcon,
  virtualFileSystemKindIcon,
  isVirtualFileSystemRemoteIcon,
  buildVirtualFileSystemVisibleRows,
  VirtualFileSystem,
  type DescriptorKind,
  type FileNodeDescriptor,
  type FileNodeKind,
  type FileNodeDescriptorValue,
  type VirtualFileSystemSchema,
  type FileNode,
  type VirtualFileSystemNode,
  type VirtualFileSystemRow,
  type VirtualFileSystemProps,
};
// #endregion 📁️VirtualFileSystem

// #region ⚙️Canvas
import {
  Canvas,
  panelGhostSessionBridge,
  setPanelGhostSessionBridge,
  MODE_CANVAS_INSET_CLASS,
  modeCollectWindowIds,
  HorizontalWindows,
  VerticalWindows,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  MODE_TEMPLATE_PREVIEW_WINDOW_ID,
  beginWindowTemplateDrag,
  endWindowTemplateDrag,
  readActiveWindowTemplateDragSession,
  windowTemplatePointerDragRef,
  beginWindowTemplatePointerDrag,
  cancelWindowTemplatePointerDrag,
  windowTemplatePaletteTreeDragController,
  modeAxisIsPerpendicularChild,
  modePerpendicularJoinSeparators,
  modeJoinCornerSpecsForSeparator,
  modeJoinCornerSpecsForCrossSeparator,
  MODE_JOIN_CORNER_TOUCH_EPS,
  resolveJoinCornerPeerCrossAxes,
  applyAxisResizeDelta,
  resolveJoinCornerResizeDeltas,
  applyAxisGroupLayoutDelta,
  modeAxisGroupLayout,
  applyModeJoinCornerResize,
  insertWindowAtDropZone,
  Mode,
  removeWindowFromLayout,
  resolveStackPathForWindowId,
  splitWithWindow,
  splitWithStack,
  extractStackFromLayout,
  applyModeDrop,
  reconcileWindows,
  normalizeLayoutToStacks,
  collapseLayout,
  computeModeDropZone,
  computeModeSplitPreviewInBody,
  resolveModeSplitSideInBody,
  computeTabInsertPreview,
  modeDockOutLayout,
  modeDockTabsWithInsertPreview,
  modeDockDragInsertTabs,
  mergeStackTabsIntoStack,
  resolveModeTabInsertPreview,
  WINDOW_STACK_CORNERS,
  resolveWindowCorner,
  modeStackTabsByCorner,
  insertWindowAsTabAtCorner,
  setWindowCornerInLayout,
  App,
  Ui,
  type ModeWindowDescriptor,
  type WindowTemplateDragSession,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
  type ModeProps,
  type ModeJoinCornerCrossAxis,
  type AppModeDescriptor,
  type AppProps,
  type UiAppDescriptor,
  type UiProps,
} from "../../../../🧱️elements/🎨️Canvas/🟦️.tsx";
export {
  Canvas,
  panelGhostSessionBridge,
  setPanelGhostSessionBridge,
  MODE_CANVAS_INSET_CLASS,
  modeCollectWindowIds,
  HorizontalWindows,
  VerticalWindows,
  COMPOSE_WINDOW_TEMPLATE_MIME,
  MODE_TEMPLATE_PREVIEW_WINDOW_ID,
  beginWindowTemplateDrag,
  endWindowTemplateDrag,
  readActiveWindowTemplateDragSession,
  windowTemplatePointerDragRef,
  beginWindowTemplatePointerDrag,
  cancelWindowTemplatePointerDrag,
  windowTemplatePaletteTreeDragController,
  modeAxisIsPerpendicularChild,
  modePerpendicularJoinSeparators,
  modeJoinCornerSpecsForSeparator,
  modeJoinCornerSpecsForCrossSeparator,
  MODE_JOIN_CORNER_TOUCH_EPS,
  resolveJoinCornerPeerCrossAxes,
  applyAxisResizeDelta,
  resolveJoinCornerResizeDeltas,
  applyAxisGroupLayoutDelta,
  modeAxisGroupLayout,
  applyModeJoinCornerResize,
  insertWindowAtDropZone,
  Mode,
  removeWindowFromLayout,
  resolveStackPathForWindowId,
  splitWithWindow,
  splitWithStack,
  extractStackFromLayout,
  applyModeDrop,
  reconcileWindows,
  normalizeLayoutToStacks,
  collapseLayout,
  computeModeDropZone,
  computeModeSplitPreviewInBody,
  resolveModeSplitSideInBody,
  computeTabInsertPreview,
  modeDockOutLayout,
  modeDockTabsWithInsertPreview,
  modeDockDragInsertTabs,
  mergeStackTabsIntoStack,
  resolveModeTabInsertPreview,
  WINDOW_STACK_CORNERS,
  resolveWindowCorner,
  modeStackTabsByCorner,
  insertWindowAsTabAtCorner,
  setWindowCornerInLayout,
  App,
  Ui,
  type ModeWindowDescriptor,
  type WindowTemplateDragSession,
  type ModeCanvasDropTarget,
  type WindowTemplateDropPayload,
  type ModeProps,
  type ModeJoinCornerCrossAxis,
  type AppModeDescriptor,
  type AppProps,
  type UiAppDescriptor,
  type UiProps,
};
// #endregion ⚙️Canvas

if (import.meta.vitest) {
  const { registerTests1 } = await import("../../../../🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx");
  await registerTests1(import.meta.vitest, { App, Button, CELEBRATE_STAMP_DURATION_MS, COMPACT_UI_DRIVER, COMPOSE_WINDOW_TEMPLATE_MIME, Canvas, CanvasPickMenu, ContextMenu, ContextMenuController, DEFAULT_GUMBALL_CONFIG, DEFAULT_UI_DRIVER, Engagement, FlowProvider, Footer, GLASS_OVERLAY_BOX_CLASS, GUMBALL_DEFAULT_SHIFT_ROTATION_SNAP, GUMBALL_DEFAULT_SHIFT_SCALE_SNAP, GUMBALL_PLANE_OFFSET, GUMBALL_PLANE_SIZE, GUMBALL_PREVIEW_DISK_RADIUS, GUMBALL_PREVIEW_MIN_EXTENT, GUMBALL_PREVIEW_RING_RADIUS, GUMBALL_RING_RADIUS, ICONS, INTRODUCTION_DEMO_IDLE_THRESHOLD_MS, INTRODUCTION_INFO_BOX_GAP_PX, Icon, Input, LEVELS, Label, Layout, LevelProvider, MODE_CANVAS_INSET_CLASS, Mode, Navbar, NotFound, OrthographicCamera, Pane, PaneHost, Panel, PanelChromeTabBar, PanelDockProvider, PanelTabBar, PerspectiveCamera, Popover, PopoverContent, PopoverTrigger, React, RouteLink, Scrollable, Search, ShellScopeProvider, SortableTreeItems, Surface, THREE, TREE_SECTION_REORDER_MIME, TextSelectionContextMenuHost, Toggle, Tree, TreeContext, TreeItem, UIIntroduction, UI_CHROME_LOCALE_STORAGE_KEY, UI_ELEMENT_REGISTRY, Ui, UiDriverProvider, UiMobileProvider, WINDOW_SILHOUETTE_BORDER_KINDS, WINDOW_SILHOUETTE_GEOMETRY_SCHEMA, WINDOW_SILHOUETTE_PATH_INSET, Window, WindowChrome, WindowMeasureTreeGroup, WindowMeasureTreeLeaf, WindowMeasuresTree, applyAxisGroupLayoutDelta, applyModeDrop, applyModeJoinCornerResize, applySearchSpaceAction, assertUniqueIconConceptAssignments, beginWindowTemplateDrag, beginWindowTemplatePointerDrag, borderNormalClass, buildTextSelectionContextMenuItems, cancelWindowTemplatePointerDrag, celebrateAllElements, celebrateElement, celebrateElements, childElementId, chromeHostedOpenPanelPositionStyle, chromeStatusBorderClass, clampIntroductionInfoBoxPosition, clampSliderValuesToReady, classifyIconSelectorMode, cn, computeModeDropZone, computeModeSplitPreviewInBody, computeTabDockDropZone, computeTabInsertPreview, createDOMEventBinding, createDiagramForceSimulation, createEvenWindowLayout, createMemoryStoragePort, createShellScope, createWindowSilhouetteGeometry, decodeIcon, defaultDiagramForceConfig, detectShellLocale, elementIdSegment, elementIdSelector, encodeIcon, endWindowTemplateDrag, engagementActionTokenEquals, filterSearchPossibles, flowFromAnchor, formatNumber, glassClass, gumballApplyHandleVisualMaterial, gumballAxisRotateAngle, gumballAxisScaleFactor, gumballConfigVisible, gumballEffectiveSnapValue, gumballHandleAllowedByPlane, gumballHandleEnabled, gumballHandleKindToTransformMode, gumballHandleRaycast, gumballHandleVisualState, gumballKindFromRaycastObject, gumballPlaneScaleCorner, gumballPlaneScaleFactors, gumballPointerConsumesCanvasEventRef, gumballPreviewWorldExtent, gumballProjectRayOntoAxis, gumballRayAxisParameter, gumballRayFromNdc, gumballRayPlanePoint, gumballRaycastOwnedAtClientPoint, gumballResolveDragSnaps, gumballResolveHandleVisual, gumballScaleAxisOffset, gumballScalePlaneAxisIndices, gumballSnapScalar, iconShotFrameClass, iconShotFrameStyle, iconSvgMarkup, initUiLocaleSync, insertWindowAsTabAtCorner, insertWindowAtDropZone, installElementsSurfaceBrowserDefaultSuppression, introductionDemoArcPoint, introductionDemoResolveVisual, introductionPointRelativeToHost, introductionRectRelativeToHost, isContextMenuPointerTarget, isElementId, isPointerEventOnDomTextSelection, isSearchSuggestionActionTarget, isUiTypingTarget, isWindowChromeIntroducedTarget, measureWindowSilhouetteMetrics, mergeTreeSectionOrder, modeCollectWindowIds, modeDockChromeGridPlacement, modeDockOutLayout, modeDockTabLabelClassName, modeDockTabsWithInsertPreview, modeJoinCornerSpecsForCrossSeparator, modeJoinCornerSpecsForSeparator, modePerpendicularJoinSeparators, modeStackTabsByCorner, navigateOwnedRoute, ndcToViewportPoint, nearestAnchor, normalizeEngagementActionText, normalizeWindowSilhouetteChips, normalizeWindowSilhouetteMetrics, parseOwnedRouteTarget, parseUiTheme, polylinePointAt, progressPanelTabSelection, publishShellNavbarTrailingEndWidthPx, rankFuzzyItems, reactHostPort, readActiveWindowTemplateDragSession, readDomTextSelection, readResizableJoinCornerSpec, readScrollerContentOverflows, reconcileWindows, referenceMediaKindFromUrl, registerIntroductionSurfaceResolver, removeWindowFromLayout, renderToStaticMarkup, resolveCatalogIconSvg, resolveGumballConfig, resolveGumballVisualPalette, resolveIntroductionPlacement, resolveIntroductionPoint, resolveJoinCornerPeerCrossAxes, resolveModeSplitSideInBody, resolveSliderDraftClear, resolveTranslationLabel, resolveWindowSilhouetteBorderKind, routeWindowSearchEscape, routeWindowSearchSpace, sampleBezierSegments, searchActiveInlineCompletion, searchControlledLineV1, searchInlineCompletion, semioTheme, setActiveUiTheme, shellFloorFillClass, shellFloorPaints, shellNavbarTrailingEndWidthByRoot, shortcodeCatalogKey, shortcodeEmoji, shouldActivateSearchPossibleOnConfirm, shouldRouteKeysToWindowSearch, singleTreeLeaf, sliderValuesMatch, splitIntroductionBodyParagraphs, splitWithWindow, sunPositionFromAzimuthElevation, surfaceClass, uiDataLabel, uiI18n, uiSpacingPx, useFirstDraggableElementAlias, useFlow, useIntroductionPointerIdle, useLevel, usePaneSlot, useSurface, windowChromeTitleChipClass, windowMeasuresDefaultWidthPx, windowSilhouetteBorderPaint, windowSilhouetteContains, windowSilhouetteOutline, windowSilhouetteOutlineViolations, windowSilhouettePath, windowTemplatePaletteTreeDragController, windowTemplatePointerDragRef }, { directory: import.meta.dir, url: import.meta.url });
}

// #endregion 🔍️Window Components

// #region 🗿️Framework Re-exports

// Re-exports of common libraries used alongside UI primitives.
// Workbench shell types and chrome live in `@semio-tech/framework-platform-core` / `@semio-tech/framework-platform-renderer-react`.

// #region 🌩️DnD Kit
export { closestCenter, DndContext, DragOverlay, PointerSensor, pointerWithin, rectIntersection, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
export type { DragEndEvent, DragOverEvent, DragStartEvent } from "@dnd-kit/core";
export { arrayMove, SortableContext, useSortable, verticalListSortingStrategy } from "@dnd-kit/sortable";
export { CSS as DndCSS } from "@dnd-kit/utilities";
// #endregion 🌩️DnD Kit

// #region 📰️Three.js
export { Select as DreiSelect, Edges, GizmoHelper, GizmoViewport, Grid, Line, OrbitControls, Sphere, useFBX, useGLTF } from "@react-three/drei";
export { Canvas as ThreeCanvas, useFrame, useLoader, useThree } from "@react-three/fiber";
export * as THREE from "three";
export { OBJLoader } from "three/addons/loaders/OBJLoader.js";
// #endregion 📰️Three.js

// #region 🎽️XY Flow (additions not already exported inline)
export { ConnectionMode, MiniMap } from "@xyflow/react";
// #endregion 🎽️XY Flow

// #region 🖋️State Management
export { assign, createActor, fromCallback, setup, type ActorRefFrom, type AnyActorRef, type SnapshotFrom } from "xstate";
// #endregion 🖋️State Management

// #region 🗿️I18n
export { useTranslation };
// #endregion 🗿️I18n

// #region 🌙️Hotkeys
export { useHotkeys };
// #endregion 🌙️Hotkeys

// #region ⛅️Date
// #endregion ⛅️Date

// #region 🔔️Search
export { rankFuzzyItems };
export type { FuzzySearchField, FuzzySearchOptions, FuzzySearchResult };
// #endregion 🔔️Search

// #region 🌨️Styling
export { styleVariants } from "../../../../🔨️modules/🧬️style-variants/🟦️.ts";
export type { StyleCompoundVariant, StyleVariantCompiler, StyleVariantConfiguration, StyleVariantProps, StyleVariantSchema, StyleVariantSelection } from "../../../../🔨️modules/🧬️style-variants/🟦️.ts";
// #endregion 🌨️Styling

// #region 📮️Resizable Panels
export * as ResizablePrimitive from "react-resizable-panels";if (import.meta.vitest) {
  const { registerTests2 } = await import("../../../../🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx");
  await registerTests2(import.meta.vitest, { applyChromeRevealAtPoint, applyDockSkeleton, applyElementsSurfaceChrome, bootstrapElementsSurfaceChromeDocument, borderNormalBottomClass, borderNormalClass, borderNormalTopClass, buildVirtualFileSystemDescriptorColumns, buildVirtualFileSystemVisibleRows, Button, ButtonGroup, ButtonGroupItem, catalogueTreeDragController, CheckIcon, ChevronDownIcon, ChevronLeftIcon, ChevronRightIcon, ChevronUpIcon, cn, COLLAPSED_FIELD_ELLIPSIS, Command, CommandItem, CommandList, COMPACT_UI_DRIVER, composeControlKeybindings, composeTutorialUi, computeTabDockDropZone, ControlTree, createBrowserStoragePort, createTreeHighlightStore, createTreeSelectionStore, createTutorialClock, DEFAULT_UI_DRIVER, defaultControlRenderer, deriveTreeDragRoles, dockSkeletonOf, dockSkeletonsEqual, DragHandle, FindInViewIcon, fitCollapsedFieldText, flowChevronIconName, FlowProvider, Footer, formatControlTooltipText, formatKeybindingShortcut, formatTutorialTime, formatVirtualFileSystemTime, getElementById, getTreeItemOrderedIds, getTreeNextSelectionState, getTreeSiblingGapPx, getVirtualFileSystemNextSelectionState, GhostProvider, GhostRegionShell, HistoryTable, humanizeControlId, humanizeControlSegment, Icon, Input, interactionMergeFromModifiers, interpolateTutorialCamera, isInternalChromeControlId, isPanelTabInSubtree, isTreeReorderDragEvent, Label, LevelProvider, loadingBorderActiveClass, loadingBorderClass, loadingBorderStateClass, markGhostTreeInteraction, measureWindowSilhouetteMetrics, Mode, modeDockTabClassName, moveTabInDock, moveTreeUnitInDock, Navbar, NavbarExampleSelect, navbarFillItem, normalizeTreeSelectedIds, Pane, PaneHost, Panel, PanelChromeTabBar, PanelDockContext, panelKindFromPanelToggleControlId, PanelRightIcon, panelTabButtonDividerClass, parseUiDriver, PresenceBar, presenceColor, presenceCssVar, pruneEmptyPanelBranches, React, readStoredUiChromeLayout, reconcileActivePath, renderToStaticMarkup, resetElementsSurfaceChromeForTests, resolveCollapsedFieldDisplayState, resolveControlLabelId, resolveSceneGizmoSnapTarget, resolveSceneGizmoViewportPlacement, resolveTranslationLabel, resolveTreeDropPosition, resolveUiDriver, resolveVirtualFileSystemSchemaIcon, resolveWindowSilhouetteBorderKind, Ribbon, RibbonItem, RibbonZone, Ring, SCENE_GIZMO_LABELS, Search, SearchIcon, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, serializeUiDriver, shellChromeBorderClass, shellChromeFrameLayerClass, shouldBeginAutomaticGhostInteraction, shouldDispatchTreeRowPointerLeave, singleTreeLeaf, Slider, Stepper, syncTreeSelectionPath, Table, Textarea, THREE, Toggle, ToggleGroup, Tree, TreeAlignedRow, TreeCheckbox, treeCompactSiblingGapPx, TreeContent, TreeContext, treeFoldChevronIcon, TreeItem, treeItemSecondaryTextClassName, TreeRow, TreeRowAlignmentContext, treeRowChromeClasses, treeRowChromeContentFillClasses, treeRowChromeShellClasses, treeRowDragPayloadAttributes, TreeSection, TreeStateProvider, TutorialBar, tutorialCameraAt, tutorialCuesBetween, tutorialSlice, UI_CHROME_LAYOUT_STORAGE_KEY, uiDataLabel, UiDriverProvider, uiI18n, UIIntroduction, UiKeybindingsProvider, useCanvasAppearanceSync, validateTutorial, VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS, VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, VirtualFileSystem, waitingBorderActiveClass, waitingBorderClass, waitingBorderStateClass, Window, WINDOW_PANE_MEASURES_ICON, WindowMeasuresTree, windowMeasureToggleClass, windowMeasureToggleCompactClass, WindowMeasureTreeGroup, WindowMeasureTreeLeaf, WindowPaneChromeToggle, windowSilhouettePath, writeStoredUiChromeLayout }, { directory: import.meta.dir, url: import.meta.url });
}
