import { writable } from "svelte/store";

export type Ticket = {
    id: number;
    title: string;
    status: string;
    description: string;
    elapsedTime?: number;
    timerRunning?: boolean;
};

export const tickets = writable<Ticket[]>([]);

export function startTimer(ticketId: number) {
    tickets.update((items) =>
        items.map((item) => {
            if (item.id === ticketId) {
                item.timerRunning = true;
                if (!item.elapsedTime) item.elapsedTime = 0;
            }
            return item;
        })
    );
}

export function stopTimer(ticketId: number) {
    tickets.update((items) =>
        items.map((item) => {
            if (item.id === ticketId) {
                item.timerRunning = false;
            }
            return item;
        })
    );
}

function incrementTimers() {
    tickets.update((items) =>
        items.map((item) => {
            if (item.timerRunning) {
                item.elapsedTime = (item.elapsedTime || 0) + 1;
            }
            return item;
        })
    );
}

// Start the interval to increment timers every second
setInterval(() => {
    incrementTimers();
}, 1000);
