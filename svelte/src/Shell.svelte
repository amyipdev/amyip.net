<script lang="ts">
    import {tick} from "svelte";
    import {Terminal} from 'xterm';
    import {FitAddon} from 'xterm-addon-fit';
    import {safeId} from "./tools";
    const term = new Terminal();
    const fa = new FitAddon();
    term.loadAddon(fa);
    async function loadShell() {
        await tick();
        term.open(safeId("xterm-container"));
        fa.fit();
    }
    loadShell();
</script>

<main>
    <div id="xterm-container-bs">
        <div id="xterm-container"></div>
    </div>
</main>

<style>
</style>