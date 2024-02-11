<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import {sw} from './stores';
	import {Row, Col, Container, Card, CardHeader, CardTitle, CardFooter, CardText, CardBody} from "sveltestrap";
</script>

<main>
	<Nbar active="about"></Nbar>
	<div class="d-flex justify-content-center mt-5">
		<div class="about-content text-center w-75">
			<div class="d-flex justify-content-center align-items-center mb-5">
				<img id="face-pic-ab" src="/gen_assets/face.png" alt="Me at the January 2024 California State University strikes" />
				<div class="text-start ps-4">
					<h2>Hello! I'm</h2>
					<h1 id="name-text-ab"><b>Amy.</b></h1>
				</div>
			</div>
			<h6>I'm currently a junior at California State University, Fullerton.&nbsp;🐘</h6>
			<h6>My focuses are on systems programming, distributed systems, and networking.&nbsp;🖥️</h6>
			<h6>I'm a happy and proud Linux&nbsp;🐧 user and Rust developer.&nbsp;🦀</h6>
			<br />
			<h6>Welcome to <b>amyip.net</b> on <em>maggie</em>, my home server cluster.</h6>
			<h6>This is the home of all my projects and self-hosted services, such as my <a href="https://cloud.amyip.net">Nextcloud</a>.</h6>
			<h6>You can use the navbar buttons at the top to explore. Check out the <a href="#_" on:click={() => sw.set(2)}>shell</a>!</h6>
			<br />
			<Container fluid class="my-4">
				<Row>
					<Col class="mb-3">
						<div class="backdrop-cardlist">
							<h4>Socials</h4>
							<Card class="card-cust-xae" style="background-color: #faf6f6;">
								<CardHeader style="background-color: #2f0c7a; color: #faf6f6;"><CardTitle><i class="bi bi-mastodon"></i>&nbsp;&nbsp;Mastodon</CardTitle></CardHeader>
								<CardBody><CardText>My random thoughts</CardText></CardBody>
								<CardFooter style="background-color: #2f0c7a; color: #faf6f6;"><a href="https://blahaj.zone/@amyipdev">@amyipdev@blahaj.zone</a></CardFooter>
							</Card>
							<!-- #46cb9f -->
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #48b9ae; color: #faf6f6;"><CardTitle><i class="bi bi-vector-pen"></i>&nbsp;&nbsp;Matrix</CardTitle></CardHeader>
								<CardBody><CardText>Secure communications</CardText></CardBody>
								<CardFooter style="background-color: #48b9ae; color: #faf6f6;"><a href="https://matrix.to/#/@amyipdev1:matrix.org">@amyipdev1:matrix.org</a></CardFooter>
							</Card>
							<!-- Instagram gradient from sarthak7gupta on stackoverflow -->
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #f7c651;"><CardTitle><i class="bi bi-instagram"></i>&nbsp;&nbsp;Instagram</CardTitle></CardHeader>
								<CardBody><CardText>Pictures and stories</CardText></CardBody>
								<CardFooter style="background-color: #f7c651;"><a href="https://instagram.com/amyipdev">@amyipdev</a></CardFooter>
							</Card>
						</div>
					</Col>
					<Col class="mb-3">
						<div class="backdrop-cardlist">
							<h4>Quick Links</h4>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #262934; color: #faf6f6;"><CardTitle><i class="bi bi-github"></i>&nbsp;&nbsp;GitHub</CardTitle></CardHeader>
								<CardBody><CardText>Projects/source code</CardText></CardBody>
								<CardFooter style="background-color: #262934; color: #faf6f6;"><a href="https://github.com/amyipdev">github/amyipdev</a></CardFooter>
							</Card>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #e34133;"><CardTitle><i class="bi bi-envelope-at-fill"></i>&nbsp;&nbsp;Email</CardTitle></CardHeader>
								<CardBody><CardText>Send (secure) email</CardText></CardBody>
								<CardFooter style="background-color: #e34133;"><a href="mailto:amy@amyip.net">amy@amyip.net</a> (<a href="https://keys.openpgp.org/vks/v1/by-fingerprint/7786034BD52149F51B0A2A14B1122F04E962DDC5">GPG</a>)</CardFooter>
							</Card>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #0a66c2; color: #faf6f6;"><CardTitle><i class="bi bi-linkedin"></i>&nbsp;&nbsp;LinkedIn</CardTitle></CardHeader>
								<CardBody><CardText>Yet another resume</CardText></CardBody>
								<CardFooter style="background-color: #0a66c2; color: #faf6f6;"><a href="https://blahaj.zone/@amyipdev">@amyipdev@blahaj.zone</a></CardFooter>
							</Card>
						</div>
					</Col>
				</Row>
			</Container>
		</div>
	</div>
</main>

<style>
	:global(.card-cust-xae) {
		margin-top: 1.5em;
		margin-bottom: 1.5em;
		background-color: #faf6f6 !important;
		color: #010117 !important;
	}
	main {
		color: #cbcac3;
	}
	h6 {
		color: #e4e3db;
	}
	h1 {
		color: #fbf7f5;
	}
	h4 {
		color: #fbf7f5 !important;
	}
	.backdrop-cardlist {
		background-color: #2a2a29;
		padding: 0.8rem;
		border-radius: 5%;
		border: 3px solid #e4e3db;
	}
	#face-pic-ab {
		height: 7.5rem;
		width: 7.5rem;
		border-radius: 30%;
		border: 5px solid #f9f1f1;
	}
	#name-text-ab {
		font-size: 3.5rem;
	}
	a {
		color: inherit;
		font-weight: bold;
		text-decoration: underline;
		transition: font-size 0.2s;
	}
	a:hover {
		font-size: 1.2rem;
	}
</style>