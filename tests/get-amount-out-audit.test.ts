import { describe, expect, it } from "bun:test";
import { connect } from "near-api-js";
import * as pkg from "pkg";
import { z } from "zod";

describe("getAmountOut()", () => {
	it("should return the same value as the contract", async () => {
		const near = await connect({
			nodeUrl: "https://rpc.mainnet.near.org",
			networkId: "mainnet",
		});

		const POOL_ID = 1910; // it's a stable pool (USDC/USDT/DAI)
		const AMOUNT_IN = 100_000_000; // $100

		const response1 = await near.connection.provider.query({
			finality: "final",
			request_type: "call_function",
			account_id: "v2.ref-finance.near",
			method_name: "get_pool",
			args_base64: Buffer.from(JSON.stringify({ pool_id: POOL_ID })).toString(
				"base64",
			),
		});

		const callFunctionResponseSchema = z.object({
			result: z.array(z.number()),
		});
		const poolSchema = z.object({
			pool_kind: z.string(),
			token_account_ids: z.array(z.string()),
			amounts: z.array(z.string()),
			total_fee: z.number(),
			shares_total_supply: z.string(),
			amp: z.number(),
		});

		const pool = poolSchema.parse(
			JSON.parse(
				Buffer.from(
					callFunctionResponseSchema.parse(response1).result,
				).toString("utf-8"),
			),
		);

		const response2 = await near.connection.provider.query({
			blockId: response1.block_hash,
			request_type: "call_function",
			account_id: "v2.ref-finance.near",
			method_name: "get_return",
			args_base64: Buffer.from(
				JSON.stringify({
					pool_id: POOL_ID,
					token_in: pool.token_account_ids[0],
					amount_in: AMOUNT_IN.toString(),
					token_out: pool.token_account_ids[1],
				}),
			).toString("base64"),
		});

		const expectedAmountOut = z
			.string()
			.parse(
				JSON.parse(
					Buffer.from(
						callFunctionResponseSchema.parse(response2).result,
					).toString("utf-8"),
				),
			);

		const amountOut1 = pkg.getAmountOut(
			Uint8Array.from([6, 6, 18]),
			pool.amounts.map((a) => BigInt(a)),
			BigInt(pool.amp),
			pool.total_fee,
			0,
			1,
			BigInt(AMOUNT_IN),
		);

		expect(amountOut1).toEqual(BigInt(expectedAmountOut));
	});
});
