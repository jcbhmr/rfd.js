import { MessageDialog } from "../index.js";

const answer = new MessageDialog().setDescription("Hello world!").show();
console.debug("answer=%s", answer);
