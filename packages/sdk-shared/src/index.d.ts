export interface Request {
  id: string;
  method: string;
  url: string;
}

export interface Response {
  id: string;
  status: number;
  body: unknown;
}

export type Handler = (req: Request) => Promise<Response>;
