<template>
  <b-container>
    <b-card no-body class="mx-auto shadow mb-4" style="max-width: 740px;">
      <b-row no-gutters>
        <b-col md="6" class="d-none d-md-block">
          <b-card-img
            src="https://steamcdn-a.akamaihd.net/steam/apps/427520/header.jpg"
            alt="Factorio"
          />
        </b-col>
        <b-col md="6">
          <b-card-body class="ml-2 ml-lg-3 py-2 py-lg-3">
            <b-card-title>{{$route.params.server_name}}</b-card-title>
            <b-card-text>
              Factorio Experimental 0.18.18
              <br />Status:
              <span class="text-success">running</span>
            </b-card-text>
            <div class="flex-column mt-auto">
              <b-button-group class="mr-1">
                <b-button size="sm" variant="danger">Stop</b-button>
              </b-button-group>
              <b-button-group class="mr-1">
                <b-button size="sm" variant="outline-secondary" pressed>Status</b-button>
                <b-button size="sm" variant="outline-secondary">Settings</b-button>
                <b-button size="sm" variant="outline-secondary">Saves</b-button>
              </b-button-group>
              <b-button-group class="mr-1">
                <b-button size="sm" variant="outline-danger">Delete</b-button>
              </b-button-group>
            </div>
          </b-card-body>
        </b-col>
      </b-row>
    </b-card>

    <router-view></router-view>
  </b-container>
</template>
