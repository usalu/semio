/** 🔧️ jack direct `change-data-property` payload mirror of `ChangeDataProperty`. */
export type JackEntityRef = { entity: "node"; id: string } | { entity: "edge"; id: string };

export interface ChangeDataProperty {
  entity: JackEntityRef;
  key: string;
  new_value: unknown;
}
