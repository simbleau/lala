<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/history">History</router-link> |
    <router-link to="/api">API</router-link> |
    <router-link to="/about">About</router-link>
  </div>
  <div id="status_container">
    <ServerStatusLabel id="server_label" v-bind:state="server_state" /> |
    <ClientStatusLabel
      id="client_label"
      v-bind:state="client_state"
      v-bind:count="client_count"
    />
  </div>
  <div id="spacer" />
</template>

<script>
import ServerStatusLabel from "@/components/ServerStatusLabel.vue";
import { SERVER_STATE } from "@/components/ServerStatusLabel.vue";
import ClientStatusLabel from "@/components/ClientStatusLabel.vue";
import { CLIENT_STATE } from "@/components/ClientStatusLabel.vue";

export default {
  name: "Navigation",
  data() {
    return {
      server_state: SERVER_STATE.QUERYING,
      client_state: CLIENT_STATE.QUERYING,
      client_count: "Fetching...",
    };
  },
  components: {
    ServerStatusLabel,
    ClientStatusLabel,
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
      this.client_state = CLIENT_STATE.QUERYING;
      this.client_count = "Fetching...";
      await this.sleep(2000);
      this.server_state = SERVER_STATE.REACHABLE;
      this.client_state = CLIENT_STATE.UNREACHABLE;
      this.client_count = "0";
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

#server_label,
#client_label,
#status_container {
  display: inline-flex;
  column-gap: 10px;
  justify-content: space-between;
  align-items: center;
}

#spacer {
  margin-bottom: 1cm;
}
</style>