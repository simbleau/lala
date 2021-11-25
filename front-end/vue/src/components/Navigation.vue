<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/history">History</router-link> |
    <router-link to="/api">API</router-link> |
    <router-link to="/about">About</router-link>
  </div>
  <div id="status_container">
    <ServerStatusLabel id="server_label" v-bind:state="server_state" />
    <template v-if="server_state == SERVER_STATE.REACHABLE">
      <span>|</span>
      <AlarmCountLabel id="alarm_label" v-bind:count="alarm_count" />
    </template>
  </div>
  <div id="spacer" />
</template>

<script>
import ServerStatusLabel from "@/components/ServerStatusLabel.vue";
import { SERVER_STATE } from "@/components/ServerStatusLabel.vue";
import AlarmCountLabel from "@/components/AlarmCountLabel.vue";

export default {
  name: "Navigation",
  data() {
    return {
      SERVER_STATE,
      server_state: SERVER_STATE.QUERYING,
      alarm_count: 0,
    };
  },
  components: {
    ServerStatusLabel,
    AlarmCountLabel,
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
      await this.sleep(5000);
      this.server_state = SERVER_STATE.REACHABLE;
      this.alarm_count = 0;
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