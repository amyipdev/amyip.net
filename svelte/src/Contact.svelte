<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import Feet from "./Feet.svelte";
</script>

<main>
	<Nbar active="contact" />
	<h1 class="mt-5 text-center">Contact</h1>
	<div class="d-flex justify-content-center mt-3">
		<div class="w-50" id="shrinkmecontact">
			<p><b>Primary Email:</b> <a href="mailto:amy@amyip.net">amy@amyip.net</a></p>
			<p><b>Backup Email:</b> <a href="mailto:amyipdev@csu.fullerton.edu">amyipdev@csu.fullerton.edu</a></p>
			<p><b>GPG:</b> <a href="https://keys.openpgp.org/vks/v1/by-fingerprint/7786034BD52149F51B0A2A14B1122F04E962DDC5">B1122F04E962DDC5</a></p>
			<p><b>Matrix:</b> <a href="https://matrix.to/#/@amyipdev1:matrix.org">@amyipdev1:matrix.org</a></p>
			<p><b>Discord:</b> <a href="https://discord.com">@amyipdev</a></p>
			<p><b>Instagram DMs:</b> <a href="https://instagram.com/amyipdev">@amyipdev</a></p>
		</div>
	</div>
	<Feet />
</main>

<style>
	h1 {
		color: #faf6f6;
	}
	a {
		color: inherit;
		text-decoration: underline;
	}
	p {
		color: #dad6d6;
	}
	@media (max-width: 501.99px) {
		#shrinkmecontact {
			width: 75% !important;
		}
	}
</style>