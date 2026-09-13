export const POLICY_APP_CONFIG_DIR = "🎚️config";
export const POLICY_APP_CONFIG_LEGACY_DIR = "🧮️config";
export const POLICY_APP_PRESENCE_DIR = "👥️presence";
export const POLICY_APP_WASM_LEGACY_DIR = "🕸️wasm";
export const POLICY_APP_SCHEMA_FACET = "🧬️schema";

export type PolicyAppSchemaOwner = {
  ownerRel: string;
  configType: string;
  presenceType: string;
  presenceRel: string;
  apps: string[];
};

/** 🏷️ Derives a presence type from its surface config binding. */
export function policyAppPresenceTypeName(configType: string): string {
  return configType.endsWith("Config") ? `${configType.slice(0, -"Config".length)}Presence` : `${configType}Presence`;
}

/** 🧭️ Resolves the taxonomy role for a surface config or presence schema facet. */
export function policyAppSchemaFacetRole(kind: "config" | "presence"): string {
  return kind === "config" ? `${POLICY_APP_CONFIG_DIR}/${POLICY_APP_SCHEMA_FACET}` : `${POLICY_APP_PRESENCE_DIR}/${POLICY_APP_SCHEMA_FACET}`;
}
