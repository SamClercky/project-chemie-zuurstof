<script lang="typescript">
	import { DEFAULT_DATA_FORM, sendInstructionToServer } from "./data";

	let formStatus = DEFAULT_DATA_FORM;

	function submitGpio(evt: any) {
		evt.preventDefault();

		sendInstructionToServer(formStatus);
	}
</script>

<style>
	form {
		display: flex;
		flex-direction: column;
	}
	legend {
		text-transform: capitalize;
	}

	#submit {
		margin: 0px;
		margin-top: 10px;
		background: var(--primaryColor);
		color: white;
		border: 0px;
		outline: solid 0px var(--secondaryColor);
		padding: 10px;
		font-weight: bold;
	}

	#submit:hover {
		outline-width: 5px;
		transition: outline 0.1s ease-out;
	}
</style>

<form on:submit={submitGpio}>
	<div>
		<label for="all_off">All off:</label>
		<input type="checkbox" name="all_off" bind:checked={formStatus.all_off}>
	</div>
	{#each formStatus.states as event (event.id)}
		<fieldset>
			<legend>{event.id}</legend>
			{#each event.state as status (status.valve_id)}
				<div>
					<label for={event.id + status.valve_id}>{status.valve_id} valve:</label>
					<input type="checkbox" name={event.id + status.valve_id} bind:checked={status.status} disabled={formStatus.all_off}>
				</div>
			{/each}
			<div>
				<label for={event.id+"time"}>Duration [ms]:</label>
				<input type="number" name={event.id+"time"} bind:value={event.time} disabled={formStatus.all_off}>
			</div>
		</fieldset>
	{/each}
	<input type="submit" name="submit" id="submit" value="Execute">
</form>
