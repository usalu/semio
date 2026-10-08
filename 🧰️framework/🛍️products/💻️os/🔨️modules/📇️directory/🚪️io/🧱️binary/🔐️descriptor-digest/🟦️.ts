import { validateDocumentDescriptorV1, type DocumentDescriptor } from "../../../🧬️schema/🟦️.ts";

export const DESCRIPTOR_DIGEST_V1_DOMAIN = "semio.document-descriptor.digest.v1\0";

function descriptorDigestInteger(value: number, width: 4 | 8, field: string): Uint8Array {
  if (!Number.isSafeInteger(value) || value < 0 || (width === 4 && value > 0xffff_ffff)) throw new Error(`descriptor.invalid-${field}`);
  const output = new Uint8Array(width);
  let remaining = BigInt(value);
  for (let index = width - 1; index >= 0; index--) {
    output[index] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }
  return output;
}

function descriptorDigestHash(value: string, field: string): Uint8Array {
  if (!/^[0-9a-f]{64}$/.test(value) || /^0{64}$/.test(value)) throw new Error(`descriptor.invalid-${field}`);
  return Uint8Array.from({ length: 32 }, (_, index) => Number.parseInt(value.slice(index * 2, index * 2 + 2), 16));
}

function descriptorDigestText(value: string, field: string): Uint8Array {
  if (value.length === 0) throw new Error(`descriptor.empty-${field}`);
  return new TextEncoder().encode(value);
}

/** 🧬️ Domain plus declaration-ordered descriptor leaves, each encoded as
 * `u64_be(payload byte length) || payload`; text is UTF-8, integers are unsigned big-endian fixed-
 * width payloads, and hash text is decoded to 32 bytes. JSON serialization never participates. */
export function descriptorDigestEncodingV1(descriptor: DocumentDescriptor): Uint8Array<ArrayBuffer> {
  validateDocumentDescriptorV1(descriptor);
  const fields = [
    descriptorDigestText(descriptor.spaceId, "space-id"),
    descriptorDigestText(descriptor.documentId, "document-id"),
    descriptorDigestText(descriptor.artifactKind, "artifact-kind"),
    descriptorDigestText(descriptor.artifactSchema, "artifact-schema"),
    descriptorDigestText(descriptor.owner.pluginId, "owner-plugin-id"),
    descriptorDigestText(descriptor.owner.packageId, "owner-package-id"),
    descriptorDigestText(descriptor.owner.version, "owner-version"),
    descriptorDigestHash(descriptor.owner.packageHash, "owner-package-hash"),
    descriptorDigestHash(descriptor.packSchemaHash, "pack-schema-hash"),
    descriptorDigestInteger(descriptor.bootstrapVersion, 4, "bootstrap-version"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.headSeq, 8, "bootstrap-head-seq"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.commitSeq, 8, "bootstrap-commit-seq"),
    descriptorDigestInteger(descriptor.bootstrapFrontier.epoch, 8, "bootstrap-epoch"),
    descriptorDigestHash(descriptor.bootstrapSnapshotHash, "bootstrap-snapshot-hash"),
  ];
  const domain = new TextEncoder().encode(DESCRIPTOR_DIGEST_V1_DOMAIN);
  const total = fields.reduce((length, field) => length + 8 + field.length, domain.length);
  if (!Number.isSafeInteger(total)) throw new Error("descriptor.length-overflow");
  const output = new Uint8Array(total);
  output.set(domain);
  let offset = domain.length;
  for (const field of fields) {
    output.set(descriptorDigestInteger(field.length, 8, "field-length"), offset);
    offset += 8;
    output.set(field, offset);
    offset += field.length;
  }
  return output;
}

/** 🔐️ Host-Web-Crypto SHA-256 over {@link descriptorDigestEncodingV1}. */
export async function descriptorDigestV1(descriptor: DocumentDescriptor): Promise<Uint8Array<ArrayBuffer>> {
  return new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", descriptorDigestEncodingV1(descriptor)));
}

