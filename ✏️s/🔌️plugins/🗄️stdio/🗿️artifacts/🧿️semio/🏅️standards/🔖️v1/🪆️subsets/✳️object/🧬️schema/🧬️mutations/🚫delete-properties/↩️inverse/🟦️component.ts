/** ↩️ `delete-properties` inverse — CreateProperties with the escrowed handle, or empty if absent. */
export interface DeletePropertiesInverse {
  restoredProperties?: { childId: string; target: string };
}
