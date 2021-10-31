<template>
  <span class="server_label_container">
    <figure v-bind:class="state.class" class="dot" />
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
span {
  font-weight: bold;
}
.dot {
  height: 30px;
  width: 30px;
  border-width: 2px;
  border-color: rgba(255, 255, 255, 0.4);
  border-style: solid;
  border-radius: 50%;
  display: inline-block;
  position: relative;
  margin: 0;
}
.dot::after {
  background-color: rgba(255, 255, 255, 0.3);
  content: "";
  height: 45%;
  width: 25%;
  position: absolute;
  top: 5%;
  left: 15%;
  border-radius: 50%;
  transform: rotate(40deg);
}
span.querying {
  color: gray;
  font-style: italic;
}
.dot.querying {
  background-color: grey;
  animation: query-anim 2s ease-out infinite;
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

@keyframes query-anim {
  0% {
    -webkit-transform: scale(1);
    transform: scale(1);
  }
  20% {
    -webkit-transform: scaleY(0.95) scaleX(1.05);
    transform: scaleY(0.95) scaleX(1.05);
  }
  48% {
    -webkit-transform: scaleY(1.1) scaleX(0.9);
    transform: scaleY(1.1) scaleX(0.9);
  }
  68% {
    -webkit-transform: scaleY(0.98) scaleX(1.02);
    transform: scaleY(0.98) scaleX(1.02);
  }
  80% {
    -webkit-transform: scaleY(1.02) scaleX(0.98);
    transform: scaleY(1.02) scaleX(0.98);
  }
  97%,
  100% {
    -webkit-transform: scale(1);
    transform: scale(1);
  }
}
</style>