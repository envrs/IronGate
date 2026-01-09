import { Request, Response } from "@irongate/sdk-shared";

export declare class Backend {
    constructor();
    handle(req: Request): Promise<Response>;
}
