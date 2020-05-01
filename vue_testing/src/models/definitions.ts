export interface Game {
  id: number;
  name: string;
  image: string;
}

export interface Server {
  id: number;
  name: string;
  game: Game;
  status: string;
}
