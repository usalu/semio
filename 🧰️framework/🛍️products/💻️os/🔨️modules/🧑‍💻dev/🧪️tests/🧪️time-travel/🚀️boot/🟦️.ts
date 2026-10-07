export type TimeTravelPlaygroundV1 = Readonly<{ variant: string; aliases: readonly string[]; ports: Readonly<{ react: number; wgpu: number }> }>;
export type TimeTravelServeSelectionV1 = Readonly<{ universal: boolean; url: string; renderer: "react" | "wgpu"; explicit?: string }>;

/** 🚀️ Selects a declared playground for a universal history serve. */
export function timeTravelServeVariantV1(selection: TimeTravelServeSelectionV1, catalog: readonly TimeTravelPlaygroundV1[]): string {
  const admit = (name: string) => {
    const rows = catalog.filter(row => row.variant === name || row.aliases.includes(name));
    if (rows.length !== 1) throw Error(`A uniquely declared playground is required: ${name}`);
    return rows[0]!.variant;
  };
  if (!selection.universal) return admit("puzzle2d");
  const url = new URL(selection.url);
  const plugin = url.searchParams.get("plugin");
  const route = plugin === null ? undefined : admit(plugin);
  const explicit = selection.explicit === undefined ? undefined : admit(selection.explicit);
  if (route !== undefined && explicit !== undefined && route !== explicit) throw Error("The route and explicit playground disagree");
  if (explicit !== undefined || route !== undefined) return explicit ?? route!;
  const port = Number(url.port || (url.protocol === "https:" ? 443 : 80));
  const rows = catalog.filter(row => row.ports[selection.renderer] === port);
  if (rows.length !== 1) throw Error("A universal serve needs a declared route plugin, variant, or catalog port");
  return rows[0]!.variant;
}
