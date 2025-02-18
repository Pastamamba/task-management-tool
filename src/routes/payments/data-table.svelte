<script lang="ts">
    import { createRender, createTable, Render, Subscribe } from "svelte-headless-table";
    import * as Table from "$lib/components/ui/table";
    import { invoke } from "@tauri-apps/api/core";
    import DataTableActions from "./data-table-actions.svelte";
    import StatusDropdown from "./status-dropdown.svelte";
    import type {Ticket} from "@/routes/payments/ticketsStore";
    import {tickets} from "@/routes/payments/ticketsStore";
    import Timers from "./DataTableActions.svelte";

    async function fetchTickets() {
        const data: Ticket[] = await invoke("get_tickets_from_db");
        tickets.set(data);
    }

    fetchTickets();

    const table = createTable(tickets);

    const columns = table.createColumns([
        table.column({
            accessor: "id",
            header: "ID",
        }),
        table.column({
            accessor: "title",
            header: "Title",
        }),
        table.column({
            accessor: "status",
            header: "Status",
            cell: ({ row }) => {
                return createRender(StatusDropdown, { row });
            },
        }),
        table.column({
            accessor: ({ id }) => id,
            header: "",
            cell: ({ value }) => {
                return createRender(DataTableActions, { id: value });
            },
        }),
        table.column({
            accessor: "description",
            header: "Description",
        }),
        table.column({
            accessor: ({ id }) => id,
            header: "Timer",
            cell: ({ value }) => {
                return createRender(Timers, { id: value });
            },
        }),
    ]);

    const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);
</script>

<div class="rounded-md border">
    <h1>Jira tickets</h1>
    <Table.Root {...$tableAttrs}>
        <Table.Header>
            {#each $headerRows as headerRow}
                <Subscribe rowAttrs={headerRow.attrs()}>
                    <Table.Row>
                        {#each headerRow.cells as cell (cell.id)}
                            <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                                <Table.Head {...attrs}>
                                    <Render of={cell.render()} />
                                </Table.Head>
                            </Subscribe>
                        {/each}
                    </Table.Row>
                </Subscribe>
            {/each}
        </Table.Header>
        <Table.Body {...$tableBodyAttrs}>
            {#each $pageRows as row (row.id)}
                <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
                    <Table.Row {...rowAttrs}>
                        {#each row.cells as cell (cell.id)}
                            <Subscribe attrs={cell.attrs()} let:attrs>
                                <Table.Cell {...attrs}>
                                    <Render of={cell.render()} />
                                </Table.Cell>
                            </Subscribe>
                        {/each}
                    </Table.Row>
                </Subscribe>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
