<template>
  <div class="container">
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
        <div class="row">
          <ServerCard
            @start-server="startServer"
            @stop-server="stopServer"
            :key="server.name"
            :server="server"
            :image="'https://steamcdn-a.akamaihd.net/steam/apps/' + server.game.image + '/header.jpg'"
            v-for="server in data ? data.servers : []"
          />
          <div class="col-lg-4 col-md-6 mt-4 d-flex">
            <div class="card shadow-sm">
              <img src="../assets/servers_create.png" class="card-img-top" alt="Create" />
              <div class="card-body d-flex flex-column">
                <div class="card-text">
                  <h2>Create new server</h2>
                </div>
                <div class="mt-auto">
                  <button type="button" class="btn btn-sm btn-primary">Create</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </ApolloQuery>
  </div>
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
