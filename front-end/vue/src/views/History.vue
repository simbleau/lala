<template>
  <div v-if="this.options.length > 0" class="history_container">
    <div id="alarm_chooser">
      <span>Alarm:</span>
      <select @change="(opt) => selector_changed(opt)">
        <option disabled selected value>Select</option>
        <option v-for="option in this.options" v-bind:key="option.addr">
          {{ option.addr }}
        </option>
      </select>
    </div>
    <br />
    <table id="history_table">
      <thead>
        <tr>
          <th>ID</th>
          <th>Date</th>
          <th>Addr</th>
          <th>Kind</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="entry in this.history" :key="entry.id">
          <th scope="row">{{ entry.id }}</th>
          <td>
            {{ entry.date }}
          </td>
          <td>{{ entry.addr }}</td>
          <td>{{ entry.kind }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { mapState } from "vuex";

export default {
  name: "History",
  components: {},
  data() {
    return {
      options: [],
      history: [],
    };
  },
  computed: mapState(["alarms"]),
  watch: {
    alarms(alarms) {
      this.load_alarms(alarms);
    },
  },
  mounted() {
    this.load_alarms(this.$store.getters.alarms);
  },
  methods: {
    selector_changed(selected) {
      this.fetch_history(selected.target.value);
    },
    clear_alarms() {
      this.options = [];
    },
    load_alarms(alarms) {
      this.clear_alarms();
      for (const alarm of alarms) {
        this.options.push(alarm);
      }
    },
    clear_history() {
      this.history = [];
    },
    load_history(payload) {
      console.log(payload);
      this.clear_history();
      for (const hist_entry of payload) {
        this.history.push(hist_entry);
      }
    },
    fetch_history: async function (addr) {
      var uri = this.$store.getters.server + "/history?server=" + addr;
      console.log(uri);
      // Perform request
      await this.axios
        .get(uri, {
          crossDomain: true,
          timeout: this.timeout,
        })
        .then((response) => {
          if (response.status != 200) {
            const error = new Error(response.statusText);
            throw error;
          }
          this.load_history(response.data);
        })
        .catch((err) => {
          console.log(err.code + ": " + err.message + "\n" + err.stack);
        });
    },
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