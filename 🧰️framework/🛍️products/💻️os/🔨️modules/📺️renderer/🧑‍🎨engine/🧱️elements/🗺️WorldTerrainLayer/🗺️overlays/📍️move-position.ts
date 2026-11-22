type Coordinates = readonly [longitude: number, latitude: number];

export type TerrainPositionMove = { readonly id: string; readonly longitude: number; readonly latitude: number };

export function parseTerrainMovePosition(value: unknown): string | undefined {
  if (typeof value !== "object" || value === null || Array.isArray(value) || Object.keys(value).length !== 1 || !("action" in value)) return undefined;
  return typeof value.action === "string" && /^[A-Za-z0-9_.-]{1,64}$/.test(value.action) ? value.action : undefined;
}

type PositionPress = {
  readonly pointer: number;
  readonly id: string;
  readonly x: number;
  readonly y: number;
  readonly release: () => void;
};

/** 🧷️ Holds one viewport drag until the pointer ends or its editor changes. */
export class TerrainPositionDrag {
  private press: PositionPress | undefined;

  begin(pointer: number, id: string, x: number, y: number, release: () => void): boolean {
    if (this.press) return false;
    this.press = { pointer, id, x, y, release };
    return true;
  }

  owns(pointer: number): boolean {
    return this.press?.pointer === pointer;
  }

  cancel(pointer?: number): void {
    if (!this.press || (pointer !== undefined && !this.owns(pointer))) return;
    const { release } = this.press;
    this.press = undefined;
    release();
  }

  finish(pointer: number, x: number, y: number, coordinates: Coordinates | undefined): TerrainPositionMove | undefined {
    const press = this.press;
    if (!press || !this.owns(pointer)) return undefined;
    this.cancel(pointer);
    if (!coordinates || Math.hypot(x - press.x, y - press.y) < 3) return undefined;
    const [longitude, latitude] = coordinates;
    return { id: press.id, longitude, latitude };
  }
}
