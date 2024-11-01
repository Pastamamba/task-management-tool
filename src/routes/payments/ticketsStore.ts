import { writable } from "svelte/store";

export type Ticket = {
    id: string;
    title: string;
    status: string;
    description: string;
};

export const tickets = writable<Ticket[]>([]);
