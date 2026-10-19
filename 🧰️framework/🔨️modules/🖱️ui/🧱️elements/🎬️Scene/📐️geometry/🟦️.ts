/** 📐 Owns the 3D point, vector, plane and camera shapes that scene tools exchange. */

/**
 * 🔘️ 3D point with x, y, z coordinates.
 **/
export interface Point {
  x: number;
  y: number;
  z: number;
}

/**
 * 📚️ 3D direction vector with x, y, z components.
 **/
export interface Vector {
  x: number;
  y: number;
  z: number;
}

/**
 * 🔲️ 3D coordinate plane defined by an origin point and two axis vectors.
 **/
export interface Plane {
  origin: Point;
  xAxis: Vector;
  yAxis: Vector;
}

/**
 * 📷️ 3D camera defined by position, forward direction, and up direction.
 **/
export interface Camera {
  position: Point;
  forward: Vector;
  up: Vector;
}
