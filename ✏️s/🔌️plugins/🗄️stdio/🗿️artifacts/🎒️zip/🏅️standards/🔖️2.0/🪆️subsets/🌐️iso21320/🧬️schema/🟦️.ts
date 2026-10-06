/** 🧬️ ZipSnapshot schema (🌐️iso21320 subset) meta — reuses the 🧱️base subset's schema verbatim. */
export const meta = {
  artifactKind: "s.stdio.zip",
  standard: "2.0",
  subset: "iso21320",
} as const;

/** 🛡️ Owned source diagnostics matching native ISO member-header policy. */
export interface ZipIso21320Diagnostic { readonly code:string; readonly severity:"Error"|"Warning"; readonly message:string }
import type {ZipSnapshot} from "../../🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";

/** 🛡️ Reads independent owned headers without materializing native archive bytes. */
export async function checkZipIso21320Conformance(snapshot:ZipSnapshot,control=new NativeDecodeControl(0,()=>true)):Promise<readonly ZipIso21320Diagnostic[]>{
  return control.scopedStage(async()=>{
  await control.beginStage(snapshot.entries.length);
  const diagnostics:ZipIso21320Diagnostic[]=[];
  for(let index=0;index<snapshot.entries.length;index++){
    const entry=snapshot.entries[index]!;
    const flags=entry.metadata.local.flags|entry.metadata.central.flags;
    const version=Math.max(entry.metadata.local.versionNeeded,entry.metadata.central.versionNeeded);
    const name=JSON.stringify(entry.name);
    if(flags&1)diagnostics.push({code:"stdio.zip.iso21320.entry-encrypted",severity:"Error",message:`entry ${index} (${name}) has general-purpose bit 0 (encryption) set -- ISO/IEC 21320-1 §4.1 forbids encrypted entries`});
    if(flags&0x2040)diagnostics.push({code:"stdio.zip.iso21320.strong-encryption-or-masked-headers",severity:"Error",message:`entry ${index} (${name}) has general-purpose bit 6 and/or bit 13 (Strong Encryption / masked local header values) set -- ISO/IEC 21320-1 forbids the Strong Encryption extension entirely`});
    if(flags&8)diagnostics.push({code:"stdio.zip.iso21320.data-descriptor-present",severity:"Warning",message:`entry ${index} (${name}) has general-purpose bit 3 (trailing data descriptor) set -- interoperability warning: not every ISO/IEC 21320-1 reader trusts streamed sizes`});
    if(version>45)diagnostics.push({code:"stdio.zip.iso21320.version-needed-high",severity:"Warning",message:`entry ${index} (${name}) declares version-needed-to-extract ${version} > 45 -- signals a feature ISO/IEC 21320-1's restricted Stored/Deflate profile shouldn't require`});
    if(entry.metadata.compressionMethod!==0&&entry.metadata.compressionMethod!==8)diagnostics.push({code:"stdio.zip.iso21320.compression-method-unsupported",severity:"Error",message:`entry ${index} (${name}) declares compression method ${entry.metadata.compressionMethod} -- ISO/IEC 21320-1 §4.4 admits only Stored (0) and Deflate (8)`});
    if((index+1)%256===0)await control.advance(256);
  }
  await control.advance(snapshot.entries.length%256);
  return diagnostics;
  });
}
