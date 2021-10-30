<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <br />
    <AlarmButton label="Alarm!" />
  </div>
</template>

<script>
import AlarmButton from "@/components/AlarmButton.vue";

export default {
  name: "Home",
  components: {
    AlarmButton,
  },
};
</script>