<template>
  <div>
    <nav class="navbar navbar-expand-md navbar-dark bg-dark">
      <div class="container">
        <a class="navbar-brand" href="#">
          gs.<b>FRUSTserver</b>.de
        </a>
        <button
          class="navbar-toggler"
          type="button"
          data-toggle="collapse"
          data-target="#navbar"
          aria-controls="navbar"
          aria-expanded="false"
          aria-label="Toggle navigation"
        >
          <span class="navbar-toggler-icon"></span>
        </button>

        <div class="collapse navbar-collapse" id="navbar">
          <ul class="navbar-nav mr-auto">
            <li class="nav-item dropdown">
              <router-link
                to="/game-servers"
                class="nav-link dropdown-toggle"
                id="dropdown_servers"
                data-toggle="dropdown"
                aria-haspopup="true"
                aria-expanded="false"
                exact
                exact-active-class="active"
              >Game servers</router-link>
              <div class="dropdown-menu" aria-labelledby="dropdown_servers">
                <a class="dropdown-item" href="#">factorio-01 (Factorio Experimental 0.18.18)</a>
                <a class="dropdown-item" href="#">satisfactory-01 (Satisfactory Early Access)</a>
                <a class="dropdown-item" href="#">Create new server</a>
              </div>
            </li>

            <li class="nav-item">
              <a class="nav-link disabled" href="#">Status</a>
            </li>
            <li class="nav-item">
              <a class="nav-link disabled" href="#">Settings</a>
            </li>
            <li class="nav-item">
              <a class="nav-link disabled" href="#">Saves</a>
            </li>
          </ul>
          <ul class="navbar-nav">
            <li class="nav-item">
              <a class="nav-link" href="#">Users</a>
            </li>
            <li class="nav-item dropdown">
              <a
                class="nav-link dropdown-toggle"
                id="dropdown_user"
                href="#"
                data-toggle="dropdown"
                aria-haspopup="true"
                aria-expanded="false"
              >TheAdmin</a>
              <div class="dropdown-menu" aria-labelledby="dropdown_user">
                <a class="dropdown-item" href="#">Account</a>
                <a class="dropdown-item" href="#">Log out</a>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </nav>
  </div>
</template>
