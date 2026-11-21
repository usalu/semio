import { lstat } from "node:fs/promises";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { fileDigest, readArtifactBytesV1, readArtifactPrefixV1 } from "../../🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import wire from "../../🧠️interpreter/🏗️instance/🧊️prepared/🧵️image-wire/🧬️schema/prepared-image.json" with { type: "json" };

export const actorPreparationPolicyV1 = Object.freeze({ ...schema.definitions.ActorPreparation.const });
const imageMagic = Buffer.from(wire.properties.prefix.const, "hex");
const imageBindingBytes = imageMagic.length + 32;

export type ActorPreparationReceiptV1 = Readonly<{
  version: 1;
  componentSha256: string;
  imageSha256: string;
  imageBytes: number;
}>;

/** 🌱️ Admits the prepared image's exact retained byte identity. */
export function actorPreparationReceiptV1(value: unknown, componentSha256: string, imageSha256: string, imageBytes: number): ActorPreparationReceiptV1 {
  if (validateJsonSchemaSubset(schema.definitions.ActorPreparationReceipt, value).length) {
    throw Error("Closed actor preparation receipt required");
  }
  const receipt = value as ActorPreparationReceiptV1;
  if (receipt.componentSha256 !== componentSha256 || receipt.imageSha256 !== imageSha256 || receipt.imageBytes !== imageBytes) {
    throw Error("Actor preparation receipt differs from the retained model");
  }
  return Object.freeze({ version: 1, componentSha256, imageSha256, imageBytes });
}

/** 🔗️ Checks the constructor's own full component binding before accepting a preparation file. */
export function assertActorPreparationBindingV1(image: Uint8Array, componentSha256: string): void {
  const header = Buffer.from(image.buffer, image.byteOffset, Math.min(image.byteLength, imageBindingBytes));
  if (!header.subarray(0, imageMagic.length).equals(imageMagic) || header.subarray(imageMagic.length).toString("hex") !== componentSha256) {
    throw Error("Actor preparation header differs from the retained component");
  }
}

/** 📦️ Observes the finite image file held by the model builder or reader. */
export async function actorPreparationImageIdentityV1(path: string, signal: AbortSignal): Promise<Readonly<{ imageSha256: string; imageBytes: number }>> {
  const before = await lstat(path, { bigint: true });
  if (!before.isFile() || before.nlink !== 1n || before.size === 0n || before.size > BigInt(actorPreparationPolicyV1.maximumBytes)) {
    throw Error("Finite retained actor image required");
  }
  const imageSha256 = await fileDigest(path, signal);
  const after = await lstat(path, { bigint: true });
  if (before.dev !== after.dev || before.ino !== after.ino || before.size !== after.size || before.ctimeNs !== after.ctimeNs) {
    throw Error("Retained actor preparation changed during verification");
  }
  return Object.freeze({ imageSha256, imageBytes: Number(before.size) });
}

/** 🧾️ Verifies the optional prepared actor under an already retained model directory. */
export async function readActorPreparationV1(image: string, receiptPath: string, componentSha256: string, signal: AbortSignal): Promise<ActorPreparationReceiptV1> {
  const bytes = await readArtifactBytesV1(receiptPath, actorPreparationPolicyV1.maximumReceiptBytes, signal);
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
  const value: unknown = JSON.parse(text);
  assertActorPreparationBindingV1(await readArtifactPrefixV1(image, imageBindingBytes, actorPreparationPolicyV1.maximumBytes, signal), componentSha256);
  const { imageSha256, imageBytes } = await actorPreparationImageIdentityV1(image, signal);
  const receipt = actorPreparationReceiptV1(value, componentSha256, imageSha256, imageBytes);
  if (text !== JSON.stringify(receipt) + "\n") throw Error("Canonical actor preparation receipt required");
  return receipt;
}

/** 📥️ Reads the bounded output of an emitter whose component may opt in to preparation. */
export async function readOptionalActorImageV1(path: string, signal: AbortSignal): Promise<Uint8Array | undefined> {
  try {
    await lstat(path);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
    throw error;
  }
  const image = await readArtifactBytesV1(path, actorPreparationPolicyV1.maximumBytes, signal);
  if (!image.length) throw Error("Empty prepared actor image");
  return image;
}
