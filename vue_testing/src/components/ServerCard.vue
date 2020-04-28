<template>
  <b-col lg="4" md="6" class="mt-4 d-flex">
    <b-card :img-src="image" :img-alt="server.game.name" no-body class="shadow">
      <b-card-body class="d-flex flex-column">
        <b-card-title>{{server.name}}</b-card-title>
        <b-card-text>
          {{server.game.name}}
          <br />Status:
          <span :class="statusClassMap[server.status]">
            {{server.status.toLowerCase()}}
            <b-spinner small v-if="server.status == 'STOPPING' || server.status == 'STARTING'" />
          </span>
        </b-card-text>
        <div class="flex-column mt-auto">
          <b-button-group class="mr-1">
            <b-button
              v-if="server.status == 'STARTING' || server.status == 'STOPPED'"
              :disabled="server.status == 'STARTING'"
              @click="startServer"
              size="sm"
              variant="success"
            >Start</b-button>
            <b-button
              v-if="server.status == 'STOPPING' || server.status == 'STARTED'"
              :disabled="server.status == 'STOPPING'"
              @click="stopServer"
              size="sm"
              variant="danger"
            >Stop</b-button>
          </b-button-group>
          <b-button-group class="mr-1">
            <b-button
              size="sm"
              variant="outline-secondary"
              :to="'/game-servers/' + server.name + '/status'"
            >Status</b-button>
            <b-button size="sm" variant="outline-secondary">Settings</b-button>
            <b-button size="sm" variant="outline-secondary">Saves</b-button>
          </b-button-group>
          <b-button-group class="mr-1">
            <b-button size="sm" variant="outline-danger">Delete</b-button>
          </b-button-group>
        </div>
      </b-card-body>
    </b-card>
  </b-col>
</template>

<script lang="ts">
import { Component, Prop, Vue, Emit } from "vue-property-decorator";

const statusClassMap = Object.freeze({
  STOPPED: "text-danger",
  STARTING: "text-success",
  STARTED: "text-success",
  STOPPING: "text-danger"
});

@Component
export default class ServerCard extends Vue {
  @Prop({ required: true }) readonly server!: Record<string, string>;
  @Prop({ required: true }) readonly image!: string;
  @Emit()
  private startServer(): Record<string, string> {
    return this.server;
  }
  @Emit()
  private stopServer(): Record<string, string> {
    return this.server;
  }
  private statusClassMap = statusClassMap;
}
</script>
