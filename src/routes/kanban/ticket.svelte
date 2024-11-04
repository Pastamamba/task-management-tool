<script lang="ts">
    export let ticket: { id: number; title: string; status: string; description: string };
</script>

<div
        class="ticket"
        draggable="true"
        on:dragstart={(event) => event.dataTransfer.setData("text/plain", JSON.stringify(ticket))}
>
    <div class="ticket-title">{ticket.title}</div>
    <div class="ticket-description">{ticket.description}</div>
</div>

<style>
    .ticket {
        background-color: #ffffff;
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 0.5rem;
        margin-bottom: 0.5rem;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        cursor: grab;
    }
    .ticket-title {
        font-weight: bold;
        font-size: 1.1rem;
    }
    .ticket-description {
        font-size: 0.9rem;
        color: #555;
    }
</style>
