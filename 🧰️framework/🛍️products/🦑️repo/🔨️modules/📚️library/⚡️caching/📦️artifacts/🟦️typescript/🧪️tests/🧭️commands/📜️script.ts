import {readFileSync} from "node:fs";
import {BundleScript} from "./../../../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import {runArtifactTypeScriptPackageMain} from "../../📜️script.ts";
const fixture=JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json",import.meta.url),"utf8")),index=Number(process.env.SEMIO_COMMAND_VECTOR);
if(!Number.isSafeInteger(index)||index<0||index>=fixture.cases.length)throw Error("A command corpus index is required");
/** 🧪️ Observes the actual router's discrete arguments from an independently authored corpus. */
class ObservedCommand extends BundleScript {
  run(args:string[]):void{console.log(JSON.stringify({selected:process.argv[2],args}));}
}
await runArtifactTypeScriptPackageMain(import.meta.dir,"command-corpus",{commands:Object.fromEntries(fixture.cases[index].commands.map((name:string)=>[name,ObservedCommand]))});
