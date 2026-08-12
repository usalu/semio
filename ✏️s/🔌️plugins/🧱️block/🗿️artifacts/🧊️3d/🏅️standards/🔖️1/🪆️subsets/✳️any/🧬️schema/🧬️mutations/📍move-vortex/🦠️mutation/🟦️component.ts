/** 📍 block3d move-vortex/🦠️mutation — a vortex's position + facing direction together. */
export interface MoveVortex {
  id: string;
  newPosition: [number, number, number];
  newDirection: [number, number, number];
}
