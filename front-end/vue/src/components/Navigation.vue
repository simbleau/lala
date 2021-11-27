<template>
  <div id="nav_container">
    <div id="nav">
      <router-link to="/">Alarms</router-link> |
      <router-link to="/history">History</router-link> |
      <router-link to="/api">API</router-link> |
      <router-link to="/about">About</router-link>
    </div>
    <div id="status_container">
      <ServerStatusLabel id="server_label" />
    </div>
    <div id="spacer" />
  </div>
</template>

<script>
import ServerStatusLabel from "@/components/ServerStatusLabel.vue";

export default {
  name: "Navigation",
  components: {
    ServerStatusLabel,
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