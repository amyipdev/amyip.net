<script lang="ts">
	import {Styles} from "sveltestrap";
	import WelcomeView from "./Welcome.svelte";
	import HomeView from "./Home.svelte";
	import ShellView from "./Shell.svelte";
	import CvView from "./Cv.svelte";
	import RunView from "./Run.svelte";
	import {fade} from "svelte/transition";
    import {SvelteComponentDev} from "svelte/internal";
	import {sw} from "./stores";

	const views = [WelcomeView,HomeView,ShellView,CvView,RunView];
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
	<link href=" https://cdn.jsdelivr.net/npm/xterm@5.3.0/css/xterm.min.css" rel="stylesheet">
	<link rel="preconnect" href="https://fonts.googleapis.com">
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
	<link href="https://fonts.googleapis.com/css2?family=Mulish:wght@300;400;600;700&family=Inconsolata&display=swap" rel="stylesheet">
</svelte:head>

<main>
	{#if vc === views[cv]}
		<div id="viewport" on:outroend={uvc} transition:fade>
			<svelte:component this={vc}></svelte:component>
		</div>
	{/if}
</main>
<div id="anti-termux-safety"></div>

<style>
	:global(body) {
		background: #121212 !important;
	}
</style>