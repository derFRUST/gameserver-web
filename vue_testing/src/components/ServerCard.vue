<template>
  <div class="col-lg-4 col-md-6 mt-4 d-flex">
    <div class="card shadow-sm">
      <img :src="image" class="card-img-top" :alt="server.game.name" />
      <div class="card-body d-flex flex-column">
        <h2>{{server.name}}</h2>
        <p>
          {{server.game.name}}
          <br />Status:
          <span :class="statusClassMap[server.status]">
            {{server.status.toLowerCase()}}
            <span
              v-if="server.status == 'STOPPING' || server.status == 'STARTING'"
              class="spinner-border spinner-border-sm"
            ></span>
          </span>
        </p>
        <div class="mt-auto">
          <div class="btn-group mr-1">
            <button
              v-if="server.status == 'STARTING' || server.status == 'STOPPED'"
              :disabled="server.status == 'STARTING'"
              @click="startServer"
              type="button"
              class="btn btn-sm btn-success"
              role="status"
            >Start</button>
            <button
              v-if="server.status == 'STOPPING' || server.status == 'STARTED'"
              :disabled="server.status == 'STOPPING'"
              @click="stopServer"
              type="button"
              class="btn btn-sm btn-danger"
            >Stop</button>
          </div>
          <div class="btn-group mr-1">
            <button type="button" class="btn btn-sm btn-outline-secondary">Status</button>
            <button type="button" class="btn btn-sm btn-outline-secondary">Settings</button>
            <button type="button" class="btn btn-sm btn-outline-secondary">Saves</button>
          </div>
          <div class="btn-group mr-1">
            <button type="button" class="btn btn-sm btn-outline-danger">Delete</button>
          </div>
        </div>
      </div>
    </div>
  </div>
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
