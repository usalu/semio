// #region 🧲️Header
// 💻️ framework/ui/elements/🫀️core/🔌️Ports/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import * as THREE from "three";
import { Clone, GizmoHelper, GizmoViewport, Grid, Line as DreiLine, OrbitControls, OrthographicCamera, Outlines, PerspectiveCamera, Text as DreiText, TransformControls, useGLTF } from "@react-three/drei";
import { Canvas as ThreeCanvas, createPortal as r3fCreatePortal, useFrame, useStore, useThree } from "@react-three/fiber";
import { ReactFlow, ReactFlowProvider } from "@xyflow/react";
// #endregion 🔌️Adapters

//#region 🔌️Ports
/**
 * 🆔️ `ReactHostPort` + the live `reactHostPort` binding, split out of the ui-react barrel into its own
 * `🧱️elements/` file (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE) — NOT deferred to a later
 * "core extraction" pass like the rest of `🔌️Ports`/`🔌️PortWiring`, because several already-extracted
 * elements (`Avatar`, `Tree`, `Panel`, `ToggleGroup`, `Canvas`, `ActionGroup`, `Scrollable`) call
 * `reactHostPort.forwardRef(...)`/`.memo(...)` at MODULE TOP LEVEL, not inside a component body. A
 * module-top-level read of a binding re-exported by the barrel (which in turn imports these same
 * elements) is a genuine ES-module circular-import initialization-order bug: whichever module the
 * loader reaches first in the cycle sees the other's `export let` still uninitialized. Elements that only
 * read `reactHostPort.*` inside function bodies (the overwhelming majority) are unaffected — evaluation
 * happens at render time, long after both modules have finished loading — so only THIS binding needed to
 * move early; `flowHostPort`/`threeHostPort`/`iconRenderPort`/`configureHostPorts` stay in the barrel for
 * now (no top-level consumer needs them yet, same deferred-core posture as everything else under
 * barrel-interim imports). `sceneHostPort` (below) needed the identical treatment once `Scene`
 * was extracted — same class of bug, same fix.
 */
export interface ReactHostPort {
  readonly createElement: typeof React.createElement;
  readonly useState: typeof React.useState;
  readonly useEffect: typeof React.useEffect;
  readonly useMemo: typeof React.useMemo;
  readonly useCallback: typeof React.useCallback;
  readonly useRef: typeof React.useRef;
  readonly useContext: typeof React.useContext;
  readonly useLayoutEffect: typeof React.useLayoutEffect;
  readonly useSyncExternalStore: typeof React.useSyncExternalStore;
  readonly useId: typeof React.useId;
  readonly useImperativeHandle: typeof React.useImperativeHandle;
  readonly memo: typeof React.memo;
  readonly forwardRef: typeof React.forwardRef;
  readonly lazy: typeof React.lazy;
  readonly Suspense: typeof React.Suspense;
  readonly createContext: typeof React.createContext;
}

/** @emoji 🔌️ Default host port — inject a test double via {@link setReactHostPort} before render. */
export let reactHostPort: ReactHostPort = {
  createElement: React.createElement,
  useState: React.useState,
  useEffect: React.useEffect,
  useMemo: React.useMemo,
  useCallback: React.useCallback,
  useRef: React.useRef,
  useContext: React.useContext,
  useLayoutEffect: React.useLayoutEffect,
  useSyncExternalStore: React.useSyncExternalStore,
  useId: React.useId,
  useImperativeHandle: React.useImperativeHandle,
  memo: React.memo,
  forwardRef: React.forwardRef,
  lazy: React.lazy,
  Suspense: React.Suspense,
  createContext: React.createContext,
};

/** @emoji 🔌️ ESM importers cannot assign an imported `export let` binding directly, so this is the only
 * way to swap {@link reactHostPort} from outside this module (the barrel's `configureHostPorts` calls
 * this instead of a direct assignment). Returns the previously-installed port. */
export function setReactHostPort(port: ReactHostPort): ReactHostPort {
  const previous = reactHostPort;
  reactHostPort = port;
  return previous;
}

/** @emoji 🧊️ Scene host surface for puzzle/cad R3F + three.js (implemented by 🔌️Adapters). */
export interface SceneHostPort {
  readonly fiber: {
    readonly canvas: typeof ThreeCanvas;
    readonly createPortal: typeof r3fCreatePortal;
    readonly useFrame: typeof useFrame;
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
  readonly three: typeof THREE;
}

/** @emoji 🔌️ Default scene host port wired to fiber/drei/three adapters — inject a test double via
 * {@link setSceneHostPort} before render. */
export let sceneHostPort: SceneHostPort = {
  fiber: {
    canvas: ThreeCanvas,
    createPortal: r3fCreatePortal,
    useFrame,
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
  three: THREE,
};

/** @emoji 🔌️ ESM importers cannot assign an imported `export let` binding directly, so this is the only
 * way to swap {@link sceneHostPort} from outside this module (the barrel's `configureHostPorts` calls
 * this instead of a direct assignment). Returns the previously-installed port. */
export function setSceneHostPort(port: SceneHostPort): SceneHostPort {
  const previous = sceneHostPort;
  sceneHostPort = port;
  return previous;
}

/** @emoji Flow host surface for diagram runtime. */
export interface FlowHostPort {
  readonly flow: typeof ReactFlow;
  readonly provider: typeof ReactFlowProvider;
}

/** @emoji Default diagram host port wired to @xyflow/react. */
export let flowHostPort: FlowHostPort = {
  flow: ReactFlow,
  provider: ReactFlowProvider,
};

/** @emoji ESM-safe setter for flowHostPort. */
export function setFlowHostPort(port: FlowHostPort): FlowHostPort {
  const previous = flowHostPort;
  flowHostPort = port;
  return previous;
}

/** @emoji JSX alias for diagram flow host. */
export const HostReactFlow = flowHostPort.flow;
/** @emoji JSX alias for diagram flow provider. */
export const HostReactFlowProvider = flowHostPort.provider;
//#endregion 🔌️Ports
