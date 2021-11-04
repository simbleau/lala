import moment from 'moment'

export default class HistoryEntry {

    constructor(id, type, ip, date) {
        this._id = id;
        this._type = type;
        this._ip = ip;
        this._date = date;
    }

    static types() {
        return {
            SERVER_CHECK_IN: "server",
            CLIENT_CHECK_IN: "client",
            ACK: "ack",
            ALARM: "1",
        };
    }

    get id() {
        return this._id;
    }

    get type() {
        return this._type;
    }

    get ip() {
        return this._ip;
    }

    get date() {
        return this._date;
    }

    get_date_string() {
        return moment(this.date).format("DD/MM/YYYY [at] HH:mm a")
    }

    is_server_checkin() {
        return this._type == HistoryEntry.types().SERVER_CHECK_IN;
    }

    is_client_checkin() {
        return this._type == HistoryEntry.types().CLIENT_CHECK_IN;
    }

    is_ack() {
        return this._type == HistoryEntry.types().ACK;
    }

    is_alarm() {
        return this._type == HistoryEntry.types().ALARM;
    }

    is_error() {
        for (const type of HistoryEntry.types()) {
            if (this._type == type) {
                return false;
            }
        }
        return true;
    }
}