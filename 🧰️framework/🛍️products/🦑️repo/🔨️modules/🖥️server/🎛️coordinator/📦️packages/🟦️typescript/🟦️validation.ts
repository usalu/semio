//#region 🔖️Contracts
type OwnedValidation<T> = { readonly success: true; readonly data: T } | { readonly success: false; readonly message: string };
export type OwnedValidationResult<T> = { readonly success: true; readonly data: T } | { readonly success: false; readonly error: { readonly message: string } };
type OwnedSchemaShape = Readonly<Record<string, OwnedSchema<unknown>>>;
type OwnedSchemaValue<TSchema> = TSchema extends OwnedSchema<infer TValue> ? TValue : never;
//#endregion 🔖️Contracts

//#region 🧱️Schemas
export class OwnedSchema<T> {
  constructor(private readonly parseValue: (value: unknown, path: string) => OwnedValidation<T>) {}

  /** 🧪️ Validates one external value without exposing an implementation-specific error type. */
  safeParse(value: unknown): OwnedValidationResult<T> {
    const parsed = this.parseValue(value, "value");
    if ("message" in parsed) return { success: false, error: { message: parsed.message } };
    return parsed;
  }

  /** 🎁️ Supplies a value only when the field is absent. */
  default(defaultValue: T): OwnedSchema<T> {
    return new OwnedSchema((value, path) => (value === undefined ? { success: true, data: defaultValue } : this.parseValue(value, path)));
  }

  /** 🫙️ Accepts an explicit null in addition to the wrapped value. */
  nullable(): OwnedSchema<T | null> {
    return new OwnedSchema((value, path) => (value === null ? { success: true, data: null } : this.parseValue(value, path)));
  }

  /** 🔬️ Runs the schema as part of a containing owned schema. */
  parse(value: unknown, path: string): OwnedValidation<T> {
    return this.parseValue(value, path);
  }
}

class OwnedStringSchema extends OwnedSchema<string> {
  constructor(private readonly minimum = 0, private readonly emailRequired = false) {
    super((value, path) => {
      if (typeof value !== "string") return { success: false, message: `${path}: expected string` };
      if (value.length < minimum) return { success: false, message: `${path}: expected at least ${minimum} character(s)` };
      if (emailRequired && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return { success: false, message: `${path}: invalid email address` };
      return { success: true, data: value };
    });
  }

  /** 📏️ Requires the declared minimum string length. */
  min(minimum: number): OwnedStringSchema {
    return new OwnedStringSchema(minimum, this.emailRequired);
  }

  /** ✉️ Requires the coordinator's canonical mailbox-shaped address. */
  email(): OwnedStringSchema {
    return new OwnedStringSchema(this.minimum, true);
  }
}

function ownedObject<TShape extends OwnedSchemaShape>(shape: TShape): OwnedSchema<{ readonly [TKey in keyof TShape]: OwnedSchemaValue<TShape[TKey]> }> {
  return new OwnedSchema<{ readonly [TKey in keyof TShape]: OwnedSchemaValue<TShape[TKey]> }>((value, path) => {
    if (typeof value !== "object" || value === null || Array.isArray(value)) return { success: false, message: `${path}: expected object` };
    const source = value as Readonly<Record<string, unknown>>;
    const data: Record<string, unknown> = {};
    for (const [key, schema] of Object.entries(shape)) {
      const parsed = schema.parse(source[key], `${path}.${key}`);
      if ("message" in parsed) return { success: false, message: parsed.message };
      data[key] = parsed.data;
    }
    return { success: true, data: data as { readonly [TKey in keyof TShape]: OwnedSchemaValue<TShape[TKey]> } };
  });
}
//#endregion 🧱️Schemas

//#region 🏭️Factory
export const ownedSchema = {
  string: (): OwnedStringSchema => new OwnedStringSchema(),
  boolean: (): OwnedSchema<boolean> => new OwnedSchema((value, path) => (typeof value === "boolean" ? { success: true, data: value } : { success: false, message: `${path}: expected boolean` })),
  literal: <const TValue extends string>(expected: TValue): OwnedSchema<TValue> => new OwnedSchema((value, path) => (value === expected ? { success: true, data: expected } : { success: false, message: `${path}: expected ${JSON.stringify(expected)}` })),
  enum: <const TValue extends readonly string[]>(values: TValue): OwnedSchema<TValue[number]> => new OwnedSchema((value, path) => (typeof value === "string" && values.includes(value) ? { success: true, data: value as TValue[number] } : { success: false, message: `${path}: expected one of ${values.join(", ")}` })),
  unknown: (): OwnedSchema<unknown> => new OwnedSchema((value) => ({ success: true, data: value })),
  array: <TValue>(item: OwnedSchema<TValue>): OwnedSchema<readonly TValue[]> => new OwnedSchema<readonly TValue[]>((value, path) => {
    if (!Array.isArray(value)) return { success: false, message: `${path}: expected array` };
    const data: TValue[] = [];
    for (let index = 0; index < value.length; index++) {
      const parsed = item.parse(value[index], `${path}.${index}`);
      if ("message" in parsed) return { success: false, message: parsed.message };
      data.push(parsed.data);
    }
    return { success: true, data };
  }),
  object: ownedObject,
} as const;
//#endregion 🏭️Factory
