import test from "node:test";
import assert from "node:assert";
import { AsyncMessageDialog } from "../index.js";

if (!process.env.CI) {
  test("AsyncMessageDialog demo", async () => {
    const answer = await new AsyncMessageDialog()
      .setDescription("Hello world!")
      .show();

    console.debug(answer);
    assert(typeof answer === "boolean");
  });
}
