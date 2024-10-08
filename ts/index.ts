import wasm, {parse_string} from "../pkg/csv_json_parser.js";

export async function parseString<T extends object>(data: string): Promise<T|null> {
    try {
    await wasm()
    return JSON.parse(parse_string(data));
    } catch (e) {
        console.error(e);
        return null;
    }
}
