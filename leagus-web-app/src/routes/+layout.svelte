<script>
	import '../app.pcss';
	import { page } from '$app/stores';
	import { NavBrand, NavHamburger, NavLi, NavUl, Navbar } from 'flowbite-svelte';

	$: activeUrl = $page.url.pathname;
</script>

<Navbar>
	<NavBrand href="/">
		<img src="/images/flowbite-svelte-icon-logo.svg" class="me-3 h-6 sm:h-9" alt="Leagus Logo" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Leagus</span>
	</NavBrand>
	<NavHamburger />
	<NavUl {activeUrl}>
		<NavLi href="/">Home</NavLi>
		<NavLi href="/leagues">Leagues</NavLi>
		<NavLi href="/settings">Settings</NavLi>
	</NavUl>
</Navbar>

<slot />
