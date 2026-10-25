// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🔌️Ports/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Box3, BoxGeometry, BufferAttribute, BufferGeometry, Camera, CanvasTexture, Color, DoubleSide, EdgesGeometry, Float32BufferAttribute, Group, HalfFloatType, LineBasicMaterial, LineSegments, LinearFilter, LinearSRGBColorSpace, MOUSE, Matrix4, Mesh, MeshBasicMaterial, MeshStandardMaterial, Object3D, OrthographicCamera as ThreeOrthographicCamera, PerspectiveCamera as ThreePerspectiveCamera, Plane, PlaneGeometry, Quaternion, RGBAFormat, Ray, Raycaster, Scene, ShaderMaterial, Texture, Vector2, Vector3, WebGLRenderTarget } from "three";
import { Clone } from "@react-three/drei/core/Clone.js";
import { GizmoHelper } from "@react-three/drei/core/GizmoHelper.js";
import { GizmoViewport } from "@react-three/drei/core/GizmoViewport.js";
import { Grid } from "@react-three/drei/core/Grid.js";
import { Line as DreiLine } from "@react-three/drei/core/Line.js";
import { OrbitControls } from "@react-three/drei/core/OrbitControls.js";
import { OrthographicCamera } from "@react-three/drei/core/OrthographicCamera.js";
import { Outlines } from "@react-three/drei/core/Outlines.js";
import { PerspectiveCamera } from "@react-three/drei/core/PerspectiveCamera.js";
import { Text as DreiText } from "@react-three/drei/core/Text.js";
import { TransformControls } from "@react-three/drei/core/TransformControls.js";
import { useGLTF } from "@react-three/drei/core/Gltf.js";
import { Canvas as ThreeCanvas, createPortal as r3fCreatePortal, useFrame, useLoader, useStore, useThree } from "@react-three/fiber";
import { ReactFlow, ReactFlowProvider } from "@xyflow/react";
// #endregion 🔌️Adapters

//#region 🔌️Ports
export { reactHostPort, setReactHostPort, type ReactHostPort } from "./⚛️react/🟦️.ts";

export { interactiveJobPort, setInteractiveJobPort, type InteractiveJobDescriptor, type InteractiveJobLease, type InteractiveJobPage, type InteractiveJobPort, type InteractiveJobPortSnapshot, type InteractiveJobTerminal } from "./📡️interactive-jobs/🟦️.ts";

/** 🧊️ The three.js constructors and constants the scene port hands its consumers, each one named — an external library
 * only behind this interface, re-exported explicitly, never its whole namespace (AGENTS.md). */
export const SCENE_THREE_BINDINGS = Object.freeze({ Box3, BoxGeometry, BufferAttribute, BufferGeometry, Camera, CanvasTexture, Color, DoubleSide, EdgesGeometry, Float32BufferAttribute, Group, HalfFloatType, LineBasicMaterial, LineSegments, LinearFilter, LinearSRGBColorSpace, MOUSE, Matrix4, Mesh, MeshBasicMaterial, MeshStandardMaterial, Object3D, OrthographicCamera: ThreeOrthographicCamera, PerspectiveCamera: ThreePerspectiveCamera, Plane, PlaneGeometry, Quaternion, RGBAFormat, Ray, Raycaster, Scene, ShaderMaterial, Texture, Vector2, Vector3, WebGLRenderTarget });

/** 🔱️ The shape of {@link SCENE_THREE_BINDINGS}: what a test double of the scene port's `three` member provides. */
export type SceneThreeBindings = typeof SCENE_THREE_BINDINGS;

/** 🎬️ Scene host surface for puzzle/cad R3F + three.js (implemented by 🔌️Adapters). */
export interface SceneHostPort {
  readonly fiber: {
    readonly canvas: typeof ThreeCanvas;
    readonly createPortal: typeof r3fCreatePortal;
    readonly useFrame: typeof useFrame;
    readonly useLoader: typeof useLoader;
    readonly useStore: typeof useStore;
    readonly useThree: typeof useThree;
  };
  readonly drei: {
    readonly Clone: typeof Clone;
    readonly GizmoHelper: typeof GizmoHelper;
    readonly GizmoViewport: typeof GizmoViewport;
    readonly Line: typeof DreiLine;
    readonly OrbitControls: typeof OrbitControls;
    readonly OrthographicCamera: typeof OrthographicCamera;
    readonly Outlines: typeof Outlines;
    readonly PerspectiveCamera: typeof PerspectiveCamera;
    readonly Text: typeof DreiText;
    readonly TransformControls: typeof TransformControls;
    readonly useGLTF: typeof useGLTF;
    readonly Grid: typeof Grid;
  };
  readonly three: SceneThreeBindings;
}

/** 🔌️ Default scene host port wired to fiber/drei/three adapters — inject a test double via
 * {@link setSceneHostPort} before render. */
export let sceneHostPort: SceneHostPort = {
  fiber: {
    canvas: ThreeCanvas,
    createPortal: r3fCreatePortal,
    useFrame,
    useLoader,
    useStore,
    useThree,
  },
  drei: {
    Clone,
    GizmoHelper,
    GizmoViewport,
    Line: DreiLine,
    OrbitControls,
    OrthographicCamera,
    Outlines,
    PerspectiveCamera,
    Text: DreiText,
    TransformControls,
    useGLTF,
    Grid,
  },
  three: SCENE_THREE_BINDINGS,
};

/** 🪢️ ESM importers cannot assign an imported `export let` binding directly, so this is the only
 * way to swap {@link sceneHostPort} from outside this module (the barrel's `configureHostPorts` calls
 * this instead of a direct assignment). Returns the previously-installed port. */
export function setSceneHostPort(port: SceneHostPort): SceneHostPort {
  const previous = sceneHostPort;
  sceneHostPort = port;
  return previous;
}

/** 🌊️ Flow host surface for diagram runtime. */
export interface FlowHostPort {
  readonly flow: typeof ReactFlow;
  readonly provider: typeof ReactFlowProvider;
}

/** 🔗️ Default diagram host port wired to @xyflow/react. */
export let flowHostPort: FlowHostPort = {
  flow: ReactFlow,
  provider: ReactFlowProvider,
};

/** 🪝️ ESM-safe setter for flowHostPort. */
export function setFlowHostPort(port: FlowHostPort): FlowHostPort {
  const previous = flowHostPort;
  flowHostPort = port;
  return previous;
}

/** 🏷️ JSX alias for diagram flow host. */
export const HostReactFlow = flowHostPort.flow;
/** 🔖️ JSX alias for diagram flow provider. */
export const HostReactFlowProvider = flowHostPort.provider;
/** 🖥️ JSX alias for the three.js canvas host. */
export const HostThreeCanvas = sceneHostPort.fiber.canvas;
//#endregion 🔌️Ports
