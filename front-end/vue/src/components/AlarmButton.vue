<template>
  <div id="alarm_button_container">
    <button
      v-bind:class="state.class"
      v-bind:disabled="state.disabled"
      v-on:click="call"
      class="button-33"
    >
      {{ state.label }}
      <div id="addr_attachment">
        <p>{{ this.alarm_addr }}</p>
      </div>
    </button>
    <h2>{{ this.info }}</h2>
  </div>
</template>

<script>
const BUTTON_STATE = {
  ON: { label: "Silence", class: "failed", disabled: false },
  OFF: { label: "Signal", class: "ready", disabled: false },
  LOADING: { label: "Loading...", class: "loading", disabled: true },
  FAILED: { label: "Failed. Try again?", class: "failed", disabled: true },
};
export default {
  name: "AlarmButton",
  data() {
    return {
      on: false,
      state: BUTTON_STATE.OFF,
      info: "",
      timeout: 5000, // in ms
      state_change: 3000, // in ms
    };
  },
  props: {
    alarm_addr: String,
  },
  methods: {
    wait: function (ms) {
      return new Promise((resolve) => setTimeout(resolve, ms));
    },
    call: async function () {
      var uri;
      if (this.on) {
        uri = this.$store.getters.server + "/silence?server=" + this.alarm_addr;
      } else {
        uri = this.$store.getters.server + "/signal?server=" + this.alarm_addr;
      }
      // Set to loading state
      var success = false;
      this.state = BUTTON_STATE.LOADING;
      // Perform request
      await this.axios
        .get(uri, {
          timeout: this.timeout,
        })
        .then((response) => {
          this.info = response.data;
          if (response.data == "on") {
            this.state = BUTTON_STATE.ON;
            this.on = true;
          } else if (response.data == "off") {
            this.state = BUTTON_STATE.OFF;
            this.on = false;
          } else {
            const error = new Error(response.statusText);
            throw error;
          }
          success = true;
        })
        .catch((err) => {
          console.log(err.code + ": " + err.message + "\n" + err.stack);
          this.state = BUTTON_STATE.FAILED;
          success = false;
        });
      // Handle result
      if (success == false) {
        await this.wait(this.state_change);
        this.state = BUTTON_STATE.OFF;
      }
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#addr_attachment {
  position: relative;
}
p {
  position: absolute;
  margin: auto;
  width: 100%;
  font-size: 0.5em;
}
.button-33 {
  width: 250px;
  height: 75px;
  border-radius: 100px;
  cursor: pointer;
  display: inline-block;
  padding: 7px 20px;
  text-align: center;
  text-decoration: none;
  transition: all 250ms;
  border: 0;
  font-size: 20px;
  user-select: none;
  -webkit-user-select: none;
  touch-action: manipulation;
}

.ready {
  background-color: #fbecc2;
  box-shadow: rgba(235, 179, 27, 0.2) 0 -25px 18px -14px inset,
    rgba(235, 179, 27, 0.15) 0 1px 2px, rgba(235, 179, 27, 0.15) 0 2px 4px,
    rgba(235, 179, 27, 0.15) 0 4px 8px, rgba(235, 179, 27, 0.15) 0 8px 16px,
    rgba(235, 179, 27, 0.15) 0 16px 32px;
  color: goldenrod;
}
.ready:hover {
  transform: scale(1.25) rotate(-1deg);
  box-shadow: rgba(235, 179, 27, 0.35) 0 -25px 18px -14px inset,
    rgba(235, 179, 27, 0.25) 0 1px 2px, rgba(235, 179, 27, 0.25) 0 2px 4px,
    rgba(235, 179, 27, 0.25) 0 4px 8px, rgba(235, 179, 27, 0.25) 0 8px 16px,
    rgba(235, 179, 27, 0.25) 0 16px 32px;
}
.loading {
  background-color: #c2c2c2;
  box-shadow: rgba(44, 44, 44, 0.2) 0 -25px 18px -14px inset,
    rgba(44, 44, 44, 0.15) 0 1px 2px, rgba(44, 44, 44, 0.15) 0 2px 4px,
    rgba(44, 44, 44, 0.15) 0 4px 8px, rgba(44, 44, 44, 0.15) 0 8px 16px,
    rgba(44, 44, 44, 0.15) 0 16px 32px;
  color: grey;
}
.success {
  background-color: #c2fbd7;
  box-shadow: rgba(44, 187, 99, 0.2) 0 -25px 18px -14px inset,
    rgba(44, 187, 99, 0.15) 0 1px 2px, rgba(44, 187, 99, 0.15) 0 2px 4px,
    rgba(44, 187, 99, 0.15) 0 4px 8px, rgba(44, 187, 99, 0.15) 0 8px 16px,
    rgba(44, 187, 99, 0.15) 0 16px 32px;
  color: green;
}
.failed {
  background-color: #fbc2c2;
  box-shadow: rgba(187, 44, 44, 0.2) 0 -25px 18px -14px inset,
    rgba(187, 44, 44, 0.15) 0 1px 2px, rgba(187, 44, 44, 0.15) 0 2px 4px,
    rgba(187, 44, 44, 0.15) 0 4px 8px, rgba(187, 44, 44, 0.15) 0 8px 16px,
    rgba(187, 44, 44, 0.15) 0 16px 32px;
  color: red;
}
</style>