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
  updateServer(serverUpdate: ServerUpdate) {
    const serverIndex = this.servers.findIndex((s) => s.id == serverUpdate.id);
    this.servers[serverIndex].name = serverUpdate.name;
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
  saveServer(serverUpdate: ServerUpdate) {
    console.log("updateServer", serverUpdate);
    return serverUpdate;
  }

  get allServers() {
    return this.servers;
  }
  get serverLookup() {
    return (id: number) => this.allServers.find((s) => s.id == id);
  }
}
