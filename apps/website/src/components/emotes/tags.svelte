<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";

	type Props = { tags?: string[] } & HTMLAttributes<HTMLDivElement>;

	let { tags = [], ...restProps }: Props = $props();
</script>

<div class="tags" {...restProps}>
	{#each tags as tag}
		<a class="tag" href="/emotes?t={tag}">{tag}</a>
	{/each}
</div>

<style lang="scss">
	.tags {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
		justify-content: center;
	}

	.tag {
		padding: 0.5rem 1rem;
		text-decoration: none;

		color: var(--text);
		font-size: 0.875rem;
		background-color: var(--secondary);
		border-radius: 0.5rem;

		&:hover,
		&:focus-visible {
			background-color: var(--secondary-hover);
		}
	}
</style>
