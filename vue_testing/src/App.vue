<template>
  <div id="app">
    <TheNavbar />
    <div class="mt-4">
      <router-view />
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import TheNavbar from "@/components/TheNavbar.vue";

@Component({
  components: {
    TheNavbar
  }
})
export default class App extends Vue {}
</script>
