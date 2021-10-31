<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link> |
    <router-link to="/history">History</router-link> |
    <router-link to="/api">API</router-link>
  </div>
  <ServerStatusLabel id="server_label" v-bind:state="server_state" />
</template>

<script>
import ServerStatusLabel from "@/components/ServerStatusLabel.vue";
import { SERVER_STATE } from "@/components/ServerStatusLabel.vue";

export default {
  name: "Navigation",
  data() {
    return {
      server_state: SERVER_STATE.QUERYING,
    };
  },
  components: {
    ServerStatusLabel,
  },
  mounted() {
    this.$nextTick(this.initialize);
  },
  methods: {
    sleep: function (ms) {
      // Expensive function
      return new Promise((resolve) => setTimeout(resolve, ms));
    },
    initialize: async function () {
      this.server_state = SERVER_STATE.QUERYING;
      await this.sleep(2000);
      this.server_state = SERVER_STATE.REACHABLE;
    },
  },
};
</script>

<style>
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