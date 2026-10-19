/** 👥️ App presence is empty; canonical window views and interaction carry live state. */
export interface Puzzle2dPresence {}
export function parsePuzzle2dPresence(value: unknown): Puzzle2dPresence {
  if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).length !== 0) throw new Error("app presence must be empty" );
  return {};
}
