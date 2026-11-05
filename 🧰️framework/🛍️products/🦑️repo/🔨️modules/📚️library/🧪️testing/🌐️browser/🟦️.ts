import schema from "./🧬️schema/🔣️.json" with {type: "json"};
const policy=schema.definitions.Policy.const;

/** 🎭️ The browser tool selected by repository acceptance runners. */
export const PLAYWRIGHT_MODULE_SPECIFIER=policy.moduleSpecifier;

/** 🌐️ Portable software WebGPU browser options shared by repository browser acceptance runners. */
export function browserLaunchOptionsV1():Readonly<{headless:boolean;args:string[]}>{return {headless:policy.headless,args:[...policy.arguments]};}

/** 🩺️ Asks the installed browser registry for its platform-specific executables without treating a directory as readiness. */
export function browserExecutableProbeV1(module:string):string{return `(async()=>{const {registry:{registry}}=require(${JSON.stringify(module)});const fs=require("node:fs");for(const name of ${JSON.stringify(policy.executables)}){const executable=registry.findExecutable(name);if(!executable)throw Error("Missing browser executable: "+name);fs.accessSync(executable.executablePathOrDie("javascript"),fs.constants.X_OK);await executable._validateHostRequirements("javascript");}})().catch(error=>{console.error(error);process.exitCode=1;});`;}
