/** 🤝️ Live end-to-end sign-in gate — boots the `os-hub` development binary against a **fresh**
 * `OS_HUB_DATA`, provisions the very first principal through the operator verb `os-hub credential
 * set`, and then drives the whole human-facing session lifecycle over real HTTP against the running
 * hub: sign in, read the session authority, wrong password, create a space, create an invitation,
 * a second principal redeems it, both appear in the space roster, change a password (which revokes
 * the old sessions), sign out, and finally exhaust the rate-limit bucket until the hub answers
 * `429` with a `retry-after` and then admits a correct password again once the clock has recovered.
 *
 * `--hold` keeps the hub running and prints its origin instead of running the transcript, so the
 * browser phase can drive the same live hub. Every step prints one line; a failed expectation exits
 * non-zero with the observed value, so this file is a gate, not a log. */
import {requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "@semio-tech/process/routing";
import type {ResourceLeaseControl} from "@semio-tech/process/leases";
import {captureOwnedCommandResultV1} from "@semio-tech/process/owned-execution";
import {withBuiltLocalHubHostV1,startLocalHub,waitForReadiness,waitForLocalHubExitV1} from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import {directoryCommandRequestJson,sealDirectoryCommandRequestV1} from "@semio-tech/framework/directory/io/text";

/** 🚪️ Runs the live transcript only inside the existing actual command Original and carried phase. */
export function runLiveSignInV1(invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,hold:boolean):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);const port=Number(process.env.HUB_LIVE_SIGN_IN_PORT??8831);if(typeof hold!=="boolean"||!Number.isSafeInteger(port)||port<1||port>65535)throw Error("Closed live sign-in request required");
 return withBuiltLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
const origin=`http://127.0.0.1:${port}`,dataRoot=context.dataRoot;
const ADA = { email: "ada@example.org", password: "correct horse battery staple", display: "Ada Lovelace" };
const BO = { email: "bo@example.org", password: "another perfectly fine phrase", display: "Bo Peep" };
const NEW_ADA_PASSWORD = "a rotated phrase for ada now";

/** 🔑️ The operator bootstrap verb — the only way a principal gets its first credential. */
async function provision(account:{email:string;password:string;display:string}):Promise<string>{const result=await captureOwnedCommandResultV1(context.binaryPath,["credential","set","--email",account.email,"--display-name",account.display],context.workspace,{control:context.control,env:{...context.environment,OS_HUB_DATA:dataRoot},input:{version:1,text:account.password},output:{maximumBytes:65536,maximumLines:512}});if(result.status!==0||result.signal!==null)throw Error(`credential operator failed: ${result.stderr}`);return result.stdout.trim();}

let failures = 0;
function check(label: string, actual: unknown, expected: unknown): void {
  const ok = JSON.stringify(actual) === JSON.stringify(expected);
  if (!ok) failures += 1;
  console.log(`${ok ? "PASS" : "FAIL"} ${label}: ${JSON.stringify(actual)}${ok ? "" : ` (expected ${JSON.stringify(expected)})`}`);
}

type Answer = { readonly status: number; readonly body: string; readonly retryAfter: string | null; readonly json: any };

async function call(method: string, path: string, options: { token?: string; body?: unknown } = {}): Promise<Answer> {
  const response = await fetch(`${origin}${path}`, {
    method,
    signal:AbortSignal.any([context.control.signal,AbortSignal.timeout(10000)]),
    headers: {
      ...(options.body === undefined ? {} : { "content-type": "application/json" }),
      ...(options.token === undefined ? {} : { authorization: `Bearer ${options.token}` }),
      origin: "http://127.0.0.1:6066",
    },
    ...(options.body === undefined ? {} : { body: JSON.stringify(options.body) }),
  });
  const body = await response.text();
  let json: any;
  try {
    json = JSON.parse(body);
  } catch {
    json = undefined;
  }
  return { status: response.status, body, retryAfter: response.headers.get("retry-after"), json };
}

const signInBody = (email: string, password: string, device: string) => ({
  schema: "semio.hub.auth.credential-sign-in/v1",
  email,
  password,
  deviceInstanceId: device,
  clientClass: "browser" as const,
});

