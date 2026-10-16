import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema.json"with{type:"json"};
const contract=JSON.parse(JSON.stringify(schema))as typeof schema,p=Object.freeze({...contract.definitions.Protocol.const});

/** 🧾️ Carries explicit fixed-image facts; UUID bytes and VHD-epoch seconds are supplied by the owner. */
export type NativeFixedVhdFooterRequestV1=Readonly<{version:1;filesystemBytes:number;maximumPhysicalBytes:number;timestampSeconds:number;uuid:string}>;

/** 💽️ Encodes the Microsoft fixed VHD footer without IO, clock, entropy or image allocation.
 * @see https://www.microsoft.com/en-us/download/details.aspx?id=23850
 */
export function encodeNativeFixedVhdFooterV1(value:unknown):Uint8Array{
 if(typeof value!=="object"||value===null||Array.isArray(value))throw Error("Closed fixed VHD footer request required");const descriptors=Object.getOwnPropertyDescriptors(value),snapshot:Record<string,unknown>={};for(const key of Reflect.ownKeys(descriptors)){if(typeof key!=="string")throw Error("Closed fixed VHD footer keys required");const descriptor=Reflect.get(descriptors,key)as PropertyDescriptor;if(!("value"in descriptor))throw Error("Fixed VHD primitive data fields required");Object.defineProperty(snapshot,key,{value:descriptor.value,enumerable:true});}if(validateJsonSchemaSubset(contract.definitions.Request,snapshot,contract).length)throw Error("Closed fixed VHD footer request required");const request=Object.freeze(snapshot)as NativeFixedVhdFooterRequestV1;
 if(request.filesystemBytes%p.unitBytes||request.maximumPhysicalBytes%p.unitBytes||request.maximumPhysicalBytes<request.filesystemBytes+p.unitBytes)throw Error("Exact fixed VHD allocation envelope required");
 const total=Math.min(request.filesystemBytes/p.sectorBytes,p.maximumCylinders*p.maximumHeads*p.maximumSectorsPerTrack);let sectors=p.smallSectorsPerTrack,heads=Math.max(p.minimumHeads,Math.floor((Math.floor(total/sectors)+p.cylinderTrackLimit-1)/p.cylinderTrackLimit));
 if(total>=p.maximumCylinders*p.maximumHeads*p.largeSectorsPerTrack){sectors=p.maximumSectorsPerTrack;heads=p.maximumHeads;}else{if(Math.floor(total/sectors)>=heads*p.cylinderTrackLimit||heads>p.maximumHeads){sectors=p.mediumSectorsPerTrack;heads=p.maximumHeads;}if(Math.floor(total/sectors)>=heads*p.cylinderTrackLimit)sectors=p.largeSectorsPerTrack;}
 const bytes=new Uint8Array(p.footerBytes),view=new DataView(bytes.buffer),ascii=(text:string,offset:number)=>{for(let index=0;index<text.length;index++)bytes[offset+index]=text.charCodeAt(index);};
 ascii(p.cookie,0);view.setUint32(8,p.features);view.setUint32(12,p.fileVersion);view.setBigUint64(16,0xffffffffffffffffn);view.setUint32(24,request.timestampSeconds);ascii(p.creatorApplication,28);view.setUint32(32,p.creatorVersion);ascii(p.creatorHost,36);view.setBigUint64(40,BigInt(request.filesystemBytes));view.setBigUint64(48,BigInt(request.filesystemBytes));view.setUint16(56,Math.floor(total/(sectors*heads)));view.setUint8(58,heads);view.setUint8(59,sectors);view.setUint32(60,p.diskType);for(let index=0;index<16;index++)bytes[68+index]=parseInt(request.uuid.slice(index*2,index*2+2),16);view.setUint8(84,p.savedState);view.setUint32(64,(~bytes.reduce((sum,byte)=>sum+byte,0))>>>0);return bytes;
}
