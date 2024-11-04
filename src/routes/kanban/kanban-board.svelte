<script lang="ts">
    import Ticket from "./ticket.svelte";
    import { writable } from "svelte/store";

    // Define Kanban statuses
    const statuses = ["Open", "In Progress", "Resolved", "In Testing", "Done"];

    // Ticket type
    type TicketType = {
        id: number;
        title: string;
        status: string;
        description: string;
    };

    // Example tickets
    let tickets = writable<TicketType[]>([
        { id: 1, title: "Fix login bug", status: "Open", description: "User cannot log in." },
        { id: 2, title: "Update dependencies", status: "In Progress", description: "Update libraries." },
        { id: 3, title: "Feature X", status: "Resolved", description: "Implement feature X." },
        { id: 4, title: "UI Testing", status: "In Testing", description: "Test the new UI changes." },
        { id: 5, title: "Release v2.0", status: "Done", description: "Deploy version 2.0." }
    ]);

    // Function to handle dropping a ticket
    function handleDrop(event, newStatus) {
        event.preventDefault();
        const ticketData = event.dataTransfer.getData("text/plain");
        const droppedTicket = JSON.parse(ticketData);

        tickets.update((items) =>
            items.map((item) =>
                item.id === droppedTicket.id ? { ...item, status: newStatus } : item
            )
        );
    }

    // Function to allow dragging over columns
    function allowDrop(event) {
        event.preventDefault();
    }
</script>

<style>
    .kanban-board {
        display: flex;
        gap: 1rem;
        padding: 1rem;
    }
    .kanban-column {
        flex: 1;
        background-color: #f8f8f8;
        border-radius: 8px;
        padding: 0.5rem;
        border: 1px solid #ddd;
        min-height: 200px;
    }
    .kanban-column h2 {
        font-size: 1.25rem;
        margin-bottom: 0.5rem;
        text-align: center;
    }
</style>

<div class="kanban-board">
    {#each statuses as status}
        <div
                class="kanban-column"
                role="region"
                aria-label={`Kanban column for ${status} tickets`}
                on:dragover={allowDrop}
                on:drop={(event) => handleDrop(event, status)}
        >
            <h2>{status}</h2>
            {#each $tickets.filter(ticket => ticket.status === status) as ticket}
                <Ticket {ticket} />
            {/each}
        </div>
    {/each}
</div>
