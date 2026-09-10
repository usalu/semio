/** 🧩️ Concatenates one packed text leaf: `value` then sorted `dataAttributes` (`01`..`32`). */
export function packedTextLeaf(
  value: string,
  dataAttributes?: { readonly [key: string]: string | null | undefined } | null,
): string {
  let payload = value;
  if (dataAttributes) {
    for (const key of Object.keys(dataAttributes).sort()) {
      const next = dataAttributes[key];
      if (typeof next === "string") payload += next;
    }
  }
  return payload;
}
