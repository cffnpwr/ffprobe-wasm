export class ComponentError extends Error {
  constructor(value: unknown) {
    const enumerable = typeof value !== "string";
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, "payload", { value, enumerable });
  }
}
