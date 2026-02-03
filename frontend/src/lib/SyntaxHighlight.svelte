<script lang="ts">
	import Prism from 'prismjs';

	import 'prismjs/components/prism-markup.js';
	import 'prismjs/themes/prism-tomorrow.css';

	interface Props {
		code: string;
		language?: string;
		readonly?: boolean;
		class?: string;
	}

	let { code, language = 'html', readonly = true, class: className = '' }: Props = $props();
	let codeElement: HTMLElement;
	let isCopied = $state(false);

	let highlightedCode = $derived.by(() => {
		return Prism.highlight(code, Prism.languages[language] || Prism.languages.html, language);
	});

	function handleClick() {
		if (readonly) {
			const selection = window.getSelection();
			const range = document.createRange();
			range.selectNodeContents(codeElement);
			selection?.removeAllRanges();
			selection?.addRange(range);
		}
	}

	function copyToClipboard() {
		if (code) {
			isCopied = true;
			navigator.clipboard
				.writeText(code)
				.then(() => {
					console.log('Code copied to clipboard');
					setTimeout(() => {
						isCopied = false;
					}, 1500);
				})
				.catch((err) => {
					console.error('Failed to copy code: ', err);
					isCopied = false;
				});
		}
	}
</script>

<div class="relative {className}">
	<button
		onclick={copyToClipboard}
		class="absolute top-3 right-3 z-10 cursor-pointer rounded-lg border border-overlay0/50 p-2 shadow-sm backdrop-blur-sm transition-all duration-200 hover:scale-110 active:scale-95 {isCopied
			? 'bg-green/80 hover:bg-green'
			: 'bg-surface1/80 hover:bg-surface2'}"
		style="color: {isCopied
			? 'var(--color-mantle)'
			: 'var(--color-text)'}; transition: background-color 200ms ease-in-out, color 200ms ease-in-out;"
		title="Copy to clipboard"
	>
		{#if isCopied}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				class="transition-all duration-200"
			>
				<!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE -->
				<path
					fill="currentColor"
					d="m9.55 15.15l8.475-8.475q.3-.3.7-.3t.7.3t.3.713t-.3.712l-9.175 9.2q-.3.3-.7.3t-.7-.3L4.55 13q-.3-.3-.288-.712t.313-.713t.713-.3t.712.3z"
				/>
			</svg>
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				class="transition-all duration-200"
			>
				<!-- Icon from Material Symbols by Google - https://github.com/google/material-design-icons/blob/master/LICENSE -->
				<path
					fill="currentColor"
					d="M9 18q-.825 0-1.412-.587T7 16V4q0-.825.588-1.412T9 2h9q.825 0 1.413.588T20 4v12q0 .825-.587 1.413T18 18zm-4 4q-.825 0-1.412-.587T3 20V7q0-.425.288-.712T4 6t.713.288T5 7v13h10q.425 0 .713.288T16 21t-.288.713T15 22z"
				/>
			</svg>
		{/if}
	</button>
	<pre
		class="overflow-x-auto rounded-md border border-overlay0 !bg-surface0/60 px-3 py-2 transition-all duration-300 ease-in-out dark:border-overlay0"><code
			bind:this={codeElement}
			class="language-{language} !bg-transparent text-sm text-text dark:text-text"
			onclick={handleClick}
			aria-label="Code block"
			role={readonly ? 'textbox' : undefined}
			style="cursor: {readonly ? 'pointer' : 'default'}">{@html highlightedCode}</code
		></pre>
</div>

<style>
	:global(.token.tag) {
		color: var(--color-blue) !important;
	}

	:global(.token.attr-name) {
		color: var(--color-yellow) !important;
	}

	:global(.token.attr-value) {
		color: var(--color-green) !important;
	}

	:global(.token.string) {
		color: var(--color-green) !important;
	}

	:global(.token.punctuation) {
		color: var(--color-text) !important;
	}

	:global(.token.comment) {
		color: var(--color-overlay2) !important;
		font-style: italic;
	}

	:global(.token.keyword) {
		color: var(--color-mauve) !important;
	}

	:global(.token.operator) {
		color: var(--color-sky) !important;
	}

	:global(.token.number) {
		color: var(--color-peach) !important;
	}

	:global(.token.boolean) {
		color: var(--color-peach) !important;
	}

	:global(.token.function) {
		color: var(--color-blue) !important;
	}

	:global(.token.class-name) {
		color: var(--color-yellow) !important;
	}

	:global(.token.selector) {
		color: var(--color-red) !important;
	}

	:global(.token.property) {
		color: var(--color-blue) !important;
	}

	:global(.token.important) {
		color: var(--color-red) !important;
		font-weight: bold;
	}
</style>
