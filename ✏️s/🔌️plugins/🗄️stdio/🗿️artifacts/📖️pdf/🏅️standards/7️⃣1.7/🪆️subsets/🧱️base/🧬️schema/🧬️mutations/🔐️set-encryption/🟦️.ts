/** 🔐️ Direct set-encryption TypeScript payload. */
import type { PdfEncryption, PdfEncryptionAlgorithm } from '../../📸️snapshot/📇️metadata/🟦️.ts';
export interface SetEncryptionMutation {
  mutation: 'setEncryption';
  encryption?: PdfEncryption | null;
}
