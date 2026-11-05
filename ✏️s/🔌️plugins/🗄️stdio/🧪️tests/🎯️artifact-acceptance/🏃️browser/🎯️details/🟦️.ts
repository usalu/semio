/** 🎯️ Resolves the native scalar control ID emitted by snapshot Details `path_id`.
 * @see ../../../../📇️registry/🧬️contract/✏️editing/🪟️details/🏷️labels-and-controls/🦀️.rs */
export function detailsControlId(pointer: string, valueKind: "text" | "number" | "toggle"): string {
  if (pointer === "") return `stdio-snapshot-details-root-${valueKind}`;
  let hash = 0xcbf29ce484222325n;
  for (const byte of new TextEncoder().encode(pointer)) hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * 0x100000001b3n);
  return `sd-${hash.toString(16).padStart(16, "0")}-${valueKind}`;
}

/** 🧭️ Selects a scalar inside a Details tree, relative to the matrix driver's window body. */
export function detailsControlSelector(pointer: string, valueKind: "text" | "number" | "toggle"): string {
  return `[role="tree"] [data-ui-node-key="${detailsControlId(pointer, valueKind)}"]`;
}
