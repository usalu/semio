import{requireIssuedResourceControlV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{createHash}from"node:crypto";
import{win32}from"node:path";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../🔒️leases/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{encodeNativeFixedVhdFooterV1}from"../📦️vhd/🟦️.ts";
import{nativeFixedWindowsKernelV1}from"../🎛️kernel/🟦️.ts";
import schema from"./🧬️schema.json"with{type:"json"};
const contract=JSON.parse(JSON.stringify(schema))as typeof schema,protocol=Object.freeze({...contract.definitions.Protocol.const});
function captureAllocationDataV1(value:unknown,depth=0):unknown{if(value===null||typeof value!=="object")return value;if(depth>1||Array.isArray(value)||Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null)throw Error("Closed primitive allocation data required");const result:Record<string,unknown>={};for(const key of Reflect.ownKeys(value)){const descriptor=Object.getOwnPropertyDescriptor(value,key)!;if(typeof key!=="string"||!("value"in descriptor))throw Error("Allocation data accessors refused");Object.defineProperty(result,key,{value:captureAllocationDataV1(descriptor.value,depth+1),enumerable:true});}return Object.freeze(result);}
type Facts=Readonly<{volume:string;volumeId:string;index:string;fileId:string;allocation:number;eof:number;links:number;attributes:number}>;
/** 🧾️ Binds the closed system creator's descriptor identity to the parent runtime's actual file descriptor. */
export type NativeFixedAllocationReceiptV1=Readonly<{version:1;path:string;written:number;footerSha256:string;before:Facts;after:Facts;named:Facts;closed:true}>;
/** 📏️ Requires complete same-handle physical allocation and exact parent descriptor facts. */
export function readNativeFixedAllocationV1(value:unknown,expected:Readonly<{path:string;filesystemBytes:number;maximumPhysicalBytes:number;footerSha256:string}>,actual:Readonly<{dev:bigint;ino:bigint;size:bigint;nlink:bigint;blocks:bigint}>,control:ResourceLeaseControl):NativeFixedAllocationReceiptV1{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);const captured=captureAllocationDataV1(value);if(validateJsonSchemaSubset(contract.definitions.Receipt,captured,contract).length)throw Error("Closed fixed allocation receipt required");const row=captured as NativeFixedAllocationReceiptV1,length=expected.filesystemBytes+512,identity=(x:Facts)=>[x.volume,x.volumeId,x.index,x.fileId].join(":"),same=(x:Facts,y:Facts)=>Object.keys(x).every(key=>x[key as keyof Facts]===y[key as keyof Facts]);if(row.path!==expected.path||row.written!==length||row.footerSha256!==expected.footerSha256||row.before.eof!==0||row.before.allocation!==0||identity(row.before)!==identity(row.after)||!same(row.after,row.named)||row.after.eof!==length||row.after.allocation<length||row.after.allocation>expected.maximumPhysicalBytes||[row.before,row.after,row.named].some(facts=>facts.links!==1||(facts.attributes&~protocol.allowedAttributes)!==0)||actual.dev!==BigInt(row.after.volume)||actual.ino!==BigInt(row.after.index)||actual.size!==BigInt(length)||actual.nlink!==1n||actual.blocks*512n<BigInt(length)||actual.blocks*512n>BigInt(expected.maximumPhysicalBytes))throw Error("Exact fully allocated fixed creator identity required");checkResourceLeaseControl(control);return Object.freeze({...row,before:Object.freeze(row.before),after:Object.freeze(row.after),named:Object.freeze(row.named)});
}
/** 🏗️ Builds the closed one-shot Windows action using first-party footer bytes and the caller's actual phase. */
export function nativeFixedAllocationScriptV1(input:Readonly<{path:string;filesystemBytes:number;maximumPhysicalBytes:number;timestampSeconds:number;uuid:string}>,control:ResourceLeaseControl):Readonly<{script:string;footerSha256:string}>{
 requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);const captured=captureAllocationDataV1(input);if(validateJsonSchemaSubset(contract.definitions.Request,captured,contract).length)throw Error("Closed sequential fixed allocation request required");const{path,filesystemBytes,maximumPhysicalBytes,timestampSeconds,uuid}=captured as typeof input;if(path.length>protocol.maximumPathUnits||!/^[A-Za-z]:\\/u.test(path)||win32.normalize(path)!==path||/[\u0000-\u001f<>:"|?*\/]/u.test(path.slice(2))||path.slice(3).split("\\").some(part=>/[. ]$/u.test(part)||/^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(part))||win32.basename(path)!=="fixed.vhd"||!path.isWellFormed())throw Error("Canonical bounded fixed creator path required");const footer=encodeNativeFixedVhdFooterV1({version:1,filesystemBytes,maximumPhysicalBytes,timestampSeconds,uuid}),footerSha256=createHash("sha256").update(footer).digest("hex"),quote=(text:string)=>"'"+text.replaceAll("'","''")+"'",script=nativeFixedWindowsKernelV1()+String.raw`
[Console]::OutputEncoding=[Text.UTF8Encoding]::new($false)
$f=${quote(path)}
$filesystem=[long]${filesystemBytes}
$maximum=[long]${maximumPhysicalBytes}
$footer=[Convert]::FromBase64String('${Buffer.from(footer).toString("base64")}')
$footerSha='${footerSha256}'
function Read-NativeBytes($handle,$kind,$size){$ptr=[Runtime.InteropServices.Marshal]::AllocHGlobal($size);try{if($kind -eq -1){$ok=$native.GetMethod('GetFileInformationByHandle').Invoke($null,[object[]]@($handle,$ptr))}else{$ok=$native.GetMethod('GetFileInformationByHandleEx').Invoke($null,[object[]]@($handle,[int]$kind,$ptr,[uint32]$size))};if($ok -eq 0){throw 'Actual file handle observation failed'};$bytes=New-Object byte[] $size;[Runtime.InteropServices.Marshal]::Copy($ptr,$bytes,0,$size);return ,$bytes}finally{[Runtime.InteropServices.Marshal]::FreeHGlobal($ptr)}}
function Observe-Creator($stream){$handle=$stream.SafeFileHandle.DangerousGetHandle();$info=Read-NativeBytes $handle -1 52;$standard=Read-NativeBytes $handle 1 24;$id=Read-NativeBytes $handle 18 24;$tag=Read-NativeBytes $handle 9 8;$attributes=[BitConverter]::ToUInt32($tag,0);$links=[BitConverter]::ToUInt32($standard,16);$allocation=[BitConverter]::ToInt64($standard,0);$eof=[BitConverter]::ToInt64($standard,8);$volume=[BitConverter]::ToUInt32($info,28);[byte[]]$indexBytes=$info[48..51]+$info[44..47];$index=[BitConverter]::ToUInt64($indexBytes,0);if($standard[20]-ne 0-or $standard[21]-ne 0-or $links-ne 1-or [BitConverter]::ToUInt32($info,40)-ne 1-or ($attributes-band (-bnot ${protocol.allowedAttributes}))-ne 0-or $attributes-ne [BitConverter]::ToUInt32($info,0)-or $allocation-lt 0-or $allocation-gt $maximum-or $eof-lt 0-or $eof-gt $maximum){throw 'Bounded ordinary exact creator handle required'};return @{volume=$volume.ToString([Globalization.CultureInfo]::InvariantCulture);volumeId=([BitConverter]::ToUInt64($id,0)).ToString([Globalization.CultureInfo]::InvariantCulture);index=$index.ToString([Globalization.CultureInfo]::InvariantCulture);fileId=([BitConverter]::ToString($id,8,16)).Replace('-','').ToLowerInvariant();allocation=$allocation;eof=$eof;links=$links;attributes=$attributes}}
function Assert-Same($a,$b){foreach($key in @('volume','volumeId','index','fileId')){if($a[$key]-cne $b[$key]){throw 'Original creator handle changed'}}}
function Observe-Named($original){$named=$null;$failed=$false;$primary=$null;$facts=$null;try{$named=[IO.FileStream]::new($f,[IO.FileMode]::Open,[IO.FileAccess]::Read,[IO.FileShare]::ReadWrite);$facts=Observe-Creator $named;Assert-Same $original $facts}catch{$failed=$true;$primary=$_};if($null-ne $named){try{$safe=$named.SafeFileHandle;$named.Dispose();if(!$safe.IsClosed){throw 'Named descriptor closure unproved'}}catch{if(!$failed){$failed=$true;$primary=$_}else{[Console]::Error.WriteLine('Named descriptor close failed after primary')}}};if($failed){throw $primary};return $facts}
$creator=$null;$failed=$false;$primary=$null;$receipt=$null
try{
 $creator=[IO.FileStream]::new($f,[IO.FileMode]::CreateNew,[IO.FileAccess]::ReadWrite,[IO.FileShare]::Read)
 $before=Observe-Creator $creator
 if($before.eof-ne 0-or $before.allocation-ne 0){throw 'Fresh zero length creator required'}
 [void](Observe-Named $before)
 $buffer=New-Object byte[] ${protocol.chunkBytes}
 for([long]$written=0;$written-lt $filesystem;){$count=[int][Math]::Min([long]$buffer.Length,$filesystem-$written);$creator.Write($buffer,0,$count);$written+=$count;if($creator.Position-ne $written){throw 'Sequential creator position changed'}}
 $creator.Write($footer,0,$footer.Length)
 $creator.Flush($true)
 $after=Observe-Creator $creator;Assert-Same $before $after
 if($after.eof-ne $filesystem+512-or $after.allocation-lt $after.eof){throw 'Complete physical fixed allocation required'}
 $named=Observe-Named $after
 foreach($key in @('allocation','eof','links','attributes')){if($after[$key]-ne $named[$key]){throw 'Named creator allocation changed'}}
 $receipt=@{version=1;path=$f;written=$creator.Position;footerSha256=$footerSha;before=$before;after=$after;named=$named;closed=$true}
}catch{$failed=$true;$primary=$_}
if($null-ne $creator){try{$safe=$creator.SafeFileHandle;$creator.Dispose();if(!$safe.IsClosed){throw 'Creator descriptor closure unproved'}}catch{if(!$failed){$failed=$true;$primary=$_}else{[Console]::Error.WriteLine('Creator close failed after primary')}}}
if($failed){throw $primary}
if($null-eq $receipt){throw 'Closed creator receipt required'}
$receipt|ConvertTo-Json -Depth 4 -Compress
`;checkResourceLeaseControl(control);return Object.freeze({script,footerSha256});
}
