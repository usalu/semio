/** 🏠️ Collaboration e2e hub start and credentials: the free port scan, the hub data directory, the hub start, the credential provisioning and the sign in. */
import {captureOwnedCommandResultV1} from "@semio-tech/process/owned-execution";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { isDevPortInUse } from "@semio-tech/repo-lib/dev";
import { awaitHttpOk } from "@semio-tech/framework-os-dev/activation/readiness";
import { withLocalHubHostTaskV1, stopLocalHubV1, waitForReadiness, type LocalHubHostV1, finishLocalHub, startLocalHub, type LocalHubRun } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import type { LocalProfile } from "../../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { issueLocalCredential } from "../../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { COLLAB_E2E_HUB_BOOT_BUDGET_MS, COLLAB_E2E_PORT_MAX, COLLAB_E2E_PORT_MIN, COLLAB_E2E_USER1_DISPLAY, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER1_PASSWORD, COLLAB_E2E_USER2_DISPLAY, COLLAB_E2E_USER2_EMAIL, COLLAB_E2E_USER2_PASSWORD } from "../⚙️constants-relay-and-recorder/🟦️.ts";
import { collabOutDir, collabScopeV1 } from "../🧭️scope-and-dev-servers/🟦️.ts";

//#region 🔖️CollabE2e
/** 🔌️ Resolves one collab-e2e port: an explicit env override, else the first free port in
 * `[COLLAB_E2E_PORT_MIN, COLLAB_E2E_PORT_MAX]` not already claimed by an earlier call this run. */
export function collabScanPort(envVar: string, taken: Set<number>): number {
  const override = process.env[envVar];
  if (override) {
    const port = Number(override);
    taken.add(port);
    return port;
  }
  for (let port = COLLAB_E2E_PORT_MIN; port <= COLLAB_E2E_PORT_MAX; port++) {
    if (taken.has(port) || isDevPortInUse("127.0.0.1", port)) continue;
    taken.add(port);
    return port;
  }
  throw new Error(`collab e2e: no free port in ${COLLAB_E2E_PORT_MIN}-${COLLAB_E2E_PORT_MAX} for ${envVar}`);
}

/** 🗄️ Exposes the actual retained Host data for the scoped scenario and its same-data restart. */
export async function collabHubDataDir():Promise<string>{return collabScopeV1().context.dataRoot;}

export type CollabHubHandle=Readonly<{child:LocalHubRun["child"];run:LocalHubRun;adminCapability:string;kill:()=>Promise<void>}>;
/** 🚀️ Boots only the actual retained Host and joins its closed termination operation. */
export async function collabStartHub(host:LocalHubHostV1,port:number,label:"boot"|"restart"):Promise<CollabHubHandle>{
 const scope=collabScopeV1();if(scope.host!==host)throw Error("Exact collaboration Host required");
 return withLocalHubHostTaskV1(host,scope.context.control,async()=>{
  const profile:LocalProfile={profileId:"collab-boot",subject:"collab-boot",displayName:"Collab Boot",allowedClientClasses:["native","mcp","react-relay","admin-relay"]},logPath=join(collabOutDir(),`hub-${label}.log`),run=await startLocalHub(host,[profile],{port,adminSubjects:["semio.local.bootstrap/v1:collab-boot"],capture:true,credentialSignIn:true});
  await waitForReadiness(run,false,COLLAB_E2E_HUB_BOOT_BUDGET_MS);
  const envelope=await issueLocalCredential(run,"collab-boot","admin-relay"),adminCapability=String(envelope.capability??"");
  if(!adminCapability.startsWith("session.v1."))throw Error("collab e2e: admin-relay envelope missing session capability");
  const overview=await awaitHttpOk(`http://127.0.0.1:${port}/admin/api/overview`,{deadlineMs:30000,intervalMs:500,init:{headers:{authorization:`Bearer ${adminCapability}`},signal:scope.context.control.signal},isDead:()=>run.child.exitCode!==null});
  writeFileSync(logPath,run.output()||"readyz+admin ok\n","utf8");if(overview!=="ready")throw Error(`admin overview not ready — see ${logPath}`);
  return Object.freeze({child:run.child,run,adminCapability,kill:async()=>{await stopLocalHubV1(run,"graceful");await finishLocalHub(run);}});
 });
}

/** 🔑️ Uses the genuine retained executable and phase for finite credential input. */
export async function collabProvisionCredentials():Promise<Readonly<Record<string,string>>>{
 const{host,context}=collabScopeV1();return withLocalHubHostTaskV1(host,context.control,async actual=>{const provisioned:Record<string,string>={};for(const account of[{email:COLLAB_E2E_USER1_EMAIL,password:COLLAB_E2E_USER1_PASSWORD,display:COLLAB_E2E_USER1_DISPLAY},{email:COLLAB_E2E_USER2_EMAIL,password:COLLAB_E2E_USER2_PASSWORD,display:COLLAB_E2E_USER2_DISPLAY}]){const output=await captureOwnedCommandResultV1(actual.binaryPath,["credential","set","--email",account.email,"--display-name",account.display],actual.workspace,{control:actual.control,env:{...actual.environment,OS_HUB_DATA:actual.dataRoot},input:{version:1,text:account.password},output:{maximumBytes:65536,maximumLines:512}});if(output.status!==0||output.signal!==null)throw Error("Credential operator child failed: "+output.stderr);provisioned[account.email]=output.stdout.trim();}return Object.freeze(provisioned);});
}

/** 🔐️ Signs one browser context in as one human, through the shell's own hub workspace — the same
 * badge, form and `POST /auth/sessions` a person uses. Waits for the shell to report a verified
 * session authority (`[data-semio-hub-session="signed-in"]`), because every hub-authenticated step
 * below (create space, share, create artifact, open a document socket) is refused until the shell
 * has one. */
export async function collabSignIn(page: import("playwright").Page, email: string, password: string): Promise<void> {
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}
//#endregion 🔖️CollabE2e
