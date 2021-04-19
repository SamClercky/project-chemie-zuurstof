<script lang='typescript'>
	import Card from "./card.svelte";
	import GpioForm from "./gpio-form.svelte";
	import Monitor from "./monitor.svelte";
</script>
  
<style>
	:global(:root) {
		--primaryColor: #003399;
		--secondaryColor: #ff6600;
	}
	:global(html, body) {
		min-width: 100vw;
		min-height: 100%;
		overflow-x: hidden;
		font-family: Arial, Helvetica, sans-serif;
		margin: 0px;
		padding: 0px;
	}

	.App {
		display: flex;
		width: 100%;
		height: min-content;

		flex-direction: column;
	}
	.wrapper {
		display: flex;
		width: 100%;
		flex-direction: row;
		flex-wrap: wrap;
	}

	h1 {
		background: var(--primaryColor);
		color: white;
		width: 100%;
		padding: 15px;
		margin: 0px;
	}
	h1 > span {
		color: var(--secondaryColor);
	}
</style>
  
<div class="App">
	<h1><span>C</span>hemie workhorse</h1>

	<div class="wrapper">
		<Card header="Monitoring">
			<Monitor></Monitor>
		</Card>
		<Card header="New instruction">
			<GpioForm></GpioForm>
		</Card>
	</div>
</div>

