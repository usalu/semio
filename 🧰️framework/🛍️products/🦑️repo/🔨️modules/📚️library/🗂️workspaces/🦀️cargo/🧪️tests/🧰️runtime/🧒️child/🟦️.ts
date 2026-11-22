import {captureCargoFixtureInProcessV1,prepareCargoFixtureInProcessV1} from "../🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {adoptInstalledRustToolchainForV1} from "../../../../../🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts";
adoptInstalledRustToolchainForV1(process.env.SEMIO_CARGO_FIXTURE_REPOSITORY!,findWorkspaceRoot(import.meta.dir));
const[mode,payload]=process.argv.slice(2),input=JSON.parse(payload??"null");
if(mode==="prepare"){await prepareCargoFixtureInProcessV1(input.storage,input.root,input.args,input.cwd,process.env);process.exit(0);}
if(mode==="capture"){const output=await captureCargoFixtureInProcessV1(input.root,input.args,process.env);process.stdout.write("\n[fixture-output] "+JSON.stringify(output)+"\n");process.exit(0);}
throw Error("Cargo fixture child mode required");
