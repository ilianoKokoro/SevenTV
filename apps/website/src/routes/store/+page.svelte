<script lang="ts">
	import Spinner from "$/components/spinner.svelte";
	import BadgeProgressComponent from "$/components/store/badge-progress.svelte";
	import Banner from "$/components/store/banner.svelte";
	import Benefits from "$/components/store/benefits.svelte";
	import MonthlyPaints from "$/components/store/monthly-paints.svelte";
	import PersonalEmotes from "$/components/store/personal-emotes.svelte";
	import YourSub from "$/components/store/your-sub.svelte";
	import { graphql } from "$/gql";
	import {
		type BadgeProgress,
		type MyStoreDataQuery,
		type Paint,
		type StoreDataQuery,
		type SubscriptionInfo,
		type SubscriptionProduct,
	} from "$/gql/graphql";
	import { gqlClient } from "$/lib/gql";
	import { Gift, Info, PaintBrush, Seal, Smiley, UserCircle } from "phosphor-svelte";
	import { t } from "svelte-i18n";
	import { user } from "$/lib/auth";
	import { PUBLIC_SUBSCRIPTION_PRODUCT_ID } from "$env/static/public";
	import type { PageData } from "./$types";
	import { isXmasEvent } from "$/lib/xmas";

	let { data }: { data: PageData } = $props();

	async function queryMyStore(userId: string) {
		let res = await gqlClient()
			.query(
				graphql(`
					query MyStoreData($userId: Id!, $productId: Id!) {
						users {
							user(id: $userId) {
								billing(productId: $productId) {
									badgeProgress {
										currentBadge {
											id
											name
											description
											images {
												url
												mime
												size
												scale
												width
												height
												frameCount
											}
										}
										nextBadge {
											badge {
												id
												name
												images {
													url
													mime
													size
													scale
													width
													height
													frameCount
												}
											}
											percentage
											daysLeft
										}
									}
									subscriptionInfo {
										totalDays
										endDate
										activePeriod {
											subscriptionProductVariant {
												kind
											}
											subscription {
												state
											}
											end
											autoRenew
											giftedBy {
												id
												mainConnection {
													platformDisplayName
												}
												style {
													activePaint {
														id
														name
														data {
															layers {
																id
																ty {
																	__typename
																	... on PaintLayerTypeSingleColor {
																		color {
																			hex
																		}
																	}
																	... on PaintLayerTypeLinearGradient {
																		angle
																		repeating
																		stops {
																			at
																			color {
																				hex
																			}
																		}
																	}
																	... on PaintLayerTypeRadialGradient {
																		repeating
																		stops {
																			at
																			color {
																				hex
																			}
																		}
																		shape
																	}
																	... on PaintLayerTypeImage {
																		images {
																			url
																			mime
																			size
																			scale
																			width
																			height
																			frameCount
																		}
																	}
																}
																opacity
															}
															shadows {
																color {
																	hex
																}
																offsetX
																offsetY
																blur
															}
														}
													}
												}
												highestRoleColor {
													hex
												}
											}
										}
									}
								}
							}
						}
					}
				`),
				{
					userId,
					productId: PUBLIC_SUBSCRIPTION_PRODUCT_ID,
				},
			)
			.toPromise();

		return res.data;
	}

	async function queryStore() {
		let res = await gqlClient()
			.query(
				graphql(`
					query StoreData($productId: Id!) {
						products {
							subscriptionProduct(id: $productId) {
								id
								name
								variants {
									id
									price {
										amount
										currency
									}
									kind
								}
							}
						}
						store {
							monthlyPaints {
								id
								name
								data {
									layers {
										id
										ty {
											__typename
											... on PaintLayerTypeSingleColor {
												color {
													hex
												}
											}
											... on PaintLayerTypeLinearGradient {
												angle
												repeating
												stops {
													at
													color {
														hex
													}
												}
											}
											... on PaintLayerTypeRadialGradient {
												repeating
												stops {
													at
													color {
														hex
													}
												}
												shape
											}
											... on PaintLayerTypeImage {
												images {
													url
													mime
													size
													scale
													width
													height
													frameCount
												}
											}
										}
										opacity
									}
									shadows {
										color {
											hex
										}
										offsetX
										offsetY
										blur
									}
								}
							}
						}
					}
				`),
				{
					productId: PUBLIC_SUBSCRIPTION_PRODUCT_ID,
				},
			)
			.toPromise();

		return res.data;
	}

	let storeData = $state<StoreDataQuery>();
	let myStoreData = $state<MyStoreDataQuery>();

	$effect(() => {
		if ($user) {
			queryMyStore($user.id).then((res) => {
				myStoreData = res;
			});
		}

		queryStore().then((res) => {
			storeData = res;
		});
	});
