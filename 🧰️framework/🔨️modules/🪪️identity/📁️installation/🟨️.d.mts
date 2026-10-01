declare const installationDirectoryV1: unique symbol;
export type InstallationDirectoryV1 = string & { readonly [installationDirectoryV1]: true };
/** 🪪️ Admits the shared physical installation identity. */
export function parseInstallationDirectoryV1(value: unknown): InstallationDirectoryV1;
/** 🧩️ Derives the admitted name's sibling emoji identity. */
export function installationDirectoryEmoji(value: unknown): string;
/** ⚠️ Finds a sibling occupying the admitted identity. */
export function installationDirectoryCollision(name: InstallationDirectoryV1, siblings: readonly string[]): string | undefined;
/** 🔁️ Binds a fresh schema snapshot read by the source owner. */
export function installationDirectoryParserV1(contract: Readonly<{ type: "string"; pattern: string; maxLength: number }>): (value: unknown) => InstallationDirectoryV1;
