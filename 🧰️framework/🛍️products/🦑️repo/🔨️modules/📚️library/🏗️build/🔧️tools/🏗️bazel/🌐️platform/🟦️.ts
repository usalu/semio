import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with { type: "json" };

const rules=schema.definitions.PlatformRules.const;
export type BazelPlatformNameV1="win32"|"darwin"|"linux";
export type BazelPlatformRulesV1={readonly separator:string;readonly launcherExtension:string;readonly sourceLink:"dir"|"junction";readonly startup:readonly string[];readonly command:readonly string[];readonly environment:Readonly<Record<string,string>>;readonly emojiLabels:"verified"|"unverified"};
export type BazelPlatformInputV1={readonly platform:BazelPlatformNameV1;readonly arch:"x64"|"arm64";readonly container:boolean;readonly outputBase:string;readonly bun:string;readonly longPaths:"enabled"|"disabled"|"unprobed"};
export type BazelPlatformProfileV1={readonly version:1;readonly platform:BazelPlatformNameV1;readonly arch:"x64"|"arm64";readonly startup:readonly string[];readonly command:readonly string[];readonly environment:Readonly<Record<string,string>>;readonly launcherExtension:string;readonly sourceLink:"dir"|"junction";readonly shell:"none";readonly emojiLabels:"verified"|"unverified";readonly pathBudget:{readonly outputBase:number;readonly execroot:number;readonly tracked:number;readonly total:number;readonly limit:number;readonly fits:boolean};readonly refusals:readonly string[];readonly warnings:readonly string[]};

/** 🚏️ Selects the authored per-platform Bazel rules; an unlisted platform is refused instead of guessed. */
export function bazelPlatformRulesV1(platform:string):BazelPlatformRulesV1{
 const row=(rules.platforms as Readonly<Record<string,BazelPlatformRulesV1>>)[platform];
 if(!row)throw Error(`Unsupported Bazel platform: ${platform}`);
 return row;
}

/** 🧭️ Renders every platform-dependent Bazel decision (flags, locale, launcher extension, source link, MAX_PATH budget) as a pure function of observed host facts, so one fixture proves all six OS and architecture pairs without running them. */
export function bazelPlatformProfileV1(input:unknown):BazelPlatformProfileV1{
 if(validateJsonSchemaSubset(schema.definitions.PlatformInput,input,schema).length)throw Error("Closed Bazel platform input required");
 const actual=input as BazelPlatformInputV1,row=bazelPlatformRulesV1(actual.platform),windows=actual.platform==="win32",refusals:string[]=[],warnings:string[]=[];
 const absolute=windows?/^(?:[A-Za-z]:\\|\\\\)[^\0]*\.exe$/iu:/^\/[^\0]*$/u;
 if(!absolute.test(actual.bun)||(!windows&&/\.exe$/iu.test(actual.bun)))refusals.push(windows?`Absolute Windows Bun executable ending in .exe required: ${actual.bun}`:`Absolute POSIX Bun executable path required: ${actual.bun}`);
 if(actual.container&&actual.platform!=="linux")refusals.push(`A devcontainer runs Linux, not ${actual.platform}`);
 const execroot=(windows?rules.execroot.win32:rules.execroot.posix).length,total=actual.outputBase.length+execroot+rules.trackedPathUnits,limit=windows?(actual.longPaths==="enabled"?rules.limits.windowsExtended:rules.limits.windows):rules.limits.posix,fits=total<=limit;
 if(!fits)refusals.push(`The Bazel output base ${actual.outputBase} (${actual.outputBase.length}) plus the execroot (${execroot}) plus a tracked path of ${rules.trackedPathUnits} units is ${total}, beyond the ${limit} unit path limit${windows?"; enable Windows long paths machine-wide or move the checkout to a shorter directory such as C:\\semio":""}.`);
 if(windows&&actual.longPaths==="unprobed")warnings.push("Windows long path support was not probed; the path budget assumes the 259 unit limit.");
 if(row.emojiLabels==="unverified")warnings.push(`Emoji source labels have not been proven on ${actual.platform}.`);
 return Object.freeze({version:1,platform:actual.platform,arch:actual.arch,startup:[...row.startup],command:[...row.command],environment:{...row.environment},launcherExtension:row.launcherExtension,sourceLink:row.sourceLink,shell:"none",emojiLabels:row.emojiLabels,pathBudget:{outputBase:actual.outputBase.length,execroot,tracked:rules.trackedPathUnits,total,limit,fits},refusals,warnings});
}
