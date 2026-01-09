import { type BytesInput, type SDK } from "irongate:workflow";

/**
 * @param {BytesInput} input
 * @param {SDK} sdk
 * @returns {MaybePromise<Data>}
 */
export function run(input: BytesInput, sdk: SDK): MaybePromise<Data> {
  const parsed = sdk.asString(Date.now().toString());
  return parsed;
}
