/** 🔒️ Cleanup borrows only original source identity, cumulative effects and funded owner charging. */
export interface NativeDecodeCloseControl {readonly ledgerIdentity:object;readonly ownedBytes:number;charge(bytes:number):Promise<void>}
