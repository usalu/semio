import {lstatSync,mkdirSync,existsSync,statfsSync} from "node:fs";
import {dirname} from "node:path";
import {captureOwnedCommandV1} from "../../../../🎛️owned-execution/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {openNativeFixedPlatformV1,nativeFixedVhdFooterV1,readNativeFixedVhdFooterV1,type NativeFixedPlatformOptionsV1,type NativeFixedPlatformPortV1} from "../🧱️fixed/🟦️.ts";
import type {NativeImageStorageV1} from "../🌐️platform/📋️contract/🟦️.ts";

const literal=(value:string)=>"'"+value.replace(/'/gu,"''")+"'";
const windows:NativeFixedPlatformPortV1={filename:"fixed.vhd",prepare:async(options,image,root,fresh)=>{
 checkResourceLeaseControl(options.control);const capture=(script:string)=>captureOwnedCommandV1("powershell.exe",["-NoLogo","-NoProfile","-NonInteractive","-Command","$ErrorActionPreference='Stop';"+script],dirname(options.directory),{control:options.control});
 await capture("$p=[Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent();if(!$p.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)){throw 'Fixed VHD requires actual elevated capability'};foreach($c in @('New-VHD','Mount-DiskImage','Get-DiskImage','Get-Disk','Initialize-Disk','New-Partition','Format-Volume','Add-PartitionAccessPath')){Get-Command $c -ErrorAction Stop|Out-Null}");
 if(!fresh){nativeFixedVhdFooterV1(readNativeFixedVhdFooterV1(image),lstatSync(image).size,options.maximumPhysicalBytes,options.filesystemBytes);await capture(`Mount-DiskImage -ImagePath ${literal(image)}|Out-Null`);return;}
 if(existsSync(options.directory))throw Error("Fresh original Windows image directory required");let parent=dirname(options.directory);while(!existsSync(parent)&&dirname(parent)!==parent)parent=dirname(parent);const fs=statfsSync(parent);if(fs.bavail*fs.bsize<options.maximumPhysicalBytes+67108864)throw Error("Host capacity cannot admit fixed Windows image");mkdirSync(root,{recursive:true});
 await capture(`$f=${literal(image)};$r=${literal(root+"\\")};New-VHD -Path $f -SizeBytes ${options.filesystemBytes} -Fixed|Out-Null;$v=Mount-DiskImage -ImagePath $f -PassThru;$d=$v|Get-Disk;if(@($d).Count-ne 1-or $d.PartitionStyle-ne 'RAW'){throw 'Only new owned RAW VHD may be initialized'};$d|Initialize-Disk -PartitionStyle MBR|Out-Null;$p=New-Partition -DiskNumber $d.Number -UseMaximumSize;$p|Format-Volume -FileSystem NTFS -NewFileSystemLabel 'semio-fixed' -Confirm:$false|Out-Null;$p|Add-PartitionAccessPath -AccessPath $r`);
},observe:async(options,image,root)=>{
 checkResourceLeaseControl(options.control);const file=lstatSync(image),size=nativeFixedVhdFooterV1(readNativeFixedVhdFooterV1(image),file.size,options.maximumPhysicalBytes,options.filesystemBytes);
 const script=`$ErrorActionPreference='Stop';$f=${literal(image)};$r=${literal(root+"\\")};$v=Get-DiskImage -ImagePath $f;if(!$v.Attached){throw 'Owned VHD not attached'};$d=@($v|Get-Disk);if($d.Count-ne 1){throw 'Exact owned VHD disk required'};$p=@($d[0]|Get-Partition|Where-Object {$_.AccessPaths-contains $r});if($p.Count-ne 1){throw 'Exact owned VHD mount required'};$fs=$p[0]|Get-Volume;if($fs.FileSystem-ne 'NTFS'-or $fs.Size-gt ${size}){throw 'Finite owned NTFS required'};@{image=$v.ImagePath;device=$v.DevicePath;disk=$d[0].Number;unique=$d[0].UniqueId;partition=$p[0].PartitionNumber;volume=$fs.UniqueId;bytes=$fs.Size}|ConvertTo-Json -Compress`;
 const text=await captureOwnedCommandV1("powershell.exe",["-NoLogo","-NoProfile","-NonInteractive","-Command",script],dirname(options.directory),{control:options.control}),record=JSON.parse(text);if(typeof record.device!=="string"||typeof record.unique!=="string"||typeof record.volume!=="string"||!Number.isSafeInteger(record.bytes)||record.bytes<1||record.bytes>size)throw Error("Actual Windows volume identity required");return JSON.stringify(record);
}};

/** 🪟️ Opens only a fixed, checksummed VHD with the exact attached NTFS mount; missing system capability refuses. */
export async function openNativeWindowsImageStorageV1(options:NativeFixedPlatformOptionsV1):Promise<NativeImageStorageV1>{checkResourceLeaseControl(options.control);if(process.platform!=="win32")throw Error("Windows fixed VHD backend unavailable on this platform");return openNativeFixedPlatformV1(options,windows);}
