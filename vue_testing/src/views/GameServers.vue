<template>
  <div class="container">
    <div class="text-center">
      <h1 class="display-4">Game servers</h1>
      <p class="lead">You can create new servers and manage your existing ones here.</p>
    </div>

    <div class="row">
      <ServerCard
        @start-server="startServer"
        @stop-server="stopServer"
        :key="server.name"
        :server="server"
        v-for="server in servers"
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
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import ServerCard from "@/components/ServerCard.vue";

const games = {
  factorio: {
    name: "Factorio Experimental 0.18.18",
    image: "https://steamcdn-a.akamaihd.net/steam/apps/427520/header.jpg"
  },
  satisfactory: {
    name: "Satisfactory Early Access",
    image: "https://steamcdn-a.akamaihd.net/steam/apps/526870/header.jpg"
  }
};

@Component({
  components: {
    ServerCard
  }
})
export default class GameServers extends Vue {
  private servers: Array<object> = [
    {
      name: "factorio-01",
      game: games.factorio,
      status: "stopped"
    },
    {
      name: "satisfactory-01",
      game: games.satisfactory,
      status: "started"
    }
  ];
  private startServer(server): void {
    // TODO: Replace by API communication
    console.log("Start server: " + server.name);
    server.status = "starting";
    setTimeout(() => {
      server.status = "started";
    }, 3000);
  }
  private stopServer(server): void {
    // TODO: Replaced by API communication
    console.log("Stop server: " + server.name);
    server.status = "stopping";
    setTimeout(() => {
      server.status = "stopped";
    }, 3000);
  }
}
</script>
