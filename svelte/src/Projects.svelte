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
				<!-- TODO: move into Minor Projects once ACC and pwalarmd are listed here -->
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
				<div class="mt-2">
					<h3>Minor Projects</h3>
					<Row>
						<Col><h5 class="text-start">RODC</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/rodc">GitHub</a> <a href="https://pypi.org/project/rodc-cli">PyPI</a></h5></Col>
					</Row>
					<Row>
						<p><b>RODC</b>, the Read-Only Data Center, is a tool used here on amyip.net for storing static files that aren't built off of or stored with the repository.
						It provides a simple command-line interface to remotely upload files; because they are static and generally shouldn't be removed, it doesn't have any modification/removal functionality,
						with files being manually removable from the RODC instance when yanking is necessary. It is a very simple Flask server and Python script which is available on PyPI
						and can be self-hosted by anyone, and is licensed under GPLv2.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">libacc</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/libacc">GitHub</a> <a href="https://crates.io/crates/libacc">Crates.io</a></Col>
					</Row>
					<Row>
						<p><b>libacc</b> is a Rust library for the ACC evasive VPN protocol. It is used in the ACC Project for encrypting/decrypting and obfuscating/deobfuscating packets.
							The library handles identification of the ACC protocol, management of version-specific BSON documents, dynamic pad-based obfuscation, and encryption/packet authentication
							using AES256-GCM-SIV.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">libacmchristmas</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/acmCSUFDev/christmas-client-py">GitHub</a> <a href="https://pypi.org/project/libacmchristmas">PyPI</a></Col>
					</Row>
					<Row>
						<p><b>libacmchristmas</b>/<b>christmas-client-py</b> is a Python library that controls Christmas trees running on <a href="https://github.com/acmCSUFDev/christmasd">christmasd</a>.
							It is a part of the greater acmCSUF <a href="https://github.com/acmCSUFDev/christmas">christmas</a> project for <a href="https://blink.acmcsuf.com/">Blinking Lights</a> 2023.
							The library is based around Protocol Buffers and WebSockets for communication with the tree, and provides both raw interfaces to the tree (individually setting each light)
							and Python-native image handling via Pillow.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">binfetch-wasm</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/binfetch-wasm">GitHub</a> <a href="https://crates.io/crates/binfetch-wasm">Crates.io</a></Col>
					</Row>
					<Row>
						<p><b>binfetch-wasm</b> is a Rust library that allows for synchronous downloads of binary data in non-WASI WASM environments.
							It is used here on amyip.net to allow IrisOS-nano (the shell mode) to download filesystems via loadwebroot.
							The downloads are done by wrapping the browser's XHR interface in synchronous mode and converting the data into a binary-acceptable format,
							which requires several HTTP header tricks.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">infsprogs</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/amyip.net/tree/main/utils/infsprogs">GitHub</a> <a href="https://crates.io/crates/infsprogs">Crates.io</a></Col>
					</Row>
					<Row>
						<p><b>infsprogs</b> is a filesystem creation tool for the IrisOS-nano File System (INFS), which is used here on amyip.net.
							It is the primary way to build new INFS filesystems from existing on-system directory trees, similar to the operation of xorriso.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">OpenPrompter Software (beta)</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/openprompter-beta">GitHub</a></Col>
					</Row>
					<Row>
						<Col><h5 class="text-start">Watch306</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/watch306">GitHub</a></Col>
					</Row>
					<Row>
						<p><b>Watch306</b> is a small Discord bot for acmCSUF that pings the administrators of the CS306 server when it goes down so that we can respond rapidly.</p>
					<Row>
						<p><b>OpenPrompter</b> is a beta remote-control DIY teleprompter software system. It was used for the 2022-23 session of Whitney HS Live.
							It allows for existing hardware (computers/Raspberry Pis, and existing displays) to be turned at very low cost into a teleprompter
							that also functions as a camera view-back. It does so using PyGame, Flask, and OpenCV2.</p>
					</Row>
					<Row>
						<Col><h5 class="text-start">random-leak</h5></Col>
						<Col><h5 class="text-end"><a href="https://github.com/amyipdev/random-leak">GitHub</a></Col>
					</Row>
					<Row>
						<p><b>random-leak</b> is a "joke" Linux kernel module that leaks random parts of kernel memory by selecting small amounts of data from pages taken up by random symbols in kallsyms.
							Retrieving this data is done through a protected, root-access-only entry in /proc.
							It is theoretically a cross-platform kernel module, as long as the underlying kernel has kprobes enabled.
							Using procfs, it also allows the user to configure how much kernel memory to leak through a simple write operation.</p>
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
