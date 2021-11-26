<template>
  <Navigation />
  <router-view />
</template>

<script>
import Navigation from "@/components/Navigation.vue";
export default {
  components: {
    Navigation,
  },
  mounted() {
    this.$store.dispatch("fetch_alarms");
  },
};
</script>

<style>
body {
  background-color: #2c3e50;
}

#app {
  font-family: CerebriSans-Regular, -apple-system, system-ui, Roboto, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #42b983;
}
</style>
