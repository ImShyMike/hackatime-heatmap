<script lang="ts">
	import { onMount } from 'svelte';
	import SyntaxHighlight from '$lib/SyntaxHighlight.svelte';

	type Theme = 'light' | 'dark' | '' | 'catppuccin' | 'catppuccin_dark' | 'catppuccin_light';
	type PageTheme = 'light' | 'dark';
	type Mode = 'simple' | 'theme-aware';
	type ConfigMode = 'simple' | 'advanced';

	const defaults = {
		mode: 'theme-aware' as Mode,
		theme: '' as Theme,
		timezone: 'UTC',
		cellSize: 15,
		padding: 2,
		rounding: 50,
		ranges: [70, 30, 10]
	};

	let configMode: ConfigMode = $state('simple');
	let mode: Mode = $state('theme-aware');
	let id: string = $state('1');
	let theme: Theme = $state('light');
	let timezone: string = $state('UTC');
	let cellSize: number = $state(15);
	let padding: number = $state(2);
	let rounding: number = $state(50);
	let ranges: Array<number> = $state([70, 30, 10]);
	let rangesString: string = $state('');
	let useAutoTimezone: boolean = $state(false);

	let prefersDark: boolean = $state(false);
	let pageTheme: PageTheme = $state(
		(() => {
			if (typeof document !== 'undefined') {
				const savedTheme = localStorage.getItem('pageTheme');
				if (savedTheme === 'light' || savedTheme === 'dark') {
					return savedTheme;
				}
				return document.documentElement.classList.contains('mocha') ? 'dark' : 'light';
			}
			return 'light'; // SSR
		})()
	);

	let darkBackground: boolean = $derived.by(() => {
		return mode === 'theme-aware'
			? pageTheme === 'dark'
			: theme === 'dark' || theme === 'catppuccin_dark';
	});

	$effect(() => {
		if (typeof document !== 'undefined') {
			document.documentElement.classList.toggle('mocha', pageTheme === 'dark');
		}
	});

	onMount(() => {
		prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;

		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		const handleChange = (e: MediaQueryListEvent) => {
			if (!localStorage.getItem('pageTheme')) {
				pageTheme = e.matches ? 'dark' : 'light';
			}
		};
		mediaQuery.addEventListener('change', handleChange);

		return () => {
			mediaQuery.removeEventListener('change', handleChange);
		};
	});

	function toggleTheme() {
		pageTheme = pageTheme === 'light' ? 'dark' : 'light';
		localStorage.setItem('pageTheme', pageTheme);
	}

	function autoTimezone() {
		return Intl.DateTimeFormat().resolvedOptions().timeZone;
	}

	let url = $derived.by(() => {
		const params = new URLSearchParams();

		params.set('id', id);

		if (useAutoTimezone) {
			params.set('timezone', autoTimezone());
		} else if (timezone !== '' && timezone !== defaults.timezone) {
			params.set('timezone', timezone);
		}

		if (cellSize !== defaults.cellSize) {
			params.set('cell_size', cellSize.toString());
		}

		if (padding !== defaults.padding) {
			params.set('padding', padding.toString());
		}

		if (rounding !== defaults.rounding) {
			params.set('rounding', rounding.toString());
		}

		if (rangesString !== defaults.ranges.join(',')) {
			params.set('ranges', rangesString);
		}

		if (theme !== defaults.theme) {
			params.set('theme', theme);
		}

		return `https://heatmap.shymike.dev?${params.toString()}`;
	});

	$effect(() => {
		if (mode === 'simple' && (theme === '' || theme === 'catppuccin')) {
			theme = prefersDark ? 'dark' : 'light';
		} else if (
			mode === 'theme-aware' &&
			(theme === 'light' ||
				theme === 'dark' ||
				theme === 'catppuccin_light' ||
				theme === 'catppuccin_dark')
		) {
			theme = '';
		}
	});

	$effect(() => {
		rangesString = ranges.join(',');
	});

	function imageError(e: Event) {
		console.error('Failed to load image:', e);
	}

	let generatedHtml = $derived.by(() => {
		if (mode === 'theme-aware' && (theme === '' || theme === 'catppuccin')) {
			return `<a href="${url}&standalone=true" title="Click to view detailed data for each day!">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="${url}${theme === 'catppuccin' ? '_' : '&theme='}dark">
        <img alt="Hackatime activity heatmap" src="${url}${theme === 'catppuccin' ? '_' : '&theme='}light">
    </picture>
</a>`;
		} else {
			return `<a href="${url}&standalone=true" title="Click to view detailed data for each day!">
    <img alt="Hackatime activity heatmap" src="${url}">
</a>`;
		}
	});
</script>

