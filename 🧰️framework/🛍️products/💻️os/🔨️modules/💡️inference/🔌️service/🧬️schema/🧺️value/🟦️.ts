export type ServiceValueV1 = null | boolean | number | string | readonly ServiceValueV1[] | Readonly<{ [key: string]: ServiceValueV1 }>;
export type ServiceSchemaRecordV1 = Readonly<Record<string, ServiceValueV1>>;
export type ServiceValueLimitsV1 = Readonly<{ nodes: number; depth: number; capacity: number }>;
export const SERVICE_PAYLOAD_LIMITS_V1: ServiceValueLimitsV1 = Object.freeze({nodes:4096,depth:32,capacity:16384});
export const SERVICE_SCHEMA_LIMITS_V1: ServiceValueLimitsV1 = Object.freeze({nodes:4096,depth:32,capacity:32768});

/** 📦 Owns a finite readonly semantic value tree without any wire representation. */
export function boundedServicePayloadV1(value: unknown, limits: ServiceValueLimitsV1 = SERVICE_PAYLOAD_LIMITS_V1): ServiceValueV1 {
  let remainingNodes = limits.nodes, remainingCapacity = limits.capacity;
  const ancestors = new Set<object>();
  const charge = (units: number): void => { remainingCapacity -= units; if (remainingCapacity < 0) throw new Error("installed-service.bounds"); };
  const visit = (item: unknown, depth: number): ServiceValueV1 => {
    if (--remainingNodes < 0 || depth > limits.depth) throw new Error("installed-service.bounds");
    charge(1);
    if (item === null || typeof item === "boolean") return item;
    if (typeof item === "string") { charge(item.length); return item; }
    if (typeof item === "number" && Number.isFinite(item) && (!Number.isInteger(item) || Number.isSafeInteger(item))) { charge(8); return item; }
    if (typeof item !== "object" || item === null || ancestors.has(item)) throw new Error("installed-service.invalid");
    const array = Array.isArray(item);
    if (Object.getPrototypeOf(item) !== (array ? Array.prototype : Object.prototype) || Object.getOwnPropertySymbols(item).length) throw new Error("installed-service.invalid");
    ancestors.add(item);
    try {
      const descriptors = Object.getOwnPropertyDescriptors(item);
      if (array) {
        const output: ServiceValueV1[] = [];
        if (Object.keys(descriptors).length !== item.length + 1) throw new Error("installed-service.invalid");
        for (let index = 0; index < item.length; index++) {
          const descriptor = descriptors[String(index)];
          if (!descriptor || !Object.hasOwn(descriptor,"value") || !descriptor.enumerable) throw new Error("installed-service.invalid");
          output.push(visit(descriptor.value, depth + 1));
        }
        return Object.freeze(output);
      }
      const output: Record<string, ServiceValueV1> = {};
      for (const [key, descriptor] of Object.entries(descriptors)) {
        if (!Object.hasOwn(descriptor,"value") || !descriptor.enumerable) throw new Error("installed-service.invalid");
        charge(key.length);
        Object.defineProperty(output,key,{value:visit(descriptor.value,depth + 1),enumerable:true,writable:false,configurable:false});
      }
      return Object.freeze(output);
    } finally { ancestors.delete(item); }
  };
  return visit(value,0);
}

/** 📜 Owns one semantic schema record under its explicit semantic capacity policy. */
export function admittedServiceSchemaRecordV1(value: unknown): ServiceSchemaRecordV1 {
  const owned = boundedServicePayloadV1(value,SERVICE_SCHEMA_LIMITS_V1);
  if (typeof owned !== "object" || owned === null || Array.isArray(owned)) throw new Error("installed-service.invalid-schema");
  return owned as ServiceSchemaRecordV1;
}
