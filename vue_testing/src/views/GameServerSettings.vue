<template>
  <b-container>
    <div class="text-center mb-4">
      <h1 class="display-4">Game server settings</h1>
    </div>

    <b-form @submit="onSubmit">
      <b-row class="justify-content-center">
        <b-col cols="4" class="text-right"
          ><label for="inline-form-input-name">Server name:</label></b-col
        >
        <b-col cols="4">
          <b-input
            id="server-name"
            v-model="name"
            class="mb-2 mr-sm-2 mb-sm-0"
          ></b-input>
        </b-col>
      </b-row>
      <b-row class="justify-content-center">
        <b-col cols="4" class="text-right"
          ><label for="inline-form-input-name">Game ID:</label></b-col
        >
        <b-col cols="4">
          <b-input
            id="server-game-id"
            v-model="gameId"
            class="mb-2 mr-sm-2 mb-sm-0"
          ></b-input>
        </b-col>
      </b-row>
      <b-row class="justify-content-center mt-4">
        <b-col cols="1">
          <b-button type="submit" variant="primary">Save</b-button>
        </b-col>
      </b-row>
    </b-form>
  </b-container>
</template>

<script lang="ts">
import { Component, Vue, Prop, Watch } from "vue-property-decorator";
import { Server, ServerUpdate } from "@/models/definitions";

@Component
export default class GameServerSettings extends Vue {
  @Prop() readonly server!: Server;
  private name = "";
  private gameId = 0;

  private onSubmit(): void {
    const serverUpdate: ServerUpdate = {
      id: this.server.id,
      name: this.name,
      gameId: this.gameId,
    };
    this.$store.dispatch("saveServer", { apollo: this.$apollo, serverUpdate });
  }

  private initForm() {
    if (this.server) {
      this.name = this.server.name;
      this.gameId = this.server.game.id;
    }
  }

  private mounted(): void {
    this.initForm();
  }

  @Watch("server")
  private onServerChange(/* to: Server, from: Server */) {
    this.initForm();
  }
}
</script>
