import { mountSDevInventoryV1, type SDevMountV1, type SDevOperationV1 } from "../🟦️.ts";
import type { installDocumentServiceWorkerV1 } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts";

export interface SDevWorkerContributionV1 {
  readonly owner: string;
  readonly serviceId: string;
  readonly create: Parameters<typeof installDocumentServiceWorkerV1>[2];
}
export interface SDevWorkerHostV1 { install(contribution: SDevWorkerContributionV1, operation: SDevOperationV1): Promise<SDevMountV1>; }

/** 👷 Installs only the caller's worker contributions and returns their retirement owner. */
export async function installSDevWorkersV1(installed: readonly SDevWorkerContributionV1[], host: SDevWorkerHostV1, operation: SDevOperationV1): Promise<SDevMountV1> {
  if (!Array.isArray(installed)) throw new Error("s-dev.missing-installed-inventory");
  return mountSDevInventoryV1(installed, (contribution, context) => host.install(contribution, context), operation);
}
