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
		display: flex;
		flex-direction: column;
	}
	h2 {
		padding: 0px;
		margin: 0px;
		margin-bottom: 10px;
		border-bottom: 1px solid var(--primaryColor);
		color: var(--primaryColor);
	}
	h2::first-letter {
		color: var(--secondaryColor);
	}
</style>

<section>
	<h2>{header}</h2>
	<slot></slot>
</section>
