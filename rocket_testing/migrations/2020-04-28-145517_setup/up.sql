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
    "status"  TEXT NOT NULL DEFAULT "STOPPED" CHECK("status" IN ("STOPPED", "STARTING", "STARTED", "STOPPING")),
    FOREIGN KEY("game_id") REFERENCES "games"("id")
);

INSERT INTO "games"("id", "name", "image") VALUES (1, "Factorio Experimental 0.18.18", "427520");
INSERT INTO "games"("id", "name", "image") VALUES (2, "Satisfactory Early Access", "526870");
INSERT INTO "games"("id", "name", "image") VALUES (3, "ARK: Survival Evolved", "346110");

INSERT INTO "servers"("name", "game_id", "status") VALUES ("factorio-01", 1, "STOPPED");
INSERT INTO "servers"("name", "game_id", "status") VALUES ("satisfactory-01", 2, "STARTED");
