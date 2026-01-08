<script lang="ts">
    import { onMount } from "svelte";

    import "../app.css";

    onMount(async () => {
        // Remove splash screen when the content is mounted
        const splashScreen = document.getElementById("splash-screen");
        if (splashScreen) splashScreen.remove();
    })
</script>

<slot />