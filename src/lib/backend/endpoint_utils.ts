import { hasProperty } from 'functional-utilities';
import type { JsonValue } from 'type-fest';

export async function get_request_body<T extends string>(
	request: Request,
	attrs: T[]
): Promise<Record<T, unknown> | undefined> {
	const body = await get_body(request);
	if (attrs.every((attr) => hasProperty(body, attr))) {
		return body as Record<T, unknown>;
	} else {
		return undefined;
	}
}

export async function get_body(request: Request): Promise<JsonValue | Error> {
	const decoded_body = await request.text();
	const body: JsonValue = JSON.parse(decoded_body.trim() ? decoded_body : '{}');
	if (typeof body !== 'object') {
		return new Error('Invalid body');
	}
	return body;
}