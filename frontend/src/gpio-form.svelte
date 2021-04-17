<script lang="typescript">
	let formStatus = [
		{
			id: "feed",
			"state": [
				{
					"valve_id": "FEED",
					"status": true,
				},
				{
					"valve_id": "OO",
					"status": false,
				},
				{
					"valve_id": "NN",
					"status": false,
				}
			],
			"time": 60,
		},
		{
			id: "delay",
			"state": [
				{
					"valve_id": "FEED",
					"status": false,
				},
				{
					"valve_id": "OO",
					"status": false,
				},
				{
					"valve_id": "NN",
					"status": false,
				}
			],
			"time": 280,
		},
		{
			id: "exhaust",
			"state": [
				{
					"valve_id": "FEED",
					"status": true,
				},
				{
					"valve_id": "OO",
					"status": false,
				},
				{
					"valve_id": "NN",
					"status": false,
				}
			],
			"time": 280,
		},
		{
			id: "end",
			"state": [
				{
					"valve_id": "FEED",
					"status": true,
				},
				{
					"valve_id": "OO",
					"status": false,
				},
				{
					"valve_id": "NN",
					"status": false,
				}
			],
			"time": 280,
		}
	];

	function submitGpio(evt: any) {
		evt.preventDefault();

		let payload: any = {};
		for (let egpio of formStatus) {
			console.log(egpio);
			payload[egpio.id] = {
				"state": egpio.state,
				"time": egpio.time,
			};
		}

		// send payload to server
		fetch("/gpio", {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			redirect: "follow",
			body: JSON.stringify(payload),
		})
			.then(resp => console.log(`Gpio server says: ${resp.status}`))
			.catch(e => console.error(e));
	}
</script>

<form on:submit={submitGpio}>
	{#each formStatus as event (event.id)}
		<fieldset>
			<legend>{event.id}</legend>
			{#each event.state as status (status.valve_id)}
				<div>
					<label for={event.id + status.valve_id}>{status.valve_id} valve:</label>
					<input type="checkbox" name={event.id + status.valve_id} bind:checked={status.status}>
				</div>
			{/each}
			<div>
				<label for={event.id+"time"}>Time:</label>
				<input type="number" name={event.id+"time"} bind:value={event.time}>
			</div>
		</fieldset>
	{/each}
	<input type="submit" name="submit" id="submit" value="Execute">
</form>
