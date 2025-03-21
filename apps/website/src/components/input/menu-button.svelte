<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLButtonAttributes, HTMLAnchorAttributes } from "svelte/elements";

	type Props = {
		href?: string;
		iconRight?: Snippet;
		hideOnMobile?: boolean;
		hideOnDesktop?: boolean;
		children?: Snippet;
	} & HTMLAnchorAttributes &
		HTMLButtonAttributes;

	let {
		href,
		iconRight = undefined,
		hideOnMobile = false,
		hideOnDesktop = false,
		children,
		...restProps
	}: Props = $props();
</script>

{#if href}
	<a
		{href}
		class:hide-on-mobile={hideOnMobile}
		class:hide-on-desktop={hideOnDesktop}
		{...restProps}
	>
		{@render children?.()}
		{#if iconRight}
			<div class="icon-right">
				{@render iconRight()}
			</div>
		{/if}
	</a>
{:else}
	<button
		type="button"
		class:hide-on-mobile={hideOnMobile}
		class:hide-on-desktop={hideOnDesktop}
		{...restProps}
	>
		{@render children?.()}
		{#if iconRight}
			<div class="icon-right">
				{@render iconRight()}
			</div>
		{/if}
	</button>
{/if}

<style lang="scss">
	a,
	button {
		padding: 0.8rem 0.75rem;
		border-radius: 0.5rem;
		color: var(--text);
		font-size: 0.875rem;
		font-weight: 500;
		text-decoration: none;

		display: flex;
		align-items: center;
		gap: 0.8rem;

		&:hover,
		&:focus-visible {
			background-color: var(--bg-light);
			text-decoration: none;
		}
	}

	.icon-right {
		flex-grow: 1;
		justify-self: end;
		text-align: right;
	}

	@media screen and (max-width: 960px) {
		a,
		button {
			padding: 1rem;
			font-size: 1rem;
			gap: 0.75rem;
		}
	}
</style>
