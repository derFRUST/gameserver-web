<template>
  <b-container>
    <div class="text-center">
      <h1 class="display-4">Game servers</h1>
      <p class="lead">You can create new servers and manage your existing ones here.</p>
    </div>

    <ApolloQuery
      :query="gql => gql`
        query {
          servers {
            name
            status
            game {
              name
              image
            }
          }
        }`"
    >
      <!-- The result will automatically updated -->
      <template slot-scope="{ result: { data, loading, error } }">
        <!-- Some content -->
        <div v-if="loading">Loading...</div>
        <div v-if="error">Error!</div>
        <b-row>
          <ServerCard
            @start-server="startServer"
            @stop-server="stopServer"
            :key="server.name"
            :server="server"
            :image="'https://steamcdn-a.akamaihd.net/steam/apps/' + server.game.image + '/header.jpg'"
            v-for="server in data ? data.servers : []"
          />
          <b-col lg="4" md="6" class="mt-4 d-flex">
            <b-card
              :img-src="require('../assets/servers_create.png')"
              img-alt="Create"
              no-body
              class="shadow"
            >
              <b-card-body class="d-flex flex-column">
                <b-card-title>Create new server</b-card-title>
                <div class="flex-column mt-auto">
                  <b-button variant="primary" size="sm">Create</b-button>
                </div>
              </b-card-body>
            </b-card>
          </b-col>
        </b-row>
      </template>
    </ApolloQuery>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import ServerCard from "@/components/ServerCard.vue";

@Component({
  components: {
    ServerCard
  }
})
export default class GameServers extends Vue {
  private startServer(server: Record<string, string>): void {
    // TODO: Replace by API communication
    console.log("Start server: " + server.name);
    server.status = "STARTING";
    setTimeout(() => {
      server.status = "STARTED";
    }, 3000);
  }
  private stopServer(server: Record<string, string>): void {
    // TODO: Replaced by API communication
    console.log("Stop server: " + server.name);
    server.status = "STOPPING";
    setTimeout(() => {
      server.status = "STOPPED";
    }, 3000);
  }
}
</script>
