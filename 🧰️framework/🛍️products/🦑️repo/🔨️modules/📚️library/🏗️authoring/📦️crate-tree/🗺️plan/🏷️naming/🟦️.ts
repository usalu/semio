/** 🏷️ The mutation family a carve slice belongs to: the subset below `subsets`, else the leaf directly below `mutations`; null for a slice outside every mutation family. */
export function mutationFamilyOf(eff: readonly string[]): string | null {
  const at = eff.indexOf("mutations");
  const subset = eff.indexOf("subsets");
  return at >= 0 && eff.length > at + 1 ? (eff[subset >= 0 ? subset + 1 : at + 1] ?? "") : null;
}

/** 🔖️ The crate label of a group whose slices carry these families: `mut-<family>` for one shared family and `mut-family` for several, whenever most of the slices sit in a mutation family (a leaf-family crate also hosts a few support slices); null for every other group. */
export function mutationCrateLabel(families: readonly (string | null)[]): string | null {
  const named = families.filter((family): family is string => family !== null);
  if (named.length * 2 <= families.length) return null;
  const distinct = [...new Set(named)];
  return `mut-${facetSlug(distinct.length === 1 ? distinct[0]!.replaceAll("_", "-") : "family")}`.replace(/-$/u, "");
}

/** 🔢️ True for a label that names a mutation family, which a duplicate keeps and numbers instead of renaming it after its first leaf. */
export function isMutationCrateLabel(label: string): boolean {
  return label.startsWith("mut-");
}

/** 🍃️ Slug of a facet directory name, stripping emoji. */
export function facetSlug(dirName: string): string {
  return dirName.replace(/^[^\p{L}\p{N}]+/u, "").replace(/[^a-z0-9]+/giu, "-").replace(/^-|-$/gu, "").toLowerCase();
}

/** 🔤️ Cargo crate name to its Rust identifier. */
export function crateIdent(name: string): string {
  return name.replaceAll("-", "_");
}

export function facetEmoji(dirName: string): string {
  const match = /^(\p{Extended_Pictographic}\uFE0F?|\p{Emoji_Presentation})/u.exec(dirName);
  return match ? match[1]! : "🧩️";
}

export function crateIdentPlaceholder(index: number): string {
  return `__CRATE_${index}__`;
}
