/** 🖌️ A terrain editor's bounded World3d action descriptor. */
export type TerrainBrushAction = {
  readonly action: string;
  readonly arguments: { readonly operation: string; readonly radius: number; readonly value: number };
};

const identifier = (value: unknown): value is string => typeof value === "string" && /^[A-Za-z0-9_.-]{1,64}$/.test(value);
const integer = (value: unknown, minimum: number, maximum: number): value is number =>
  typeof value === "number" && Number.isInteger(value) && value >= minimum && value <= maximum;
const object = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);

/** 🎟️ Decodes the interaction lane before dispatching a hit on the editor's surface. */
export function parseTerrainBrushAction(value: unknown): TerrainBrushAction | undefined {
  if (!object(value) || Object.keys(value).length !== 2 || !identifier(value.action)) return undefined;
  const args = value.arguments;
  if (!object(args) || Object.keys(args).length !== 3) return undefined;
  const { operation, radius, value: strength } = args;
  if (!identifier(operation) || !integer(radius, 0, 32) || typeof strength !== "number") return undefined;
  const valid = operation === "smooth"
    ? integer(strength, 0, 100)
    : strength >= -32768 && strength <= 32767 && Math.round(strength * 1000) / 1000 === strength;
  if (!valid) return undefined;
  return { action: value.action, arguments: { operation, radius, value: strength } };
}
