<template>
  <div id="label_container">
    <figure v-bind:class="state.class" class="dot" />
    <span v-bind:class="state.class">Server: {{ state.label }}</span>
  </div>
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
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
span {
  font-weight: bold;
}
.dot {
  height: 1em;
  width: 1em;
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
  color: darkgray;
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
    -webkit-transform: scaleY(0.85) scaleX(1.15);
    transform: scaleY(0.85) scaleX(1.15);
  }
  48% {
    -webkit-transform: scaleY(1.2) scaleX(0.8);
    transform: scaleY(1.2) scaleX(0.8);
  }
  68% {
    -webkit-transform: scaleY(0.88) scaleX(1.12);
    transform: scaleY(0.88) scaleX(1.12);
  }
  80% {
    -webkit-transform: scaleY(1.12) scaleX(0.88);
    transform: scaleY(1.12) scaleX(0.88);
  }
  97%,
  100% {
    -webkit-transform: scale(1);
    transform: scale(1);
  }
}
</style>