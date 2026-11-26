import type {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import type {ConsentProbeV1} from "../../🔐️consent/🟨️.mjs";
import type {LaunchdRunV1} from "../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";
export function runHostCommandV1(options:{args:readonly string[];languages:readonly string[];implementation:string;runForeground():Promise<number>;command:readonly string[];environment?:Readonly<Record<string,string|undefined>>;platform?:string;cwd?:string;stdout?:{write(text:string):unknown};stderr?:{write(text:string):unknown};signal?:AbortSignal;policy?:Readonly<Partial<typeof HOST_POLICY_V1>>;probe?:(root:string,options:{platform?:string})=>Promise<ConsentProbeV1>;launchctl?:LaunchdRunV1}):Promise<number>;
