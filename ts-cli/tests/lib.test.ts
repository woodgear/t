import { assert } from "chai";
import { ping } from "src/lib";

describe("", () => {
    it("pin should ok", async () => {
        assert.equal("pong", await ping())
    });
})