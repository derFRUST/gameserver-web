import Vue from "vue";
import { Action, Module, Mutation, VuexModule } from "vuex-module-decorators";
import DollarApollo from "vue-apollo";
import gql from "graphql-tag";
import { Server, UpdateServerInput, ServerPayload } from "@/models/definitions";

@Module
export default class Servers extends VuexModule {
  servers: Server[] = [];

  @Mutation
  setServers(servers: Server[]) {
    this.servers = servers;
  }

  @Mutation
  updateServer(serverPayload: ServerPayload) {
    const serverIndex = this.servers.findIndex(
      (s) => s.id == serverPayload.server.id
    );
    Vue.set(this.servers, serverIndex, serverPayload.server);
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
    input,
  }: {
    apollo: DollarApollo;
    input: UpdateServerInput;
  }) {
    try {
      const response = await apollo.mutate({
        mutation: gql`
          mutation {
            updateServer(
              input: { id: ${input.id}, name: "${input.name}", gameId: ${input.gameId} }
            ) {
              server {
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
          }
        `,
      });
      return response.data.updateServer;
    } catch (e) {
      console.log(e);
      return undefined;
    }
  }

  get allServers() {
    return this.servers;
  }
  get serverLookup() {
    return (id: string) => this.allServers.find((s) => s.id == id);
  }
}
