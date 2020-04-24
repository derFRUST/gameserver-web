<template>
  <div class="col-lg-4 col-md-6 mt-4 d-flex">
    <div class="card shadow-sm">
      <img :src="server.game.image" class="card-img-top" :alt="server.game.name" />
      <div class="card-body d-flex flex-column">
        <h2>{{server.name}}</h2>
        <p>
          {{server.game.name}}
          <br />Status:
          <span :class="statusClassMap[server.status]">
            {{server.status}}
            <span
              v-if="server.status == 'stopping' || server.status == 'starting'"
              class="spinner-border spinner-border-sm"
            ></span>
          </span>
        </p>
        <div class="mt-auto">
          <div class="btn-group mr-1">
            <button
              v-if="server.status == 'starting' || server.status == 'stopped'"
              :disabled="server.status == 'starting'"
              @click="startServer"
              type="button"
              class="btn btn-sm btn-success"
              role="status"
            >Start</button>
            <button
              v-if="server.status == 'stopping' || server.status == 'started'"
              :disabled="server.status == 'stopping'"
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
  stopped: "text-danger",
  starting: "text-success",
  started: "text-success",
  stopping: "text-danger"
});

@Component
export default class ServerCard extends Vue {
  @Prop({ required: true }) readonly server: Record<string, string>;
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
