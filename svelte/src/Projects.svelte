<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import Feet from "./Feet.svelte";
	import {Row, Col} from "sveltestrap";
</script>

<main>
	<Nbar active="projects" />
	<div class="d-flex justify-content-center mt-4">
		<div id="make-me-bigger-plsss" class="w-50">
			<div>
				<h1>Projects</h1>
				<br />
				<div class="mt-2">
					<Row>
						<Col><h3 class="text-start">NYN Fitness</h3></Col>
						<Col><h5 class="text-end"><em>2023/08 - 2023/12</em></h5></Col>
					</Row>
					<Row>
						<img src="/gen_assets/project_images/nyn.png" alt="NYN Fitness user interface, desktop view"/>
					</Row>
					<Row>
						<em class="small-em">NYN main user interface, logged in, desktop view</em>
					</Row>
					<Row>
						<p class="mt-2">NYN Fitness is a dynamic workout recommendations app that was built by me, Dat Lam, Kevin Ramirez,
						Srinidhi Chevvuri, Theresa Nguyen, and Nicholas Goulart for our CPSC 362 course. It provides users with
						a curated collection of over 300 workouts, which are recommended based on a littany of user-settable
						preferences and goals. The site is able to serve 500 recommendation requests per worker thread per second,
						with fully linear scaling to any distribution size. The app is available on the web at <a href="https://nyn.amyip.net">nyn.amyip.net</a>,
						on iOS/Android as a <a href="https://www.cdc.gov/niosh/mining/content/hearingloss/installPWA.html">PWA</a>, and
						on <a href="https://github.com/amyipdev/nyn-fitness/releases/tag/v1.0.0">Windows, macOS, and Linux</a>. You can view
						the source code on my <a href="https://github.com/amyipdev/nyn-fitness">GitHub</a>.</p>
					</Row>
				</div>
				<div class="mt-2">
					<Row>
						<Col><h3 class="text-start">SSVP <em>(semi-deprecated)</em></h3></Col>
						<Col><h5 class="text-end"><em>2023/07 - 2023/08</em></h5></Col>
					</Row>
					<Row>
						<img src="/gen_assets/project_images/ssvp.png" alt="SSVP running at status.amyip.net"/>
					</Row>
					<Row>
						<em class="small-em">SSVP running at status.amyip.net</em>
					</Row>
					<Row>
						<p class="mt-2">
							SSVP stands for the Server Statistics Viewer Project. I originally developed it as an alternative to Uptime Kuma
							that was more lightweight and configurable; it was designed not to reveal more sensitive internal information,
							essentially being an open-source alternative to Atlassian Statuspage. I could've continued developing it, but
							when I started building it, my web development skills were not as strong, and the structure of SSVP was not prime
							for further extensibility. However, SSVP still (for now) powers <a href="https://status.amyip.net">status.amyip.net</a>,
							You can view the source code on <a href="https://github.com/amyipdev/ssvp">GitHub</a>.
						</p>
					</Row>
				</div>
				<div class="mt-2">
					<Row>
						<Col><h3 class="text-start">XAS-CHIP8 <em>(Deprecated)</em></h3></Col>
						<Col><h5 class="text-end"><em>2022/07 - 2022/12</em></h5></Col>
					</Row>
					<Row>
						<img src="/gen_assets/project_images/libxas.png" alt="Pong running on C0dysharma's CHIP-8 emulator"/>
					</Row>
					<Row>
						<em class="small-em">Pong running on <a href="https://github.com/c0dysharma/chip8-interpreter">C0dysharma's CHIP-8 emulator</a>; this Pong assembles perfectly on libxas</em>
					</Row>
					<Row>
						<p class="mt-2">
							The Extendable Assembler Library was a project of mine to create an assembler system that would be easily extensible
							to any architecture, including 128-bit and 256-bit ones. It was primarily designed around having generic interfaces
							to the maximum extent possible; while this made development harder, it allowed for expansion to an extremely wide variety
							of architectures. It also shared assembler syntax across architectures, favoring universal approaches instead of individualized ones;
							this reduced the amount of work necessary to create an assembler for a new architecture. The fundamental structures of the library
							still exist, but I would not recommend building off of them, as there are several core issues; currently, the project contains
							a working CHIP-8 assembler, and the beginnings of a 6502 assembler. You can view the code on <a href="https://github.com/amyipdev/xas-chip8-old">GitHub</a>.
						</p>
					</Row>
				</div>
			</div>
		</div>
	</div>
	<Feet />
</main>

<style>
	.small-em {
		font-size: 0.75rem;
		text-align: center;
	}
	h1 {
		color: #fbf7f5;
		text-align: center;
	}
	h3 {
		color: #faf6f6;
	}
	h5 {
		color: #f0f0f0;
	}
	img {
		width: 100%;
		max-width: 50vw;
	}
	@media (max-width: 900px) {
		#make-me-bigger-plsss {
			width: 75% !important;
			max-width: 75% !important;
		}
		img {
			max-width: 75vw !important;
		}
	}
	a {
		text-decoration: underline;
		color: inherit;
	}
	p {
		color: #d5d9dd;
	}
</style>