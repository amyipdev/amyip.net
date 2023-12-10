<script lang="ts">
    import {tick} from "svelte";
    import FontFaceObserver from "fontfaceobserver";
    import init, {fit} from "amyip-net-shell";

    window.onscroll = () => window.scrollTo(0, 0);
    async function loadShell() {
        await tick();
        await new FontFaceObserver("Inconsolata").load();
        await init();
        window.onresize = () => fit();
        window.dispatchEvent(new Event("resize"));
    }
    loadShell();
</script>

<main>
    <p id="hacky-spacer"><br /></p>
    <div id="terminal"></div>
</main>

<style>
    @media (max-height: 720px) {
        #hacky-spacer {
            font-size: 0.1em !important;
        }
    }
    #hacky-spacer {
        font-size: 0.5em;
    }
    #terminal {
        height: 95vh;
        width: 82.5vw;
        margin: auto;
    }
</style>