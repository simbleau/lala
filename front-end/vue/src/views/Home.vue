<template>
  <div class="home">
    <template v-if="this.$store.getters.reachable == true">
      <template v-if="this.$store.getters.alarms.length > 0">
        <div class="alarm_container">
          <template
            v-for="alarm in this.$store.getters.alarms"
            :key="alarm.addr"
          >
            <h2>{{ alarm.addr }}</h2>
            <br />
            <AlarmButton />
          </template>
        </div>
      </template>
      <h2 v-else>No alarms active.</h2>
    </template>
    <h2 v-else>Server is unreachable</h2>
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