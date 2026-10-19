import type { CargoWorkspaceProviderV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import { cargoRepositoryPackages, cargoWorkspaceForManifest, prepareCargoWorkspaceInvocation } from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { prepareCargoCapabilityLinksV1 } from "../../../🗂️workspaces/🦀️cargo/🧩️capabilities/🟦️.ts";
import { runCargoCapabilityContributionChecks } from "../../../🗂️workspaces/🦀️cargo/🧩️capabilities/🧪️tests/🟦️.ts";
import { completeCargoPreparationObservationV1, observeCargoPreparationInputV1, observeCargoPreparationOutputV1, observeCargoPreparationSourceV1 } from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts";
import { repositoryCargoPreparationStorageV1 } from "../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";

/** 🔌️ Implements the framework Cargo workspace port with the repository's workspace discovery, preparation storage and custody observations. */
export const provider: CargoWorkspaceProviderV1 = {
  observeSource: observeCargoPreparationSourceV1,
  observeInput: observeCargoPreparationInputV1,
  observeOutput: observeCargoPreparationOutputV1,
  completeObservation: completeCargoPreparationObservationV1,
  prepareCapabilityLinks: prepareCargoCapabilityLinksV1,
  runCapabilityContributionChecks: runCargoCapabilityContributionChecks,
  repositoryPackages: cargoRepositoryPackages,
  workspaceForManifest: cargoWorkspaceForManifest,
  prepareInvocation: (root, args, cwd, environment, control) => prepareCargoWorkspaceInvocation(repositoryCargoPreparationStorageV1(root), root, args, cwd, environment, control),
};
