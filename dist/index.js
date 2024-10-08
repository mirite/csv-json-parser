var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
import wasm, { parse_string } from "../pkg/csv_json_parser.js";
export function parseString(data) {
    return __awaiter(this, void 0, void 0, function* () {
        try {
            yield wasm();
            return JSON.parse(parse_string(data));
        }
        catch (e) {
            console.error(e);
            return null;
        }
    });
}
