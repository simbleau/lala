<template>
  <div class="history_container">
    <div id="alarm_chooser">
      <span>Alarm:</span>
      <v-select :options="options" label="title"></v-select>
    </div>
    <br />
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
          <td>
            {{ entry.type }}
          </td>
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
  data() {
    return {
      options: ["foo", "bar"],
    };
  },
  mounted() {
    // TODO load data in
    let mock_entry = new HistoryEntry(0, "server", "localhost", new Date());
    this.$store.commit("add_history", mock_entry);
  },
};
</script>

<style scoped>
.v-select {
  background-color: white;
  border-radius: 3px;
}
.history_container {
  display: block;
}
#alarm_chooser {
  width: 200px;
}
#history_table,
#alarm_chooser {
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