import { describe, it, expect } from "vitest";

import { parseString } from "../ts/index.js";

describe("Parsing", () => {
  it("should parse an object", async () => {
    const result = await parseString("A,B,C\n1,2,3");
    expect(result).toEqual({
      A: 1,
      B: "D",
      C: 3,
    });
  });
});
