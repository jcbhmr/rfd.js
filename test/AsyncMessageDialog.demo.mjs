import { AsyncMessageDialog } from "../index.js";

const answer = new AsyncMessageDialog().setDescription("Hello world!").show();
console.debug("answer=%s", answer);
