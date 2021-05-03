<script lang="typescript">
	import {
		subscribeToStatus,
		subscribeToProgress,
	} from "./data";

	function niceTimeFormat(time: number) {
		time = time / 1000; // convert to sec
		let min = Math.floor(time/60);
		let sec = time % 60;

		if (min == 0) {
			return `${sec}sec`;
		} else {
			return `${min}min ${sec}sec`;
		}
	}
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

	td {
		padding-left: 10px;
	}
</style>

<div>
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
		<tr>
			<th>Metric</th>
			<th>Value</th>
		</tr>
		<tr>
			<td>Time in cyclus</td>
			<td>{niceTimeFormat($subscribeToProgress.timeInCycle)}</td>
		</tr>
		<tr>
			<td>Time since last update</td>
			<td>{niceTimeFormat($subscribeToProgress.timeInCycle - $subscribeToProgress.latestEvent.time)}</td>
		</tr>
		<tr>
			<td>Number of events passed</td>
			<td>{$subscribeToProgress.numberEvents}</td>
		</tr>
		<tr>
			<td>Number of iterations passed</td>
			<td>{$subscribeToProgress.numberIterations}</td>
		</tr>
	</table>
</div>
