/** 🧬️ Transparent PDF mutation TypeScript union assembled from direct owners. */

import type { InsertPageMutation } from './📥️insert-page/🟦️component.ts';
import type { RemovePageMutation } from './🗑️remove-page/🟦️component.ts';
import type { SetPageMediaBoxMutation } from './📐️set-page-media-box/🟦️component.ts';
import type { SetPageCropBoxMutation } from './✂️set-page-crop-box/🟦️component.ts';
import type { AppendPageContentMutation } from './➕️append-page-content/🟦️component.ts';
import type { SetInfoMutation } from './ℹ️set-info/🟦️component.ts';
import type { InsertObjectMutation } from './📦️insert-object/🟦️component.ts';
import type { RemoveObjectMutation } from './🧹️remove-object/🟦️component.ts';
import type { SetObjectValueMutation } from './🔧️set-object-value/🟦️component.ts';
import type { SetDictEntryMutation } from './🔑️set-dict-entry/🟦️component.ts';
import type { RemoveDictEntryMutation } from './🚫️remove-dict-entry/🟦️component.ts';
import type { SetTrailerEntryMutation } from './🧳️set-trailer-entry/🟦️component.ts';
import type { RemoveTrailerEntryMutation } from './🧽️remove-trailer-entry/🟦️component.ts';
import type { MovePageMutation } from './🔀️move-page/🟦️component.ts';
import type { SetPageContentMutation } from './✏️set-page-content/🟦️component.ts';
import type { SetPageRotationMutation } from './🔄️set-page-rotation/🟦️component.ts';

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
