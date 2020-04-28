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
            <b-nav-item to="/game-servers" active-class="active">Game servers</b-nav-item>
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
