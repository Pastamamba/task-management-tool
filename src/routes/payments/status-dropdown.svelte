<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    export let row;

    const statuses = ["Open", "In Progress", "Resolved", "In Testing", "Done"];

    async function changeStatus(newStatus: string) {
        console.log(`Ticket ${row.original.id} status changed to ${newStatus}`);

        await invoke("update_ticket_status", { ticketId: parseInt(row.original.id), newStatus: newStatus });
        // Update local store to reflect the change
        tickets.update((items) =>
            items.map((item) =>
                item.id === row.original.id ? { ...item, status: newStatus } : item
            )
        );
    }
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button
                variant="ghost"
                builders={[builder]}
                size="icon"
                class="relative h-8 w-8 p-0"
        >
            {row.original.status}
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content>
        {#each statuses as status}
            <DropdownMenu.Item on:click={() => changeStatus(status)}>
                {status}
            </DropdownMenu.Item>
        {/each}
    </DropdownMenu.Content>
</DropdownMenu.Root>

