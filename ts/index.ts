import wasm, { parse_to_object } from "./pkg/csv_json_parser.js";

/**
 * Parse a string into a JSON object.
 *
 * @template T The type of the JSON object.
 * @param data The string to parse.
 * @returns The JSON object or null if parsing failed.
 */
export async function parseString<T extends object>(
	data: string,
): Promise<null | T> {
	await wasm();
	try {
		return parse_to_object(data);
	} catch (e) {
		console.error(e);
		return null;
	}
}
