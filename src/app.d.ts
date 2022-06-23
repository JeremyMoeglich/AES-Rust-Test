/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare namespace App {
	// interface Locals {}
	// interface Platform {}
	// interface Session {}
	// interface Stuff {}
}

declare module 'uint8-to-base64' {
	function encode(uint8: Uint8Array): string;
	function decode(base64: string): Uint8Array;
}
