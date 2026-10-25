/** 📍️ The directory of artifact instances: the instance reference and the directory that registers and finds them.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
//#region 🔖️InstanceDirectory
/** 🗺️ Where one artifact instance lives — mirrors the Rust host's `InstanceDirectory` entry shape
 * (contract freeze, W2 ownership doc: "`InstanceDirectory` mapping an artifact ref to `(pluginId,
 * instanceId, artifactKind)`"). */
export type ArtifactInstanceRef = {
  readonly pluginId: string;
  readonly instanceId: number;
  readonly artifactKind: string;
};

/** 📂️ Host-side registry from artifact id to the plugin instance that owns it — the transaction
 * coordinator's `InstanceDirectory(target) → (plugin, instance)` lookup (contract freeze §5.3).
 * Registration/unregistration is the caller's responsibility (on `createApp`/`loadDocument` and on
 * `destroyApp`), matching how the Rust host's directory is populated. */
export class InstanceDirectory {
  private readonly byArtifactId = new Map<string, ArtifactInstanceRef>();

  register(artifactId: string, ref: ArtifactInstanceRef): void {
    this.byArtifactId.set(artifactId, ref);
  }

  unregister(artifactId: string): void {
    this.byArtifactId.delete(artifactId);
  }

  resolve(artifactId: string): ArtifactInstanceRef | undefined {
    return this.byArtifactId.get(artifactId);
  }

  entries(): ReadonlyArray<readonly [string, ArtifactInstanceRef]> {
    return [...this.byArtifactId.entries()];
  }
}
//#endregion 🔖️InstanceDirectory
// #endregion 🎠️Kernel
