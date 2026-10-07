import { createHash } from "node:crypto";

export interface NativeCodecPublicationReceiptV1 {
  schemaVersion: 1;
  artifactKind: string;
  artifactSchema: string;
  factoryId: string;
  extension: string;
  packSchemaHash: string;
  protocolSourceSha256: string;
}
type Document = Record<string, any>;
export interface NativeCodecPublicationInputV1 {
  receipt: NativeCodecPublicationReceiptV1;
  protocol: string;
  factories: Document;
  definition: Document;
  catalog: Document;
}

/** 📣️ Admits one current native codec receipt into its authored contribution documents. */
export function projectNativeCodecReceiptPublicationV1(input: NativeCodecPublicationInputV1): Omit<NativeCodecPublicationInputV1, "receipt" | "protocol"> {
  const receipt = input.receipt;
  const keys = ["schemaVersion", "artifactKind", "artifactSchema", "factoryId", "extension", "packSchemaHash", "protocolSourceSha256"];
  if (!receipt || Object.keys(receipt).length !== keys.length || keys.some(key => !(key in receipt)) || receipt.schemaVersion !== 1) throw new Error("native-codec.publication.receipt-shape");
  for (const key of ["artifactKind", "artifactSchema", "factoryId", "extension"] as const) if (typeof receipt[key] !== "string" || !receipt[key].length) throw new Error("native-codec.publication.identity");
  for (const key of ["packSchemaHash", "protocolSourceSha256"] as const) if (!/^[a-f0-9]{64}$/.test(receipt[key]) || /^0+$/.test(receipt[key])) throw new Error("native-codec.publication.digest");
  if (createHash("sha256").update(input.protocol).digest("hex") !== receipt.protocolSourceSha256) throw new Error("native-codec.publication.stale-source");
  const { factories, definition, catalog } = structuredClone(input);
  if (definition.id !== receipt.artifactKind) throw new Error("native-codec.publication.definition-identity");
  const exact = (rows: any[], field: string): any => {
    if (!Array.isArray(rows)) throw new Error("native-codec.publication.rows");
    const matches = rows.filter(row => row?.[field] === receipt.factoryId);
    if (matches.length !== 1) throw new Error("native-codec.publication.factory-identity");
    return matches[0];
  };
  const factory = exact(factories.receipts, "factory_id");
  const binding = exact(definition.codecs.map((codec: any) => codec.native_factory).filter(Boolean), "factory_id");
  const codec = exact(catalog.nativeCodecs, "factoryId");
  for (const row of [factory, binding]) if (row.artifact_kind !== receipt.artifactKind || row.artifact_schema !== receipt.artifactSchema || row.extension !== receipt.extension) throw new Error("native-codec.publication.binding-identity");
  if (codec.artifactKind !== receipt.artifactKind || codec.artifactSchema !== receipt.artifactSchema || codec.extension !== receipt.extension) throw new Error("native-codec.publication.catalog-identity");
  factory.pack_schema_hash = binding.pack_schema_hash = codec.packSchemaHash = receipt.packSchemaHash;
  factory.protocol_source_sha256 = codec.protocolSourceSha256 = receipt.protocolSourceSha256;
  for (const target of catalog.openTargets ?? []) if (target.factoryId === receipt.factoryId) {
    if (target.artifactKind !== receipt.artifactKind || target.artifactSchema !== receipt.artifactSchema || target.extension !== receipt.extension) throw new Error("native-codec.publication.target-identity");
    target.packSchemaHash = receipt.packSchemaHash;
    target.protocolSourceSha256 = receipt.protocolSourceSha256;
  }
  return { factories, definition, catalog };
}
