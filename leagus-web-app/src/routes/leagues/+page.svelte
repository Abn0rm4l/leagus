<script>
	import {
		Breadcrumb,
		BreadcrumbItem,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		TableSearch
	} from 'flowbite-svelte';

	let searchTerm = '';

	// TODO: Get them items from the API.
	let items = [
		{ name: 'Tuesday Social', season: 'February 2024' },
		{ name: 'Friday Chill Vibes', season: 'Jan-April 2024' },
		{ name: 'Wednesday Ladies', season: '2024' }
	];

	// TODO: Fuzzy match?
	$: filteredItems = items.filter(
		(item) => item.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);
</script>

<Breadcrumb class="mb-4">
	<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
	<BreadcrumbItem>Leagues</BreadcrumbItem>
</Breadcrumb>

<TableSearch placeholder="Search by name" hoverable={true} bind:inputValue={searchTerm}>
	<caption
		class="bg-white p-5 text-left text-lg font-semibold text-gray-900 dark:bg-gray-800 dark:text-white"
	>
		Active Leagues
		<p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">
			Search the list of active leagues.
		</p>
	</caption>
	<TableHead>
		<TableHeadCell>League</TableHeadCell>
		<TableHeadCell>Current Season</TableHeadCell>
	</TableHead>
	<TableBody class="divide-y">
		{#each filteredItems as item}
			<TableBodyRow>
				<TableBodyCell>{item.name}</TableBodyCell>
				<TableBodyCell>{item.season}</TableBodyCell>
			</TableBodyRow>
		{/each}
	</TableBody>
</TableSearch>
