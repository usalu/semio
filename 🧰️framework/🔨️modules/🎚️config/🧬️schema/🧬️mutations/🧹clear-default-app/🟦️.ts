/** 🧹 Authoritative direct TypeScript leaf for unpinning one viewer/editor default. */

import type { OpeningPreferences } from "../../🟦️.ts";
import type { ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import type { AppRole } from "../../../../🛂️manifest/🧬️schema/🟦️.ts";
import type { ClearDefaultApp, OpeningConfigMutation } from "../📂️opening-config-mutation/🟦️.ts";
import { setDefaultApp } from "../📂️opening-config-mutation/🟦️.ts";

//#region 🔖️Mutation
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
