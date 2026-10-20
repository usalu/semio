/** 📂️ Payloads, dispatch union and constructors of the opening-preferences mutation vocabulary, shared by its two direct leaves. */

import type { ArtifactDialect } from "@semio-tech/schema-ts/artifact-reference";
import type { AppRef, AppRole } from "@semio-tech/framework-manifest/schema";

//#region 🔖️Payloads
/** 📌️ Pins `app` for one `(dialect, role)` coordinate, replacing any prior pin. */
export interface SetDefaultApp {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly app: AppRef;
}

/** 🧹 Removes the pinned default for one `(dialect, role)` coordinate, if present. */
export interface ClearDefaultApp {
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
}
//#endregion 🔖️Payloads

//#region 🔖️Dispatch
/** 🎚️ Typed, invertible opening-preferences mutation vocabulary. */
export type OpeningConfigMutation = ({ readonly mutation: "setDefaultApp" } & SetDefaultApp) | ({ readonly mutation: "clearDefaultApp" } & ClearDefaultApp);

/** 🏗️ Wraps a set-default-app payload in the opening-config dispatch union. */
export function setDefaultApp(dialect: ArtifactDialect, role: AppRole, app: AppRef): OpeningConfigMutation {
  return { mutation: "setDefaultApp", dialect, role, app };
}

/** 🔨️ Wraps a clear-default-app payload in the opening-config dispatch union. */
export function clearDefaultApp(dialect: ArtifactDialect, role: AppRole): OpeningConfigMutation {
  return { mutation: "clearDefaultApp", dialect, role };
}
//#endregion 🔖️Dispatch
