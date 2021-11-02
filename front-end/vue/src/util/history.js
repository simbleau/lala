class HistoryEntry {
    constructor(type, ip, date) {
        this.type = type;
        this.ip = ip;
        this.date = date;
    }

    get type() {
        return this.type;
    }

    get ip() {
        return this.ip;
    }

    get date() {
        return this.date;
    }
}