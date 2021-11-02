<template>
  <div class="history_container">
    <table id="history_table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Type</th>
          <th>IP</th>
          <th>Date-time</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in this.$store.getters.get_history" :key="entry.id">
          <th scope="row">{{ entry.id }}</th>
          <td>{{ entry.type }}</td>
          <td>{{ entry.ip }}</td>
          <td>{{ entry.get_date_string() }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import HistoryEntry from "@/util/history";

export default {
  name: "History",
  components: {},
  mounted() {
    let mock_entry = new HistoryEntry(0, "server", "localhost", new Date());
    this.$store.commit("add_history", mock_entry);
  },
};
</script>

<style scoped>
.history_container {
  display: flex;
}
#history_table {
  margin-left: auto;
  margin-right: auto;
}

thead {
  border: 5px solid #ffffff;
}
tr:hover {
  background-color: rgba(255, 255, 255, 0.1);
}
th,
td {
  padding: 15px;
  text-align: left;
}
</style>