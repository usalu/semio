import{withRepositoryToolingOperationV1}from"../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {requireNativeStorageReservationV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type {RepositoryToolingStoreV1} from "../../../../../../📦️packages/🟦️typescript/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { runtimeParseTomlV1 } from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/📄️parse/🟦️.ts";
/** 🔤️ Parses TOML through the owned runtime TOML port behind a first-party bounded data port. */
export function rustupTomlValueV1(source:string,workspace:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<unknown>{return withRepositoryToolingOperationV1(workspace,store,control,actual=>rustupTomlValueOwnedV1(source,store,actual));}
async function rustupTomlValueOwnedV1(source:string,store:RepositoryToolingStoreV1,control:ResourceLeaseControl):Promise<unknown>{requireIssuedResourceControlInvocationV1(control,store.invocation);requireIssuedResourceControlRefinementV1(store.control,control);requireNativeStorageReservationV1(store.storage,store.control);if(Buffer.byteLength(source)>schema.definitions.Protocol.const.maximumDistributionManifestBytes)throw Error("Bounded Rustup TOML bytes required");await store.storage.recheck();return runtimeParseTomlV1(source);}
