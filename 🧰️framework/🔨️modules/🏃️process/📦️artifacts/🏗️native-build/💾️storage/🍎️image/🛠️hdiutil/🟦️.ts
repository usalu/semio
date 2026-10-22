/** 🛠️ The single place that names the macOS disk-image tool; every other module receives the program and its closed environment from here. See https://ss64.com/mac/hdiutil.html */
export const nativeImageHdiutilProgramV1="/usr/bin/hdiutil";

const environment=Object.freeze({PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"});
/** 🧾️ Builds the owned command data of one hdiutil invocation: the closed program, the caller's arguments and a locale-free environment. */
export function nativeImageHdiutilCommandV1(args:readonly string[],cwd:string):Readonly<{program:string;args:readonly string[];cwd:string;env:Readonly<Record<string,string>>}>{return Object.freeze({program:nativeImageHdiutilProgramV1,args:Object.freeze([...args]),cwd,env:environment});}
