import { existsSync } from "node:fs";
import { BUILD_HOST_CAPACITY_POLICY_V1, withBuildCapacityV1 } from "../../../../🌡️host-capacity/🟦️.ts";
import { readHostCapacityPolicyV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import { installOutputContractV1, writeResultLineV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts";

const args = process.argv.slice(2), directory = process.env.FIXTURE_CAPACITY_DIRECTORY!, release = process.env.FIXTURE_CAPACITY_RELEASE!;
installOutputContractV1(args);
const wait = BUILD_HOST_CAPACITY_POLICY_V1.wait, policy = readHostCapacityPolicyV1({ ...BUILD_HOST_CAPACITY_POLICY_V1, wait: { ...wait, pollMilliseconds: 10, progressMilliseconds: 20, heartbeatMilliseconds: 20, sampleMaximumAgeMilliseconds: 0 } });
const sample = async () => ({ host: { version: 1 as const, atMs: Date.now(), platform: "darwin" as const, cores: 10, totalMegabytes: 32768, availableMegabytes: existsSync(release) ? 28000 : 100, pressure: "normal" as const, swapUsedMegabytes: 0, swapOutMegabytesPerSecond: 0, load: 1, openFiles: 8000, openFilesMaximum: 245760, attachedImages: 10 }, swappedOutMegabytes: 0 });
await withBuildCapacityV1(args, { ...process.env, SEMIO_LANGUAGE: "en" }, async () => {
  console.log("[noise] console.log while admitted");
  process.stdout.write("[noise] process.stdout.write while admitted\n");
  writeResultLineV1(JSON.stringify(["a", "b"]));
}, { directory, policy, sample });
