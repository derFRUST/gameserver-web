import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'
import GameServers from '../views/GameServers.vue'

Vue.use(VueRouter)

const routes: Array<RouteConfig> = [
  {
    path: '/game-servers',
    name: 'Game servers',
    component: GameServers
    // route level code-splitting
    // this generates a separate chunk (about.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    // component: () => import(/* webpackChunkName: "game-servers" */ '../views/GameServers.vue')
  }
]

const router = new VueRouter({
  routes
})

export default router
