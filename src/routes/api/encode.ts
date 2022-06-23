import { get_request_body } from '$lib/backend/endpoint_utils';
import { java_port } from '$lib/backend/java_port';
import type { RequestHandler } from '@sveltejs/kit';
import type { JSONValue } from '@sveltejs/kit/types/private';

export const post: RequestHandler = async ({ request }) => {
	const body = await get_request_body(request, ['input']);
	if (body instanceof Error) {
		return {
			status: 400,
			body: {
				error: body.message
			}
		};
	}
	console.log(body)
	const response: JSONValue = await (
		await fetch(`http://localhost:${java_port}/encode`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: body?.input as string
		})
	).text();

	return {
		status: 200,
		body: response
	};
};
