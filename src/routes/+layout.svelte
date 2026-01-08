<script lang="ts">
    import TitleBar from "../components/TitleBar.svelte";
    import { onMount } from "svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    import "../app.css";

    onMount(async () => {
        // Show the window when the layout is mounted (to avoid white flash)
        await getCurrentWindow().show();
    })
</script>

<TitleBar />
<slot />