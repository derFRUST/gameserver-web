<template>
  <div class="col-lg-4 col-md-6 mt-4 d-flex">
    <div class="card shadow-sm">
      <img :src="server.game.image" class="card-img-top" :alt="server.game.name" />
      <div class="card-body d-flex flex-column">
        <h2>{{server.name}}</h2>
        <p>
          {{server.game.name}}
          <br />Status:
          <span :class="statusClassMap[server.status]">{{server.status}}</span>
        </p>
        <div class="mt-auto">
          <div class="btn-group mr-1">
            <button
              v-if="server.status == 'stopped'"
              type="button"
              class="btn btn-sm btn-success"
            >Start</button>
            <button
              v-if="server.status == 'started'"
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

<script>
const statusClassMap = Object.freeze({
  stopped: "text-danger",
  starting: "text-warning",
  started: "text-success",
  stopping: "text-warning"
});

export default {
  name: "ServerCard",
  props: {
    server: Object
  },
  data() {
    return {
      statusClassMap
    };
  }
};
</script>