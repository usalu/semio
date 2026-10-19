/** 📇️ PDF 1.7 snapshot types of document information, encryption and viewer settings. */
import type { ArtifactRef } from "@semio-tech/framework/schema/artifact-reference";
import type { PdfDictEntry } from "../🧩️cos/🟦️.ts";
import type { PdfDate } from "../📆️date/🟦️.ts";

export interface PdfInfo {
  title?: string | null;
  author?: string | null;
  subject?: string | null;
  keywords?: string | null;
  creator?: string | null;
  producer?: string | null;
  creationDate?: PdfDate | null;
  modificationDate?: PdfDate | null;
  trapped?: string | null;
  extra?: PdfDictEntry[];
}

export interface PdfEncryption {
  algorithm: PdfEncryptionAlgorithm;
  permissions?: number;
  userPassword?: string;
  ownerPassword?: string | null;
  encryptMetadata?: boolean;
}

export type PdfEncryptionAlgorithm =
  | "rc4_40"
  | "rc4_128"
  | "aes128"
  | "aes256";

export interface PdfMarkInfo {
  marked?: boolean;
  userProperties?: boolean;
  suspects?: boolean;
}

export interface PdfViewerPreferences {
  hideToolbar?: boolean;
  hideMenubar?: boolean;
  hideWindowUi?: boolean;
  fitWindow?: boolean;
  centerWindow?: boolean;
  displayDocTitle?: boolean;
  nonFullScreenPageMode?: PdfPageMode | null;
  direction?: string | null;
  viewArea?: string | null;
  viewClip?: string | null;
  printArea?: string | null;
  printClip?: string | null;
  printScaling?: string | null;
  duplex?: string | null;
  pickTrayByPdfSize?: boolean;
  printPageRange?: number[];
  numCopies?: number | null;
  extra?: PdfDictEntry[];
}

export type PdfPageMode =
  | "useNone"
  | "useOutlines"
  | "useThumbs"
  | "fullScreen"
  | "useOc"
  | "useAttachments";

export type PdfPageLayout =
  | "singlePage"
  | "oneColumn"
  | "twoColumnLeft"
  | "twoColumnRight"
  | "twoPageLeft"
  | "twoPageRight";

export interface PdfOutputIntent {
  subtype: string;
  conditionIdentifier: string;
  condition?: string | null;
  registryName?: string | null;
  info?: string | null;
  profile?: ArtifactRef | null;
}

export interface PdfEmbeddedFile {
  id: string;
  fileName: string;
  description?: string | null;
  mimeType?: string | null;
  data: number[];
  creationDate?: PdfDate | null;
  modificationDate?: PdfDate | null;
  relationship?: string | null;
  listed?: boolean;
}