</script>

<svelte:head>
	<title>{$t("common.subscriptions", { values: { count: 1 } })} - {$t("page_titles.suffix")}</title>
</svelte:head>

{#snippet banner(subbed: boolean)}
	<Banner
		title={subbed
			? $t("pages.store.subscription.banner_title_subbed")
			: $t("pages.store.subscription.banner_title_unsubbed")}
		subtitle={subbed
			? $t("pages.store.subscription.banner_subtitle_subbed")
			: $t("pages.store.subscription.banner_subtitle_unsubbed")}
	>
		<div class="banner-icons hide-on-mobile">
			<PaintBrush size="1.8rem" />
			<UserCircle size="1.8rem" />
			<Seal size="1.8rem" />
			<Smiley size="1.8rem" />
		</div>
	</Banner>
{/snippet}

{#if myStoreData}
	{@render banner(!!myStoreData?.users.user?.billing.subscriptionInfo.activePeriod)}
{:else}
	{@render banner(false)}
{/if}
<!-- All things called grid here aren't actually css grids -->
<div class="grid">
	{#if data.success}
		<div class="bar">
			<Info />
			Your purchase was successfully completed
		</div>
	{/if}
	{#if data.redeemSuccess}
		<div class="bar">
			<Info />
			Your successfully redeemed your trial subscription
		</div>
	{/if}
	{#if isXmasEvent()}
		<div class="bar">
			<Gift />
			X-MAS 2024 EVENT: GIFT 1 SUB TO GET A SPECIAL BADGE
		</div>
	{/if}

	{#if myStoreData}
		{#if !myStoreData.users.user?.billing.subscriptionInfo.activePeriod}
			<Benefits />
		{/if}
	{:else}
		<Benefits />
	{/if}
	<div class="top-grid">
		{#if storeData}
			{#if myStoreData}
				<div class="subgrid">
					{#if myStoreData.users.user && storeData.products.subscriptionProduct}
						<YourSub
							bind:subInfo={myStoreData.users.user.billing.subscriptionInfo as SubscriptionInfo}
							product={storeData.products.subscriptionProduct as SubscriptionProduct}
						/>
						<BadgeProgressComponent
							progress={myStoreData.users.user.billing.badgeProgress as BadgeProgress}
						/>
					{/if}
				</div>
			{/if}
			<MonthlyPaints paints={storeData.store.monthlyPaints as Paint[]} />
		{:else}
			<div class="spinner-container">
				<Spinner />
			</div>
		{/if}
	</div>
	<PersonalEmotes />
	{#if myStoreData?.users.user?.billing.subscriptionInfo.activePeriod}
		<Benefits />
	{/if}
</div>

<style lang="scss">
	.banner-icons {
		padding: 0 2.75rem;

		display: flex;
		gap: 3.5rem;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
	}

	.grid {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		flex-wrap: wrap;

		max-width: 70rem;
		margin-top: 1rem;
		margin-inline: auto;
	}

	.bar {
		background-color: var(--bg-light);
		color: var(--text);
		padding: 0.5rem;
		border-radius: 0.25rem;
		border: 1px solid var(--store);

		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
	}

	.top-grid {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;

		& > .subgrid {
			flex-grow: 1;

			display: flex;
			flex-direction: column;
			gap: 1rem;
			flex-wrap: wrap;
		}
	}

	.spinner-container {
		margin: 0 auto;
	}
</style>
