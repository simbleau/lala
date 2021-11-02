import moment from 'moment'
export default class HistoryEntry {

    constructor(id, type, ip, date) {
        this.id = id;
        this.type = type;
        this.ip = ip;
        this.date = date;
    }

    get_date_string() {
        return moment(this.date).format("DD/MM/YYYY [at] HH:mm a")
    }
}