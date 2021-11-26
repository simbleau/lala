import { createStore } from "vuex"
import axios from 'axios'

const store = createStore({
    state: {
        server: "http://0.0.0.0:8081",
        reachable: false,
        alarms: [],
    },
    getters: {
        server: state => {
            return state.server;
        },
        reachable: state => {
            return state.reachable;
        },
        alarms: state => {
            return state.alarms;
        },
    },
    mutations: {
        set_reachable(state, reachable) {
            state.reachable = reachable;
        },
        clear_alarms(state) {
            state.alarms = [];
        },
        update_alarms(state, alarms) {
            state.alarms = alarms;
        },
    },
    actions: {
        fetch_alarms({ commit }) {
            axios
                .get(this.state.server + "/servers", {
                    timeout: this.timeout,
                })
                .then((response) => {
                    if (response.status != 200) {
                        const error = new Error(response.statusText);
                        throw error;
                    }
                    commit('set_reachable', true);
                    commit('update_alarms', response.data);
                })
                .catch((err) => {
                    commit('set_reachable', false);
                    commit('clear_alarms');
                    console.log(err.code + ": " + err.message + "\n" + err.stack);
                });
        },
    },
})

export default store
