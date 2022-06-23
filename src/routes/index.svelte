<script lang="ts">
	import { browser } from '$app/env';

	import init, { encrypt, decrypt, get_cipher } from '$lib/aeslib/aeslib.js';
	import { encode as base64_encode, decode as base64_decode } from 'uint8-to-base64';

	let password = '';
	let right = '';
	let left = '';
	let size = '128';
	let direction: 'to' | 'from' = 'to';
	let ready = false;
	let error = '';
	let action: 'aes' | 'huffman' = 'aes';

	if (browser) {
		init().then(() => {
			ready = true;
		});
	}

	async function encode_huffman(text: string): Promise<string> {
		if (!text) {
			return '';
		}
		const response = await (
			await fetch('/api/encode', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					input: text
				})
			})
		).text();

		return response;
	}

	async function decode_huffman(text: string): Promise<string> {
		if (!text) {
			return '';
		}

		const response = await (
			await fetch('/api/decode', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					input: text
				})
			})
		).text();

		return response;
	}

	$: {
		if (ready) {
			if (action === 'aes') {
				if (direction === 'to') {
					right = base64_encode(encrypt(password, left, size));
					error = '';
				} else {
					try {
						left = decrypt(password, base64_decode(right), size);
						error = '';
					} catch (e) {
						if (typeof e === 'string') {
							error = e;
						} else if (e instanceof Error) {
							error = e.message;
						} else {
							error = 'Unknown error';
						}
					}
				}
			} else {
				(async () => {
					if (direction === 'to') {
						right = await encode_huffman(left);
					} else {
						left = await decode_huffman(right);
					}
				})();
			}
		}
	}
</script>

<div class="outer">
	<div>
		<label for="action">Action</label>
		<select id="action" bind:value={action}>
			<option value="aes">AES</option>
			<option value="huffman">Huffman</option>
		</select>
	</div>
	{#if action === 'aes'}
		<div>
			<label for="password">Password</label>
			<input name="password" type="text" bind:value={password} />
		</div>
		<div>
			<p>Cipher</p>
			{#if ready}
				{@html get_cipher(password, size).replace(/\n/g, '<br/>')}
			{/if}
		</div>
	{/if}

	<div class="side_layout">
		<div class="area">
			<label for="left">{action === 'aes' ? 'Decrypted' : 'Uncompressed'}</label>
			<textarea name="left" bind:value={left} />
			<p>Length: {left.length} Chars</p>
		</div>
		<div class="center">
			<button
				on:click={() => {
					direction = direction === 'from' ? 'to' : 'from';
				}}
			>
				{#if direction === 'to'}
					Change to {action === 'aes' ? 'decrypt' : 'decompress'}
				{:else}
					Change to {action === 'aes' ? 'encrypt' : 'compress'}
				{/if}
			</button>
			{#if action === 'aes'}
				<div class="size_picker">
					<label for="size">Size</label>
					<select name="size" bind:value={size}>
						<option value="128">128</option>
						<option value="192">192</option>
						<option value="256">256</option>
					</select>
				</div>
			{/if}
			{#if error}
				<div class="error">{error}</div>
			{/if}
		</div>
		<div class="area">
			<label for="right">{action === 'aes' ? 'Encrypted' : 'Compressed'}</label>
			<textarea name="right" bind:value={right} />
			<p>Length: {right.length} Chars, {base64_decode(right).length} Bytes</p>
		</div>
	</div>
</div>

<style>
	.side_layout {
		display: flex;
		justify-content: space-between;
		align-items: center;
		min-height: 300px;
	}
	.error {
		color: red;
		max-width: 150px;
	}
	.center {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 20px;
	}

	.outer {
		display: flex;
		flex-direction: column;
		gap: 20px;
		justify-content: center;
		padding: 20px;
		border: 1px solid #ccc;
		border-radius: 5px;
		box-shadow: 0 0 10px #ccc;
	}
	textarea {
		width: 100%;
		height: 200px;
		resize: none;
	}
	.area {
		height: 100%;
	}
	p {
		margin-top: 0px;
	}
</style>
