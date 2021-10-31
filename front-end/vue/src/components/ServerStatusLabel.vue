<template>
  <span class="server_label_container">
    <span v-bind:class="state.class" class="dot" />
    <span v-bind:class="state.class">{{ state.label }}</span>
  </span>
</template>

<script>
export const SERVER_STATE = {
  QUERYING: { label: "Querying...", class: "querying" },
  REACHABLE: { label: "Online", class: "reachable" },
  UNREACHABLE: { label: "Offline", class: "unreachable" },
};
export const QUERY_ADDRESS = "https://imbleau.com/lala/get.php?who=Lala";
export default {
  name: "ServerStatusLabel",
  props: {
    state: SERVER_STATE,
  },
  mounted() {
    this.$nextTick(function () {
      console.log("Rendered");
    });
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.server_label_container {
  display: inline-flex;
  column-gap: 10px;
  justify-content: space-between;
  align-items: center;
}
.dot {
  height: 30px;
  width: 30px;
  background-color: #bbb;
  border-radius: 50%;
  display: inline-block;
}
span.querying {
  color: yellow;
}
.dot.querying {
  background-color: yellow;
}
span.reachable {
  color: green;
}
.dot.reachable {
  background-color: green;
}
span.unreachable {
  color: red;
}
.dot.unreachable {
  background-color: red;
}
</style>