/** ♻️ The caller retains the actual ledger, recipient and capacity admission. */
export interface OriginalRetirementCloseSource{readonly ledgerIdentity:object;readonly retirementRecipientIdentity:object|null;charge(bytes:number):void}
/** 🧷️ Cleanup erasure borrows the same original capacity callback without a replacement counter. */
export interface OriginalRetirementCloseLoan{readonly ledgerIdentity:object;readonly retirementRecipientIdentity:object|null;charge(bytes:number):void}
/** 🎟️ Trusted receiving composition supplies its actual original retirement source. */
export function borrowOriginalRetirementClose(source:OriginalRetirementCloseSource):OriginalRetirementCloseLoan{const ledgerIdentity=source.ledgerIdentity,retirementRecipientIdentity=source.retirementRecipientIdentity;return Object.freeze({ledgerIdentity,retirementRecipientIdentity,charge(bytes:number){source.charge(bytes)}})}
