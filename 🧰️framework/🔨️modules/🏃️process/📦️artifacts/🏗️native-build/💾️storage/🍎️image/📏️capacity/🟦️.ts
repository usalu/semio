import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
const protocol=schema.definitions.Protocol.const,image=imageSchema.definitions.ImageProtocol.const;
const size=(bytes:number)=>{if(!Number.isSafeInteger(bytes)||bytes<image.minimumFilesystemBytes)throw Error("Native image size at or above the minimum filesystem required");return bytes;};
/** 📏️ Predicts the byte size `statfs` reports for an empty APFS volume created in a fixed image of the given size. */
export function nativeImageVolumeBytesV1(imageBytes:number):number{return size(imageBytes)-protocol.containerFixedBytes-(imageBytes>protocol.largeContainerThresholdBytes?protocol.largeContainerReserveBytes:0);}
/** 🧮️ Bounds the bytes an empty volume already holds for filesystem metadata, rounded up. */
export function nativeImageVolumeMetadataBytesV1(imageBytes:number):number{return protocol.metadataFixedBytes+Math.ceil(nativeImageVolumeBytesV1(imageBytes)/protocol.metadataDivisor);}
/** 🪫️ Bounds the bytes the stored payload may use after filesystem metadata and the journal with its checkpoint are reserved. */
export function nativeImageUsableBytesV1(imageBytes:number):number{return nativeImageVolumeBytesV1(imageBytes)-nativeImageVolumeMetadataBytesV1(imageBytes)-image.journalMaximumBytes-image.journalCheckpointBytes;}
