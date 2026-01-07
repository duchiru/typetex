<script lang="ts">
    import { onMount } from "svelte";

    onMount(async () => {
        // Remove splash screen when the content is mounted
        const splashScreen = document.getElementById("splash-screen");
        if (splashScreen) splashScreen.remove();
    })
</script>

<slot />