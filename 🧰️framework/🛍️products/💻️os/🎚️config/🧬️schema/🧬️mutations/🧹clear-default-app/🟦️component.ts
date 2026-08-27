/** 🧹 Authoritative direct TypeScript leaf for unpinning one viewer/editor default. */

import type { AppRole, ArtifactDialect, OpeningPreferences } from "../../🟦️component.ts";
import type { OpeningConfigMutation } from "../🟦️component.ts";
import { setDefaultApp } from "../📌️set-default-app/🟦️component.ts";

//#region 🔖️Mutation
/** 🧹 Removes the pinned default for one `(dialect, role)` coordinate, if present. */
export interface ClearDefaultApp {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
}

/** 🏗️ Wraps a clear-default-app payload in the opening-config dispatch union. */
export function clearDefaultApp(dialect: ArtifactDialect, role: AppRole): OpeningConfigMutation {
  return { mutation: "clearDefaultApp", dialect, role };
}

/** 🔺️ Removes the selected opening-preference coordinate. */
export function diff(payload: ClearDefaultApp, base: OpeningPreferences): OpeningPreferences {
  if (!base.defaults.some((entry) => sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role))) return base;
  return { defaults: base.defaults.filter((entry) => !sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role)) };
}

/** ↩️ Restores the prior pin or emits no step when the coordinate was already clear. */
export function inverse(payload: ClearDefaultApp, base: OpeningPreferences): OpeningConfigMutation[] {
  const prior = base.defaults.find((entry) => sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role));
  return prior ? [setDefaultApp(payload.dialect, payload.role, prior.app)] : [];
}

function sameCoordinate(leftDialect: ArtifactDialect, leftRole: AppRole, rightDialect: ArtifactDialect, rightRole: AppRole): boolean {
  return leftRole === rightRole && leftDialect.artifactKind === rightDialect.artifactKind && leftDialect.standard === rightDialect.standard && leftDialect.subset === rightDialect.subset;
}
//#endregion 🔖️Mutation
