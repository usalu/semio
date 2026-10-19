type SchemaObject = Readonly<Record<string, unknown>>;

/** 🧬️ A closed validation document retaining only the requested definition and its local reference graph. */
export interface ContractProjection {
  readonly $ref: string;
  readonly $defs: Readonly<Record<string, unknown>>;
}

const ANNOTATIONS = new Set(["title", "description", "$comment", "examples", "default"]);
const MAPS = new Set(["properties", "patternProperties", "dependentSchemas"]);
const ARRAYS = new Set(["allOf", "anyOf", "oneOf", "prefixItems"]);
const SCHEMAS = new Set(["additionalProperties", "unevaluatedProperties", "propertyNames", "items", "contains", "not", "if", "then", "else"]);
const object = (value: unknown): value is SchemaObject => typeof value === "object" && value !== null && !Array.isArray(value);

/** 📽️ Projects normative assertions at build time without copying authoring prose or unreachable definitions into the browser. */
export function projectContractDefinitions(source: unknown, root: string): ContractProjection {
  if (!object(source) || !object(source.$defs)) throw new Error("Schema definitions are required");
  const definitions = source.$defs;
  const projected: Record<string, unknown> = {};
  const retain = (name: string): void => {
    if (Object.hasOwn(projected, name)) return;
    if (!Object.hasOwn(definitions, name)) throw new Error(`Missing schema definition: ${name}`);
    projected[name] = undefined;
    projected[name] = project(definitions[name]);
  };
  const project = (schema: unknown): unknown => {
    if (typeof schema === "boolean") return schema;
    if (!object(schema)) throw new Error("Invalid schema node");
    if (typeof schema.$ref === "string") {
      if (!/^#\/\$defs\/[^/]+$/.test(schema.$ref)) throw new Error(`Unsupported schema reference: ${schema.$ref}`);
      retain(schema.$ref.slice("#/$defs/".length).replaceAll("~1", "/").replaceAll("~0", "~"));
    }
    return Object.fromEntries(Object.entries(schema).filter(([key]) => !ANNOTATIONS.has(key)).map(([key, value]) => {
      if (MAPS.has(key) && object(value)) return [key, Object.fromEntries(Object.entries(value).map(([name, child]) => [name, project(child)]))];
      if (ARRAYS.has(key) && Array.isArray(value)) return [key, value.map(project)];
      if (SCHEMAS.has(key)) return [key, Array.isArray(value) ? value.map(project) : project(value)];
      return [key, structuredClone(value)];
    }));
  };
  retain(root);
  return { $ref: `#/$defs/${root.replaceAll("~", "~0").replaceAll("/", "~1")}`, $defs: Object.fromEntries(Object.entries(projected).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)) };
}
