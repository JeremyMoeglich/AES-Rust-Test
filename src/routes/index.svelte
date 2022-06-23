<script lang="ts">
	import { encrypt } from 'aeslib';

	let value: string = '';
	let password: string = '';

	let encrypted: string = '';
</script>

<div>
	<div>
		<label for="password">password</label>
		<input name="password" type="password" bind:value={password} />
	</div>
	<div>
		<label for="value">value</label>
		<input name="value" type="text" bind:value />
	</div>
</div>

<button
	on:click={() => {
		encrypted = encrypt(password, value, '256').toString();
	}}>Encrypt</button
>

<div>
	<h3>Output</h3>
	<p>
		{encrypted}
	</p>
</div>
