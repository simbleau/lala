<template>
  <button
    v-bind:class="state.class"
    v-bind:disabled="state.disabled"
    v-on:click="call"
    class="button-33"
  >
    {{ state.label }}
  </button>
  <h2>{{ this.info }}</h2>
</template>

<script>
export const QUERY_ADDRESS = "http://imbleau.com/lala/get.php?who=Lala";
export const BUTTON_STATE = {
  READY: { label: "Alarm!", class: "ready", disabled: false },
  LOADING: { label: "Loading...", class: "loading", disabled: true },
  FAILED: { label: "Failed. Try again?", class: "failed", disabled: true },
  SUCCESS: { label: "Succeeded!", class: "success", disabled: true },
};
export default {
  name: "AlarmButton",
  data() {
    return {
      state: BUTTON_STATE.READY,
      info: String,
      timeout: 5000, // in ms
      state_change: 3000, // in ms
    };
  },
  methods: {
    wait: function (ms) {
      return new Promise((resolve) => setTimeout(resolve, ms));
    },
    call: async function () {
      // Set to loading state
      this.state = BUTTON_STATE.LOADING;
      this.axios
        .get("https://api.coindesk.com/v1/bpi/currentprice.json", {
          timeout: this.timeout,
        })
        .then((response) => {
          this.info = response;
          this.state = BUTTON_STATE.SUCCESS;
        })
        .catch((err) => {
          console.log(err.code);
          console.log(err.message);
          console.log(err.stack);
          this.state = BUTTON_STATE.FAILED;
        })
        .then(await this.wait(this.state_change))
        .then(() => {
          this.state = BUTTON_STATE.READY;
        });
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
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