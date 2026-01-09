declare module "irongate:utils" {
  /**
   * The SDK for the runtime information.
   * @category Runtime
   */
  export type RuntimeSDK = {
    /**
     * Get the current version of Irongate.
     */
    get version(): string;
  };
}
