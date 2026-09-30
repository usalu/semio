import type { ShellBrand } from "@semio-tech/framework";

/** 🏷️ Selects a shell identity from an explicit owner contribution. */
export function resolveShellBrandById(brands: readonly ShellBrand[], id: string | undefined): ShellBrand | undefined {
  if (!id) return undefined;
  const brand = brands.find((entry) => entry.id === id);
  if (!brand) throw new Error(`Unknown contributed shell brand: ${id}`);
  return brand;
}
