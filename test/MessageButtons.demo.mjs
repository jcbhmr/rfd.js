import { MessageButtons, MessageDialog } from "../index.js";

const answer1 = new MessageDialog()
  .setButtons(MessageButtons.Ok)
  .setDescription("Hello world! Ok")
  .show();
console.debug("answer1=%s", answer1);

const answer2 = new MessageDialog()
  .setButtons(MessageButtons.OkCancel)
  .setDescription("Hello world! OkCancel")
  .show();
console.debug("answer2=%s", answer2);

const answer3 = new MessageDialog()
  .setButtons(MessageButtons.YesNo)
  .setDescription("Hello world! YesNo")
  .show();
console.debug("answer3=%s", answer3);
