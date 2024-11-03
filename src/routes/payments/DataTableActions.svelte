<script lang="ts">
    import { startTimer, stopTimer, tickets } from "@/routes/payments/ticketsStore";
    import { derived } from "svelte/store";

    export let id: number;

    // Get the current ticket from the store based on the ID
    const ticket = derived(tickets, ($tickets) => $tickets.find(t => t.id === id));

    // Format seconds into hh:mm:ss
    function formatTime(seconds: number) {
        const hours = Math.floor(seconds / 3600).toString().padStart(2, "0");
        const minutes = Math.floor((seconds % 3600) / 60).toString().padStart(2, "0");
        const secs = (seconds % 60).toString().padStart(2, "0");
        return `${hours}:${minutes}:${secs}`;
    }

    function handleStart() {
        startTimer(id);
    }

    function handleStop() {
        stopTimer(id);
    }
</script>

<div class="timer-controls">
    <button on:click={handleStart} disabled={$ticket?.timerRunning}>Start</button>
    <button on:click={handleStop} disabled={!$ticket?.timerRunning}>Stop</button>
    <span>{$ticket?.elapsedTime ? formatTime($ticket.elapsedTime) : "00:00:00"}</span>
</div>
