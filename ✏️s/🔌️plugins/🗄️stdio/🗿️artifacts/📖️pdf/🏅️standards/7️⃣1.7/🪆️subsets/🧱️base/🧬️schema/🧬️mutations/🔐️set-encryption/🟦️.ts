/** 🔐️ Direct set-encryption TypeScript payload. */
import type { PdfEncryption, PdfEncryptionAlgorithm } from '../../📸️snapshot/🟦️.ts';
export interface SetEncryptionMutation {
  mutation: 'setEncryption';
  encryption?: PdfEncryption | null;
}
