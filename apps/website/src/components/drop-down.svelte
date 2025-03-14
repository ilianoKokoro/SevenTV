<script lang="ts" module>
	let dropDownIndex = 0;
</script>

<script lang="ts">
	import mouseTrap from "$/lib/mouseTrap";
	import { fade } from "svelte/transition";
	import type { HTMLAttributes } from "svelte/elements";
	import type { Snippet } from "svelte";

	type Props = {
		dropdown?: Snippet<[() => void]>;
		align?: "left" | "right";
		children?: Snippet;
		hover?: boolean;
	} & HTMLAttributes<HTMLDivElement>;

	let { dropdown, align = "right", children, hover, ...restProps }: Props = $props();

	const CLOSE_DELAY = 400; // ms

	let hideOnMobile = $state(false);
	let hideOnDesktop = $state(false);

	let index = dropDownIndex++;
	let hoverTimout: NodeJS.Timeout | undefined;

	let expanded = $state(false);

	function toggle(e: MouseEvent) {
		e.preventDefault();
		expanded = !expanded;
	}

	function handleMouseLeave() {
		if (!hover) {
			return;
		}
		hoverTimout = setTimeout(() => {
			expanded = false;
		}, CLOSE_DELAY);
	}

	function handleMouseEnter() {
		if (!hover) {
			return;
		}
		if (hoverTimout) {
			clearTimeout(hoverTimout);
		}
		expanded = true;
	}

	export function close() {
		expanded = false;
	}
</script>

<div
	class="dropdown"
	use:mouseTrap={close}
	onmouseenter={handleMouseEnter}
	onmouseleave={handleMouseLeave}
	class:hide-on-mobile={hideOnMobile}
	class:hide-on-desktop={hideOnDesktop}
	role="none"
>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- This is just a wrapper element to catch the underlying click event -->
	<div
		class="input-wrapper"
		onclick={toggle}
		aria-expanded={expanded}
		aria-controls="dropdown-list-{index}"
		{...restProps}
	>
		{@render children?.()}
	</div>
	{#if expanded}
		<div
			class="dropped"
			id="dropdown-list-{index}"
			transition:fade={{ duration: 100 }}
			style={align === "left" ? "left: 0" : "right: 0"}
		>
			{@render dropdown?.(close)}
		</div>
	{/if}
</div>

<style lang="scss">
	.dropdown {
		position: relative;

		.input-wrapper {
			display: flex;
			align-items: center;
			gap: 0.5rem;

			cursor: pointer;
		}
	}

	.dropped {
		z-index: 10;

		position: absolute;
		top: 100%;
		margin: 0;
		margin-top: 0.25rem;
		padding: 0;
		border: var(--border-active) 1px solid;
		border-radius: 0.5rem;

		background-color: var(--bg-medium);
		box-shadow: 4px 4px 8px rgba(0, 0, 0, 0.1);
	}
</style>
