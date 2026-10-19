/** 🧰️ The shared plugin lifecycle types: the extension ledger entry the lifecycle callbacks exchange. */
//#region FrameworkOsShell
/** 🧩️ Durable-in-session extension ledger mirror — space document ops are dispatched
 * best-effort when a space/studio session can accept them; this state keeps Settings + contribution
 * filtering coherent even when the space app is not the active session. */
export type ExtensionLedgerEntry = {
  readonly extensionId: string;
  readonly version: string;
  readonly sourceUri: string;
  readonly packageHash: string;
  readonly enabled: boolean;
  readonly extendsHost: string;
};
