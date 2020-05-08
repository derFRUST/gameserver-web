<template>
  <div>
    <b-navbar toggleable="md" type="dark" variant="dark">
      <b-container>
        <b-navbar-brand href="#">
          gs.<b>FRUSTserver</b>.de
        </b-navbar-brand>

        <b-navbar-toggle target="navbar"></b-navbar-toggle>

        <b-collapse id="navbar" is-nav>
          <b-navbar-nav>
            <b-nav-item
              :to="{ name: 'game servers' }"
              exact-active-class="active"
              >Game servers</b-nav-item
            >
            <b-nav-item-dropdown
              v-if="server"
              :class="$route.name == 'game server' ? 'active' : ''"
              :text="server.name"
            >
              <b-dropdown-item
                v-for="server in allServers"
                :key="server.name"
                :to="{
                  name: $route.name,
                  params: { server_id: server.id },
                }"
                active-class="active"
                >{{ server.name }}</b-dropdown-item
              >
            </b-nav-item-dropdown>
            <b-nav-item-dropdown
              v-if="server && Object.keys(details).includes($route.name)"
              class="active"
              :text="displayName"
            >
              <b-dropdown-item
                v-for="[d_key, d_value] of Object.entries(details)"
                :key="d_key"
                :to="{
                  name: d_key,
                  params: { server_id: $route.params.server_id },
                }"
                exact-active-class="active"
                >{{ d_value }}</b-dropdown-item
              >
            </b-nav-item-dropdown>
          </b-navbar-nav>
          <b-navbar-nav class="ml-auto">
            <b-nav-item href>Users</b-nav-item>
            <b-nav-item-dropdown href="test" text="TheAdmin">
              <b-dropdown-item href="#">Account</b-dropdown-item>
              <b-dropdown-item href="#">Log out</b-dropdown-item>
            </b-nav-item-dropdown>
          </b-navbar-nav>
        </b-collapse>
      </b-container>
    </b-navbar>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { mapGetters } from "vuex";
import { Server } from "@/models/definitions";

@Component({
  computed: mapGetters(["allServers", "serverLookup"]),
})
export default class TheNavbar extends Vue {
  serverLookup!: (id: number) => Server;
  private details: { [name: string]: string } = {
    "game server settings": "Settings",
    "game server saves": "Saves",
  };
  get displayName() {
    if (this.$route.name) {
      return this.details[this.$route.name];
    }
    return "";
  }
  get server() {
    return this.serverLookup(+this.$route.params.server_id);
  }
}
</script>
