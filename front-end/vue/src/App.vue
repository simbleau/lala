<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link> |
    <router-link to="/history">History</router-link> |
    <router-link to="/api">API</router-link>
  </div>
  <ServerStatusLabel id="server_label" v-bind:state="server_state" />
  <router-view />
</template>

<script>
import ServerStatusLabel from "@/components/ServerStatusLabel.vue";
import { SERVER_STATE } from "@/components/ServerStatusLabel.vue";

export default {
  data() {
    return {
      server_state: SERVER_STATE.QUERYING,
    };
  },
  components: {
    ServerStatusLabel,
  },
};
</script>

<style>
body {
  background-color: #2c3e50;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #42b983;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #c0d6ec;
}

#nav a.router-link-exact-active {
  color: #42b983;
}
</style>
