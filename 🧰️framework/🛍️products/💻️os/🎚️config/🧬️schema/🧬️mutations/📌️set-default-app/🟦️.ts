/** 📌️ Authoritative direct TypeScript leaf for pinning one viewer/editor default. */

import type { AppRef, AppRole, ArtifactDialect, OpeningPreferences } from "../../🟦️.ts";
import type { OpeningConfigMutation } from "../🟦️.ts";
import { clearDefaultApp } from "../🧹clear-default-app/🟦️.ts";

//#region 🔖️Mutation
/** 📌️ Pins `app` for one `(dialect, role)` coordinate, replacing any prior pin. */
export interface SetDefaultApp {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly app: AppRef;
}

/** 🏗️ Wraps a set-default-app payload in the opening-config dispatch union. */
export function setDefaultApp(dialect: ArtifactDialect, role: AppRole, app: AppRef): OpeningConfigMutation {
  return { mutation: "setDefaultApp", dialect, role, app };
}

/** 🔺️ Replaces the selected opening-preference coordinate. */
export function diff(payload: SetDefaultApp, base: OpeningPreferences): OpeningPreferences {
  if (base.defaults.some((entry) => sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role) && sameApp(entry.app, payload.app))) return base;
  return { defaults: [...base.defaults.filter((entry) => !sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role)), { dialect: payload.dialect, role: payload.role, app: payload.app }] };
}

/** ↩️ Restores the prior pin or clears the previously unpinned coordinate. */
export function inverse(payload: SetDefaultApp, base: OpeningPreferences): OpeningConfigMutation[] {
  const prior = base.defaults.find((entry) => sameCoordinate(entry.dialect, entry.role, payload.dialect, payload.role));
  return prior ? [setDefaultApp(payload.dialect, payload.role, prior.app)] : [clearDefaultApp(payload.dialect, payload.role)];
}

function sameCoordinate(leftDialect: ArtifactDialect, leftRole: AppRole, rightDialect: ArtifactDialect, rightRole: AppRole): boolean {
  return leftRole === rightRole && leftDialect.artifactKind === rightDialect.artifactKind && leftDialect.standard === rightDialect.standard && leftDialect.subset === rightDialect.subset;
}

function sameApp(left: AppRef, right: AppRef): boolean {
  return left.pluginId === right.pluginId && left.appId === right.appId;
}
//#endregion 🔖️Mutation
