import wasm, { parse_to_object } from "./pkg/csv_json_parser.js";

/**
 * Parse a string into a JSON object.
 * @param data The string to parse.
 * @returns The JSON object or null if parsing failed.
 * @template T The type of the JSON object.
 */
export async function parseString<T extends object>(
  data: string,
): Promise<T | null> {
  await wasm();
  try {
    return parse_to_object(data);
  } catch (e) {
    console.error(e);
    return null;
  }
}
