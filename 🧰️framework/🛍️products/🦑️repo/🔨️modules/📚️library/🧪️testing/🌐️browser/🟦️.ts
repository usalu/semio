import schema from "./🧬️schema/🔣️.json" with {type: "json"};
const policy=schema.definitions.Policy.const;

/** 🎭️ The browser tool selected by repository acceptance runners. */
export const PLAYWRIGHT_MODULE_SPECIFIER=policy.moduleSpecifier;

/** 🌐️ Portable software WebGPU browser options shared by repository browser acceptance runners. */
export function browserLaunchOptionsV1():Readonly<{headless:boolean;args:string[]}>{return {headless:policy.headless,args:[...policy.arguments]};}

/** 🌐️ Launches the repository-selected Playwright through the new headless mode (`channel: "chromium"`), the same mode as the ui launcher (`🖱️ui/🧪️tests/🌐️chromium-launch`): the legacy headless shell never delivers wheel or key input and cannot start WebGPU or downloads on macOS. `options` pass through unchanged except the channel, which is fixed here. */
export async function launchBrowserV1(options:import("playwright").LaunchOptions={}):Promise<import("playwright").Browser>{
  const {chromium}:typeof import("playwright")=await import(PLAYWRIGHT_MODULE_SPECIFIER);
  return chromium.launch({headless:true,...options,channel:"chromium"});
}

/** 🩺️ Asks the installed browser registry for its platform-specific executables without treating a directory as readiness. */
export function browserExecutableProbeV1(module:string):string{return `(async()=>{const {registry:{registry}}=require(${JSON.stringify(module)});const fs=require("node:fs");for(const name of ${JSON.stringify(policy.executables)}){const executable=registry.findExecutable(name);if(!executable)throw Error("Missing browser executable: "+name);fs.accessSync(executable.executablePathOrDie("javascript"),fs.constants.X_OK);await executable._validateHostRequirements("javascript");}})().catch(error=>{console.error(error);process.exitCode=1;});`;}
