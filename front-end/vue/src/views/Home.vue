<template>
  <div class="home">
    <AlarmButton />
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

<style scoped>
.home {
  margin: 50px;
}
</style>