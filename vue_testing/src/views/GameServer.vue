<template>
  <b-container>
    <p v-if="!server" class="text-center">Loading server data ...</p>
    <ServerCard
      v-if="server"
      layout="horizontal"
      :server="server"
      :image="
        'https://steamcdn-a.akamaihd.net/steam/apps/' +
          server.game.image +
          '/header.jpg'
      "
    />
    <router-view></router-view>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import { mapGetters } from "vuex";
import ServerCard from "@/components/ServerCard.vue";
import { Server } from "@/models/definitions";

@Component({
  components: {
    ServerCard,
  },
  computed: mapGetters(["allServers"]),
})
export default class GameServer extends Vue {
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
