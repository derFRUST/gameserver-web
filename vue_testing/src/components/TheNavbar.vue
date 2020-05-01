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
              v-if="$route.params.server_name"
              :class="$route.name == 'game server' ? 'active' : ''"
              :text="$route.params.server_name"
            >
              <ApolloQuery
                :query="
                  (gql) => gql`
                    query {
                      servers {
                        name
                      }
                    }
                  `
                "
              >
                <!-- The result will automatically updated -->
                <template slot-scope="{ result: { data, loading, error } }">
                  <!-- Some content -->
                  <div v-if="loading">Loading...</div>
                  <div v-if="error">Error!</div>
                  <b-dropdown-item
                    v-for="server in data ? data.servers : []"
                    :key="server.name"
                    :to="{
                      name: $route.name,
                      params: { server_name: server.name },
                    }"
                    active-class="active"
                    >{{ server.name }}</b-dropdown-item
                  >
                </template>
              </ApolloQuery>
            </b-nav-item-dropdown>
            <b-nav-item-dropdown
              v-if="Object.keys(details).includes($route.name)"
              class="active"
              :text="displayName"
            >
              <b-dropdown-item
                v-for="[d_key, d_value] of Object.entries(details)"
                :key="d_key"
                :to="{
                  name: d_key,
                  params: { server_name: $route.params.server_name },
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

@Component
export default class TheNavbar extends Vue {
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
}
</script>
