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
  computed: mapGetters(["serverLookup"]),
})
export default class GameServer extends Vue {
  serverLookup!: (id: number) => Server;

  get server() {
    return this.serverLookup(+this.$route.params.server_id);
  }
}
</script>
