<script>
    import { Button } from "$lib/components/ui/button";
    let items = [
        { id: 1, name: "Element 1", description: "This is the first element" },
        { id: 2, name: "Element 2", description: "This is the second element" },
        { id: 3, name: "Element 3", description: "This is the third element" },
    ];
</script>

<main class="container">
    <h1>Table Page</h1>
    <Button>Click me</Button>
    <table>
        <thead>
        <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Description</th>
        </tr>
        </thead>
        <tbody>
        {#each items as item}
            <tr>
                <td>{item.id}</td>
                <td>{item.name}</td>
                <td>{item.description}</td>
            </tr>
        {/each}
        </tbody>
    </table>
</main>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding-top: 20px;
    }

    table {
        border-collapse: collapse;
        width: 80%;
        margin: 20px 0;
    }

    th, td {
        border: 1px solid #ddd;
        padding: 8px;
        text-align: left;
    }

    th {
        background-color: #396cd8;
        color: white;
    }

    tr:nth-child(even) {
        background-color: #f2f2f2;
    }
</style>
