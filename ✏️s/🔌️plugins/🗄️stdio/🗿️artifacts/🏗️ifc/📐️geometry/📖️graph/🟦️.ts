/** 🌐️ The standard profile controlling format-specific representation availability. */
export type GeometryStandard = "IFC2X3" | "IFC4";

/** 🧩️ A borrowed type constituent with its original positional arguments. */
export interface GeometryConstituent<V> {name: string; args: readonly V[]}

/** 🆔️ A borrowed instance view; constituentCount preserves complex source structure. */
export interface GeometryEntity<V> extends GeometryConstituent<V> {id: bigint; constituentCount: number}

/** 🔤️ Intrinsic positional-value reads without converting the source artifact model. */
export interface GeometryValues<V> {
  reference(value: V | undefined): bigint | undefined;
  text(value: V | undefined): string | undefined;
  enumeration(value: V | undefined): string | undefined;
  real(value: V | undefined): number | undefined;
  integer(value: V | undefined): bigint | undefined;
  aggregate(value: V | undefined): readonly V[] | undefined;
  typed(value: V | undefined): {name: string; items: readonly V[]} | undefined;
  absent(value: V | undefined): boolean;
  unset(value: V | undefined): boolean;
  derived(value: V | undefined): boolean;
}

/** 📚️ Indexed graph reads retain native identities, typed values and all complex constituents. */
export interface GeometryGraph<V> {
  standard: GeometryStandard;
  values: GeometryValues<V>;
  length: number;
  sourceIdentity(): object;
  entity(index: number): GeometryEntity<V> | undefined;
  constituent(instance: number, index: number): GeometryConstituent<V> | undefined;
}
