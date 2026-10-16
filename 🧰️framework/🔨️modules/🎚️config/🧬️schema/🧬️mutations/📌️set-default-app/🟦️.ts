/** 📌️ Authoritative direct TypeScript leaf for pinning one viewer/editor default. */

import type { OpeningPreferences } from "../../🟦️.ts";
import type { ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import type { AppRole, AppRef } from "../../../../🛂️manifest/🧬️schema/🟦️.ts";
import type { OpeningConfigMutation, SetDefaultApp } from "../📂️opening-config-mutation/🟦️.ts";
import { clearDefaultApp, setDefaultApp } from "../📂️opening-config-mutation/🟦️.ts";

//#region 🔖️Mutation
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
