// No need for actual 'test()' functions since if this successfully imports it
// all, then that's good enough for CI. The real testing is manual tinkering.
import {
  AsyncFileDialog,
  AsyncMessageDialog,
  FileDialog,
  FileHandle,
  MessageButtons,
  MessageDialog,
  MessageLevel,
} from "./index.js";

// Exposed for 'npm run test:repl' to play with!
globalThis.AsyncFileDialog = AsyncFileDialog;
globalThis.AsyncMessageDialog = AsyncMessageDialog;
globalThis.FileDialog = FileDialog;
globalThis.FileHandle = FileHandle;
globalThis.MessageButtons = MessageButtons;
globalThis.MessageDialog = MessageDialog;
globalThis.MessageLevel = MessageLevel;
