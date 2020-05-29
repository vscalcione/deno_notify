import { notify, pluginId } from "../ts/prepared.ts";
import { assert } from "https://deno.land/std@v0.53.0/testing/asserts.ts";

Deno.test("Check plugin id", () => {
  assert(pluginId !== null);
});

// Need to send complete notification before simple because the icon can't change after being set on mac
Deno.test("Send complete notification", () => {
  notify({
    title: "Hey",
    message: "Hello World",
    icon: {
      app: "Safari",
    },
    sound: "Basso",
  });
});

Deno.test("Send simple notification", () => {
  notify("Message");
});
