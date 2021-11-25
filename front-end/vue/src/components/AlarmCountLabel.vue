<template>
  <span>Alarms: {{ count }}</span>
</template>

<script>
export default {
  name: "AlarmCountLabel",
  props: {
    count: Number,
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
span {
  font-weight: bold;
}
</style>