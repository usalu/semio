/** 🧬️ Transparent PDF mutation TypeScript union assembled from direct owners. */

import type { InsertPageMutation } from './📥️insert-page/🟦️.ts';
import type { RemovePageMutation } from './🗑️remove-page/🟦️.ts';
import type { SetPageMediaBoxMutation } from './📐️set-page-media-box/🟦️.ts';
import type { SetPageCropBoxMutation } from './✂️set-page-crop-box/🟦️.ts';
import type { AppendPageContentMutation } from './➕️append-page-content/🟦️.ts';
import type { SetInfoMutation } from './ℹ️set-info/🟦️.ts';
import type { InsertObjectMutation } from './📦️insert-object/🟦️.ts';
import type { RemoveObjectMutation } from './🧹️remove-object/🟦️.ts';
import type { SetObjectValueMutation } from './🔧️set-object-value/🟦️.ts';
import type { SetDictEntryMutation } from './🔑️set-dict-entry/🟦️.ts';
import type { RemoveDictEntryMutation } from './🚫️remove-dict-entry/🟦️.ts';
import type { SetTrailerEntryMutation } from './🧳️set-trailer-entry/🟦️.ts';
import type { RemoveTrailerEntryMutation } from './🧽️remove-trailer-entry/🟦️.ts';
import type { MovePageMutation } from './🔀️move-page/🟦️.ts';
import type { SetPageContentMutation } from './✏️set-page-content/🟦️.ts';
import type { SetPageRotationMutation } from './🔄️set-page-rotation/🟦️.ts';

export type PdfMutation =
  | InsertPageMutation
  | RemovePageMutation
  | SetPageMediaBoxMutation
  | SetPageCropBoxMutation
  | AppendPageContentMutation
  | SetInfoMutation
  | InsertObjectMutation
  | RemoveObjectMutation
  | SetObjectValueMutation
  | SetDictEntryMutation
  | RemoveDictEntryMutation
  | SetTrailerEntryMutation
  | RemoveTrailerEntryMutation
  | MovePageMutation
  | SetPageContentMutation
  | SetPageRotationMutation;
