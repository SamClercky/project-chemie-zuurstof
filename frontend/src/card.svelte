<script lang="typescript">
	export let header = "";
</script>

<style>
	section {
		background: white;
		margin: 15px;
		padding: 15px;
		box-shadow: 5px 5px 10px gray;
		border-radius: 5px;
		max-width: 700px;
		width: 100%;
		min-width: 300px;
	}
	h2 {
		padding: 0px;
		margin: 0px;
		border-bottom: 1px solid var(--primaryColor);
	}
</style>

<section>
	<h2>{header}</h2>
	<slot></slot>
</section>
