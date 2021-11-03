<template>
  <div id="swagger_container">
    <Swagger id="swagger" />
  </div>
</template>

<script>
import Swagger from "@/components/Swagger.vue";

export default {
  name: "API",
  components: { Swagger },
};
</script>
<style scoped>
#swagger_container {
  display: flex;
  text-align: center;
}
#swagger {
  margin: auto;
}
</style>