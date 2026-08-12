/** mutation payload — mirrors `CreateColumn`. */
export interface CreateColumn {
  name: string;
  kind: "null" | "bool" | "int" | "float" | "str" | "bytes";
  index?: number | null;
}
