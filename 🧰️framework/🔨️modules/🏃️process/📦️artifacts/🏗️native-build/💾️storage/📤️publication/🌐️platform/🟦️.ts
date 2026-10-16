import{publishNativeImagePublicationV1,type NativeImagePublicationOptionsV1,type NativeImagePublicationReceiptV1,type NativeImagePublicationResultV1}from"../🎛️owner/🟦️.ts";
export type{NativeImagePublicationOptionsV1,NativeImagePublicationReceiptV1,NativeImagePublicationResultV1};
/** 📤️ Publishes current assets explicitly; ordinary open never loads or falls back to this producer. */
export function publishNativePlatformImageV1(options:NativeImagePublicationOptionsV1,request:unknown):Promise<NativeImagePublicationResultV1>{return publishNativeImagePublicationV1(options,request);}
