<template>
  <div id="label_container">
    <figure v-bind:class="state.class" class="dot" />
    <span v-bind:class="state.class">Server: {{ state.label }}</span>
    <template v-if="state == SERVER_STATE.REACHABLE">
      <span>|</span>
      <AlarmCountLabel id="alarm_label" v-bind:count="alarm_count" />
    </template>
  </div>
</template>

<script>
import AlarmCountLabel from "@/components/AlarmCountLabel.vue";
import { mapState } from "vuex";

export const SERVER_STATE = {
  QUERYING: { label: "Fetching...", class: "querying" },
  REACHABLE: { label: "Online", class: "reachable" },
  UNREACHABLE: { label: "Unreachable", class: "unreachable" },
};
export default {
  name: "ServerStatusLabel",
  data() {
    return {
      SERVER_STATE,
      state: SERVER_STATE.QUERYING,
      alarm_count: 0,
    };
  },
  components: {
    AlarmCountLabel,
  },
  computed: mapState(["alarms"]),
  watch: {
    alarms(alarms) {
      this.alarm_count = alarms.length;
    },
  },
  created() {
    this.unsubscribe = this.$store.subscribe((mutation, state) => {
      if (mutation.type === "set_reachable") {
        if (state.reachable) {
          this.state = SERVER_STATE.REACHABLE;
        } else {
          this.state = SERVER_STATE.UNREACHABLE;
        }
      }
    });
  },
  beforeUnmount() {
    this.unsubscribe();
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
  animation: query-anim 0.5s ease-out infinite;
}
span.reachable {
  color: #42b983;
}
.dot.reachable {
  background-color: #42b983;
}
span.unreachable {
  color: #b94242;
}
.dot.unreachable {
  background-color: #b94242;
}

@keyframes query-anim {
  0% {
    -webkit-transform: scale(1);
    transform: scale(1);
  }
  33% {
    -webkit-transform: scaleY(0.85) scaleX(1.15);
    transform: scaleY(0.85) scaleX(1.15);
  }
  68% {
    -webkit-transform: scaleY(1.15) scaleX(0.85);
    transform: scaleY(1.15) scaleX(0.85);
  }
  100% {
    -webkit-transform: scale(1);
    transform: scale(1);
  }
}
</style>