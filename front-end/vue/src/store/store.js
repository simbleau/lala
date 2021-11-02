import { createStore } from "vuex"

const store = createStore({
    state: {
        HISTORY_CAPACITY: 200,
        servers: [],
        responders: [],
        history: [],
    },
    getters: {
        get_servers: state => {
            return state.servers;
        },
        get_num_servers: state => {
            return state.servers.length
        },
        get_responders: state => {
            return state.responders;
        },
        get_num_responders: state => {
            return state.responders.length
        },
        get_history: state => {
            return state.history;
        },
    },
    mutations: {
        add_server(state, server) {
            state.servers.push(server);
        },
        remove_server(state, server) {
            const index = state.servers.indexOf(server);
            if (index > -1) {
                state.servers.splice(index, 1);
            }
        },
        add_responder(state, responder) {
            state.responders.push(responder);
        },
        remove_responder(state, responder) {
            const index = state.responders.indexOf(responder);
            if (index > -1) {
                state.responders.splice(index, 1);
            }
        },
        add_history(state, log) {
            state.history.push(log)
            if (state.history.length > state.HISTORY_CAPACITY) {
                state.history.shift();
            }
        }
    },
})

export default store
