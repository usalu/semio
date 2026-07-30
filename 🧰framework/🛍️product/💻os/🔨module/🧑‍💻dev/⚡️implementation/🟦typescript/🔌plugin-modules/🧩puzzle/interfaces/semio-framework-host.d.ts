/** @module Interface semio:framework/host **/
export function backboneSend(uri: string, message: Uint8Array): void;
export function backbonePoll(uri: string): Array<Uint8Array>;
