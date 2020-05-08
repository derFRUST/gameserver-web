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
  private onSubmit(): void {
    const serverUpdate: ServerUpdate = {
      id: this.server.id,
      name: this.name,
      gameId: this.server.game.id,
    };
    this.$store.dispatch("saveServer", serverUpdate);
  }

  private initForm() {
    if (this.server) {
      this.name = this.server.name;
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
