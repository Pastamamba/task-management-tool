<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    export let row;

    const statuses = ["Open", "In Progress", "Resolved", "In Testing", "Done"];

    function changeStatus(newStatus: string) {
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

