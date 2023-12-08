<script lang="ts">
	import {Styles} from "sveltestrap";
	import WelcomeView from "./Welcome.svelte";
	import HomeView from "./Home.svelte";
	import {fade} from "svelte/transition";
    import {SvelteComponentDev} from "svelte/internal";
	import {sw} from "./stores";

	const views = [WelcomeView,HomeView];
	let cv: number = 0;
	let vc: typeof SvelteComponentDev = views[cv];

	function uvc(): void {
		vc = views[cv];
	}
	function tv(n: number): void {
		cv = n;
	}
	$: $sw != -1 && tv($sw);
</script>

<Styles />

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Mulish:wght@300;400;600;700&family=Open+Sans:wght@600;700&display=swap" rel="stylesheet">
</svelte:head>

<main>
	{#if vc === views[cv]}
		<div id="viewport" on:outroend={uvc} transition:fade>
			<svelte:component this={vc}></svelte:component>
		</div>
	{/if}
</main>

<style>
	:global(body) {
		background: #121212 !important;
	}
</style>