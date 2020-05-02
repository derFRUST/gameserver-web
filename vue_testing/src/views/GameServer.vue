<template>
  <b-container>
    <p v-if="!server" class="text-center">Loading server data ...</p>
    <b-card
      v-if="server"
      no-body
      class="mx-auto shadow mb-4"
      style="max-width: 740px;"
    >
      <b-row no-gutters>
        <b-col md="6" class="d-none d-md-block">
          <b-card-img
            :src="
              'https://steamcdn-a.akamaihd.net/steam/apps/' +
                server.game.image +
                '/header.jpg'
            "
            :alt="server.game.name"
          />
        </b-col>
        <b-col md="6">
          <b-card-body class="ml-2 ml-lg-3 py-2 py-lg-3">
            <b-card-title>{{ server.name }}</b-card-title>
            <b-card-text>
              {{ server.game.name }}
              <br />Status:
              <span :class="statusClassMap[server.status]">
                {{ server.status.toLowerCase() }}
                <b-spinner
                  small
                  v-if="
                    server.status == 'STOPPING' || server.status == 'STARTING'
                  "
                />
              </span>
            </b-card-text>
            <div class="flex-column mt-auto">
              <b-button-group class="mr-1">
                <b-button
                  v-if="
                    server.status == 'STARTING' || server.status == 'STOPPED'
                  "
                  :disabled="server.status == 'STARTING'"
                  size="sm"
                  variant="success"
                  >Start</b-button
                >
                <b-button
                  v-if="
                    server.status == 'STOPPING' || server.status == 'STARTED'
                  "
                  :disabled="server.status == 'STOPPING'"
                  size="sm"
                  variant="danger"
                  >Stop</b-button
                >
              </b-button-group>
              <b-button-group class="mr-1">
                <b-button
                  :to="{
                    name: 'game server',
                    params: { sever_name: $route.params.server_name },
                  }"
                  size="sm"
                  variant="outline-secondary"
                  exact-active-class="active"
                  >Status</b-button
                >
                <b-button
                  :to="{
                    name: 'game server settings',
                    params: { sever_name: $route.params.server_name },
                  }"
                  size="sm"
                  variant="outline-secondary"
                  exact-active-class="active"
                  >Settings</b-button
                >
                <b-button
                  :to="{
                    name: 'game server saves',
                    params: { sever_name: $route.params.server_name },
                  }"
                  size="sm"
                  variant="outline-secondary"
                  exact-active-class="active"
                  >Saves</b-button
                >
              </b-button-group>
              <b-button-group class="mr-1">
                <b-button size="sm" variant="outline-danger">Delete</b-button>
              </b-button-group>
            </div>
          </b-card-body>
        </b-col>
      </b-row>
    </b-card>

    <router-view></router-view>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { mapGetters } from "vuex";
import { statusClassMap } from "@/utils/constants";
import { Server } from "@/models/definitions";

@Component({
  computed: mapGetters(["allServers"]),
})
export default class GameServer extends Vue {
  private statusClassMap = statusClassMap;

  allServers!: Server[];

  get server() {
    if (this.allServers) {
      return this.allServers.find(
        (s) => s.name == this.$route.params.server_name
      );
    }
    return undefined;
  }
}
</script>
