<script lang="ts">
	import { page } from "$app/stores";
	import { type Snippet } from "svelte";
	import Button from "./input/button.svelte";
	import { type Page } from "@sveltejs/kit";

	interface Props {
		title?: string;
		href?: string;
		big?: boolean;
		responsive?: boolean;
		matcher?: (currentPage: Page, href?: string) => boolean;
		children?: Snippet;
		active?: Snippet;
		iconRight?: Snippet;
		onclick?: () => void;
	}

	function defaultMatcher(currentPage: Page, href?: string): boolean {
		return currentPage.url.pathname === href;
	}

	let {
		title,
		href,
		big = false,
		responsive = false,
		matcher = defaultMatcher,
		children,
		active,
		iconRight: tabLinkIconRight,
		onclick,
	}: Props = $props();

	function scrollIntoView(e: MouseEvent) {
		if (e.target instanceof HTMLElement) {
			e.target.scrollIntoView({ behavior: "smooth", block: "center", inline: "center" });
		}
		onclick?.();
	}

	let isActive = $derived(matcher($page, href));
</script>

{#if responsive}
	<Button
		{href}
		{big}
		secondary={isActive}
		draggable="false"
		style={isActive ? null : "color: var(--text-light)"}
		onclick={scrollIntoView}
		hideOnMobile
		iconRight={tabLinkIconRight}
		icon={isActive ? active : children}
	>
		{#if tabLinkIconRight}
			<span style="flex-grow: 1">{title}</span>
		{:else}
			{title}
		{/if}
	</Button>
	<Button
		{href}
		{big}
		secondary={isActive}
		draggable="false"
		style={isActive ? null : "color: var(--text-light)"}
		onclick={scrollIntoView}
		hideOnDesktop
		icon={isActive ? active : children}
	>
		{#if isActive}
			{title}
		{/if}
	</Button>
{:else}
	<Button
		{href}
		{big}
		secondary={isActive}
		draggable="false"
		style={isActive ? null : "color: var(--text-light)"}
		onclick={scrollIntoView}
		iconRight={tabLinkIconRight}
		icon={isActive ? active : children}
	>
		{#if tabLinkIconRight}
			<span style="flex-grow: 1">{title}</span>
		{:else}
			{title}
		{/if}
	</Button>
{/if}
