<script lang="ts">
	import Nbar from "./Nbar.svelte";
	import Feet from "./Feet.svelte";
	import {Row, Col, Container} from "sveltestrap";
</script>

<main>
	<!-- A lot more left to add... and hopefully more to come... but this is a start! -->
	<Nbar active="cv" />
	<div class="d-flex justify-content-center mt-4 text-light">
		<div class="w-75">
			<h1 class="text-center">Resume/CV</h1>
			<h3>Education</h3>
			<Container>
				<Row>
					<Col><Row><b>California State University, Fullerton</b></Row><Row><em>Fullerton, CA</em></Row></Col>
					<Col><Row class="text-end"><span>Bachelor of Science, Computer Science</span></Row><Row class="text-end"><em>August 2023 - May 2025</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<b>Relevant coursework: </b>Operating Systems, Compilers, Databases and Filesystems, Cybersecurity,
				                                    Algorithm Engineering, Professional Ethics, Software Engineering, Calculus III, Linear Algebra, Statistics
					</div>
				</Row>
				<br />
				<Row>
					<Col><Row><b>Cerritos College</b></Row><Row><em>Norwalk, CA</em></Row></Col>
					<Col><Row class="text-end"><span>Associate of Arts, Computer Information Systems: Programmer</span></Row><Row class="text-end"><em>October 2019 - August 2023</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<b>Relevant coursework: </b>Data Structures, Discrete Mathematics, Advanced C++, Java, Python, Computer Organization/Assembly Language,
						                            Database Design and SQL, Networking Fundamentals (CCNA), Unix/Linux Systems, Web Page Development,
						                            Calculus I/II
					</div>
				</Row>
			</Container>
			<br /><br />
			<h3>Leadership</h3>
			<Container>
				<Row>
					<Col><Row><b>Dev Board Officer</b></Row><Row><em>ACM Chapter at CSUF (acmCSUF)</em></Row></Col>
					<Col><Row class="text-end"><em>May 2023 - present</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<ul>
							<li>Helped over 150 people at Fall 2023 Linuxfest install and learn how to use Linux</li>
							<li>Led 3-4 workshops each semester inspiring usage of technologies in personal projects like SQLite, Python, and Git</li>
							<li>Provided programming assistance and homework help to students in CS courses</li>
						</ul>
					</div>
				</Row>
				<br />
				<Row>
					<Col><Row><b>Founding Member</b></Row><Row><em>Open Information Collective</em></Row></Col>
					<Col><Row class="text-end"><em>February 2023 - present</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<ul>
							<li>Successfully executed public records requests at nearly 75 agencies across 6 states and the federal government</li>
							<li>Managed a team of 6 people across 3 states in coordinating requests</li>
							<li>Won an appeal against the New York State Executive Chamber (Governor's Office)</li>
							<li>Prevented illegal violations of open government laws at the November 2023 CSU Board of Trustees meeting</li>
						</ul>
					</div>
				</Row>
				<br />
				<Row>
					<Col><Row><b>Director of Equity and Diversity</b></Row><Row><em>Associated Students of Cerritos College</em></Row></Col>
					<Col><Row class="text-end"><em>September 2022 - May 2023</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<ul>
							<li>Expanded the number of campus gender neutral bathrooms from 1 to nearly 30</li>
							<li>Uncovered monitoring of sensitive student internet traffic</li>
							<li>Worked to develop better access to menstrual products on campus under AB 367</li>
						</ul>
					</div>
				</Row>
				<br />
				<Row>
					<Col><Row><b>Deputy Director of Equity and Diversity</b></Row><Row><em>Associated Students of Cerritos College</em></Row></Col>
					<Col><Row class="text-end"><em>September 2021 - May 2022</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<ul>
							<li>Reformed the campus preferred name system to cover most technology systems</li>
							<li>Passed the Direction Bill, which provided a stronger foundation for the ASCC Constitution and Bylaws</li>
							<li>Fought to allow students to use open-source software in their courses</li>
						</ul>
					</div>
				</Row>
			</Container>
			<br /><br />
			<h3>Featured Projects</h3>
			<Container>
				<Row>
					<Col><Row><b>NYN Fitness</b></Row><Row><em><a href="https://nyn.amyip.net">https://nyn.amyip.net</a></em></Row></Col>
					<Col><Row class="text-end"><em>August 2023 - December 2023</em></Row></Col>
				</Row>
				<Row>
					<div class="mt-2" style="font-size: 0.75em;">
						<ul>
							<li>Generates dynamic workout recommendations based on 16 user preferences and previous workouts</li>
							<li>Built using C++, Python, Flask, MariaDB/MySQL, and Bootstrap/Sveltestrap</li>
							<li>Evaluates hundreds of workouts at under 2ms per user-initiated request on a single core</li>
						</ul>
					</div>
				</Row>
			</Container>
		</div>
	</div>
	<Feet />
</main>

<style>
	a {
		color: inherit;
		text-decoration: underline;
	}
</style>