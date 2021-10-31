<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <br />
    <AlarmButton
      id="alarm_button"
      v-bind:state="button_state"
      v-on:click="call()"
    />
  </div>
</template>

<script>
import AlarmButton from "@/components/AlarmButton.vue";
import { BUTTON_STATE } from "@/components/AlarmButton.vue";

export default {
  name: "Home",
  data: function () {
    return {
      button_state: BUTTON_STATE.READY,
    };
  },
  components: {
    AlarmButton,
  },
  methods: {
    sleep: function (ms) {
      // Expensive function
      return new Promise((resolve) => setTimeout(resolve, ms));
    },
    call: async function () {
      this.button_state = BUTTON_STATE.LOADING;
      await this.sleep(2000); // Send the call
      // Evaluate the result, show result
      let success = Math.random() < 0.5 ? true : false;
      this.button_state = success ? BUTTON_STATE.SUCCESS : BUTTON_STATE.FAILED;
      // Revert to ready state
      await this.sleep(2000); // Expensive function
      this.button_state = BUTTON_STATE.READY;
    },
  },
};
</script>