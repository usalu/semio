import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import sparseSchema from"../🪶️sparse/🧬️schema/🔣️.json" with { type: "json" };
const protocol=schema.definitions.Protocol.const,image=imageSchema.definitions.ImageProtocol.const,sparse=sparseSchema.definitions.Protocol.const;
const size=(bytes:number)=>{if(!Number.isSafeInteger(bytes)||bytes<image.minimumFilesystemBytes)throw Error("Native image size at or above the minimum filesystem required");return bytes;};
/** 📏️ Predicts the byte size `statfs` reports for an empty APFS volume of a fixed image of the given size: an image created at its declared size keeps its container header, an image stretched from its initial extent hands the whole size to the volume. */
export function nativeImageVolumeBytesV1(imageBytes:number):number{const bytes=size(imageBytes);return bytes<=sparse.initialBytes?bytes-protocol.containerFixedBytes:bytes;}
/** 🧮️ Bounds the bytes an empty volume already holds for filesystem metadata, rounded up; APFS adds about one more MiB once the volume passes 2 GiB. */
export function nativeImageVolumeMetadataBytesV1(imageBytes:number):number{const volume=nativeImageVolumeBytesV1(imageBytes);return protocol.metadataFixedBytes+(volume>protocol.metadataLargeThresholdBytes?protocol.metadataLargeExtraBytes:0)+Math.ceil(volume/protocol.metadataDivisor);}
/** 🪫️ Bounds the bytes the stored payload may use after filesystem metadata and the journal with its checkpoint are reserved. */
export function nativeImageUsableBytesV1(imageBytes:number):number{return nativeImageVolumeBytesV1(imageBytes)-nativeImageVolumeMetadataBytesV1(imageBytes)-image.journalMaximumBytes-image.journalCheckpointBytes;}
