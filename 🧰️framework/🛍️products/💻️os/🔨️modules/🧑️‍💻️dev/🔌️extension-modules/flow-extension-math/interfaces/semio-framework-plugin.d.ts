/** @module Interface semio:framework/plugin **/
export function manifest(): Uint8Array;
export function instantiateApp(appId: string, instanceId: string): number;
export function exchange(instanceId: number, commands: Array<Uint8Array>): Array<Uint8Array>;
export function migrateDocument(input: MigrateDocumentInput): MigrateDocumentOutput;
export function clearInstanceGuard(): void;
export type MigrateDocumentInput = import('./semio-framework-types.js').MigrateDocumentInput;
export type MigrateDocumentOutput = import('./semio-framework-types.js').MigrateDocumentOutput;
export type PluginError = import('./semio-framework-types.js').PluginError;
