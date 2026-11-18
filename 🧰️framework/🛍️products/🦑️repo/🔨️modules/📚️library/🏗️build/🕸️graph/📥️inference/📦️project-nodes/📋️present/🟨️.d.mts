/** 🗝️ The file stem a stored record of one owner root carries. */
export function ownerRecordKeyV1(root: string): string;
/** 📋️ Which of the owner roots already have a stored record in a directory, from one listing. */
export function ownerRecordsPresentV1(directory: string, roots: readonly string[]): Readonly<{ present: ReadonlySet<string>; total: number }>;
/** 💬️ The one line that tells how many owner records are present. */
export function ownerRecordsPresentTextV1(language: string, census: Readonly<{ present: ReadonlySet<string>; total: number }>): string;
