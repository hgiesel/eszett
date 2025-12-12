<script lang="ts">
	import Counter from './Counter.svelte';
	import welcome from '$lib/images/svelte-welcome.webp';
	import welcomeFallback from '$lib/images/svelte-welcome.png';

	import { Counter as CounterC } from "@hgiesel/eszett-client-web"
	import * as f from "@hgiesel/eszett-client-web"
	import * as g from "@hgiesel/eszett-dto"
	import { onMount } from 'svelte';


	function myfun2(x: g.LanguageDto.English) {

	}

	function myfun(x: g.LanguageDto) {
		switch (x) {
			case "Arabic":
				break

			case 'French':
			case 'Spanish':
			case 'Italian':
			case 'German':
			case 'Mandarin':
			case 'Japanese':
		}

	}
	console.log('--- g:')
	onMount(async () => {
		f.Foobar.English
		console.log(await f.query_foo("hgiesel/eszett"))
	})

	function createCounter() {
		let counter = $state(new CounterC());
		const value = $derived.by(() => counter.value);

		return { 
			increment() {
				counter.increment()
				counter = counter
				console.log(value)
			},
			get value() {
				return value
			},
		};
	}

	function createCounter3() {
		let value = $state(0);

		function increment() {
			value++
		}

		return { 
			get value() { return value },
			increment
		}
	}

	const c = createCounter()

	const c3 = createCounter3()
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	Let's see what shows:{c.value}
	<button onclick={() => c.increment()}>
		Increment!
	</button>

	Let's see what shows:{c3.value}
	<button onclick={() => c3.increment()}>
		Increment!
	</button>

	<h1>
		<span class="welcome">
			<picture>
				<source srcset={welcome} type="image/webp" />
				<img src={welcomeFallback} alt="Welcome" />
			</picture>
		</span>

		to your new<br />SvelteKit app
	</h1>

	<h2>
		Try editing <strong>src/routes/+page.svelte</strong>
	</h2>


	<Counter />
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
	}

	h1 {
		width: 100%;
	}

	.welcome {
		display: block;
		position: relative;
		width: 100%;
		height: 0;
		padding: 0 0 calc(100% * 495 / 2048) 0;
	}

	.welcome img {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		display: block;
	}
</style>