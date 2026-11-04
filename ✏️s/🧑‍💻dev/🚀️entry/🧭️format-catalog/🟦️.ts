/** 🧭️ The Open file format catalog of the `s` product: the generated stdio format claims (`🤖️generated/stdio-format-claims.json`, produced by the `generate-stdio-format-claims` target of the `s` project) mapped to the dialect coordinate of each owner editor. */
import type { ShellHostPortsV1 } from "@semio-tech/framework-renderer-react";
import claims from "./🤖️generated/stdio-format-claims.json" with { type: "json" };

type FormatCatalogV1 = NonNullable<ShellHostPortsV1["formatCatalog"]>;
type RevisionClaimV1 = Readonly<{ kindId: string; artifactRef: string; sniff: Readonly<{ window: number; contains: readonly string[] }> | null }>;
type FormatClaimV1 = Readonly<{ kindId: string; shortId: string; aliases: readonly string[]; mimes: readonly string[]; extensions: readonly string[]; artifactRef: string; revisions?: readonly RevisionClaimV1[] }>;

/** 🔬️ The Open file catalog port plus, for formats shared by several revisions, each revision's declared content check (owner first). */
export type SFormatCatalogV1 = FormatCatalogV1 & Readonly<{ revisionsOf(kindId: string): readonly Readonly<{ kindId: string; window: number; contains: readonly string[] }>[] }>;

const formats: readonly FormatClaimV1[] = claims.formats;
const owners = new Map(formats.flatMap((format) => [[format.kindId, format.artifactRef] as const, ...(format.revisions ?? []).map((revision) => [revision.kindId, revision.artifactRef] as const)]));
const revisions = new Map(formats.map((format) => [format.kindId, (format.revisions ?? []).flatMap((revision) => (revision.sniff === null ? [] : [{ kindId: revision.kindId, window: revision.sniff.window, contains: revision.sniff.contains }]))]));

/** 🪝️ Port the shell host reads for natural-file open; descriptors keep their declared extension order. `artifactRefOf` answers every revision kind id, so a sniffed revision routes to its own editor. */
export const S_FORMAT_CATALOG_V1: SFormatCatalogV1 = Object.freeze({
  descriptors: () => formats,
  artifactRefOf: (kindId: string) => owners.get(kindId),
  revisionsOf: (kindId: string) => revisions.get(kindId) ?? [],
});
