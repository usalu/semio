/** 🎚️ dag `set-slider` payload mirror of `SetSlider` (internally tagged `mutation: "setSlider"`), with its closed-schema parser. */
export type DagSliderField = "value" | "min" | "max";

export interface SetSlider {
  mutation: "setSlider";
  id: string;
  field: DagSliderField;
  value: number;
}

/** 🚪️ Parses one `set-slider` payload the way its JSON Schema admits it, or throws. */
export function parseSetSlider(value: unknown): SetSlider {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("set-slider: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["mutation", "id", "field", "value"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`set-slider: unknown field ${unknownKey}`);
  if (row.mutation !== "setSlider") throw new TypeError("set-slider: mutation must be setSlider");
  if (typeof row.id !== "string" || row.id.length === 0) throw new TypeError("set-slider: id must be a node id");
  if (row.field !== "value" && row.field !== "min" && row.field !== "max") throw new TypeError("set-slider: field must be value, min or max");
  if (typeof row.value !== "number" || !Number.isFinite(row.value)) throw new TypeError("set-slider: value must be a finite number");
  return { mutation: "setSlider", id: row.id, field: row.field, value: row.value };
}
