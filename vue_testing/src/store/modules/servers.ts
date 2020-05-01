import gql from "graphql-tag";
import { Action, Module, Mutation, VuexModule } from "vuex-module-decorators";
import { Server } from "@/models/definitions";

@Module
export default class Servers extends VuexModule {
  servers: Server[] = [];

  @Mutation
  setServers(servers: Server[]) {
    this.servers = servers;
  }

  @Action({ commit: "setServers" })
  async fetchServers(apollo: any) {
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

  get allServers() {
    return this.servers;
  }
}
