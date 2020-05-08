import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import GameServers from "../views/GameServers.vue";
import GameServer from "../views/GameServer.vue";
import GameServerStatus from "../views/GameServerStatus.vue";
import GameServerSettings from "../views/GameServerSettings.vue";
import GameServerSaves from "../views/GameServerSaves.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    redirect: "/game-servers",
  },
  {
    path: "/game-servers",
    name: "game servers",
    component: GameServers,
  },
  {
    path: "/game-servers/:server_id",
    component: GameServer,
    children: [
      {
        path: "",
        name: "game server",
        component: GameServerStatus,
      },
      {
        path: "settings",
        name: "game server settings",
        component: GameServerSettings,
      },
      {
        path: "saves",
        name: "game server saves",
        component: GameServerSaves,
      },
    ],
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    // component: () => import(/* webpackChunkName: "game-servers" */ '../views/GameServers.vue')
  },
];

const router = new VueRouter({
  routes,
});

export default router;
