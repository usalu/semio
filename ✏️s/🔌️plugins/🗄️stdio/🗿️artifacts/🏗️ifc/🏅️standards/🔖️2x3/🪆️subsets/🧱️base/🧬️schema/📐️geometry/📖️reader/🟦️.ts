import type {GeometryGraph, GeometryEntity, GeometryConstituent, GeometryValues} from "../../../../../../../📐️geometry/📖️graph/🟦️.ts";
import type {Part21Document, Part21Value} from "../../📸️snapshot/🟦️.ts";

/** 🔤️ Reads IFC2x3's own exact decimal and positional value model. */
const values: GeometryValues<Part21Value> = {
  reference: value => value?.kind === "ref" ? value.value : undefined,
  text: value => value?.kind === "str" ? value.value : undefined,
  enumeration: value => value?.kind === "enum" ? value.value : undefined,
  integer: value => value?.kind === "int" ? value.value : undefined,
  real: value => {
    if (value?.kind === "int") return Number(value.value);
    if (value?.kind !== "real") return undefined;
    const decimal = value.value;
    return Number((decimal.negative ? "-" : "") + decimal.coefficient + "e" + ((decimal.exponent ?? 0) - decimal.scale));
  },
  aggregate: value => value?.kind === "list" ? value.values : undefined,
  typed: value => value?.kind === "typed" ? {name: value.typeName, items: value.values} : undefined,
  absent: value => !value || value.kind === "unset" || value.kind === "derived",
  unset: value => value?.kind === "unset",
  derived: value => value?.kind === "derived",
};

/** 📚️ A borrowed IFC2x3 graph that never constructs an IFC4 snapshot or rewrites source values. */
export class Ifc2x3GeometryGraph implements GeometryGraph<Part21Value> {
  readonly standard = "IFC2X3" as const;
  readonly values = values;
  constructor(readonly document: Part21Document) {}
  sourceIdentity(): object { return this.document; }
  get length(): number { return this.document.instances.length; }
  constituent(instance: number, index: number): GeometryConstituent<Part21Value> | undefined {
    const type = this.document.instances[instance]?.entities[index];
    return type ? {name: type.typeName, args: type.arguments} : undefined;
  }
  entity(index: number): GeometryEntity<Part21Value> | undefined {
    const source = this.document.instances[index], type = this.constituent(index, 0);
    return source && type ? {id: source.id, ...type, constituentCount: source.entities.length} : undefined;
  }
}
