import test from "node:test";
import assert from "node:assert";
import { MessageDialog } from "../index.js";

if (!process.env.CI) {
  test("MessageDialog demo", () => {
    const answer = new MessageDialog().setDescription("Hello world!").show();

    console.debug(answer);
    assert(typeof answer === "boolean");
  });
}
