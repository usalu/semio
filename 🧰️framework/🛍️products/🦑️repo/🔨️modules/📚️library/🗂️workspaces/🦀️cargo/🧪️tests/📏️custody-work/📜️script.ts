import { cargoWorkspaceForManifest, cargoWorkspaceMembers } from "../../🟦️.ts";
import { readCargoListingSnapshotV1, seedCargoPriorListingsV1, writeCargoListingSnapshotV1, type CargoPhysicalObservationsV1 } from "../../🛠️preparation/👁️witness/🟦️.ts";

const meter = (globalThis as { semioFsMeter?: { lstat: number; readdir: number } }).semioFsMeter;
if (!meter) throw new Error("Preloaded filesystem meter required");
const [root, mode, within] = process.argv.slice(2) as [string, "walk" | "within" | "replay", string | undefined];
const owner = cargoWorkspaceForManifest(root, "Cargo.toml");
const measure = <T>(work: () => T): { value: T; lstat: number; readdir: number } => {
  meter.lstat = 0;
  meter.readdir = 0;
  const value = work();
  return { value, lstat: meter.lstat, readdir: meter.readdir };
};
const manifests = (rows: readonly { manifest: string }[]): string[] => rows.map(row => row.manifest);
if (mode === "walk") {
  const run = measure(() => manifests(cargoWorkspaceMembers(root, owner)));
  process.stdout.write(JSON.stringify({ members: run.value, lstat: run.lstat, readdir: run.readdir }));
} else if (mode === "within") {
  const full = manifests(cargoWorkspaceMembers(root, owner)).filter(manifest => manifest.startsWith(within + "/"));
  const run = measure(() => manifests(cargoWorkspaceMembers(root, owner, undefined, within)).filter(manifest => manifest.startsWith(within + "/")));
  process.stdout.write(JSON.stringify({ members: run.value, full, lstat: run.lstat, readdir: run.readdir }));
} else {
  await Bun.sleep(2300);
  const storage = { version: 1 as const, directory: root + "-snapshot", leaseDirectory: root, program: root };
  const { mkdirSync } = await import("node:fs");
  mkdirSync(storage.directory, { recursive: true });
  const first: CargoPhysicalObservationsV1 = new Map();
  const cold = measure(() => manifests(cargoWorkspaceMembers(root, owner, first)));
  writeCargoListingSnapshotV1(storage, root, storage.directory, first);
  const second: CargoPhysicalObservationsV1 = new Map();
  seedCargoPriorListingsV1(second, readCargoListingSnapshotV1(storage, root));
  const warm = measure(() => manifests(cargoWorkspaceMembers(root, owner, second)));
  process.stdout.write(JSON.stringify({ cold: cold.value, warm: warm.value, coldReaddir: cold.readdir, warmReaddir: warm.readdir, coldLstat: cold.lstat, warmLstat: warm.lstat }));
}
