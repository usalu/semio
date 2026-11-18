import{nativeImageAttachProgramV1}from"../👤️owners/🟦️.ts";
/** 🛠️ The single place that names the macOS disk-image tool; every other module receives the program and its closed environment from here. See https://ss64.com/mac/hdiutil.html */
export const nativeImageHdiutilProgramV1="/usr/bin/hdiutil";

const environment=Object.freeze({PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"});
/** 🧾️ Builds the owned command data of one image tool invocation: the closed program (`diskutil` for the `image` verb family that attaches through the UNMAP-capable stack, `hdiutil` for everything else), the caller's arguments and a locale-free environment. See https://ss64.com/mac/diskutil.html */
export function nativeImageHdiutilCommandV1(args:readonly string[],cwd:string):Readonly<{program:string;args:readonly string[];cwd:string;env:Readonly<Record<string,string>>}>{return Object.freeze({program:args[0]==="image"?nativeImageAttachProgramV1:nativeImageHdiutilProgramV1,args:Object.freeze([...args]),cwd,env:environment});}