<div class="min-h-screen bg-base transition-colors duration-500 ease-in-out">
	<div class="mx-auto max-w-4xl space-y-6 p-6">
		<div class="flex items-center justify-between">
			<h1 class="text-3xl font-bold text-text transition-colors duration-500 ease-in-out">
				Hackatime Heatmap
			</h1>
			<button
				class="group relative shrink-0 cursor-pointer overflow-hidden rounded-full border border-overlay0 bg-surface0 p-2 transition-all duration-400 ease-in-out hover:bg-surface1"
				onclick={toggleTheme}
				aria-label="Toggle Theme"
			>
				<div
					class="relative z-10 text-subtext1 transition-colors duration-400 ease-in-out group-hover:text-text"
				>
					{#if pageTheme === 'light'}
						<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24"
							><!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE --><path
								fill="currentColor"
								d="M12 21q-3.775 0-6.387-2.613T3 12q0-3.45 2.25-5.988T11 3.05q.325-.05.575.088t.4.362t.163.525t-.188.575q-.425.65-.638 1.375T11.1 7.5q0 2.25 1.575 3.825T16.5 12.9q.775 0 1.538-.225t1.362-.625q.275-.175.563-.162t.512.137q.25.125.388.375t.087.6q-.35 3.45-2.937 5.725T12 21"
							/></svg
						>
					{:else}
						<svg xmlns="http://www.w3.org/2000/svg" width="28" height="28" viewBox="0 0 24 24"
							><!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE --><path
								fill="currentColor"
								d="M12 17q-2.075 0-3.537-1.463T7 12t1.463-3.537T12 7t3.538 1.463T17 12t-1.463 3.538T12 17M2 13q-.425 0-.712-.288T1 12t.288-.712T2 11h2q.425 0 .713.288T5 12t-.288.713T4 13zm18 0q-.425 0-.712-.288T19 12t.288-.712T20 11h2q.425 0 .713.288T23 12t-.288.713T22 13zm-8-8q-.425 0-.712-.288T11 4V2q0-.425.288-.712T12 1t.713.288T13 2v2q0 .425-.288.713T12 5m0 18q-.425 0-.712-.288T11 22v-2q0-.425.288-.712T12 19t.713.288T13 20v2q0 .425-.288.713T12 23M5.65 7.05L4.575 6q-.3-.275-.288-.7t.288-.725q.3-.3.725-.3t.7.3L7.05 5.65q.275.3.275.7t-.275.7t-.687.288t-.713-.288M18 19.425l-1.05-1.075q-.275-.3-.275-.712t.275-.688q.275-.3.688-.287t.712.287L19.425 18q.3.275.288.7t-.288.725q-.3.3-.725.3t-.7-.3M16.95 7.05q-.3-.275-.288-.687t.288-.713L18 4.575q.275-.3.7-.288t.725.288q.3.3.3.725t-.3.7L18.35 7.05q-.3.275-.7.275t-.7-.275M4.575 19.425q-.3-.3-.3-.725t.3-.7l1.075-1.05q.3-.275.712-.275t.688.275q.3.275.288.688t-.288.712L6 19.425q-.275.3-.7.288t-.725-.288"
							/></svg
						>
					{/if}
				</div>
			</button>
		</div>

		<div class="space-y-4">
			<!-- Configuration Mode Toggle -->
			<div class="space-y-2">
				<label
					for="config-mode"
					class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
				>
					Configuration Mode:
				</label>
				<div
					id="config-mode"
					class="relative grid grid-cols-2 rounded-lg border border-overlay0 bg-surface0/40 p-1 transition-colors duration-500 ease-in-out dark:bg-surface0/60"
				>
					<div
						class="absolute inset-1 h-[calc(100%-0.5rem)] w-[calc(50%-0.25rem)] rounded-md bg-surface1 shadow-sm transition-all duration-300 ease-in-out {configMode ===
						'advanced'
							? 'translate-x-full'
							: 'translate-x-0'}"
					></div>

					<input
						type="radio"
						id="simple"
						bind:group={configMode}
						value="simple"
						class="peer/simple sr-only"
					/>
					<label
						for="simple"
						class="relative z-10 cursor-pointer px-4 py-2 text-center text-sm font-medium text-subtext0 transition-colors duration-300 ease-in-out peer-checked/simple:text-text"
					>
						Simple
					</label>

					<input
						type="radio"
						id="advanced"
						bind:group={configMode}
						value="advanced"
						class="peer/advanced sr-only"
					/>
					<label
						for="advanced"
						class="relative z-10 cursor-pointer px-4 py-2 text-center text-sm font-medium text-subtext0 transition-colors duration-300 ease-in-out peer-checked/advanced:text-text"
					>
						Advanced
					</label>
				</div>
			</div>

			<!-- User ID -->
			<div class="space-y-2">
				<label
					for="user-id"
					class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
				>
					User:
				</label>
				<input
					id="user-id"
					type="text"
					bind:value={id}
					placeholder="Slack/Hackatime ID"
					class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text placeholder-subtext0 shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
				/>
			</div>

			<!-- Mode -->
			<div class="space-y-2">
				<label
					for="mode-nselect"
					class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
				>
					Mode:
				</label>
				<select
					id="mode-select"
					bind:value={mode}
					class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
				>
					<option value="simple">Simple</option>
					<option value="theme-aware">Theme Aware</option>
				</select>
			</div>

			<!-- Theme -->
			<div class="space-y-2">
				<label
					for="theme-select"
					class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
				>
					Theme:
				</label>
				<select
					id="theme-select"
					bind:value={theme}
					class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
				>
					{#if mode === 'theme-aware'}
						<option value="">Auto</option>
						<option value="catppuccin">Auto Catppuccin</option>
					{:else}
						<option value="light">Light</option>
						<option value="dark">Dark</option>
						<option value="catppuccin_light">Catppuccin Light</option>
						<option value="catppuccin_dark">Catppuccin Dark</option>
					{/if}
				</select>
			</div>

			<!-- Advanced Options -->
			{#if configMode === 'advanced'}
				<!-- Timezone -->
				<div class="space-y-2">
					<label
						for="timezone"
						class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
					>
						Timezone:
					</label>
					<input
						id="timezone"
						type="text"
						bind:value={timezone}
						placeholder="UTC"
						readonly={useAutoTimezone}
						class="{useAutoTimezone
							? 'cursor-not-allowed bg-surface0/60 text-text/80 dark:bg-surface0/60'
							: 'bg-base text-text dark:bg-base'} w-full rounded-md border border-overlay0 px-3 py-2 placeholder-subtext0 shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none dark:placeholder-subtext0"
					/>
					<div class="flex items-center gap-2">
						<input
							type="checkbox"
							id="auto-timezone"
							bind:checked={useAutoTimezone}
							class="h-5 w-5 rounded border border-overlay0 bg-base text-blue accent-blue duration-300 ease-in-out [&:checked]:bg-blue/60"
						/>
						<label
							for="auto-timezone"
							class="text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
						>
							Use automatic timezone detection
						</label>
					</div>
				</div>

				<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
					<!-- Cell size -->
					<div class="space-y-2">
						<label
							for="cell-size"
							class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
						>
							Cell Size:
						</label>
						<input
							id="cell-size"
							type="number"
							bind:value={cellSize}
							min="5"
							max="50"
							class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
						/>
					</div>

					<!-- Padding -->
					<div class="space-y-2">
						<label
							for="padding"
							class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
						>
							Padding:
						</label>
						<input
							id="padding"
							type="number"
							bind:value={padding}
							min="0"
							max="10"
							class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
						/>
					</div>

					<!-- Rounding -->
					<div class="space-y-2">
						<label
							for="rounding"
							class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
						>
							Rounding:
						</label>
						<input
							id="rounding"
							type="number"
							bind:value={rounding}
							step="10"
							min="0"
							max="100"
							class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
						/>
					</div>
				</div>

				<!-- Ranges -->
				<div class="space-y-2">
					<label
						for="ranges"
						class="block text-sm font-medium text-subtext1 transition-colors duration-500 ease-in-out"
					>
						Ranges (3 values, comma-separated, descending):
					</label>
					<input
						id="ranges"
						type="text"
						bind:value={rangesString}
						oninput={() =>
							(ranges = rangesString
								.split(',')
								.map((n) => parseInt(n.trim()))
								.filter((n) => !isNaN(n)))}
						class="w-full rounded-md border border-overlay0 bg-base px-3 py-2 text-text placeholder-subtext0 shadow-sm transition-all duration-300 ease-in-out focus:border-transparent focus:ring-2 focus:ring-blue focus:outline-none"
						placeholder="e.g., 70,30,10"
					/>
				</div>
			{/if}
		</div>

		<!-- Generated -->
		<div class="space-y-2">
			<h2 class="text-lg font-semibold text-text transition-colors duration-500 ease-in-out">
				Generated:
			</h2>
			<SyntaxHighlight code={generatedHtml} language="html" readonly={true} class="w-full" />
		</div>

		<!-- Heatmap Preview -->
		<div class="space-y-2">
			<h2 class="text-lg font-semibold text-text transition-colors duration-500 ease-in-out">
				Preview:
			</h2>
			<div
				class="rounded-lg p-4 transition-all duration-500 ease-in-out"
				style:background-color={darkBackground
					? 'var(--color-github-dark)'
					: 'var(--color-github-light)'}
				style:border={darkBackground
					? '1px solid var(--color-github-border-dark)'
					: '1px solid var(--color-github-border-light)'}
			>
				<img
					src="{url}{theme === 'catppuccin'
						? '_' + pageTheme
						: theme === ''
							? '&theme=' + pageTheme
							: ''}"
					alt="Error loading heatmap preview..."
					onerror={imageError}
					class="h-auto max-w-full rounded-md text-red transition-opacity duration-300 ease-in-out"
				/>
			</div>
		</div>
	</div>
</div>
