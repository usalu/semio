/** 🧭️ Artifact opening relay: resolves the opening preferences, role and router a relayed artifact opening carries. */

import type { AppRef, AppRole, AppRouter, ArtifactDialect, OpeningPreferences } from "@semio-tech/framework";
import { dialectCoordinate, parseDialectCoordinate } from "../../🚪️io/🧬️schema/🟦️.ts";
import { parseSurfaceAppId } from "../../🛂️manifest/🧬️schema/🟦️.ts";
import { resolveOpeningApp } from "../../🎠️kernel/📖️opening-resolver/🟦️.ts";

//#region 🔖️ArtifactOpeningRelay
/** 📂️ Canonical shell relay after parsing role aliases, validating an exact "Open with…"
 * choice against the live router, and resolving a plain open through the event-sourced opening
 * preferences projection. A document is attachable only as the complete `(documentId, schema)`
 * pair; `spaceId` remains optional because local-only documents deliberately have none. */
export type ResolvedArtifactOpeningRelay = {
  readonly artifactRef: string;
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly app: AppRef;
  readonly documentId?: string;
  readonly spaceId?: string;
  readonly schema?: string;
};

function openingRelayError(code: string): never {
  throw new Error(code);
}

function openingRelayRole(value: unknown): AppRole | undefined {
  if (value === undefined) return undefined;
  if (value === 0 || value === "viewer") return "viewer";
  if (value === 1 || value === "editor") return "editor";
  return openingRelayError("opening.invalid-role");
}

function openingRelayString(record: Readonly<Record<string, unknown>>, field: string, errorCode = `opening.invalid-${field}`): string | undefined {
  const value = record[field];
  if (value === undefined) return undefined;
  if (typeof value !== "string" || value.trim() === "") return openingRelayError(errorCode);
  return value;
}

/** 🧭 Resolves `os.open-artifact`/`os.open-artifact-with` without mutating the router or
 * preferences. Missing role means editor, matching both shells' declared boot default. */
export function resolveArtifactOpeningRelay(
  actionId: string,
  args: unknown,
  router: AppRouter,
  preferences: OpeningPreferences,
): ResolvedArtifactOpeningRelay {
  if (actionId !== "os.open-artifact" && actionId !== "os.open-artifact-with") return openingRelayError("opening.invalid-action");
  if (!args || typeof args !== "object" || Array.isArray(args)) return openingRelayError("opening.invalid-args");
  const record = args as Readonly<Record<string, unknown>>;
  const rawArtifactRef = openingRelayString(record, "artifactRef", "opening.invalid-artifact-ref");
  if (!rawArtifactRef) return openingRelayError("opening.invalid-artifact-ref");

  let dialect: ArtifactDialect;
  let surfaceRole: AppRole | undefined;
  try {
    if (rawArtifactRef.includes("#")) {
      const surface = parseSurfaceAppId(rawArtifactRef);
      dialect = surface.dialect;
      surfaceRole = surface.role;
    } else {
      dialect = parseDialectCoordinate(rawArtifactRef);
    }
  } catch {
    return openingRelayError("opening.invalid-artifact-ref");
  }

  const wireRole = openingRelayRole(record.role);
  if (surfaceRole && wireRole && surfaceRole !== wireRole) return openingRelayError("opening.role-mismatch");
  const role = wireRole ?? surfaceRole ?? "editor";
  const pluginId = openingRelayString(record, "pluginId");
  const appId = openingRelayString(record, "appId");
  if (Boolean(pluginId) !== Boolean(appId)) return openingRelayError("opening.partial-app-ref");
  if (actionId === "os.open-artifact-with" && (!pluginId || !appId)) return openingRelayError("opening.explicit-app-required");

  let app: AppRef;
  if (pluginId && appId) {
    const explicit = { pluginId, appId };
    if (!router.entriesFor(dialect, role).some((entry) => entry.pluginId === explicit.pluginId && entry.appId === explicit.appId)) return openingRelayError("opening.app-mismatch");
    app = explicit;
  } else {
    app = resolveOpeningApp(router, dialect, role, preferences);
  }

  const documentId = openingRelayString(record, "artifactId") ?? openingRelayString(record, "documentId");
  const schema = openingRelayString(record, "schema");
  if (Boolean(documentId) !== Boolean(schema)) return openingRelayError("opening.partial-document-ref");
  const spaceId = openingRelayString(record, "spaceId");
  return {
    artifactRef: dialectCoordinate(dialect),
    dialect,
    role,
    app,
    ...(documentId && schema ? { documentId, schema } : {}),
    ...(spaceId ? { spaceId } : {}),
  };
}
//#endregion 🔖️ArtifactOpeningRelay
