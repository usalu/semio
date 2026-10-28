/** 🧪 Registers every UiDocumentStore typed-wire topic suite into the in-source vitest run of the store module. */
import { type TypedWireVitest, type TypedWireDependencies, type TypedWireSource } from "../🪛️typedwire-helpers/🟦️.tsx";
import { registerScalarSourceOraclesTests } from "../🔢️scalar-source-oracles/🟦️.tsx";
import { registerPagedPayloadTests } from "../📄️paged-payload/🟦️.tsx";
import { registerResidentPoolAndMetadataTests } from "../🧠️resident-pool-and-metadata/🟦️.tsx";
import { registerInstanceMaintenanceAndCloseTests } from "../🏢️instance-maintenance-and-close/🟦️.tsx";
import { registerNodeFieldsAndValidationTests } from "../🧩️node-fields-and-validation/🟦️.tsx";
import { registerOwnedSceneTests } from "../🎬️owned-scene-binding/🟦️.tsx";
import { registerNativeChildAndWireOperationTests } from "../📥️native-child-and-wire-operation/🟦️.tsx";
import { registerOwnedIntakeAndSurfaceTests } from "../🧾️owned-intake-and-surface/🟦️.tsx";
import { registerOwnedStagedControlTests } from "../🪢️owned-staged-controls/🟦️.tsx";
import {registerOwnedFacadeLeaseTests} from '../🪟️owned-facade/🟦️.tsx';
import { registerOwnedWireAndBytePagesTests } from "../🏅️owned-wire-and-byte-pages/🟦️.tsx";
import { registerRetainedAndStoreTests } from "../🔏️retained-and-store/🟦️.tsx";

export async function registerTypedWireTests(vitest: TypedWireVitest, dependencies: TypedWireDependencies, source: TypedWireSource): Promise<void> {
  await registerScalarSourceOraclesTests(vitest, dependencies, source);
  await registerPagedPayloadTests(vitest, dependencies);
  await registerResidentPoolAndMetadataTests(vitest, dependencies);
  await registerInstanceMaintenanceAndCloseTests(vitest, dependencies);
  await registerNodeFieldsAndValidationTests(vitest, dependencies);
  await registerOwnedSceneTests(vitest, dependencies);
  await registerNativeChildAndWireOperationTests(vitest, dependencies);
  await registerOwnedIntakeAndSurfaceTests(vitest, dependencies);
  await registerOwnedStagedControlTests(vitest, dependencies);
  await registerOwnedFacadeLeaseTests(vitest,dependencies);
  await registerOwnedWireAndBytePagesTests(vitest, dependencies);
  await registerRetainedAndStoreTests(vitest, dependencies, source);
}
