import { MessageLevel, MessageDialog } from "../index.js";

const answer1 = new MessageDialog()
  .setLevel(MessageLevel.Error)
  .setDescription("Hello world!")
  .show();
console.debug("answer1=%s", answer1);
