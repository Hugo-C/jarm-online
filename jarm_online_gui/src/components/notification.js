class Notification {

    constructor() {
        this.isDisplayed = false;
        this.title = null;
        this.body = null;
    }

    display(title, body=null) {
        this.isDisplayed = true;
        this.title = title;
        this.body = body;
    }

    clear() {
        this.isDisplayed = false;
        this.title = null;
        this.body = null;
    }
}

export const notification = new Notification();
