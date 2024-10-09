import wasm, { parse_string } from "../pkg/csv_json_parser.js";

/**
 *
 * @param data
 */
export async function parseString<T extends object>(
  data: string,
): Promise<T | null> {
  try {
    await wasm();
    const str = parse_string(data);
    console.log(str);
    return JSON.parse(str);
  } catch (e) {
    console.error(e);
    return null;
  }
}
