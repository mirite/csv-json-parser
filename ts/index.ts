import wasm, { parse_string } from "./pkg/csv_json_parser.js";

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
    const str = parse_string(data) as string;
    return JSON.parse(str);
  } catch (e) {
    console.error(e);
    return null;
  }
}
