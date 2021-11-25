import { createStore } from "vuex"

const store = createStore({
    state: {
        server: "http://0.0.0.0:8081",
        history_limit: 200,
        alarms: [],
        history: [],
    },
    getters: {
        get_server: state => {
            return state.server;
        },
        get_alarms: state => {
            return state.alarms;
        },
        get_history: state => {
            return state.history;
        },
    },
    mutations: {
        add_alarm(state, server) {
            state.alarms.push(server);
        },
        remove_alarm(state, server) {
            const index = state.alarms.indexOf(server);
            if (index > -1) {
                state.alarms.splice(index, 1);
            }
        },
        clear_history(state) {
            state.history.length = 0;
        },
        add_history(state, log) {
            console.log("called");
            state.history.push(log)
            if (state.history.length > state.HISTORY_CAPACITY) {
                state.history.shift();
            }
        }
    },
})

export default store
