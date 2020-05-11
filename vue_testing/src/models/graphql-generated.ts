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

export type Mutation = {
  __typename?: "Mutation";
  startStopServer: Server;
  updateServer: Server;
};

export type MutationStartStopServerArgs = {
  serverId: Scalars["Int"];
};

export type MutationUpdateServerArgs = {
  serverUpdate: ServerUpdate;
};

export type Game = {
  __typename?: "Game";
  id: Scalars["Int"];
  name: Scalars["String"];
  image: Scalars["String"];
};

export type Server = {
  __typename?: "Server";
  id: Scalars["Int"];
  name: Scalars["String"];
  game: Game;
  status: Scalars["String"];
};

export type ServerUpdate = {
  id: Scalars["Int"];
  name: Scalars["String"];
  gameId: Scalars["Int"];
};
