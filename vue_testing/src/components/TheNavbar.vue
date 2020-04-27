<template>
  <div>
    <b-navbar toggleable="md" type="dark" variant="dark">
      <b-container>
        <b-navbar-brand href="#">
          gs.<b>FRUSTserver</b>.de
        </b-navbar-brand>

        <b-navbar-toggle target="navbar"></b-navbar-toggle>

        <b-collapse id="navbar" is-nav>
          <b-navbar-nav>
            <b-nav-item-dropdown text="Game servers">
              <b-dropdown-item href="#">factorio-01 (Factorio Experimental 0.18.18)</b-dropdown-item>
              <b-dropdown-item href="#">satisfactory-01 (Satisfactory Early Access)</b-dropdown-item>
              <b-dropdown-item href="#">Create new server</b-dropdown-item>
            </b-nav-item-dropdown>

            <b-nav-item href="#" disabled>Status</b-nav-item>
            <b-nav-item href="#" disabled>Settings</b-nav-item>
            <b-nav-item href="#" disabled>Saves</b-nav-item>
          </b-navbar-nav>
          <b-navbar-nav class="ml-auto">
            <b-nav-item href>Users</b-nav-item>
            <b-nav-item-dropdown text="TheAdmin">
              <b-dropdown-item href="#">Account</b-dropdown-item>
              <b-dropdown-item href="#">Log out</b-dropdown-item>
            </b-nav-item-dropdown>
          </b-navbar-nav>
        </b-collapse>
      </b-container>
    </b-navbar>
  </div>
</template>
