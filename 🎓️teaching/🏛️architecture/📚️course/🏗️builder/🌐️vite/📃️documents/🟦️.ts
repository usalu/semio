export type CourseAssignmentPdfDocument = Readonly<{ namespace: "course"; id: string; producer: string; file: string }>;

/** 📚️ Deduplicates exact publications while retaining every file and refusing conflicting public paths. */
export function courseAssignmentPdfDocumentsV1(values: readonly Readonly<{ namespace: string; id: string; producer: string; file: string }>[]): readonly CourseAssignmentPdfDocument[] {
  const files = new Map<string, CourseAssignmentPdfDocument>();
  for (const document of values) {
    if (document.namespace !== "course" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/.test(document.id) || document.id.length > 80 || !/^@[a-z][a-z0-9-]*\/[a-z][a-z0-9-]*:[a-z][a-z0-9-]*$/.test(document.producer) || document.producer.length > 160 || document.file !== document.id + ".pdf") throw Error("Declared assignment PDF publication required");
    const prior = files.get(document.file);
    if (prior && (prior.namespace !== document.namespace || prior.id !== document.id || prior.producer !== document.producer)) throw Error("Conflicting assignment PDF producers: " + document.file);
    if (!prior) files.set(document.file, { namespace: document.namespace as CourseAssignmentPdfDocument["namespace"], id: document.id, producer: document.producer, file: document.file });
  }
  return [...files.values()];
}
