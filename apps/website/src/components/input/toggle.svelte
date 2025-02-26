<script lang="ts">
	import type { Snippet } from "svelte";
	import type { HTMLLabelAttributes } from "svelte/elements";

	type Props = {
		value?: boolean;
		disabled?: boolean;
		onclick?: (e: MouseEvent) => void;
		leftLabel?: Snippet;
		children?: Snippet;
	} & HTMLLabelAttributes;

	let {
		value = $bindable(false),
		disabled = false,
		onclick,
		leftLabel,
		children,
		...restProps
	}: Props = $props();

	function handleClick(e: MouseEvent) {
		e.stopPropagation();
		onclick?.(e);
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<label {...restProps} onclick={handleClick}>
	{@render leftLabel?.()}
	<span class="switch">
		<input type="checkbox" {disabled} bind:checked={value} />
		<span class="slider"></span>
	</span>
	{@render children?.()}
</label>

<style lang="scss">
	label {
		display: flex;
		gap: 0.5rem;

		cursor: pointer;
	}

	/* The switch - the box around the slider */
	.switch {
		flex-shrink: 0;

		position: relative;
		display: inline-block;
		width: 2.25rem;
		height: 1.25rem;

		/* Hide default HTML checkbox */
		input {
			opacity: 0;
			width: 0;
			height: 0;
		}
	}

	/* The slider */
	.slider {
		cursor: pointer;
		background-color: var(--border-active);
		border-radius: 0.75rem;

		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;

		transition: background-color 0.2s;
	}

	.slider:before {
		content: "";
		height: 1rem;
		width: 1rem;
		border-radius: 50%;

		position: absolute;
		left: 0.125rem;
		bottom: 0.125rem;

		background-color: var(--primary-text);
		transition: transform 0.2s;
	}

	input:focus-visible + .slider {
		outline: 1px solid var(--primary);
	}

	input:checked + .slider {
		background-color: var(--primary);
	}

	input:disabled + .slider {
		cursor: not-allowed;
	}

	input:checked:disabled + .slider {
		background-color: var(--primary-disabled);
	}

	input:checked + .slider:before {
		transform: translateX(1rem);
	}

	input:disabled + .slider::before {
		background-color: var(--text-light);
	}
</style>
