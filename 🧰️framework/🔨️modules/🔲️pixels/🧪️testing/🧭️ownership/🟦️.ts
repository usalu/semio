/** 🧭️ The language neutral Pixels task descriptor admits only neutral module selections. */
export interface PixelsTaskDescriptorV1 {
  readonly schema: "pixels.task-ownership.v1";
  readonly strictRoots: readonly string[];
  readonly testRoots: readonly string[];
  readonly inputs: readonly string[];
}

/** 🔲️ Admits a closed descriptor without product roots, duplicates or escaping path segments. */
export function readPixelsTaskDescriptorV1(value: unknown): PixelsTaskDescriptorV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Pixels task descriptor must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 4 || row.schema !== "pixels.task-ownership.v1") throw Error("Pixels task descriptor shape is invalid");
  for (const key of ["strictRoots", "testRoots", "inputs"] as const) {
    const paths = row[key];
    if (!Array.isArray(paths) || !paths.length || new Set(paths).size !== paths.length) throw Error(`Pixels task ${key} must be unique and nonempty`);
    for (const path of paths) {
      if (typeof path !== "string") throw Error(`Pixels task ${key} must contain paths`);
      if (key === "inputs" && (path === "default" || path === "^default")) continue;
      const root = key === "inputs" ? path.replace(/^\{workspaceRoot\}\//, "") : path;
      if (key === "inputs" && root === path) throw Error("Pixels task inputs must be workspace rooted");
      const prefix = "🧰️framework/🔨️modules/";
      if (!root.startsWith(prefix) || root.length === prefix.length || root.split("/").some(segment => segment === ".." || segment === "🛍️products")) throw Error(`Pixels task ${key} selects outside neutral modules`);
    }
  }
  return value as PixelsTaskDescriptorV1;
}
