import { type API as Irongate } from "@irongate/sdk-frontend";
import { type API } from "workflows-backend";

export type FrontendSDK = Irongate<API>;
