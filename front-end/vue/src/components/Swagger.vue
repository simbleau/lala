<template>
  <div class="swagger-ui" id="swagger"></div>
</template>

<script>
import SwaggerUI from "swagger-ui";
import "@/assets/swagger-ui.css";

export default {
  name: "Swagger",
  mounted() {
    const spec = require("@/assets/swagger-spec.json");
    SwaggerUI({
      spec: spec,
      dom_id: "#swagger",
    });
  },
};
</script>

<style scoped>
.swagger-ui {
  padding: 50px;
  width: 100%;
  max-width: 600px;
  background-color: white;
  border-radius: 10px;
}
</style>