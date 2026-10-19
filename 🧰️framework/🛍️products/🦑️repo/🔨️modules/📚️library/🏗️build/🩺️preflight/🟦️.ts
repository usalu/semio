import {access,readFile,readdir,stat,statfs} from "node:fs/promises";
import {readFileSync,constants} from "node:fs";
import {delimiter,isAbsolute,join,relative,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json";
import registry from "./🔣️policy.json";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {checkScriptInvocation,requireIssuedScriptInvocationV1,advanceScriptInvocation,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🩺️ Declares finite OS requirements before a repository store can be provisioned. */
export interface BuildPreflightRequirementsV1 {version:1;platform:"linux";architecture:"x64"|"arm64";distribution:string;distributionVersion:string;minimumAvailableBytes:number;minimumFreeLoops:number;tools:string[]}
/** 🔬️ Captures read-only facts from the actual invoking process and its workspace filesystem. */
export interface BuildPreflightFactsV1 {platform:string;architecture:string;distribution:string;distributionVersion:string;capEff:string;availableBytes:number;loopControl:boolean;freeLoops:number;tools:string[]}

/** 🎯️ Accepts only the authored owner selection; preflight has no bypass or provisioning options. */
export function readBuildPreflightScopeV1(args:readonly string[]):string {
 if(args.length!==3||args[0]!=="preflight"||args[1]!=="--scope"||!/^@[a-z0-9-]+\/[a-z0-9-]+$/u.test(args[2]!))throw Error("Expected repo preflight --scope <owner>");return args[2]!;
}
/** 📋️ Resolves one registered owner's closed requirement metadata without invoking discovery or opening stores. */
export function readBuildPreflightRequirementsV1(root:string,owner:string):BuildPreflightRequirementsV1 {
 if(validateJsonSchemaSubset(schema.definitions.Registry,registry).length)throw Error("Closed build preflight registry required");
 const rows=registry.scopes.filter(row=>row.owner===owner);if(rows.length!==1)throw Error(`Unknown build preflight scope: ${owner}`);
 const path=resolve(root,rows[0]!.project),local=relative(resolve(root),path);if(isAbsolute(local)||local===".."||local.startsWith("..\\")||local.startsWith("../")||path.length>256)throw Error("Owned build preflight project required");
 const project=JSON.parse(readFileSync(path,"utf8")),requirements=project.metadata?.semio?.preflight;
 if(project.name!==owner||validateJsonSchemaSubset(schema.definitions.Requirements,requirements).length)throw Error("Closed build preflight requirements required");return requirements;
}
/** ⚖️ Reports every unmet authored OS requirement without pretending that observations grant compiler authority. */
export function buildPreflightProblemsV1(requirements:unknown,facts:unknown):string[] {
 if(validateJsonSchemaSubset(schema.definitions.Requirements,requirements).length)throw Error("Closed build preflight requirements required");if(validateJsonSchemaSubset(schema.definitions.Facts,facts).length)throw Error("Closed build preflight facts required");
 const required=requirements as BuildPreflightRequirementsV1,actual=facts as BuildPreflightFactsV1,problems:string[]=[];
 if(actual.platform!==required.platform)problems.push(`platform ${actual.platform} differs from required ${required.platform}`);
 if(actual.architecture!==required.architecture)problems.push(`architecture ${actual.architecture} differs from required ${required.architecture}`);
 if(actual.distribution!==required.distribution||actual.distributionVersion!==required.distributionVersion)problems.push(`distribution ${actual.distribution} ${actual.distributionVersion} differs from required ${required.distribution} ${required.distributionVersion}`);
 if((BigInt("0x"+actual.capEff)&(1n<<21n))===0n)problems.push("CAP_SYS_ADMIN is absent from the invoking process");
 if(actual.availableBytes<required.minimumAvailableBytes)problems.push(`available bytes ${actual.availableBytes} are below required ${required.minimumAvailableBytes}`);
 if(!actual.loopControl)problems.push("writable /dev/loop-control is unavailable");
 if(actual.freeLoops<required.minimumFreeLoops)problems.push(`writable unused loop devices ${actual.freeLoops} are below required ${required.minimumFreeLoops}`);
 for(const tool of required.tools)if(!actual.tools.includes(tool))problems.push(`missing executable: ${tool}`);return problems;
}
/** 🧭️ Checks actual Linux capabilities and finite capacity before any managed allocation is attempted. */
export async function runBuildPreflightV1(root:string,args:readonly string[],invocation:ScriptInvocation):Promise<void> {
 requireIssuedScriptInvocationV1(invocation);checkScriptInvocation(invocation);const owner=readBuildPreflightScopeV1(args),required=readBuildPreflightRequirementsV1(root,owner);
 await advanceScriptInvocation(invocation,`preflight:${owner}`,"running");
 const readable=async(path:string)=>{try{return await readFile(path,"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return "";throw error;}};
 const writable=async(path:string,kind:"block"|"character")=>{try{const entry=await stat(path);await access(path,constants.R_OK|constants.W_OK);return kind==="block"?entry.isBlockDevice():entry.isCharacterDevice();}catch{return false;}};
 const tools:string[]=[];for(const tool of required.tools){for(const directory of(process.env.PATH??"").split(delimiter)){if(!directory)continue;try{const path=join(directory,tool);if(!(await stat(path)).isFile())continue;await access(path,constants.X_OK);tools.push(tool);break;}catch{}}checkScriptInvocation(invocation);}
 const disk=await statfs(root,{bigint:true}),availableBytes=Number(disk.bavail*disk.bsize);let distribution="",distributionVersion="",capEff="0",freeLoops=0,loopControl=false;
 if(process.platform==="linux"){
  const release=await readable("/etc/os-release"),field=(name:string)=>release.match(new RegExp(`^${name}=(?:"([^"]*)"|([^\\r\\n]*))$`,"mu"))?.slice(1).find(value=>value!==undefined)??"";
  distribution=field("ID");distributionVersion=field("VERSION_ID");capEff=(await readable("/proc/self/status")).match(/^CapEff:\s*([0-9a-f]+)$/mu)?.[1]??"0";
  loopControl=await writable("/dev/loop-control","character");const devices=(await readdir("/sys/class/block")).filter(name=>/^loop[0-9]{1,3}$/u.test(name));
  for(const device of devices){checkScriptInvocation(invocation);if(await writable(join("/dev",device),"block")&&!(await readable(join("/sys/class/block",device,"loop/backing_file"))).trim())freeLoops++;}
 }
 checkScriptInvocation(invocation);const facts={platform:process.platform,architecture:process.arch,distribution,distributionVersion,capEff,availableBytes,loopControl,freeLoops,tools},problems=buildPreflightProblemsV1(required,facts);
 console.log(`[DEBUG] Repository preflight ${owner} ${JSON.stringify({requirements:required,facts,problems})}`);
 if(problems.length)throw Error(`Repository preflight refused before store allocation:\n${problems.join("\n")}`);
 await advanceScriptInvocation(invocation,`preflight:${owner}`,"complete");
}
