<script lang="ts">
	import {sleep} from "./sleep";
	import {fade} from "svelte/transition";
	import {sw} from "./stores";

	let vis = {
		lin1: false,
		lin2: false
	};

	async function compInit() {
		await sleep(1500);
		vis["lin1"] = true;
		await sleep(2000);
		vis["lin2"] = true;
		await sleep(2500);
		sw.set(1);
	}
	compInit();
</script>

<main class="viewport-vc text-center d-flex align-items-center justify-content-center">
	<p>
		{#if vis["lin1"]}
			<h3 id="lin1" class="xcol-old-lace" in:fade={{duration:500}}>Welcome to</h3>
		{/if}
		{#if vis["lin2"]}
			<h1 id="lin2" class="xcol-alabaster" in:fade={{duration:500}}>amyip.net</h1>
		{/if}
	</p>
</main>

<style>
	.xcol-old-lace {
		color: #f5f1e3;
	}
	.xcol-alabaster {
		color: #f6f2e6;
	}
	.viewport-vc {
		height: 90vh !important;
		min-height: 90vh !important;
	}
	@media (max-width: 511px) {
		#lin2 {
			font-size: 3rem !important;
		}
	}
	#lin1 {
		font-weight: 600;
	}
	#lin2 {
		font-size: 4.5rem;
		font-weight: 700;
	}
</style>