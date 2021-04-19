<script lang="typescript">
	import {subscribeToStatus} from "./data";
</script>

<style>
	table {
		margin: 10px;
		border-collapse: collapse;
		text-transform: lowercase;
		width: calc(100% - 20px);
	}

	table, th, td {
		border: solid var(--primaryColor) 1px;
	}

	th {
		background: var(--primaryColor);
		color: white;
	}
	th::first-letter { color: var(--secondaryColor); text-transform: uppercase;}
	td::first-letter {text-transform: uppercase;}
</style>

<table>
	<tr>
		<th>Valve</th>
		<th>Status</th>
	</tr>

	{#each $subscribeToStatus as status}
		<tr>
			<td>{status.valve_id}</td>
			<td>{status.status}</td>
		</tr>
	{/each}
</table>
