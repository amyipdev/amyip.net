<script lang="ts">
    import {tick} from "svelte";
    import {Terminal} from 'xterm';
    import {FitAddon} from 'xterm-addon-fit';
    import {safeId} from "./tools";
    const term = new Terminal({
        fontFamily: "Inconsolata"
    });
    const fa = new FitAddon();
    term.loadAddon(fa);
    async function loadShell() {
        await tick();
        term.open(safeId("xterm-container"));
        term.write("Hello");
        window.addEventListener("resize", () => fa.fit());
        await tick();
        fa.fit();
        window.onscroll = () => window.scrollTo(0, 0);
    }
    loadShell();
</script>

<main>
    <div id="xterm-container-bs">
        <div id="xterm-container"></div>
    </div>
</main>

<style>
    #xterm-container {
        height: 95vh;
        width: 75vw;
        margin: auto;
    }
</style>