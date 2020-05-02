<template>
  <b-container>
    <div class="text-center">
      <h1 class="display-4">Game servers</h1>
      <p class="lead">
        You can create new servers and manage your existing ones here.
      </p>
    </div>

    <b-row>
      <b-col
        lg="4"
        md="6"
        class="d-flex mt-2 mt-lg-0 mt-xl-2 px-lg-1 px-xl-3"
        :key="server.name"
        v-for="server in allServers"
      >
        <ServerCard
          @start-server="startServer"
          @stop-server="stopServer"
          layout="vertical"
          :server="server"
        />
      </b-col>
      <b-col lg="4" md="6" class="d-flex mt-2 mt-lg-0 mt-xl-2 px-lg-1 px-xl-3">
        <ServerCard
          layout="vertical"
          create="true"
        />
      </b-col>
    </b-row>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { mapGetters } from "vuex";
import ServerCard from "@/components/ServerCard.vue";

@Component({
  components: {
    ServerCard,
  },
  computed: mapGetters(["allServers"]),
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
