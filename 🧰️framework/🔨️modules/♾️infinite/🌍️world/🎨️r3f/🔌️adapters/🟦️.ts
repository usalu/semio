/** 🔌️ The scene host port adapters: the fiber and drei bindings and the destructured three constructors with their instance types that every other r3f module builds on.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { sceneHostPort, type NormalBufferAttributes } from "@semio-tech/ui-react";

// #region 🔌️Adapters
export const Canvas = sceneHostPort.fiber.canvas;
export const useFrame = sceneHostPort.fiber.useFrame;
export const useThree = sceneHostPort.fiber.useThree;
export const GizmoHelper = sceneHostPort.drei.GizmoHelper;
export const Grid = sceneHostPort.drei.Grid;
export const OrthographicCamera = sceneHostPort.drei.OrthographicCamera;
export const PerspectiveCamera = sceneHostPort.drei.PerspectiveCamera;
export const {
  Box3,
  BoxGeometry,
  BufferGeometry,
  CanvasTexture,
  Color,
  DoubleSide,
  EdgesGeometry,
  Group,
  HalfFloatType,
  LineBasicMaterial,
  LineSegments,
  LinearFilter,
  LinearSRGBColorSpace,
  Matrix4,
  Mesh,
  MeshBasicMaterial,
  MOUSE,
  Object3D,
  OrthographicCamera: ThreeOrthographicCamera,
  PerspectiveCamera: ThreePerspectiveCamera,
  PlaneGeometry,
  Quaternion,
  Ray,
  RGBAFormat,
  Scene,
  ShaderMaterial,
  Vector2,
  Vector3,
  WebGLRenderTarget,
} = sceneHostPort.three;
export type Camera = InstanceType<typeof sceneHostPort.three.Camera>;
export type Box3 = InstanceType<typeof Box3>;
export type BufferGeometry = InstanceType<typeof BufferGeometry<NormalBufferAttributes>>;
export type Matrix4 = InstanceType<typeof Matrix4>;
export type Vector3 = InstanceType<typeof Vector3>;
export type Quaternion = InstanceType<typeof Quaternion>;
export type Object3D = InstanceType<typeof Object3D>;
export type Mesh = InstanceType<typeof Mesh>;
export type LineSegments = InstanceType<typeof LineSegments>;
export type Vector2 = InstanceType<typeof Vector2>;
export type Group = InstanceType<typeof Group>;
export type ThreePerspectiveCamera = InstanceType<typeof ThreePerspectiveCamera>;
// #endregion 🔌️Adapters
