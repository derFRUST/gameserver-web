export type Maybe<T> = T | null;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Query = {
  __typename?: "Query";
  games: Array<Game>;
  servers: Array<Server>;
};

export type Game = {
  __typename?: "Game";
  id: Scalars["ID"];
  name: Scalars["String"];
  image?: Maybe<Scalars["String"]>;
};

export enum ServerStatus {
  Stopped = "STOPPED",
  Starting = "STARTING",
  Started = "STARTED",
  Stopping = "STOPPING",
}

export type Server = {
  __typename?: "Server";
  id: Scalars["ID"];
  name: Scalars["String"];
  game: Game;
  status: ServerStatus;
};

export type Mutation = {
  __typename?: "Mutation";
  updateServer: ServerPayload;
  startServer: ServerPayload;
  stopServer: ServerPayload;
};

export type MutationUpdateServerArgs = {
  input: UpdateServerInput;
};

export type MutationStartServerArgs = {
  input: StartServerInput;
};

export type MutationStopServerArgs = {
  input: StopServerInput;
};

export type UpdateServerInput = {
  id: Scalars["ID"];
  name: Scalars["String"];
  gameId: Scalars["ID"];
};

export type ServerPayload = {
  __typename?: "ServerPayload";
  server: Server;
};

export type StartServerInput = {
  id: Scalars["ID"];
};

export type StopServerInput = {
  id: Scalars["ID"];
};
