<script lang="ts">
    import TitleBar from "../components/TitleBar.svelte";
    import { onMount } from "svelte";

    import "../app.css";

    onMount(async () => {
        // Remove splash screen when the content is mounted
        const splashScreen = document.getElementById("splash-screen");
        if (splashScreen) splashScreen.remove();
    })
</script>

<TitleBar />
<slot />