-- Your SQL goes here

CREATE TABLE "games" (
    "id"    INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name"  TEXT NOT NULL,
    "image" TEXT NOT NULL
);

CREATE TABLE "servers" (
    "id"      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name"    TEXT NOT NULL,
    "game_id" INTEGER NOT NULL,
    "status"  TEXT NOT NULL DEFAULT 'STOPPED' CHECK("status" IN ('STOPPED', 'STARTING', 'STARTED', 'STOPPING')),
    FOREIGN KEY("game_id") REFERENCES "games"("id")
);
