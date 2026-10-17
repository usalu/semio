import {readScriptProcessJsonV1} from "@semio-tech/process/routing/invocation/process/json";
import { isLocalSessionProfileIdV1 } from "@semio-tech/framework-os/modules/directory/local-session/broker/schema";

/** 🔌️ Selects a separately supervised first-party service; no command or path is carried. */
export type DevLocalHubProviderV1=Readonly<{version:1;service:"local-hub";defaultProfileId:string}>;
/** 📥️ Captures only the closed service selection DATA. */
export function parseDevLocalHubProviderV1(value:unknown):DevLocalHubProviderV1{const row=readScriptProcessJsonV1(value);if(!row||typeof row!=="object"||Array.isArray(row)||Object.keys(row).sort().join(",")!=="defaultProfileId,service,version"||row.version!==1||row.service!=="local-hub"||!isLocalSessionProfileIdV1(row.defaultProfileId))throw Error("dev-local-hub.invalid-service");return Object.freeze({version:1,service:"local-hub",defaultProfileId:row.defaultProfileId});}
