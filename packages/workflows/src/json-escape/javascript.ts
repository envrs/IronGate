import { type BytesInput, type SDK } from "irongate:workflow";

export function run(input: BytesInput, sdk: SDK) {
  const parsed = sdk.asString(input);
  return JSON.stringify(parsed);
}
