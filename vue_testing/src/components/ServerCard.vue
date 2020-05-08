<template>
  <b-card no-body class="mx-auto shadow mb-4" style="max-width: 740px;">
    <b-row no-gutters>
      <b-col
        :md="layout == 'horizontal' ? 6 : 12"
        :class="layout == 'horizontal' ? 'd-none d-md-block' : ''"
      >
        <b-card-img
          v-if="!create"
          :src="
            'https://steamcdn-a.akamaihd.net/steam/apps/' +
            server.game.image +
            '/header.jpg'
          "
          :alt="server.game.name"
        />
        <b-card-img
          v-if="create"
          :src="require('../assets/servers_create.png')"
          alt="New server"
        />
      </b-col>
      <b-col :md="layout == 'horizontal' ? 6 : 12">
        <b-card-body
          v-if="!create"
          :class="
            layout == 'horizontal'
              ? 'd-flex flex-column py-md-2 py-lg-3'
              : 'd-flex flex-column p-2 p-xl-3'
          "
        >
          <b-card-title>{{ server.name }}</b-card-title>
          <b-card-text>
            {{ server.game.name }}
            <br />Status:
            <span :class="statusClassMap[server.status]">
              {{ server.status.toLowerCase() }}
              <b-spinner
                small
                v-if="
                  server.status == 'STOPPING' || server.status == 'STARTING'
                "
              />
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
                >Start</b-button
              >
              <b-button
                v-if="server.status == 'STOPPING' || server.status == 'STARTED'"
                :disabled="server.status == 'STOPPING'"
                @click="stopServer"
                size="sm"
                variant="danger"
                >Stop</b-button
              >
            </b-button-group>
            <b-button-group class="mr-1">
              <b-button
                size="sm"
                variant="outline-secondary"
                :to="{
                  name: 'game server',
                  params: { server_id: server.id },
                }"
                exact-active-class="active"
                >Status</b-button
              >
              <b-button
                size="sm"
                variant="outline-secondary"
                :to="{
                  name: 'game server settings',
                  params: { server_id: server.id },
                }"
                exact-active-class="active"
                >Settings</b-button
              >
              <b-button
                size="sm"
                variant="outline-secondary"
                :to="{
                  name: 'game server saves',
                  params: { server_id: server.id },
                }"
                exact-active-class="active"
                >Saves</b-button
              >
            </b-button-group>
            <b-button-group>
              <b-button size="sm" variant="outline-danger">Delete</b-button>
            </b-button-group>
          </div>
        </b-card-body>
        <b-card-body v-if="create" class="d-flex flex-column p-2 p-xl-3">
          <b-card-title>Create new server</b-card-title>
          <b-card-text> <br /><br /> </b-card-text>
          <div class="flex-column mt-auto">
            <b-button variant="primary" size="sm">Create</b-button>
          </div>
        </b-card-body>
      </b-col>
    </b-row>
  </b-card>
</template>

<script lang="ts">
import { Component, Prop, Vue, Emit } from "vue-property-decorator";
import { statusClassMap } from "@/utils/constants";
import { Server } from "@/models/definitions";

@Component
export default class ServerCard extends Vue {
  @Prop({ required: true }) readonly layout!: string;
  @Prop({ default: false }) readonly create!: boolean;
  @Prop({ required: false }) readonly server!: Server;
  @Emit()
  private startServer(): Server {
    return this.server;
  }
  @Emit()
  private stopServer(): Server {
    return this.server;
  }
  private statusClassMap = statusClassMap;
}
</script>
