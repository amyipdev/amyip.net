<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import Feet from "./Feet.svelte";
	import {sw} from './stores';
	import {Row, Col, Container, Card, CardHeader, CardTitle, CardFooter, CardText, CardBody} from "sveltestrap";
</script>

<main>
	<Nbar active="about" />
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
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #2f0c7a; color: #faf6f6;"><CardTitle><i class="bi bi-mastodon"></i>&nbsp;&nbsp;Mastodon</CardTitle></CardHeader>
								<!--<CardBody><CardText>My random thoughts</CardText></CardBody>-->
								<CardFooter style="background-color: #2f0c7a; color: #faf6f6;"><a href="https://transfem.social/@amyipdev" class="mobile-08">@amyipdev@transfem.social</a></CardFooter>
							</Card>
							<!-- #46cb9f -->
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #faf6f6;"><CardTitle><i class="bi bi-vector-pen"></i>&nbsp;&nbsp;Matrix</CardTitle></CardHeader>
								<!--<CardBody><CardText>Secure communications</CardText></CardBody>-->
								<CardFooter style="background-color: #faf6f6;"><a href="https://matrix.to/#/@amyipdev1:matrix.org" class="mobile-08">@amyipdev1:matrix.org</a></CardFooter>
							</Card>
							<!-- Instagram gradient from sarthak7gupta on stackoverflow -->
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #f7c651;"><CardTitle><i class="bi bi-instagram"></i>&nbsp;&nbsp;Instagram</CardTitle></CardHeader>
								<!--<CardBody><CardText>Pictures and stories</CardText></CardBody>-->
								<CardFooter style="background-color: #f7c651;"><a href="https://instagram.com/amyipdev" class="mobile-08">@amyipdev</a></CardFooter>
							</Card>
						</div>
					</Col>
					<Col class="mb-3">
						<div class="backdrop-cardlist">
							<h4>Quick Links</h4>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #262934; color: #faf6f6;"><CardTitle><i class="bi bi-github"></i>&nbsp;&nbsp;GitHub</CardTitle></CardHeader>
								<!--<CardBody><CardText>Projects/source code</CardText></CardBody>-->
								<CardFooter style="background-color: #262934; color: #faf6f6;"><a href="https://github.com/amyipdev" class="mobile-08">github/amyipdev</a></CardFooter>
							</Card>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #e34133;"><CardTitle><i class="bi bi-envelope-at-fill"></i>&nbsp;&nbsp;Email</CardTitle></CardHeader>
								<!--<CardBody><CardText>Send (secure) email</CardText></CardBody>-->
								<CardFooter style="background-color: #e34133;" class="mobile-08"><a href="mailto:amy@amyip.net">amy@amyip.net</a> (<a href="https://keys.openpgp.org/vks/v1/by-fingerprint/7786034BD52149F51B0A2A14B1122F04E962DDC5">GPG</a>)</CardFooter>
							</Card>
							<Card class="card-cust-xae">
								<CardHeader style="background-color: #0a66c2; color: #faf6f6;"><CardTitle><i class="bi bi-linkedin"></i>&nbsp;&nbsp;LinkedIn</CardTitle></CardHeader>
								<!--<CardBody><CardText>Yet another resume</CardText></CardBody>-->
								<CardFooter style="background-color: #0a66c2; color: #faf6f6;"><a href="https://linkedin.com/in/amyiris" class="mobile-08">in/amyiris</a></CardFooter>
							</Card>
						</div>
					</Col>
				</Row>
			</Container>
			<h1>More About Me</h1>
			<div class="text-start" style="color: #e4e3db !important;">
				<br />
				<p>My name is Amy Iris Parker (<b>amyipdev</b>), and I <a href="https://www.glsen.org/activity/pronouns-guide-glsen">use</a> she/her/hers.
				   I'm a software developer, researcher, CS major, and community activist looking to make the world a better place however I can.
				   I'm from the Greater Los Angeles area (currently in Fullerton, CA). I also am a Founding Member of the <a href="https://oic.amyip.net">Open Information Collective</a>,
				   an open government organization which promotes access to government meetings and public records.<br /><br />

				   I'm currently a junior at <b>California State University, Fullerton</b>, seeking a Bachelor of Science in Computer Science.
				   Currently, I serve as an officer on the Dev Board for the CSUF chapter of the Association for Computing Machinery, <a href="https://acmcsuf.com">acmCSUF</a>.
				   Working with the University Registrar, I fought for <a href="https://dailytitan.com/news/students-may-now-change-their-preferred-name-on-titan-cards/article_e05bbb3c-3fa6-11ee-8a80-fffdbeb1719b.html">preferred name system reforms</a>,
				   which is still ongoing. I also am a member of the Students Against Tuition Hikes coalition, which is working to protect the CSU from dangerous tuition increases and fee hikes.<br /><br />

				   In 2023, I graduated with my Associate of Arts in Computer Information Systems from Cerritos College.
				   I also served for two years as the Director (2022-23) and Deputy Director (2021-22) of Equity and Diversity for the Associated Students of Cerritos College.
				   While I was there, I fought against attempts to <a href="https://oic.amyip.net/wp-content/uploads/2023/03/Information-Technology-ED-Report-1.pdf">spy on students</a>,
				   <a href="https://www.cerritos.edu/activities/_includes/docs/1_Approved_Legislation/2022-2023-Legislation/JR-2223-04_ua.pdf">worked</a> to increase the number of student-accessible gender neutral bathrooms from 1 to nearly 30,
				   and <a href="https://www.cerritos.edu/activities/_includes/docs/1_Approved_Legislation/JR-2122-08_Signed_ua.pdf">reformed</a> the campus preferred name system.<br /><br />

				   In 2019, I left Windows (and other proprietary software) behind, moving to Linux entirely across my devices. Most of my systems run <b>Fedora Linux</b>, which I highly recommend;
				   this includes my desktop, laptop, and tablet. I am, however, working on moving to Arch Linux. My servers run <b>Proxmox VE</b>, which is based on <b>Debian Linux</b>; most of
				   the containers which comprise the services on amyip.net are also Debian-based.</p>
			</div>
			<p class="text-secondary">&lt;/about&gt;</p>
		</div>
	</div>
	<Feet />
</main>

<style>
	@media (max-width: 400px) {
		:global(.mobile-08) {
			font-size: 0.8em;
		}
	}
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
