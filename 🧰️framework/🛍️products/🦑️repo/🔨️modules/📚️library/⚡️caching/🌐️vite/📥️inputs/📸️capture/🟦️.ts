import { closeSync, fstatSync, lstatSync, openSync, type BigIntStats } from "node:fs";
import { artifactReadFlagsV1 } from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import { lstat, mkdir, utimes } from "node:fs/promises";
import { dirname, join } from "node:path";
import { copyCompilationFileV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📁️copy/🟦️.ts";
import sourceSchema from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/📥️sources/🧬️schema/🔣️.json" with { type: "json" };
import { requireNativeStoragePathV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { withRequestedIssuedResourceControlV1, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type { RepositoryCacheStoreV1 } from "../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import { withRepositoryArtifactGroupV1, withRepositoryArtifactGroupTaskV1 } from "../../../📦️artifacts/👓️reading/📚️group/🟦️.ts";
import { inspectArtifactV1, readArtifactManifestV1 } from "../../../📦️artifacts/👓️reading/📥️inspection/🟦️.ts";
import type { RepositoryArtifactReaderOptionsV1, RepositoryArtifactRequestV1 } from "../../../📦️artifacts/👓️reading/📋️contract/🟦️.ts";
import { readViteArtifactInputsV1 } from "../🟦️.ts";

const manifestFile = ".semio-artifact.json";
const identityKeys = ["dev", "ino", "mode", "nlink", "size", "mtimeNs", "ctimeNs"] as const;
const identityMatches = (left: BigIntStats, right: BigIntStats) => identityKeys.every(key => left[key] === right[key]);

export type ViteCapturedInputsV1 = Readonly<{
  rootFor(publishedRoot: string): string;
  sealProjection(): void;
  openFile(path: string): number | undefined;
  recheck(): Promise<void>;
}>;

/** 📸️ Captures exactly the admitted files into the Vite store's private session directory. */
export async function captureViteArtifactInputsV1(options: Readonly<{
  workspace: string;
  invocation: ScriptInvocation;
  store: RepositoryCacheStoreV1;
  control: ResourceLeaseControl;
  requests: readonly RepositoryArtifactRequestV1[];
  temporary: string;
}>): Promise<ViteCapturedInputsV1> {
  const { workspace, invocation, store, control } = options;
  if (store.invocation !== invocation) throw Error("Actual Vite capture owner required");
  const requests = readViteArtifactInputsV1(options.requests);
  if (requests.length === 0) throw Error("Captured Vite configuration requires declared artifact inputs");
  const root = requireNativeStoragePathV1(store.root, join(options.temporary, "inputs"));
  await store.recheck();
  checkResourceLeaseControl(control);
  await mkdir(root, { mode: 0o700 });
  const originalRoot = await lstat(root, { bigint: true });
  const assertRootIdentity = () => {
    const actual = lstatSync(requireNativeStoragePathV1(store.root, root), { bigint: true });
    if (!actual.isDirectory() || actual.isSymbolicLink() || actual.dev !== originalRoot.dev || actual.ino !== originalRoot.ino) {
      throw Error("Private Vite artifact root changed");
    }
  };
  const checkRoot = async (phase: ResourceLeaseControl) => {
    checkResourceLeaseControl(phase);
    await store.recheck();
    assertRootIdentity();
    checkResourceLeaseControl(phase);
  };
  const scope = { workspace, invocation, storage: store.storage, control: store.control };
  const files = new Map<string, BigIntStats>();
  const snapshots = await withRepositoryArtifactGroupV1({ ...scope, phaseControl: control, requests, retainFileDigests: true }, token =>
    withRepositoryArtifactGroupTaskV1(token, invocation, async group => {
      let remaining: number = sourceSchema.properties.maximumRetainedCompilationBytes.const;
      const result = new Map<string, Readonly<{ request: RepositoryArtifactRequestV1; files: readonly string[]; manifest: string; fingerprints: readonly string[] }>>();
      for (const [index, artifact] of group.artifacts.entries()) {
        await checkRoot(group.control);
        const source = requests[index];
        if (!source || artifact.path !== source.path || artifact.owner !== source.owner) throw Error("Vite capture order changed");
        const directory = requireNativeStoragePathV1(store.root, join(root, String(index)));
        await mkdir(directory, { mode: 0o700 });
        const admittedDigests = artifact.fileDigests;
        if (!admittedDigests) throw Error("Vite artifact has no admitted file identities");
        for (const name of [manifestFile, ...artifact.files]) {
          const content = admittedDigests[name];
          if (content === undefined) throw Error("Vite artifact file has no admitted content digest");
          const output = requireNativeStoragePathV1(directory, join(directory, name));
          await mkdir(dirname(output), { recursive: true, mode: 0o700 });
          const input = requireNativeStoragePathV1(artifact.path, join(artifact.path, name));
          const copied = await copyCompilationFileV1(input, output, content, remaining, group.control);
          remaining -= copied.byteLength;
          const timestamps = await lstat(input);
          await utimes(output, timestamps.atime, timestamps.mtime);
          files.set(output, await lstat(output, { bigint: true }));
        }
        const request = { ...source, path: directory };
        const read: RepositoryArtifactReaderOptionsV1 = { ...scope, request };
        const manifest = await readArtifactManifestV1(read, group.control);
        const digests = new Map<string, string>();
        const fingerprints = await inspectArtifactV1(read, artifact.files, group.control, manifest.fingerprint, undefined, digests);
        for (const [name, digest] of digests) {
          if (digest !== admittedDigests[name]) throw Error("Private Vite artifact differs from admitted publication");
        }
        result.set(artifact.path, Object.freeze({ request, files: artifact.files, manifest: manifest.fingerprint, fingerprints }));
      }
      await checkRoot(group.control);
      return result;
    }));
  const assertCurrent = (path: string, expected: BigIntStats) => {
    const actual = lstatSync(requireNativeStoragePathV1(root, path), { bigint: true });
    if (!actual.isFile() || actual.isSymbolicLink() || !identityMatches(actual, expected)) {
      throw Error("Private Vite artifact file changed");
    }
  };
  const used = new Set<string>();
  let projecting = true;
  return Object.freeze({
    rootFor(publishedRoot: string) {
      checkResourceLeaseControl(control);
      const entry = snapshots.get(publishedRoot);
      if (!projecting || !entry) throw Error("Vite configuration requested an undeclared artifact input");
      used.add(publishedRoot);
      return entry.request.path;
    },
    sealProjection() {
      if (!projecting || used.size !== snapshots.size) throw Error("Vite configuration did not consume every declared artifact input");
      projecting = false;
    },
    openFile(path: string) {
      checkResourceLeaseControl(control);
      const expected = files.get(path);
      if (!expected) return undefined;
      assertRootIdentity();
      assertCurrent(path, expected);
      const fd = openSync(path, artifactReadFlagsV1());
      try {
        if (!identityMatches(fstatSync(fd, { bigint: true }), expected)) throw Error("Private Vite artifact descriptor changed");
        assertCurrent(path, expected);
        return fd;
      } catch (error) {
        closeSync(fd);
        throw error;
      }
    },
    async recheck() {
      assertRootIdentity();
      for (const [path, expected] of files) assertCurrent(path, expected);
      if (control.signal.aborted) return;
      return withRequestedIssuedResourceControlV1(control, 0, async phase => {
        await checkRoot(phase);
        for (const entry of snapshots.values()) {
          const read = { ...scope, request: entry.request };
          const manifest = await readArtifactManifestV1(read, phase);
          if (manifest.fingerprint !== entry.manifest) throw Error("Private Vite artifact manifest changed");
          const fingerprints = await inspectArtifactV1(read, entry.files, phase, entry.manifest);
          if (JSON.stringify(fingerprints) !== JSON.stringify(entry.fingerprints)) throw Error("Private Vite artifact bytes or identity changed");
        }
        await checkRoot(phase);
      });
    },
  });
}
