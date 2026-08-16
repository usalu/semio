/** 🧬️ OpeningPreferences */

export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export type AppRole = "viewer" | "editor";

export interface AppRef {
  pluginId: string;
  appId: string;
}

export interface DefaultApp {
  dialect: ArtifactDialect;
  role: AppRole;
  app: AppRef;
}

export interface OpeningPreferences {
  /** @state config */
  defaults: DefaultApp[];
}
