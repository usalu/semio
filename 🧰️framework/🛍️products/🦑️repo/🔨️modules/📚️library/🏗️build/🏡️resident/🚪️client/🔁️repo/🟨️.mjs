/** 🔁 The CLI-side decision to hand a repo command to the resident service: which commands qualify, which flags disqualify, how the answer is reported. */
import {forwardCommandV1} from "../🟨️.mjs";
import {POLICY_V1,SUPPORTED_LANGUAGES_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

const TASK_COMMANDS=Object.freeze({run:"run",test:"test","run-many":"run-many"});

/** 🧭 The service mode from `--service[=]` or `SEMIO_BUILD_SERVICE`; refuses unknown values. */
export function serviceModeV1(args,environment){
 let mode=environment.SEMIO_BUILD_SERVICE;
 for(let at=0;at<args.length;at++){
  if(args[at]==="--")break;
  if(args[at]==="--service")mode=args[++at];
  else if(args[at].startsWith("--service="))mode=args[at].slice(10);
 }
 if(mode===undefined||mode==="")return"auto";
 if(!["auto","off","require"].includes(mode))throw new Error("--service expects auto|off|require, got "+mode);
 return mode;
}

/** ✅ True when the command is a plain task command: plain targets and only flags that cannot change what a task does. */
export function serviceEligibleV1(args){
 let at=0;
 while(at<args.length&&args[at].startsWith("--service="))at++;
 const command=args[at];
 if(command===undefined||!(command in TASK_COMMANDS))return false;
 const ids=[];
 for(const argument of args.slice(at+1)){
  if(argument==="--")return false;
  if(!argument.startsWith("-")){ids.push(argument);continue;}
  if(!POLICY_V1.eligibleFlags.some(flag=>flag.endsWith("=")?argument.startsWith(flag)&&argument.length>flag.length:argument===flag))return false;
 }
 return ids.length>0;
}

/** 🔁 Forwards an eligible command. Returns the exit code, or undefined when the caller must run the command itself. */
export async function forwardRepoCommandV1({root,args,environment,languageOf,stdout=process.stdout,stderr=process.stderr,cwd=process.cwd(),forward=forwardCommandV1}){
 const mode=serviceModeV1(args,environment);
 if(mode==="off"||!serviceEligibleV1(args))return undefined;
 let verbAt=0;
 while(args[verbAt].startsWith("--service="))verbAt++;
 let languages;
 try{const language=languageOf();if(!SUPPORTED_LANGUAGES_V1.includes(language))throw new Error("unresolved language");languages=[language,...SUPPORTED_LANGUAGES_V1.filter(other=>other!==language)];}catch{return mode==="require"?1:undefined;}
 const ids=args.slice(verbAt+1).filter(argument=>!argument.startsWith("-"));
 const forwardedEnvironment={};
 for(const key of POLICY_V1.bindEnvironment)if(environment[key]!==undefined)forwardedEnvironment[key]=environment[key];
 const result=await forward({root,command:args[verbAt],args:ids,cwd,languages,env:forwardedEnvironment,environment,stdout,stderr});
 if(result.handled)return result.exit;
 if(mode==="require"){stderr.write("--service=require: no resident service answered ("+result.reason+")\n");return 1;}
 return undefined;
}
