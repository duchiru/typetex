<script lang="ts">
	import { XIcon, SquareIcon, MinusIcon } from "@lucide/svelte";

	import { getCurrentWindow } from "@tauri-apps/api/window";

    const appWindow = getCurrentWindow();
</script>

<div class="title-bar grid grid-cols-3 h-8">
	<div class="title-bar-left"></div>
	<div class="title-bar-center"></div>
	<div class="title-bar-right">
		<button onclick={() => appWindow.close()} class="title-bar-btn w-11 hover:bg-red-500">
			<XIcon size={20} strokeWidth={1.5} />
		</button>
		<button onclick={() => appWindow.toggleMaximize()} class="title-bar-btn w-11 hover:bg-gray-500">
			<SquareIcon size={16} strokeWidth={1.75} />
		</button>
		<button onclick={() => appWindow.minimize()} class="title-bar-btn w-11 hover:bg-gray-500">
            <MinusIcon size={18} strokeWidth={1.75} />
        </button>
	</div>
</div>

<style lang="sass postcss">
	.title-bar {
		-webkit-app-region: drag;
		user-select: none;
	}

	.title-bar .title-bar-right {
		display: flex;
		flex-direction: row-reverse;
	}

	.title-bar .title-bar-right .title-bar-btn {
		-webkit-app-region: no-drag;
		display: flex;
		align-items: center;
		justify-content: center;
	}
</style>
