/** 🔗️ Stable field addresses connect authored controls, labels and validation messages. */
export interface FieldAssociation {
  readonly target?: string;
  readonly label?: string;
  readonly description?: string;
  readonly error?: string;
  readonly describedBy?: string;
}

export function fieldAssociation(id?: string, controlId?: string | null, hasDescription = false, hasError = false): FieldAssociation {
  const target = controlId === null ? undefined : controlId || (id ? `${id}.control` : undefined);
  const description = id && hasDescription ? `${id}.description` : undefined;
  const error = id && hasError ? `${id}.error` : undefined;
  const describedBy = [description, error].filter(Boolean).join(" ") || undefined;
  return { ...(target ? { target } : {}), ...(id ? { label: `${id}.label` } : {}), ...(description ? { description } : {}), ...(error ? { error } : {}), ...(describedBy ? { describedBy } : {}) };
}
