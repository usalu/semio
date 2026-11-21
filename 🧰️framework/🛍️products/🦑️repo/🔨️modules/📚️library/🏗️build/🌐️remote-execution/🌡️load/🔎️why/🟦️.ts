/** 🔎️ Why the build worker of a machine is not working: a closed code, the same sentence in English and German. */
export type RemoteExecutionWorkerWhyCodeV1="starting"|"no-scheduler"|"scheduler-unreachable"|"identity-unregistered"|"no-toolchain"|"provisioning"|"sandbox-unavailable"|"action-unsupported"|"disk-full"|"policy-off"|"not-registered"|"no-headroom"|"failed";

/** 🗣️ A reason a worker is absent, as a machine reports it to the scheduler (`WorkerWhy` of the schema). */
export type RemoteExecutionWorkerWhyV1=Readonly<{code:RemoteExecutionWorkerWhyCodeV1;en:string;de:string}>;

const TEXTS:Readonly<Record<RemoteExecutionWorkerWhyCodeV1,Readonly<{en:string;de:string}>>>=Object.freeze({
 "starting":{en:"the worker is starting",de:"der Worker startet"},
 "no-scheduler":{en:"the fleet facts name no scheduler yet",de:"die Flottenangaben nennen noch keinen Scheduler"},
 "scheduler-unreachable":{en:"the scheduler is not reachable ({detail})",de:"der Scheduler ist nicht erreichbar ({detail})"},
 "identity-unregistered":{en:"the signed worker identity of this machine is not accepted ({detail}): its worker key is not registered or pinned, or its clock is off",de:"die signierte Worker-Identität dieser Maschine wird nicht angenommen ({detail}): ihr Worker-Schlüssel ist nicht registriert oder festgeschrieben, oder ihre Uhr geht falsch"},
 "no-toolchain":{en:"the pinned Rust toolchain is not installed and could not be provisioned ({detail})",de:"die festgeschriebene Rust-Toolchain ist nicht installiert und konnte nicht bereitgestellt werden ({detail})"},
 "provisioning":{en:"the pinned Rust toolchain is being provisioned ({detail})",de:"die festgeschriebene Rust-Toolchain wird bereitgestellt ({detail})"},
 "sandbox-unavailable":{en:"no confining sandbox on this host while the policy requires one ({detail})",de:"keine einschränkende Sandbox auf diesem Rechner, obwohl die Richtlinie eine verlangt ({detail})"},
 "action-unsupported":{en:"Rust offload requires a POSIX action runner on this version of the worker",de:"Rust-Offload erfordert in dieser Worker-Version eine POSIX-Ausführungsumgebung"},
 "disk-full":{en:"too little free disk space for the worker: {free} GiB free on {path}, {slots} of {concurrency} slots usable, one action needs {oneAction} GiB",de:"zu wenig freier Speicherplatz für den Worker: {free} GiB frei auf {path}, {slots} von {concurrency} Plätzen nutzbar, eine Aktion braucht {oneAction} GiB"},
 "policy-off":{en:"the worker policy runs no actions (worker.actions is none)",de:"die Worker-Richtlinie führt keine Aktionen aus (worker.actions ist none)"},
 "not-registered":{en:"the machine reports its load but its worker is not registered with this scheduler (not yet, or since the scheduler restarted)",de:"die Maschine meldet ihre Last, aber ihr Worker ist bei diesem Scheduler nicht angemeldet (noch nicht, oder seit dem Neustart des Schedulers)"},
 "no-headroom":{en:"the machine has no headroom for one more action ({detail})",de:"die Maschine hat keinen Spielraum für eine weitere Aktion ({detail})"},
 "failed":{en:"the worker stopped with an error ({detail})",de:"der Worker ist mit einem Fehler beendet worden ({detail})"},
});

/** 🧾️ Builds the reason for a code; `detail` (bounded to 160 characters, one line) fills the `{detail}` placeholder of the sentences that have one and `fields` the named placeholders of the others (a missing one reads `?`). */
export function remoteExecutionWorkerWhyV1(code:RemoteExecutionWorkerWhyCodeV1,detail:string="",fields:Readonly<Record<string,string|number>>={}):RemoteExecutionWorkerWhyV1{
 const text=TEXTS[code],clean=detail.replace(/\s+/gu," ").trim().slice(0,160),fill=(sentence:string):string=>sentence.replace(" ({detail})",clean===""?"":` (${clean})`).replace("{detail}",clean).replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??"?").replace(/\s+/gu," ").slice(0,80));
 return Object.freeze({code,en:fill(text.en),de:fill(text.de)});
}

/** 🪟️ Native Windows cannot execute the admitted POSIX Rust action. */
export function remoteExecutionActionPlatformWhyV1(actions:string,platform:string=process.platform):RemoteExecutionWorkerWhyV1|undefined{
 return actions==="rustc"&&platform==="win32"?remoteExecutionWorkerWhyV1("action-unsupported"):undefined;
}

/** 🔬️ Classifies what a worker or reporter said when it could not reach the scheduler: a refusal of the signature (HTTP 401 or 403), a required confinement the host lacks, a connection that failed, or anything else. */
export function remoteExecutionWorkerWhyOfMessageV1(message:string):RemoteExecutionWorkerWhyV1{
 if(/answered (401|403)\b/u.test(message))return remoteExecutionWorkerWhyV1("identity-unregistered",message);
 if(/^Confinement required|Sandbox strategy not available/u.test(message))return remoteExecutionWorkerWhyV1("sandbox-unavailable",message);
 if(/Unable to connect|socket|timed out|timeout|ECONN|ENOTFOUND|EHOSTUNREACH|fetch failed|reconnect/iu.test(message))return remoteExecutionWorkerWhyV1("scheduler-unreachable",message);
 return remoteExecutionWorkerWhyV1("failed",message);
}
