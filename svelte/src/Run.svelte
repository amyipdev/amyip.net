<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import Feet from "./Feet.svelte";
	import {Row, Col, Card, CardHeader, CardBody, CardText, CardTitle} from "sveltestrap";
</script>

<main>
	<Nbar active="run" />
	<div class="d-flex justify-content-center pt-5">
		<div class="w-75 text-center">
			<Row>
				<Col>
					<h3>Projects/Services</h3>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>NYN Fitness</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://nyn.amyip.net">nyn.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>AmyIP Nextcloud</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://cloud.amyip.net">cloud.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>RoundCube Webmail</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://mail.amyip.net">mail.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>Open Information Collective</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://oic.amyip.net">oic.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>SSVP Status</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://status.amyip.net">status.amyip.net</a></CardText></CardBody>
					</Card>
				</Col>
				<Col>
					<h3>Internal</h3>
					<Card color=danger-subtle class="mb-3 text-white">
						<CardHeader><CardTitle>phpMyAdmin</CardTitle></CardHeader>
						<CardBody><CardText><a href="javascript:alert('This endpoint has been temporarily shut down for security reasons.');">maria.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=danger-subtle class="mb-3 text-white">
						<CardHeader><CardTitle>Sappho PVE</CardTitle></CardHeader>
						<CardBody><CardText><a href="javascript:alert('This endpoint has been temporarily shut down for security reasons.');">sappho.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>Collabora</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://collabora.amyip.net">collabora.amyip.net</a></CardText></CardBody>
					</Card>
					<Card color=body-secondary class="mb-3">
						<CardHeader><CardTitle>Vaultwarden (nogui)</CardTitle></CardHeader>
						<CardBody><CardText><a href="https://vault.amyip.net">vault.amyip.net</a></CardText></CardBody>
					</Card>
				</Col>
			</Row>
		</div>
	</div>
	<Feet />
</main>

<style>
	a {
		color: inherit;
		text-decoration: underline;
		transition: font-size 0.2s;
	}
	a:hover {
		font-size: 1.2em;
	}
	h3 {
		margin-top: 0.375rem;
		margin-bottom: 0.375rem;
		color: #faf6f6;
	}
</style>
