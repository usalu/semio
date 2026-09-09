/** 🎯️ Cross-plugin app addresses belong to the manifest contract. */
import { dialectCoordinate, parseDialectCoordinate, type ArtifactDialect } from "../../🚪️io/🧬️schema/🟦️.ts";
export type AppRole = "viewer" | "editor";
export interface AppRef { pluginId: string; appId: string }

/** 🪟️ Serializes one dialect's app role. */
export function surfaceAppId(dialect: ArtifactDialect, role: AppRole): string {
  return `${dialectCoordinate(dialect)}#${role}`;
}

/** 🪟️ Resolves the final role separator against the canonical IO coordinate. */
export function parseSurfaceAppId(id: string): { dialect: ArtifactDialect; role: AppRole } {
  const separator = id.lastIndexOf("#");
  if (separator < 0) throw new Error("surface id is missing '#'");
  const role = id.slice(separator + 1);
  if (role !== "viewer" && role !== "editor") throw new Error("surface id requires viewer or editor role");
  return { dialect: parseDialectCoordinate(id.slice(0, separator)), role };
}