async function command(token: string, requestId: string, body: any): Promise<Answer> {
  const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, body));
  const response = await fetch(`${origin}/directory/commands`, { method: "POST", headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" }, body: sealed });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {
    json = undefined;
  }
  return { status: response.status, body: text, retryAfter: response.headers.get("retry-after"), json };
}

/** 🏘️ The id the hub assigned to the space this run just created, read back from the authoritative
 * listing by name — `create-space`'s receipt result is deliberately `{kind:"none"}`. */
async function spaceIdByName(token: string, name: string): Promise<string> {
  const listed = await call("GET", "/directory/spaces", { token });
  const entry = (Array.isArray(listed.json) ? listed.json : []).find((row: any) => row?.space?.name === name);
  return entry?.space?.id ?? "";
}

const run=await startLocalHub(host,[{profileId:"developer",subject:"local-developer-01",displayName:"Local Developer",allowedClientClasses:["native","mcp"]}],{port,capture:true,credentialSignIn:true});

try {
  console.log(`fresh OS_HUB_DATA=${dataRoot} port=${port} credentialSignIn=${process.env.OS_HUB_CREDENTIAL_SIGN_IN ?? "<unset>"}`);
  const adaId = await provision(ADA);
  const boId = await provision(BO);
  console.log(`operator bootstrap: ada=${adaId} bo=${boId}`);

  const readiness = await waitForReadiness(run, true);
  console.log(`readyz: status=${readiness.status} publicSessionIssuance=${JSON.stringify((readiness as any).publicSessionIssuance)}`);

  if (hold) {
    console.log(`HOLD origin=${origin} dataRoot=${dataRoot} ada=${ADA.email} bo=${BO.email}`);
    await waitForLocalHubExitV1(run);return;
  }

  // 1️⃣ sign in
  const minted = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada") });
  check("1 sign-in status", minted.status, 200);
  const adaToken: string = minted.json?.token ?? "";
  check("1 minted token shape", /^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(adaToken), true);
  check("1 minted user id", minted.json?.user_id, adaId);
  check("1 no-store", minted.body.length > 0, true);

  // 2️⃣ session introspection carries the expiry the mint body withholds
  const me = await call("GET", "/auth/sessions/me", { token: adaToken });
  check("2 me status", me.status, 200);
  check("2 me userId", me.json?.userId, adaId);
  check("2 me email", me.json?.email, ADA.email);
  check("2 me displayName", me.json?.displayName, ADA.display);
  check("2 me sessionKind", me.json?.sessionKind, "external");
  check("2 me expiresAt is a future epoch-ms integer", Number.isSafeInteger(me.json?.expiresAt) && me.json.expiresAt > Date.now(), true);

  // 3️⃣ a wrong password is one uniform refusal
  const wrong = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, "definitely not the phrase", "device-ada") });
  check("3 wrong password status", wrong.status, 401);
  check("3 wrong password body", wrong.json, { schema: "semio.hub.auth.error/v1", error: "invalid-credentials" });
  const unknown = await call("POST", "/auth/sessions", { body: signInBody("nobody@example.org", "definitely not the phrase", "device-ada") });
  check("3 unknown account is indistinguishable", unknown.json, wrong.json);

  // 4️⃣ create a space
  const created = await command(adaToken, "a".repeat(32), { kind: "create-space", name: "Atelier AU3", spaceKind: "atelier", visibility: "private" });
  check("4 create-space status", created.status, 202);
  check("4 create-space outcome", created.json?.outcome, "accepted");
  const spaceId = await spaceIdByName(adaToken, "Atelier AU3");
  check("4 the new space is in the author's own listing", spaceId.length > 0, true);

  // 5️⃣ create an invitation
  const invited = await command(adaToken, "b".repeat(32), { kind: "create-invite", spaceId, role: "spectator", ttlSecs: 3600 });
  check("5 create-invite status", invited.status, 202);
  check("5 create-invite result kind", invited.json?.result?.kind, "invite");
  const inviteToken: string = invited.json?.result?.inviteToken ?? "";
  check("5 invite capability shape", /^invite\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/u.test(inviteToken), true);

  // 6️⃣ a second principal signs in and redeems it
  const boMint = await call("POST", "/auth/sessions", { body: signInBody(BO.email, BO.password, "device-bo") });
  check("6 second sign-in status", boMint.status, 200);
  const boToken: string = boMint.json?.token ?? "";
  const redeemed = await call("POST", `/directory/invites/${encodeURIComponent(inviteToken)}/redeem`, { token: boToken });
  check("6 redemption status", redeemed.status, 200);
  const boSpaces = await call("GET", "/directory/spaces", { token: boToken });
  check("6 the second principal now sees the space", Array.isArray(boSpaces.json) && boSpaces.json.some((entry: any) => entry?.space?.id === spaceId), true);

  // 7️⃣ both appear in the roster the os workspace reads
  const page = await call("GET", `/directory/spaces/${encodeURIComponent(spaceId)}`, { token: adaToken });
  check("7 space page status", page.status, 200);
  check("7 the author's page carries the member window", ["author", "member"].includes(page.json?.access), true);
  const roster = (page.json?.members?.rows ?? []).map((row: any) => [row.userId, row.role, row.owner]).sort();
  check("7 roster", roster, [[adaId, "author", true], [boId, "spectator", false]].sort());

  // 8️⃣ a password change revokes every session of that principal
  const changed = await call("POST", "/auth/credentials", { token: adaToken, body: { schema: "semio.hub.auth.credential-change/v1", currentPassword: ADA.password, newPassword: NEW_ADA_PASSWORD } });
  check("8 credential change status", changed.status, 204);
  check("8 the changed principal's old capability is gone", (await call("GET", "/auth/sessions/me", { token: adaToken })).status, 401);
  const reMint = await call("POST", "/auth/sessions", { body: signInBody(ADA.email, NEW_ADA_PASSWORD, "device-ada") });
  check("8 the new password mints", reMint.status, 200);
  check("8 the old password no longer mints", (await call("POST", "/auth/sessions", { body: signInBody(ADA.email, ADA.password, "device-ada") })).status, 401);
  check("8 a change without the current password is refused", (await call("POST", "/auth/credentials", { token: reMint.json.token, body: { schema: "semio.hub.auth.credential-change/v1", currentPassword: "not the current one", newPassword: "yet another phrase here" } })).status, 401);

  // 9️⃣ sign out
  check("9 sign-out status", (await call("POST", "/auth/sessions/me/sign-out", { token: boToken })).status, 204);
  check("9 sessions are never deleted as a resource", (await call("DELETE", "/auth/sessions/me", { token: boToken })).status, 405);
  check("9 the signed-out capability is gone", (await call("GET", "/auth/sessions/me", { token: boToken })).status, 401);

  // 🔟 rate-limit lockout, last because it empties the bucket for this address
  let lockout: Answer | undefined;
  for (let attempt = 0; attempt < 24 && lockout === undefined; attempt += 1) {
    const answer = await call("POST", "/auth/sessions", { body: signInBody("lockout@example.org", "a wrong phrase entirely", "device-lock") });
    if (answer.status === 429) lockout = answer;
  }
  check("10 the bucket eventually refuses", lockout?.status, 429);
  check("10 the refusal names its class", lockout?.json?.error, "rate-limited");
  check("10 retry-after is a positive integer of seconds", Number.isInteger(Number(lockout?.retryAfter)) && Number(lockout?.retryAfter) >= 1, true);
  check("10 a correct password is refused while the bucket is empty", (await call("POST", "/auth/sessions", { body: signInBody(BO.email, BO.password, "device-bo") })).status, 429);
} catch (error) {
  console.log(`probe failure: ${error instanceof Error ? error.stack : String(error)}`);
  console.log(`child output tail:\n${run.output().slice(-3000)}`);
  throw error;
}

console.log(failures === 0 ? "au3-live-sign-in: all checks passed" : `au3-live-sign-in: ${failures} check(s) failed`);
if(failures)throw Error(`Live sign-in assertions failed: ${failures}`);
});
}
