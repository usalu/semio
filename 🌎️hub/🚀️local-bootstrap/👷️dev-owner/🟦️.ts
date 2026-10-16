import {scriptCompilerAdmissionControlV1} from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {join} from "node:path";
import {BundleScript} from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withRepositoryDevelopmentStateV1} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/💾️development/🟦️.ts";
import {DEV_LOCAL_HUB_DEFAULT_URL,parseHubPort} from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import {withMaterializedTrustedCatalogBundleV1,validateAndPublishTrustedStdioGisCandidate} from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import {trustedBootstrapSelectPackages} from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
import {LOCAL_ADMIN_CAPABILITY_FILE,withLocalSessionBrokerV1} from "../🔐️credential-issuance/🟦️.ts";
import {withPublishedDevelopmentLocalHubHostV1,stopLocalHubV1,waitForLocalHubExitV1,LOCAL_HUB_ADMINISTRATOR_PROFILE,LOCAL_HUB_ADMINISTRATOR_SUBJECT,LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES,LOCAL_HUB_DEVELOPMENT_PROFILES,startLocalHub,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS,waitForReadiness} from "../🏃️execution/🟦️.ts";

/** 🗄️ Retains genuine persistent development data, executable and catalog through actual child settlement. */
export class DevLocalHubScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const[hubArg,dataArg,...extra]=segments;
  if(dataArg!==undefined||extra.length)throw Error("Development data belongs to the fixed persistent-state owner; free directories are unsupported");
  const hubUrl=(hubArg??DEV_LOCAL_HUB_DEFAULT_URL).replace(/\/+$/u,""),port=parseHubPort(hubUrl),phase=scriptCompilerAdmissionControlV1(this.invocation,"hub:development",0);
  return withRepositoryDevelopmentStateV1(this.invocation,phase,(state,statePhase)=>withPublishedDevelopmentLocalHubHostV1(this.invocation,statePhase,state,(host,context)=>withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES),async(receipt,actualPhase)=>{
   await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,receipt);
   const profiles=[...LOCAL_HUB_DEVELOPMENT_PROFILES,LOCAL_HUB_ADMINISTRATOR_PROFILE],run=await startLocalHub(host,profiles,{port,credentialSignIn:true,adminToken:process.env.OS_HUB_ADMIN_TOKEN??"dev-local-hub-admin",adminSubjects:[LOCAL_HUB_ADMINISTRATOR_SUBJECT],capture:false});
   let stopWork:ReturnType<typeof stopLocalHubV1>|undefined;
   const stop=():void=>{if(!stopWork){stopWork=stopLocalHubV1(run,"graceful");void stopWork.catch(()=>{});}};
   process.once("SIGINT",stop);process.once("SIGTERM",stop);
   const failures:unknown[]=[];
   try{await waitForReadiness(run,false,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);await withLocalSessionBrokerV1(run,profiles,2,LOCAL_HUB_ADMINISTRATOR_PROFILE.profileId,async()=>{console.log(`[dev-local-hub] ready at ${hubUrl}; admin capability in ${join(context.dataRoot,LOCAL_ADMIN_CAPABILITY_FILE)}`);await waitForLocalHubExitV1(run);});}catch(error){failures.push(error);}
   process.off("SIGINT",stop);process.off("SIGTERM",stop);
   if(stopWork)try{await stopWork;}catch(error){if(!failures.includes(error))failures.push(error);}
   if(failures.length===1)throw failures[0];if(failures.length)throw new AggregateError(failures,"Development Hub and broker settlement failed");
  })));
 }
}
