import Vue from "vue";
import { Action, Module, Mutation, VuexModule } from "vuex-module-decorators";
import DollarApollo from "vue-apollo";
import gql from "graphql-tag";
import { Server, ServerUpdate } from "@/models/definitions";

@Module
export default class Servers extends VuexModule {
  servers: Server[] = [];

  @Mutation
  setServers(servers: Server[]) {
    this.servers = servers;
  }

  @Mutation
  updateServer(server: Server) {
    if (server) {
      const serverIndex = this.servers.findIndex((s) => s.id == server.id);
      Vue.set(this.servers, serverIndex, server);
    }
  }

  @Action({ commit: "setServers" })
  async fetchServers(apollo: DollarApollo) {
    let result = [];
    try {
      const response = await apollo.query({
        query: gql`
          query Servers {
            servers {
              id
              name
              game {
                id
                name
                image
              }
              status
            }
          }
        `,
      });
      result = response.data.servers;
    } catch (e) {
      console.log(e);
    }
    return result;
  }

  @Action({ commit: "updateServer" })
  async saveServer({
    apollo,
    serverUpdate,
  }: {
    apollo: DollarApollo;
    serverUpdate: ServerUpdate;
  }) {
    try {
      const response = await apollo.mutate({
        mutation: gql`
          mutation rename {
            server: updateServer(
              serverUpdate: { id: ${serverUpdate.id}, name: "${serverUpdate.name}", gameId: ${serverUpdate.gameId} }
            ) {
              id
              name
              game {
                id
                name
                image
              }
              status
            }
          }
        `,
      });
      return response.data.server;
    } catch (e) {
      console.log(e);
      return undefined;
    }
  }

  get allServers() {
    return this.servers;
  }
  get serverLookup() {
    return (id: number) => this.allServers.find((s) => s.id == id);
  }
}
