<script lang="ts">
    import {
		Navbar,
		Collapse,
		NavbarBrand,
		NavItem,
		NavbarToggler,
		NavLink,
		Nav
	} from "sveltestrap";
    let isOpen = false;
    function hu(e: CustomEvent) {
		isOpen = e.detail.isOpen;
	}
    import {sw} from './stores';
    export let active: string;

    // undo Termux control element
    const dblc = document.body.lastChild as HTMLElement;
    if (dblc.id != "anti-termux-safety") {
        dblc.remove();
    }
</script>

<main>
    <!-- TODO: move this navbar to its own component -->
    <Navbar expand="md">
        <NavbarBrand><span id="navbar-brand">amyip.net</span></NavbarBrand>
        <NavbarToggler on:click={() => (isOpen = !isOpen)} />
        <Collapse {isOpen} navbar expand="md" on:update={hu}>
            <Nav class="ms-auto" navbar>
                <NavItem>
                    <NavLink active={active == "about"} on:click={() => sw.set(1)}>
                        about
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink on:click={() => sw.set(2)}>
                        shell
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink active={active == "cv"} disabled on:click={() => {}}>
                        cv
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink active={active == "blog"} disabled on:click={() => {}}>
                        blog
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink active={active == "run"} disabled on:click={() => {}}>
                        run
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink active={active == "projects"} disabled on:click={() => {}}>
                        projects
                    </NavLink>
                </NavItem>
                <NavItem>
                    <NavLink active={active == "contact"} disabled on:click={() => {}}>
                        contact
                    </NavLink>
                </NavItem>
            </Nav>
        </Collapse>
    </Navbar>

</main>

<style>
    #navbar-brand {
        font-weight: 700;
        color: #f6f2e6 !important;
    }
</style>