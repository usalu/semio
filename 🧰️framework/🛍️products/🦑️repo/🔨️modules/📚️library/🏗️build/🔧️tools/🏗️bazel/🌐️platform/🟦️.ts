import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with { type: "json" };

const rules=schema.definitions.PlatformRules.const;
export type BazelPlatformNameV1="win32"|"darwin"|"linux";
export type BazelPlatformRulesV1={readonly separator:string;readonly launcherExtension:string;readonly sourceLink:"dir"|"junction";readonly startup:readonly string[];readonly command:readonly string[];readonly environment:Readonly<Record<string,string>>;readonly emojiLabels:"verified"|"unverified"};
export type BazelPlatformInputV1={readonly platform:BazelPlatformNameV1;readonly arch:"x64"|"arm64";readonly container:boolean;readonly outputBase:string;readonly bun:string;readonly longPaths:"enabled"|"disabled"|"unprobed";readonly closure:BazelPathClosureV1};
export type BazelPlatformProfileV1={readonly version:1;readonly platform:BazelPlatformNameV1;readonly arch:"x64"|"arm64";readonly startup:readonly string[];readonly command:readonly string[];readonly environment:Readonly<Record<string,string>>;readonly launcherExtension:string;readonly sourceLink:"dir"|"junction";readonly shell:"none";readonly emojiLabels:"verified"|"unverified";readonly pathBudget:BazelPathBudgetV1;readonly refusals:readonly string[];readonly warnings:readonly string[]};

const paths=schema.definitions.PathBudget.const;
export type BazelPathSizeV1={readonly units:number;readonly bytes:number};
export type BazelPathClosureV1={readonly alias:BazelPathSizeV1;readonly real:BazelPathSizeV1};
export type BazelPathBudgetV1={readonly outputBase:BazelPathSizeV1;readonly families:readonly (BazelPathSizeV1&{readonly name:string})[];readonly limitUnits:number;readonly limitBytes:number|null;readonly slackUnits:number;readonly slackBytes:number|null;readonly fits:boolean};
export type BazelPathBudgetInputV1=Pick<BazelPlatformInputV1,"platform"|"arch"|"outputBase"|"longPaths"|"closure">;

/** 📏️ Measures the longest in-package source labels of the aliased and of the repository-layout view in UTF-16 units and UTF-8 bytes, the two lengths a host path limit is stated in. */
export function bazelPathClosureV1(alias:Iterable<string>,real:Iterable<string>):BazelPathClosureV1{
 const encoder=new TextEncoder(),size=(labels:Iterable<string>)=>{let units=0,bytes=0;for(const text of labels){units=Math.max(units,text.length);bytes=Math.max(bytes,encoder.encode(text).length);}return{units,bytes};};
 return{alias:size(alias),real:size(real)};
}

/** 📐️ Computes every path family Bazel creates beneath an output base from the measured source labels: the repository rule of 256 UTF-16 units applies everywhere, and each host adds its own limit (PATH_MAX in UTF-8 bytes on macOS and Linux, MAX_PATH in UTF-16 units on Windows unless long paths are enabled); `slackUnits` and `slackBytes` are what the longest family leaves free, negative when it does not fit. */
export function bazelPathBudgetV1(input:BazelPathBudgetInputV1):BazelPathBudgetV1{
 const row=paths.platforms[input.platform],runfiles=!bazelPlatformRulesV1(input.platform).command.includes("--noenable_runfiles"),encoder=new TextEncoder(),base={units:input.outputBase.length,bytes:encoder.encode(input.outputBase).length},families:(BazelPathSizeV1&{name:string})[]=[];
 const filler:Readonly<Record<string,string>>={configuration:row.configuration[input.arch],task:paths.task,file:paths.file,sandbox:row.sandboxes.reduce((longest,name)=>name.length>longest.length?name:longest,""),sandboxId:"0".repeat(paths.sandboxDigits)};
 for(const family of paths.families as readonly {name:string;template:string;view?:"alias"|"real";needs?:"runfiles"|"sandbox"}[]){
  if((family.needs==="runfiles"&&!runfiles)||(family.needs==="sandbox"&&!row.sandboxes.length))continue;
  const source=family.view===undefined?{units:0,bytes:0}:input.closure[family.view];if(family.view!==undefined&&source.units===0)continue;
  const prefix=family.template.replace("{source}","").replace(/\{(\w+)\}/gu,(_match,key:string)=>filler[key]!).length;
  families.push({name:family.name,units:base.units+1+prefix+source.units,bytes:base.bytes+1+prefix+source.bytes});
 }
 const operating=input.platform==="win32"?(input.longPaths==="enabled"?paths.platforms.win32.extendedMax:row.pathMax)-1:Infinity,limitUnits=Math.min(paths.repositoryUnits,operating),limitBytes=row.unit==="utf8"?row.pathMax-1:null;
 const slackUnits=limitUnits-Math.max(...families.map(family=>family.units)),slackBytes=limitBytes===null?null:limitBytes-Math.max(...families.map(family=>family.bytes));
 return Object.freeze({outputBase:base,families,limitUnits,limitBytes,slackUnits,slackBytes,fits:slackUnits>=0&&(slackBytes===null||slackBytes>=0)});
}

/** 🚧️ Names the family that exceeds the budget most and how many characters shorter the checkout must be. */
function bazelPathRefusalV1(input:BazelPathBudgetInputV1,budget:BazelPathBudgetV1):string{
 const excess=(family:BazelPathSizeV1)=>Math.max(family.units-budget.limitUnits,budget.limitBytes===null?-Infinity:family.bytes-budget.limitBytes),worst=[...budget.families].sort((left,right)=>excess(right)-excess(left))[0]!,windows=input.platform==="win32";
 return `The Bazel path budget is exceeded: ${worst.name} reaches ${worst.units} UTF-16 units (${worst.bytes} bytes) beneath the output base ${input.outputBase}, beyond the limit of ${budget.limitUnits} units${budget.limitBytes===null?"":` and ${budget.limitBytes} bytes`} (repository rule ${paths.repositoryUnits}${windows?`, Windows MAX_PATH ${paths.platforms.win32.pathMax-1} without long paths`:""}); move the checkout to a directory at least ${excess(worst)} characters shorter${windows?", for example C:\\semio":""}.`;
}

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
 const pathBudget=bazelPathBudgetV1(actual);
 if(!pathBudget.fits)refusals.push(bazelPathRefusalV1(actual,pathBudget));
 if(windows&&actual.longPaths==="unprobed")warnings.push("Windows long path support was not probed; the path budget assumes the 259 unit limit.");
 if(row.emojiLabels==="unverified")warnings.push(`Emoji source labels have not been proven on ${actual.platform}.`);
 return Object.freeze({version:1,platform:actual.platform,arch:actual.arch,startup:[...row.startup],command:[...row.command],environment:{...row.environment},launcherExtension:row.launcherExtension,sourceLink:row.sourceLink,shell:"none",emojiLabels:row.emojiLabels,pathBudget,refusals,warnings});
}
