/** 📏️ Tracked-path budget measured in the units real checkouts enforce: UTF-16 code units for the whole repo-relative path (Windows MAX_PATH semantics) and UTF-8 bytes per component (Linux/macOS NAME_MAX). Both limits live in `collisionPolicy` of the taxonomy. */
export interface PathBudgetPolicy {
  readonly maxPathUtf16Units: number;
  readonly maxComponentUtf8Bytes: number;
}

/** 🚧️ Violation codes of the path budget. */
export type PathBudgetCode = "path-too-long" | "path-component-too-long";

const ENCODER = new TextEncoder();

/** 🔢️ UTF-16 code units of a path; U+FE0F counts one, astral emoji two. */
export function pathUtf16Units(path: string): number {
  return path.length;
}

/** 🛤️ UTF-8 bytes of the longest component of a path. */
export function pathLongestComponentUtf8Bytes(path: string): number {
  let longest = 0;
  for (const component of path.replaceAll("\\", "/").split("/")) longest = Math.max(longest, ENCODER.encode(component).length);
  return longest;
}

/** 💰️ Budget codes violated by a repo-relative path. */
export function pathBudgetCodes(path: string, policy: PathBudgetPolicy): readonly PathBudgetCode[] {
  const rows: PathBudgetCode[] = [];
  if (pathUtf16Units(path) > policy.maxPathUtf16Units) rows.push("path-too-long");
  if (pathLongestComponentUtf8Bytes(path) > policy.maxComponentUtf8Bytes) rows.push("path-component-too-long");
  return rows;
}

/** 🧮️ True when a root path plus a reserved descendant suffix (in UTF-16 units) still fits the policy. */
export function pathBudgetFitsWithReserve(root: string, reserveUtf16Units: number, policy: PathBudgetPolicy): boolean {
  return pathUtf16Units(root) + reserveUtf16Units <= policy.maxPathUtf16Units && pathLongestComponentUtf8Bytes(root) <= policy.maxComponentUtf8Bytes;
}

/** 📐️ Human-readable statement of the policy for violation messages. */
export function pathBudgetStatement(policy: PathBudgetPolicy): string {
  return `${policy.maxPathUtf16Units} UTF-16 units per path and ${policy.maxComponentUtf8Bytes} UTF-8 bytes per component`;
}
